# 2. 编译器架构与流水线

## 2.1 组件

工作区由四个 crate 组成：

| crate | 职责 |
| --- | --- |
| `sloth-frontend` | 词法分析、递归下降 + Pratt 语法分析、AST、类型表示（`ty.rs`） |
| `sloth-codegen` | 语义分析 Pass（`sem.rs`）：符号解析、类型推断与检查，独占诊断；发射 Pass（`irgen/`）把 AST 转成 MLIR；`pass.rs`/`jit.rs` 走 LLVM 管线并 JIT 执行；`module.rs`/`context.rs` 是 MLIR C API 封装 |
| `sloth-rt` | 运行时 `libsloth_rt.so`：裸分配、ARC/弱引用、字符串、对象/虚表、panic，以及 I/O/网络/张量/协程/线程的 C-ABI 入口与 extern 示例（Array/Map/range/值盒的实现改由自举 prelude 提供） |
| `slothc` | 命令行前端（`check` / `ir` / `run` / `build`） |

## 2.2 实际流水线

设计文档设想的 `[3] 名称解析 → [4] 类型检查 → [5] 单态化 → [6] MLIR` 四个
独立阶段，在当前实现中落为**两个显式 Pass**：

```text
源代码 (.sl)
   │  词法分析 Lexer
   ▼
Token 流
   │  语法分析 Parser（递归下降 + Pratt）
   ▼
AST（Program { imports, decls, stmts }）
   │  Pass 1 — 语义分析 sem/（ModEmitter, check_mode = true）
   │   ├─ 收集符号 / 类 / trait / 模块依赖（sem/collect.rs, module.rs）
   │   ├─ 类型注册表 + 词面（word-class）推断/兼容检查（sem/types.rs）
   │   ├─ 泛型函数/类的单态化实例缓存（emit_gfunc_call）
   │   ├─ NodeId 类型侧表（frame, node id) → TyId）
   │   └─ 诊断的唯一产生者（`slothc check` 只跑此 Pass）
   ▼
   │  Pass 2 — 发射 irgen（ModEmitter, check_mode = false，诊断静默）
   │   ├─ 消费 Pass 1 的类型侧表（`emit_expr` 以之为权威表达式类型）
   │   ├─ ARC/coercion 发射（coerce.rs）
   │   └─ 拼接 MLIR 文本：ARC 发射 sloth.rc_retain/release，其余标准 dialect
   ▼
含 sloth.* 的 MLIR
   │  单点 lowering（dialect.rs::lower_parsed / slothLowerModule）
   ▼
MLIR（标准 dialect + 对 libsloth_rt / 自举 prelude 的调用）
   │  PassManager: canonicalize, cse, one-shot-bufferize,
   │              linalg-fuse-elementwise-ops, convert-linalg-to-loops,
   │              convert-scf-to-cf, convert-math-to-llvm, convert-func-to-llvm,
   │              convert-arith-to-llvm, convert-index-to-llvm,
   │              convert-cf-to-llvm, finalize-memref-to-llvm,
   │              reconcile-unrealized-casts
   ▼
LLVM IR ──► JIT（`run`）或 目标文件（`build`，经 mlir-opt/mlir-translate/clang）
```

要点：

- **`sloth` dialect 只是一个最小语义层**：ARC 的 `sloth.rc_retain` /
  `sloth.rc_release` 在 parse 后被**单点 lowering** 为标准 `func.call
  @__sloth_rc_*`；完成 lowering 后 IR 中不再出现任何 `sloth.*`（JIT/AOT 双路径
  均有泄漏闸门）。其余发射全部使用标准 dialect，运行时能力通过
  `func.func private @__sloth_*` 的 C-ABI 调用表达。
- **自举 prelude**：`lib/prelude/{containers,core}.slt` 以 Sloth 自身实现
  Array/Map/range/值盒，编译期注入根模块，因此在生成的模块里这些
  `@__sloth_arr_*` / `@__sloth_map_*` 等是被**定义**的函数而非外部声明。
- **诊断批量收集**：发射器不 fail-fast，一次编译尽量报多个错误（`Diag` 列表）。
- **引用计数的插入点在发射期静态确定**（见 §23），因此无需栈图 / statepoint。

## 2.3 MLIR 类型映射

语言类型（值类型与引用类型）在 MLIR 层**统一映射为 `i64` 词**（无 tag）。
下表是源码类型到 MLIR 类型与运行时表示的对应：

| 源码类型 | MLIR 类型 | 运行时词面编码 |
| --- | --- | --- |
| `int` | `i64` | 原生 i64（64-bit，环绕） |
| `int8`/`int16`/`int32`、`uint`/`uint8`/`uint16`/`uint32`/`uint64` | `i64` | 原生 i64，按声明宽度截断/环绕；无符号按位模式解释 |
| `float` | `i64`（调用边界转 `f64`） | 原生 f64 位模式（bitcast） |
| `bool` | `i64` | `0` / `1` |
| `nil` | `i64` | `0` |
| `str` / `Array<T>` / `Map<K,V>` / 类实例 / 闭包 / `dyn` | `i64` | 裸 payload 指针（`0` = nil） |
| `T?`（值型 `T`） | `i64` | 指向 payload 盒的引用词，`0` = `nil` |
| `T?`（引用型 `T`） | `i64` | 句柄本身，`0` = `nil` |
| `range` | `i64` | 指向 `{lo, hi}` 双词盒的引用词 |

注意：**所有** SSA 值、局部槽（`memref<1xi64>`）、对象字段、容器元素都是这一套
词面；`mlir_ret_ty` / `mlir_word_ty` 恒为 `i64`。数值只在标量域转换：`float`
用 `llvm.bitcast`；`int` 直接就是 `i64`。引用/值的区分由编译期信息在 ARC
级联处完成（对象走 codegen 发射的 `@...__cascade`，其余走种类标志；不再有运行期
`arith.shli/shrsi` tag 编解码）。

## 2.4 运行时符号面

发射器会为每个模块补充一份**固定前导**：空程序约有 200 条
`func.func private @__sloth_*` 声明，来源是编译器注入的 `lib/prelude/abi.slt`
（标准库所需的运行时 extern）与内部 prelude。它覆盖 ARC
（`rc_retain`/`rc_release`/`rc_live`/`rc_drops`）、弱引用（`weak_*`）、值型
optional 盒（`box_*`）、字符串与构建器、对象与虚表、panic 通道，以及
I/O/网络/张量/协程/线程的全部 C-ABI 入口。数组/Map/range/值盒的实现函数不属于
这份私有前导，而是由自举 prelude 在模块内定义。

`__sloth_` 是**保留命名空间**：只有编译器注入的 prelude 可以声明这些符号；普通
模块若要直接调用它们，必须带伪导入 `import "__sloth";`（见 §22.4）。完整符号
清单见[附录 B](appendix_b_mlir.md)。示例 MLIR 中这份私有前导已被剥离。
