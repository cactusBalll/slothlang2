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
| P3 对象与闭包 | **部分完成** | class（单模块 + 跨模块）字段/方法/ctor 可跑（63e0413、a358058）；闭包快照捕获完成（cecfab7）；字符串池/插值完成。**未做**：`class A: B` 继承与 super 调用、虚表/虚方法（现为**静态重载符号直调** `sloth__(mod)__(Cls)__method`，无 devirt 需求但也无动态分派）、trait、`dyn Trait`、`is` 类型测试 |
| P4 泛型与单态化 | **未开始** | parser 支持 `type_params`，codegen 全面无单态化；`Iterable`/`Iterator` 协议、`Array<T>`/`Map<K,V>` 仅 `list`/`@(k: v)` 字面量解析（irgen 无容器运行时） |
| P5 模块与标准库 | **核心完成** | 编译期 `import`（递归、canonicalize 去重、`as` 别名、`mod.fn()`/`mod.g` 限定调用/读取，ec5a2c7 + a7a8b0d）；`pub var` 全局单元格 `memref.global @{mod}_g_{name} {mutable}` + 每模块 `__ginit` 启动时调用；类跨模块可见。**缺**：`pub` 可见性未强制（非 pub 也导出）、循环依赖检测、标准库（无 Array/Map 运行时方法，仅 print/len 等 builtin 白名单） |
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
5. `expr_to_path`（parser.rs:968）已修（this/super 段）；但 `super.x` 路径在 irgen 未消费。

## 建议下一步顺序（与文档 §8 对齐）

1. 收 P3 尾：`class A: B` 单继承 + `super.__init__`/`super.x`（信息在 ClassInfo.superclass，IR 无需虚表即可先静态化）。
2. trait MVP：`impl` 校验 + 静态 trait 方法直调 + `dyn Trait` (data_ptr, vt_ptr) 双字 + `call_indirect`。
3. 容器运行时：`sloth_array_new/push/get/set`（带边界检查）+ 迭代协议 inline。
4. `pub` 可见性强制 + 循环依赖诊断 + `is`/`nil`/`?:`。
5. 用 example（§9.1/9.2 改写版）做回归金标。

——本文件为状态快照，随阶段推进应更新对应行。
