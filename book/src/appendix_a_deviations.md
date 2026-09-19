# 附录 A. 与设计文档的偏差清单

本附录汇总《sloth-lang 2.0 设计文档》（v1.0，2026-09-14）与**当前实现**的差异。
标注"定案"的是有意保留的取舍；标注"未实现"的表示该特性目前不可用。

## A.1 编译器架构

| 设计 | 实现 | 性质 |
| --- | --- | --- |
| §4.1 十个阶段的流水线，含独立的 [3] 名称解析、[4] 类型检查/推断、[5] 单态化 | 解析后由单一发射器**一趟融合**完成符号收集、局部推断、约束检查、单态化与 MLIR 生成 | 定案 |
| §4.3 自定义 `sloth` dialect（`!sloth.string`、`sloth.gc_alloc` 等） | **没有** sloth dialect；直接发射标准 `func`/`arith`/`cf`/`memref`/`llvm`，运行时能力用 `func.func private @sloth_*` C-ABI 调用表达 | 定案 |
| §4.1 循环依赖通过依赖图拓扑检测 | `resolve_program` 在按 `import` 递归装配时用 DFS 栈比对规范化路径，命中即报 `circular import`（`crates/sloth-codegen/src/irgen/mod.rs:205`）；`done` 集合去重。无独立依赖图/拓扑排序阶段 | 定案（等价检测，无独立阶段） |
| §4.2 闭包捕获结构体 + 逃逸性分析 | 闭包统一为 2 词 `{ fnptr, env }` 对象；捕获语义见 §10 | 等价实现 |

## A.2 类型与内存表示

| 设计 | 实现 | 性质 |
| --- | --- | --- |
| `int` 为 i64 | `int` 为原生 i64（词面无 tag，64-bit 环绕） | 已对齐 |
| `float` 为 f64 | `float` 为原生 f64（bitcast 进 i64 词面，满精度） | 已对齐 |
| `T?`（值型）用 `{payload, has_value}` 标签布局（16 字节 `int?`） | 值型 `T?` 是 rc 跟踪的一词 payload 盒；槽仍 1 词，`0` = `nil` | 定案（ABI 简化） |
| `dyn Trait` 为"数据指针 + 虚表指针"两字 fat pointer | `dyn` 是单字段（虚表指针）+ class-id，运行时等价 | 定案 |
| `range` 是两 `i64` 值类型 | `range` 是 rc 双词盒 `{lo, hi}` 的**引用词** | 偏离 |
| §5.1 启用 mark & sweep GC（早期用 Boehm 兜底） | 移除全部 GC，改为 **ARC + `Weak<T>`**，malloc 基确定性分配器 | 定案（架构改向） |

> **去 tag 迁移（2026-09-19，PLAN §14）**：词面早先采用 LSB tag
> （引用 `ptr|1`、int `v<<1` 63-bit、f64 f63、bool `0/2`），代价是 int 收窄与
> 浮点精度损失。现改为**无 tag 词面**：引用为裸指针、int/float 为原生位模式，
> 运行期靠编译期引用掩码（对象 `refmask`、数组 `elref`、Map `vref`、通道/协程
> `eref`）驱动 ARC 级联。此前的浮点编码掩码不一致（旧 A.2.1）随该迁移消除。


## A.3 语法

| 设计 | 实现 |
| --- | --- |
| `for_stmt ::= 'for' '(' 'var' IDENT ':' expr ')' block` | 两种形式：`for x in expr {}` 与 `for (var x: expr) {}` |
| `if`/`while` 条件必须带括号 | 带/不带括号均可 |
| 逻辑运算符 `and`/`or`/`not` | 另有 `&&`/`||` 同义 |
| （设计未列）位运算 | **增补** int-only `& \| ^ << >> ~`（TE-P0） |
| （设计未列）复合赋值 | **增补** `+=` `-=`（展开为 `a = a op b`） |
| （设计未列）`Weak<T>` / `Tensor<T,R>` | **增补类型**（`Tensor` 秩语法 0..=8、语义 1..=3，见 A.7） |
| §5.4 `extern func`（设计已声明） | 已实现；`EBNF §3.9` 原未列，现补入 `extern func` / `extern type` |
| §2.2 "函数无返回标注且体仅单个 `return` 时可推断返回类型" | **不推断**：省略返回标注即 `unit`；`return expr;` 在 unit 函数里静默丢弃，不报错 |
| §3.4 运算符重载以 trait 参数化（`Add<Rhs,Out>` 等） | 直接用魔术方法名（`__add__`/`__eq__`/…），不做 trait 参数化 |
| §4.3.2 `sloth.string_literal` 等高层操作 | 字符串字面量内联为 8 字节打包的 `i64` 常量 + `sloth_str_push`/`str_finish` |

## A.4 尚未实现 / 受限

- **`?.` 可选链**：未实现（设计明确推迟）。对可空接收者直接取字段报诊断；链式表达式
  的收窄不支持，需先取出中间值。
- **一等函数值**的范围受限：泛型 / 可变参 / `extern` / 跨模块函数**不能**作值；
  方法引用命中 trait 方法时走静态解析而非虚分派。
