# 22. FFI：extern func / extern type

## 22.1 声明

```sloth
extern func floor(x: float): float;   // 链接期按符号名解析
extern type Tok;                      // 不透明类型
extern func tok_new(): Tok;
extern func tok_val(t: Tok): int;
```

- `extern func` 的符号名即 C 符号，走 **C ABI**。参数/返回值在边界做词面 ↔ C 类型
  的 marshalling：
  - `float` 形参：词是原生 f64 位模式，`llvm.bitcast` 即得 `f64`；返回同理；
  - `int`/`bool` 形参：词就是原始标量，直通；
  - 标量返回值直通；不透明 extern-type 指针词直通。
- `extern func` 不能是泛型，也不能可变参；参数必须有显式类型。
- `extern type` 是不透明句柄，脚本侧**不能**解引用、取字段或构造，只能经 extern
  函数传递（编译期诊断）。
- 对本仓库的运行时符号（`__sloth_*` 等）**不要**用 `extern func` 再次声明——它们
  已由模块前导预声明；`__sloth_` 是保留前缀，用户声明会直接被拒绝（见 §22.4）。

## 22.2 与运行时链接

JIT（`run`）会链接 `libsloth_rt.so`；AOT（`build`）用 clang 把生成的目标文件与
`libsloth_rt.so` 链接。运行时里内置了几个示例目标（`sloth_extern_floor`、
`sloth_extern_powf`、`sloth_extern_tok_new`、`sloth_extern_tok_val`）。

## 22.3 示例

```sloth
{{#include examples/ffi.sl}}
```

```mlir
{{#include examples/ffi.mlir}}
```

`extern func` 的声明保留在剥离后的 MLIR 里（`@sloth_extern_floor` 等；用户 extern
符号不带保留前缀）。可以看到浮点参数在调用边界只做一次
`llvm.bitcast : i64 to f64`（词就是原生 f64 位模式）→
`call @sloth_extern_floor(%f)`，返回值再 `llvm.bitcast` 回 `i64`（无掩码/移位）。

## 22.4 保留命名空间 `__sloth_` 与伪导入

运行时与标准库的底层 ABI 统一使用 `__sloth_*` 前缀，唯一声明点是编译器注入的
`lib/prelude/abi.slt`（自举容器/值盒另见 `containers.slt`/`core.slt`）。前端对
这一前缀设了两道闸门：

- **声明闸门**：任何用户代码（函数、字段、参数、局部绑定）声明以 `__sloth_` 开头
  的名字都报 `reserved symbol `...` may only be declared in the prelude`。
- **调用闸门**：普通模块**调用** `__sloth_*` 报
  `reserved symbol `...` may only be called from a module that imports "__sloth"`，
  除非该模块带伪导入：

  ```sloth
  import "__sloth";   // 伪导入：只解锁保留符号的调用，不加载真实文件
  ```

标准库 `.slt`（`lib/sloth/*.slt`）正是通过 `import "__sloth";` 使用这些低层入口，
从而把「哪些运行时能力对用户可见」收敛为标准库表面。
