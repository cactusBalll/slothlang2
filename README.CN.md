# sloth-lang 2.0（slothlang2）

[English](README.md) | [简体中文](README.CN.md)

一门静态类型、AOT 编译的系统级语言，采用 **Rust 前端 + MLIR/LLVM 后端**。
`sloth-lang 2.0`（sloth2）是 `sloth-lang 1.0` 解释器的彻底重写：字节码虚拟机、
全装箱值表示与标记清理 GC 全部移除，代之以静态类型系统、MLIR 流水线、原生代码
生成与自动引用计数。

```sloth
func main() {
    let name = "sloth";
    print("hello, ${name}!");
}
```

```sh
$ ./target/debug/slothc run book/src/examples/hello.sl
hello, sloth!
```

## 特性

- **静态强类型** —— 类型在编译期确定或局部推断；动态能力收敛到显式的
  `dyn Trait`。
- **基于 MLIR + LLVM 的 AOT / JIT** —— 源码 → MLIR → LLVM IR → 原生机器码，
  提供 `run`（JIT 执行）与 `build`（原生可执行文件）两条路径。
- **无 tag 的单 i64 词面** —— 所有 SSA 值、栈槽、字段与容器元素都是一个
  `i64`；引用是裸指针，数值是原生位模式，运行时不做 tag 解码。
- **ARC 内存管理** —— 确定性引用计数，配合 `Weak<T>` 破环；无追踪式收集器。
- **C-like 语法 + 现代惯用法** —— 管道 `|>`、字符串插值 `${expr}`、迭代器
  `for`、运算符重载、泛型单态化、类/继承与 trait。
- **自举标准库** —— `Array`、`Map`、`range`、值盒（boxopt）、`Result`、
  `Entry`、`print` 前导与 `StrChars` 迭代器均以 Sloth 自身实现
  （`lib/prelude/*.slt`），编译期注入。
- **核心之上的扩展模块** —— 张量、协程、OS 线程以及异步 I/O / 网络 reactor 以
  普通函数调用形式叠加在核心语言之上，不新增关键字。

## 架构

工作区由四个 crate 与一份自举标准库组成：

| crate | 职责 |
| --- | --- |
| `sloth-frontend` | 词法分析、递归下降 + Pratt 语法分析、AST、类型表示 |
| `sloth-codegen` | 语义分析 Pass（`sem/`）、MLIR 发射（`irgen/`）、最小 `sloth` dialect、LLVM 管线与 JIT（`pass.rs`/`jit.rs`）、MLIR C-API 封装 |
| `sloth-rt` | 运行时 `libsloth_rt.so`：内存分配、ARC/弱引用、字符串、对象/虚表、panic，以及 I/O、网络、张量、协程、线程的 C-ABI 入口 |
| `slothc` | 命令行前端（`check` / `ir` / `run` / `build`） |

解析之后，编译当前落为**两个显式 Pass**：

```text
源代码 (.sl)
  │  词法分析
  ▼
Token 流 ─► 语法分析 ─► AST（Program { imports, decls, stmts }）
  │
  │  Pass 1 — sem/（ModEmitter, check_mode = true）
  │    符号/类/trait/模块收集、类型推断与约束检查、泛型单态化、
  │    NodeId 类型侧表，并独占全部诊断
  ▼
  │  Pass 2 — irgen/（ModEmitter, check_mode = false，诊断静默）
  │    回放类型侧表，发射 ARC / coercion / 宽度转换与 MLIR 文本
  │    （`sloth.rc_retain`/`sloth.rc_release` + 标准 dialect）
  ▼
MLIR ─► 单点 lowering：`sloth.*` → `func.call @__sloth_*`
  ▼
MLIR（标准 dialect + 对 libsloth_rt / 自举 prelude 的调用）
  │  canonicalize → cse → one-shot-bufferize → linalg-fuse-elementwise-ops
  │  → convert-linalg-to-loops → convert-scf-to-cf → convert-math-to-llvm
  │  → convert-func-to-llvm → convert-arith-to-llvm → convert-index-to-llvm
  │  → convert-cf-to-llvm → finalize-memref-to-llvm
  │  → reconcile-unrealized-casts
  ▼
LLVM IR ──► JIT（`run`）或目标文件 + clang 链接（`build`）
```

