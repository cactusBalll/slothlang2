# 实现状态对照（截至 2026-09-14，commit cecfab7）

参照《sloth-lang 2.0 设计文档》（本仓库 `sloth-lang-2.0设计文档.md`，基准版本 2026-09-08）逐节核对当前代码实现。

## 环境 / 构件

- MLIR/LLVM **21.1**（`/usr/lib/llvm-21`），`mlir-sys = "=210.0.4"`；构建需 `MLIR_SYS_210_PREFIX=/usr/lib/llvm-21`。
- 工作区 4 个 crate：`sloth-frontend`（lexer 463 + parser 1007 + ty 154 + ast 322 行）、`sloth-codegen`（irgen 2152 行文本式发射 + jit/pass/context/module）、`sloth-rt`（208 行 cdylib：print/panic/gc_alloc/strpool/对象 rt）、`slothc` CLI。
- 测试：codegen 18（`pass.rs`，均为 JIT 端到端）+ frontend 20 = **38 全绿**。
- 未创建自定义 sloth dialect；IR 为文本拼接（`func/arith/memref/cf` + `call @sloth_rt_*`），随后 `normalize_indices` 把索引常量改写为 `index` 类型（作用域感知，按 func 分块）。

## 对照路线图（§8）

| 阶段 | 状态 | 说明 |
| --- | --- | --- |
| P0 语言定稿 | 部分跳过 | 无 200 条规范用例集；语法以 §3.9 EBNF 落地（`tests/`、`examples/` 为空） |
| P1 前端 | **基本完成（弱化版）** | lexer/parser/AST 完整，20 个 parse 测试；§4.2 的名称解析、两遍类型检查、trait 约束求解、is 收窄、单态化均**未实现**——仅 `ty.rs` 局部 Reg 类型缓存与 irgen 内失败即报 `Diag{line,col,msg}`（fail-fast 于 codegen 阶段） |
| P2 MLIR 端到端 MVP | **完成** | 算术/控制流/函数/字符串 JIT + AOT 全通（commit 7280a77 → d4921f3）；偏差：无 sloth dialect 中间层，直接出 func/arith/memref/cf 文本 |
| P3 对象与闭包 | **基本完成** | class（单模块 + 跨模块）字段/方法/ctor 可跑（63e0413、a358058）；闭包快照捕获完成（cecfab7）；字符串池/插值完成；继承 `class A: B` + `super.__init__`/`super.x`(读写)/`super.m()` 静态直调完成（patch #2，字段槽基类优先布局，方法沿链解析到定义类符号）；**trait MVP + dyn 虚表分派完成**（patch #4，用户要求从 cls_id 链升级为真虚表）：`impl` known-trait/方法存在/arity/ABI 校验、(trait,method) 全局槽位全局编址、对象头 word1 存 vt 指针、vt=[cap, slot0..] 为 i64 fn ptr 数组、槽值 = `llvm.mlir.addressof`+`llvm.ptrtoint`、分派 = `sloth_vt_get` + `llvm.inttoptr` + 间接 `llvm.call`（形如 `llvm.call %vp(args): !llvm.ptr, (i64) ret`）、`llvm.func` 混合 func 方言体方法直接成为 vt 项（无 thunk 层）、静态调用点对 llvm 标记方法发 `llvm.call`、无满足槽时 `sloth_vt_get`=0 → `sloth_panic_noimpl`；测试 47 全绿，JIT/AOT 双路一致。**未做**：`is` 类型测试、trait 默认方法体、异构容器 `Array<dyn T>`、vtable 全局缓存（每对象重建） |
| P4 泛型与单态化 | **部分推进** | parser 支持 `type_params`；**容器运行时（Array MVP）完成**（patch #5）：`sloth_arr_new/get(_f64)/set(_f64)/len`（rt [len, e0..] 布局 + 越界 panic）、`[...]` 字面量发射（元素统一：均匀取元素类型 / 混浮点归一为 Array(f64)/对象字为词）、`a[i]` 读、`a[i] = v` 单层索引赋值、`len(a)` 数组长度、`for x in arr` 迭代（槽式计数器 + FeTy 元素取词）；`Array<dyn T>` 元素走 dyn 分派 ✓。**未做**：Map 运行时（仅 `@(k: v)` 解析）、数组 push/append（字面量定长 MVP）、`Iterator`/`Iterable` trait 协议、泛型单态化 |
| P5 模块与标准库 | **核心完成** | 编译期 `import`（递归、canonicalize 去重、`as` 别名、`mod.fn()`/`mod.g` 限定调用/读取，ec5a2c7 + a7a8b0d）；`pub var` 全局单元格 `memref.global @{mod}_g_{name} {mutable}` + 每模块 `__ginit` 启动时调用；类跨模块可见。**patch #7**：`pub` 强制（parser 记录 `Decl.visible`；register_import 对非 pub fn/var/class 记 `hidden` 集，限定访问点 guard 报 "`x` is private to its module"；compile_multimod 补 diags 检查——此前 codegen diags 被静默吞掉）；循环依赖检测（resolve 栈 + done 集，报 `circular import: a.sl -> b.sl -> a.sl`）。**缺**：标准库 |
| P1 补遗 | **部分** | `is`/`is not`/`nil` 落地（patch #7）：`nil` 作为 primary（lexer 产出 Ident 因由记工程债 11）；`ctx is nil` = cmpi eq 0、`x is C` = 静态成员集（C 及 class_order 中可达 C 的后代）对 `sloth_obj_cls_id` 逐 id or 链、结果 Ty::Bool（print 走 print_bool）；`?:` Elvis = cmpi ne + `arith.select`（仅 i64/对象词，float/str 报错；RHS 无条件求值 MVP）；测试 irgen_p4：priv/pub 跨模块、is 链（Animal/Cat/Robot）、elvis、circular_diag | |
| P6 精确 GC | **未开始 / 替代路径** | 采用自研 `sloth_gc_alloc`（sloth-rt 内标记-清理式 stub，非 Boehm、非 statepoint）；`O = [info_ptr, cls_id, fields...]` 布局，`sloth_obj_field/set_field` 手工寻址（偏移 +2） |

