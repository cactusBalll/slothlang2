# sloth-lang 2.0 语言指南

本指南描述 **sloth-lang 2.0（sloth2）** 的语言与运行时语义，内容以
`slothlang2` 仓库的**当前实现**为准，而不是设计文档的纸面约定。凡是当前
实现与《sloth-lang 2.0 设计文档》（v1.0，2026-09-14 评审稿）不一致之处，
本指南按实现描述，并在[附录 A](appendix_a_deviations.md)集中列出偏差。

本指南的每一段示例代码都配有一份**由当前编译器实际生成的 MLIR**，二者来自
同一份源文件：

- 源码位于 `book/src/examples/*.sl`；
- MLIR 由 `book/gen-mlir.sh` 调用 `slothc ir` 生成，存于同名 `*.mlir`。

```sh
# 重新生成所有示例的 MLIR（在工作区根目录执行）
./book/gen-mlir.sh
```

生成脚本会剥离每个模块都会原样发射的"固定运行时前导声明"
（`func.func private @__sloth_*`）与自举 prelude 的函数体，以使 MLIR 可读；
被剥离的前导与完整的模块骨架见[附录 B](appendix_b_mlir.md)。示例的
`extern func` 声明不会被剥离。

## 阅读约定

- 代码块标注 `sloth` 的是源程序，标注 `mlir` 的是它对应的 MLIR。
- MLIR 使用标准 dialect（`func` / `arith` / `cf` / `memref` / `llvm` 及张量段
  的 `tensor`/`linalg`/`scf`/`math`），运行时调用表现为对 `libsloth_rt` 导出
  符号的 `func.func private @__sloth_*` 声明（或自举 prelude 在模块内的定义）。
- 本指南不保证 MLIR 文本逐字稳定（SSA 编号、常量折叠细节可能随实现演进），
  但其结构与所调用的运行时符号是有意展示的部分。
