# 2. 编译器架构与流水线

## 2.1 组件

工作区由四个 crate 组成：

| crate | 职责 |
| --- | --- |
| `sloth-frontend` | 词法分析、递归下降 + Pratt 语法分析、AST、类型表示（`ty.rs`） |
| `sloth-codegen` | 发射器（`irgen/`）把 AST 直接转成 MLIR；`pass.rs`/`jit.rs` 走 LLVM 管线并 JIT 执行；`module.rs`/`context.rs` 是 MLIR C API 封装 |
| `sloth-rt` | 运行时 `libsloth_rt.so`：分配器、ARC、字符串池、容器、对象/虚表、panic、extern 示例 |
| `slothc` | 命令行前端（`check` / `ir` / `run` / `build`） |

## 2.2 实际流水线

设计文档设想的 `[3] 名称解析 → [4] 类型检查 → [5] 单态化 → [6] MLIR` 四个
独立阶段，在当前实现中被**融合进一次发射过程**：

```text
源代码 (.sl)
   │  词法分析 Lexer
   ▼
Token 流
   │  语法分析 Parser（递归下降 + Pratt）
   ▼
AST（Program { imports, decls, stmts }）
   │  ModEmitter：一趟融合
   │   ├─ 收集符号 / 类 / trait / 模块依赖（collect.rs, module.rs）
   │   ├─ 局部类型推断 + 词面（word-class）surface 兼容检查（tybind.rs）
   │   ├─ 泛型函数/类的单态化实例缓存（emit_gfunc_call）
   │   └─ 直接拼接 MLIR 文本（func/arith/cf/memref/llvm）
   ▼
MLIR（标准 dialect + 对 libsloth_rt 的调用）
   │  PassManager: canonicalize, cse, convert-{func,arith,index,cf}-to-llvm,
   │              finalize-memref-to-llvm, reconcile-unrealized-casts
   ▼
LLVM IR ──► JIT（`run`）或 目标文件（`build`，经 mlir-opt/mlir-translate/clang）
```

要点：

- **没有独立的 `sloth` dialect**。发射器只使用标准 dialect，运行时能力通过
  `func.func private @sloth_*` 的 C-ABI 调用表达。
- **诊断批量收集**：发射器不 fail-fast，一次编译尽量报多个错误（`Diag` 列表）。
- **引用计数的插入点在发射期静态确定**（见 §23），因此无需栈图 / statepoint。

## 2.3 MLIR 类型映射

语言类型（值类型与引用类型）在 MLIR 层**统一映射为带 tag 的 `i64` 词**。
下表是源码类型到 MLIR 类型与运行时表示的对应：

| 源码类型 | MLIR 类型 | 运行时词面编码 |
| --- | --- | --- |
| `int` | `i64` | `v << 1`（63-bit，环绕） |
| `float` | `i64`（调用边界转 `f64`） | `(bits & !1) >> 1`（f63） |
| `bool` | `i64` | `0` / `2` |
| `nil` | `i64` | `0` |
| `str` / `Array<T>` / `Map<K,V>` / 类实例 / 闭包 / `dyn` | `i64` | `ptr \| 1`（tag=1 表示引用句柄） |
| `T?`（值型 `T`） | `i64` | 指向 payload 盒的引用词，`0` = `nil` |
| `T?`（引用型 `T`） | `i64` | 句柄本身，`0` = `nil` |
| `range` | `i64` | 指向 `{lo, hi}` 双词盒的引用词 |

注意：**所有** SSA 值、局部槽（`memref<1xi64>`）、对象字段、容器元素都是这一套
词面；`mlir_ret_ty` / `mlir_word_ty` 恒为 `i64`。真实的 tag 编解码可在示例
MLIR 里直接看到（`arith.shli` / `arith.shrsi` / `llvm.bitcast`）。

## 2.4 运行时符号面

发射器会为每个模块补充一份**固定前导**：约 60 条 `func.func private @sloth_*`
声明，覆盖 ARC（`rc_retain`/`rc_release`/`rc_live`/`rc_drops`）、弱引用
（`weak_new`/`weak_upgrade`）、值型 optional 盒（`box_new`/`box_get`）、
字符串池与构建器、数组、Map、对象与虚表、panic 通道。完整清单见
[附录 B](appendix_b_mlir.md)。示例 MLIR 中这份前导已被剥离。