## 设计文档关键决策的偏离（记录在案）

1. **无 sloth dialect（§4.3）**：直接文本发射 func/arith/memref/cf；§4.3.2 的 `sloth.call_virtual/type_test/closure_create` 等以直接 rt 调用 + 静态符号分派替代。
2. **可空 `T?`（§2.6）**：`Type::Optional` 解析为 `Ty::Opt`，irgen 中数值一律 i64/f64 词宽，nil=0；值类型可空的 tagged 布局未实现。
3. **闭包（§2.6）**：设计为 环境+函数指针两字；实现为 GC frame 对象（词数组）+ 静态符号，捕获为值快照（逃逸安全，可变捕获渲染为创建时值，非设计语义中的更新共享）。
4. **GC（§5.1）**：未用 Boehm；sloth-rt 自带简化 gc_alloc。对象头不含字段指针位图/祖先表。
5. **类型系统（§2）**：无静态强类型 enforced —— `var x = 1; x = "s"` 不会被报错；推断只有 codegen 期的 is_float/i64 分派。
6. **extern/FFI（§5.4）**：未实现。
7. **字符串插值**：编译期展开 rt str_push/concat，未做 `Display` trait 约束。
8. **lambda 解析语法**：`|x: int| -> { ... }`（§3.9 的 `( '->' type )? block`），未注参数默认 int。

## CLI 现状（`slothc`）

- `slothc check|ir|run|build file.sl [out]`；`run`=JIT（llvmlite pass 管线 canonicalize→cse→func/arith/index/cf→finalize-memref→reconcile + `Engine::invoke("sloth_main")`），`build`=mlir-opt→mlir-translate→clang 链接 libsloth_rt.so 的 AOT（@main shim 注入）。
- **JIT 与 AOT 输出一致已验证**（multimod/lambda/object 用例）。

## 已知工程债 / 风险

