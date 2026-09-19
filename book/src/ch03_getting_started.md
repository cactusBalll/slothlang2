# 3. 快速开始

## 3.1 构建工具链

在工作区根目录构建：

```sh
cargo build -p slothc          # 生成 target/debug/slothc 与 libsloth_rt.so
```

`slothc` 需要 LLVM/MLIR（本仓库使用 `/usr/lib/llvm-21`）。`run` 模式通过 JIT
执行，链接 `target/debug/libsloth_rt.so`。

## 3.2 命令行

```text
slothc <check|ir|run|build> file.sl [out]
```

| 模式 | 作用 |
| --- | --- |
| `check` | 只做解析 / 发射 / 诊断，成功打印 `check ok`，失败非零退出并把诊断写到 stderr |
| `ir` | 打印生成的 MLIR 文本 |
| `run` | JIT 编译并执行（等价于 `main` 入口） |
| `build` | 走 MLIR→LLVM→目标文件→clang 链接，生成原生可执行文件（默认 `sloth_app`） |

> 若源码中出现 `import`，`check`/`ir`/`run`/`build` 会自动切换到多模块编译
> （见 §21）。

## 3.3 第一个程序

```sloth
{{#include examples/hello.sl}}
```

运行：

```sh
$ ./target/debug/slothc run book/src/examples/hello.sl
hello, sloth!
```

## 3.4 它对应的 MLIR

```mlir
{{#include examples/hello.mlir}}
```

这段 MLIR 值得逐块读一遍：

- `@sloth_main__ginit` 是**模块初始化函数**：顶层 `var`/`let` 的初值在这里写入
  全局 cell。没有全局变量时它是空体。
- `@sloth_main` 是脚本入口，带 `llvm.emit_c_interface`，由 JIT 的
  `invokePacked` 调用。
- 字符串字面量被**内联为 8 字节一组的 `i64` 常量**，经
  `sloth_str_push(builder, word, len)` 拼进字符串构建器，
  最后 `sloth_str_finish` 收口分配出一个 `str` 句柄（不做驻留）。
- `${name}` 插值把值装箱为 `any`，经 `sloth_rt_write(any)` 渲染成 `str`，
  再用 `sloth_str_pushp` 推进构建器。
- 插值结果交给 stdlib `print`，其内部 `sloth_rt_puts(write(v))` 打印后立即
  `sloth_rc_release`——这是 ARC 的**语句级临时量结算**（见 §23）。
- 局部变量 `name` 存放在 `memref<1xi64>` 的栈槽里；绑定那一刻发出
  `sloth_rc_retain`，作用域退出时 `sloth_rc_release`。

完整的（**未剥离前导**）模块见[附录 B](appendix_b_mlir.md)。
