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
- 对本仓库的运行时符号（`sloth_rc_*` 等）**不要**用 `extern func` 再次声明——它们
  已由模块前导预声明，会冲突。

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

`extern func` 的声明保留在剥离后的 MLIR 里（`@sloth_extern_floor` 等）。可以看到
浮点参数在调用边界只做一次 `llvm.bitcast : i64 to f64`（词就是原生 f64 位模式）
→ `call @sloth_extern_floor(%f)`，返回值再 `llvm.bitcast` 回 `i64`（无掩码/移位）。