1. irgen 仍是**文本拼接 + 事后 normalize_indices 正规化**：新 op 类型（如 index以外的 memref 类型、f64 全）需逐处验证；跨模块 `%vN` 名冲突历史 bug 即源于全局索引改写。
2. `pass.rs` 测试追加多次出现 `#[test]`/函数头重复的编辑事故（python str.replace 锚点），每次需人工修复——建议改为 append-only 或 fixture 文件驱动。
3. `emit_call` 分支顺序复杂（ctor→方法→本地函数→lambda→跨模块→builtin），限定名 `lib.fn()` 的 ctor 合法性仅查 class_ids；建议重构为符号表驱动。
4. `run` 模式 segfault 史（context 生存期）：临时表达式持 raw 指针；已在 multimod 路径用绑定变量规避，仍未全局审计。
5. `expr_to_path`（parser.rs:968）已修（this/super 段）；`super.x` 路径已在 irgen 消费（Assign super 段 → 本对象字段槽写入）。
6. 修掉两个潜在病理：field_index 基类优先布局（旧版子类字段先匹配导致索引错位）；`-> ()` 的 func.call 不得绑定结果（method/本地 fn/跨模块 fn 三处已改为无绑定语句形式）。
7. **trait/dyn 分派**（§4.3.2 虚表已落地）：对象头 word1 = vt 指针（rt `sloth_obj_set_vtable`/`sloth_obj_vtable`）；vt 是 rt 分配的 `[cap, slotN...]` fn ptr 词数组（`sloth_vt_new/set/get`，越界-slot 返回 0；vt==0 安全 → panic 路径）。槽位全局唯一 (trait, method)→slot，在 check_impls + finalize_vt（emit_module 开头，`declared traits × 方法`）预分配，vt 容量 `vt_cap` 在发射前冻结。**关键机制**：参与 impl 链的方法 emit 为 `llvm.func`（体允许混合 arith/memref/cf + 尾部 `llvm.return`，且体内 `call @` 必须 `func.call @` 显式化）；ctor 发射 `emit_vt_build`（addressof/ptrtoint 存槽），`emit_dyn_call` 用 `llvm.call 间接形（`!llvm.ptr, (tyN) -> ret` 2-tail type 语法）`。每对象重建 vt（投入小，优化待做）。trait 方法默认实现未实现。跨模块：foreign traits/class 参与同一 vt（register_import 里 finalize）。ABI 校验：impl 的参数/返回 float 词宽与 trait 签名不匹配 → codegen 错误。
8. `fw.cjump` 接收 i64 条件（内部 trunci），cmpi 结果需先 `extsi i1→i64` 送入；`llvm.mlir.addressof` 要求目标为 `llvm.func`/global（不能 func.func）；不可绑定零结果 ops 的 `func.call`；AOT（mlir-opt 直解析）与 JIT 一致性对上述均敏感，两边验证过（glub/woof、override 0/12、float trait 12/7.2、panic 路径）。
9. arith 浮点提升：混合 i64/f64 操作数补 `arith.sitofp`（此前 expr_arith 直接混用 i64 与 f64 操作数会在 llvm mapping 报错）；normalize_indices 按 `func.func` **与 `llvm.func`** 分块（后者此前被并块，index 改写跨函数失效）。
10. **Array MVP 偏差**：无 push（字面量定长）；异构元素数组静态类型归一为 `Array(i64)`（元素上的方法分派需依赖声明 `Array<dyn T>` 的参数/变量类型）；嵌套索引赋值 `a[1][2]=..` 未支持；别名语义（共享可变数组）受 rt 定长布局限制；越界 = rt `eprintln + exit`（无 sloth 异常）。

## 建议下一步顺序（与文档 §8 对齐）

1. ~~收 P3 尾：`class A: B` 单继承 + `super.__init__`/`super.x`~~ **已完成**（测试 irgen_p3c：inherit/super_method/super_assign + unit_calls 回归）。
2. ~~trait MVP：`impl` 校验 + 静态 trait 方法直调 + `dyn Trait`~~ **已完成**（patch #3 cls_id 链 MVP → patch #4 真虚表（用户指令）：`llvm.func` 方法 + addressof/ptrtoint 槽值 + 间接 `llvm.call`；测试 irgen_p3d：dispatch/inherited/override/missing_diag/unknown_diag）。
3. ~~容器运行时：`sloth_array_new/push/get/set`（带边界检查）+ 迭代协议 inline~~ **Array MVP 已完成**（_rt arr get/set/len/越界 panic、字面量、a[i]读写、len、for-in；无 push/Map/迭代 trait 协议，见债 10）。
4. ~~`pub` 可见性强制 + 循环依赖诊断 + `is`/`nil`/`?:`~~ **已完成**（patch #7：guard_hidden/可见性 gate + resolve 栈环诊断 + is/nil/?:；**注意**：非 pub 访问现在报错——旧 fixture 中无 pub 前缀签名如 `func twice` 已全部改为 `pub`）。
5. 用 example（§9.1/9.2 改写版）做回归金标。

——本文件为状态快照，随阶段推进应更新对应行。