- **嵌套闭包捕获**：内层 lambda 无法捕获外层 lambda 的形参/捕获。
- **字符串/数值互转**：`int(str)`、`float(str)` 不支持（MVP）。
- **函数重载**：不支持。
- **trait 关联类型 / 泛型 trait**：不支持。
- **强引用环**：ARC 固有泄漏，需 `Weak<T>` 破环。

## A.5 标准库与方法解析

- §5.3 "基础类型方法解析为 stdlib 泛型函数"：实现改为**直接发射运行时调用**
  （`arr.len()` → `sloth_arr_len` 等），不生成 stdlib 泛型函数。
- `Result<T,E>` 与 `Entry<K,V>` 由编译器**自动注入**为标准库类（源码形态见 §17）。
- `Hashable` 类若无 `__hash__` 家族方法会回退到指针恒等，并给编译期提示。
- 迭代协议是**结构化**的（提供 `iter()`/`next()` 即可），不要求显式 `impl Iterable`。

## A.6 诊断

- 一次编译尽量报多个错误（batch diagnostics），不做 fail-fast，与设计 §4.2 一致。
- 错误消息为英文，携带源码位置（行:列）。

## A.7 张量扩展（TE-P0–TE-P4）

《张量扩展设计文档》以自定义 dialect + GC 为假设；实现改为标准 dialect + ARC
（见第 25 章）。主要偏差：

| 设计 | 实现 | 性质 |
| --- | --- | --- |
| §5.1 自定义 `!sloth.tensor` / `sloth.tensor_*` | 标准 `tensor`/`linalg`/`memref`/`scf`/`math` 组合 + `sloth_tensor_*` C-ABI 助手 | 定案 |
| §5.1 数据区免 GC 扫描 + mmap 外部根 | 无 GC；`Tensor` = ARC 描述符（7 词）+ 非追踪 `calloc` 数据缓冲，视图用 `owner` 保活（无扫描器，数据区天然不参与） | 定案（架构改向） |
| §4.4 `view_as_f32` 零拷贝 f32 视图 | 元素是 f64，checkpoint 是 f32 → **一次性加宽拷贝**（`sloth_tensor_from_f32_ptr`），内存 ×2；真零拷贝需后续 `Tensor<f32,R>` | 定案 |
| rank 作为泛型整型常量参数 | `Ty::Tensor(TyId, u32)`，仅整数字面量；parser 接受 0..=8（`parser.rs:374`），语义限 rank 1..=3（`tensor.rs::tensor_rank_ok`），**独立通道**不进泛型单态化帧 | 定案（受控扩展） |
| 广播逐元素运算 | 要求 rank 与各轴完全同形，否则运行期 panic（`sloth_tensor_shape_eq`） | 受限 |
| `sort_desc_index`/`cumsum`/`sample_topp` 作内置 | 排序/前缀扫描/采样在手写 `.slt` 中实现（`tokenizer.slt`/`llama.slt`） | 等价实现 |
| top-p/multinomial 依赖 stdlib 排序 | `sample_topp` 手写插入排序 + CDF；`argmax` 手写扫描 | 等价实现 |
| run.c 权重指针算术 | 整块加宽为扁平张量 + `reshape` 共享视图按层切 `(dim,dim)` | 定案 |
| 同 seed 采样逐位对齐 | RNG 为 31-bit `XorShift`，与 run.c 的 64-bit `xorshift64*` **未**逐位对齐；greedy（temp=0）路径不用 RNG，token 序列与 run.c 完全一致 | 未实现（后续项） |

## A.8 协程扩展（CE-P0–CE-P2）

《协程扩展设计文档》的若干实现与设计稿不同：

| 设计 | 实现 | 性质 |
| --- | --- | --- |
| §4.3 载荷按「转移」语义做发射器插桩（owned 临时不插 release；借用值先 retain 再转移） | 载荷按普通 **borrowed 实参**递交，由运行时 `sloth_fiber_*` 在接收侧 `retain`；`yield`/`resume` 的引用返回仍走规则 4/5 的 +1 交付 | 定案（消除设计稿风险 #2「转移插桩遗漏」） |
| §4.3.5 / §4.4 弃置或取消时，跳过帧的 `release` 不执行、引用滞留 | 局部槽**逐帧登记**（发射器在 ref 局部声明/退出处插入 `@sloth_fiber_track`/`@sloth_fiber_untrack`，仅协程上下文生效）：错误经 `terminate`、取消经 `cancel_abort` 在**栈仍有效时**释放所有在册槽，弃置则在析构处释放。跳过帧不再泄漏引用（temp 级极端情形除外） | 定案（完整拆栈） |
| §4.4 `fiber.cancel` 用 `setjmp`/`longjmp` 直达入口 | 同左；差别在于出栈前先结算在册局部槽，再 `longjmp`（`longjmp` 会重置栈指针，必须先于其完成释放） | 定案 |
| §4.1 仅保存 x86_64 的 `rsp/rbp/rbx/r12-r15`、aarch64 的 `x19-x30/sp/lr` | aarch64 额外保存 AAPCS64 被调用者保存的 `d8-d15`（设计稿遗漏） | 修正 |
| §4.1 自研汇编以 `global_asm!` 内联于 `sloth-rt` | `sloth_fiber_switch_asm`/`sloth_fiber_trampoline` 为 crate 内 `global_asm!`，无需 `build.rs` 或额外链接 | 等价实现 |