要点：

- `sloth` dialect 只是一个**最小语义层**：仅发射 ARC 的
  `sloth.rc_retain`/`sloth.rc_release`，随后单点降为
  `func.call @__sloth_rc_*`，对外可见 IR 中不含任何 `sloth.*`。
- **运行时 ABI 使用保留命名空间**（`__sloth_*`），普通模块不得声明这些符号。
- 诊断**批量收集**，不 fail-fast。

## 仓库结构

| 路径 | 内容 |
| --- | --- |
| `crates/` | 上表四个 Rust crate |
| `lib/prelude/` | 编译期注入的标准库（`containers`、`core`、`print`、`result`、`strchars`、`abi`） |
| `lib/sloth/` | 可导入的标准库模块（`array`、`str`、`net`、`http`、`event`、`io`、`fs`、`tensor`、`random`） |
| `book/` | mdBook 语言指南（中文），每个示例附编译器实际生成的 MLIR |
| `examples/` | 示例程序与运行脚本：`arc`、`diff`、`fiber`、`fs`、`llama`、`net`、`tensor`、`threads` |
| `test_workspace/` | 分面测试工作区及其 bug 报告 |
| `docs/` | 设计说明（如运行时自举评估） |
| `slothlang2-syntax/` | VS Code 语法高亮扩展（`.sl` / `.slt`） |
| `crates/slothc/tests/spec`、`.../seed` | 由 `spec_suite.rs` / `seed_suite.rs` 驱动的回归测试套件 |

仓库根目录下的设计文档（`sloth-lang-2.0设计文档.md` 及张量 / 协程 / 多线程 /
I/O 扩展文档）是原始中文规格；语言指南记录了实现相对设计稿有意保留的偏差。

## 环境要求

- **Rust**（nightly；当前工作区以 `rustc 1.99.0-nightly` 构建）。
- **LLVM/MLIR 21**（含 CMake 包），以及 `clang`、`mlir-opt`、`mlir-translate`。
  构建固定 `mlir-sys = 210.0.4`，默认前缀为 `/usr/lib/llvm-21`（可用
  `MLIR_SYS_210_PREFIX` 覆盖）。
- **CMake**（推荐 Ninja），用于构建 C++ 的 `sloth` dialect sidecar。
- C++ 工具链与 `libstdc++`。

工作区的 `.cargo/config.toml` 已设置 `MLIR_SYS_210_PREFIX=/usr/lib/llvm-21`。

## 构建

```sh
cargo build -p slothc      # 生成 target/debug/slothc 与 libsloth_rt.so
```

构建全部（库 crate、运行时与驱动）：

```sh
cargo build
```

如需优化后的原生运行时，以 release 模式构建运行时：

```sh
cargo build --release -p sloth-rt
```

## 用法

```text
slothc <check|ir|run|build> file.sl [out]
```

| 模式 | 作用 |
| --- | --- |
| `check` | 只做解析与语义分析；成功打印 `check ok`，失败把诊断写到 stderr |
| `ir` | 打印生成的 MLIR 文本 |
| `run` | JIT 编译并执行（`main` 入口） |
| `build` | MLIR → LLVM → 目标文件 → `clang` 链接为原生可执行文件（默认输出 `sloth_app`） |

源码中出现 `import` 时，`check`/`ir`/`run`/`build` 会自动切换到多模块编译。

```sh
./target/debug/slothc check examples/hello2.sl
./target/debug/slothc ir    book/src/examples/hello.sl
./target/debug/slothc run   book/src/examples/hello.sl
./target/debug/slothc build examples/threads/matvec_parallel.sl my_app
```

## 语言速览

```sloth
trait Speaker { func say(): unit; }

class Mammal impl Speaker {
    let kind: str;
    func __init__() { this.kind = "Mammal"; }
    func say(): unit { print("Mammal kind is: ${this.kind}\n"); }
}

class Cat: Mammal {
    func __init__() { super.__init__(); this.kind = "Cat"; }
    func say(): unit { print("meow\n"); super.say(); }
}

pub func main(): unit {
    let l: Array<dyn Speaker> = [Cat(), Mammal()];
    for (var m: l) { m.say(); }
}
```

完整类型集包括 `unit`、`bool`、`int`（64-bit）、`float`（f64）、定宽整数族
（`int8`/`int16`/`int32`、`uint`/`uint8`/…/`uint64`）、`str`、`range`、
`Array<T>`、`Map<K,V>`、`T?`、一等函数与闭包、单继承类、trait / `dyn`、`any`、
`Weak<T>`、`Tensor<T,R>`、`Fiber<Y>`、`JoinHandle<R>` 与 `Channel<T>`。

### 标准库与扩展模块

- 经编译期模块系统从 `lib/sloth/` 导入，如 `import "sloth/array.slt";` 或
  `import "sloth/str.slt";`。
- **张量**（`Tensor<T,R>` + `linalg`）—— 见 `examples/tensor`、`examples/llama`。
- **协程** —— 有栈协程（`fiber.create`/`resume`/`yield`），见 `examples/fiber`。
- **多线程** —— `thread.spawn`、`JoinHandle`、`Channel`、`Mutex`、`AtomicInt`，
  见 `examples/threads`。
- **I/O / 网络** —— 以 Sloth 实现的事件 reactor（`io_uring`/`epoll`），提供
  TCP/UDP/HTTP 辅助，见 `examples/net` 与 `lib/sloth/{io,net,http,event}.slt`。

## 测试

```sh
cargo test --workspace
```

编译器回归套件位于 `crates/slothc/tests/`：

- `spec_suite.rs` —— 规格/行为测试（`tests/spec/*.sl`），经驱动运行。
- `seed_suite.rs` —— 精选回归测试（`tests/seed/*.sl`），使用 `// expect:`、
  `// diag:` 与 `// bug:` 指令。

JIT 与 AOT 的差分校验可用 `bash test_workspace/aot_diff.sh`；各分域示例的运行
脚本位于 `examples/*/run.sh`（用 `SLOTHC=/path/to/slothc` 指定已构建的驱动）。

## 文档

- **语言指南（mdBook，中文）：** `book/` —— 用
  [`mdbook`](https://rust-lang.github.io/mdBook/) 构建：
  ```sh
  mdbook build book     # 输出到 book/book/
  mdbook serve book
  ```
  指南中每个示例都附有编译器实际生成的 MLIR（`book/gen-mlir.sh` 可重新生成）。
- **原始设计规格（中文）：** 仓库根目录下的 `sloth-lang-2.0*.md`。
- **与设计文档的偏差清单：** `book/src/appendix_a_deviations.md`。
- **运行时自举评估：** `docs/self-hosting-evaluation.md`。
- **测试工作区报告：** `test_workspace/BUGS.md`。

## 项目状态

语言特性面已基本对齐设计目标：类型系统（含定宽/无符号整数）、泛型单态化、
类/继承/虚分派、trait 与 `dyn`、闭包与一等函数、容器、可空类型、`Result`、
迭代协议、编译期模块、FFI 与 ARC 均已落地，张量、协程、多线程与 I/O 扩展亦
可用。

有两处与原始设计的结构性偏差属于定案取舍：编译器为**两 Pass** 前端（先分析、
后发射，仍共享同一推断引擎），`sloth` dialect 有意保持**最小**（仅 ARC）。完整
清单见 `book/src/appendix_a_deviations.md`。

## 许可证

MIT（见工作区 `Cargo.toml`）。
