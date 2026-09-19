# 10. 一等函数与闭包

## 10.1 函数类型与 lambda

```sloth
func apply(f: (int) -> int, x: int): int { return f(x); }

let sq  = |x: int| { return x * x; };   // 匿名函数
let gen = || { return 1; };             // 零参用 ||
let ann = |x: int| -> int { return x; };// 可标注返回类型
```

- 函数类型写作 `(A, B) -> R`，`() -> R` 表示零参。
- lambda 参数类型可由上下文推断，也可显式标注。
- 具名函数可作值取出（`let g = inc;`）；也支持 IIFE `(|x| {...})(9)`。

## 10.2 表示与调用

闭包统一发射为 **2 词对象 `{ fnptr, env }`**（`sloth_closure_new`，
`dtor` 只释放 env）。任何函数值调用都经过为每个目标生成的 **bridge**，ABI 为
`(env, args...) -> word`。

## 10.3 捕获语义（已冻结）

| 捕获物 | 语义 |
| --- | --- |
| 标量（`int`/`float`/`bool`） | **构造时值快照**；此后外部重绑不可见 |
| `str`/`Array`/`Map`/类/盒 | **引用共享**（fill 时 `retain`）；外部改内容，闭包内可见，反之亦然 |

## 10.4 示例

```sloth
{{#include examples/closures.sl}}
```

```mlir
{{#include examples/closures.mlir}}
```

注意 `make_adder`：它把捕获的形参 `n` 存进闭包环境槽（`memref.alloca` +
`sloth_closure_new`），返回的闭包是 owned 引用值。`snap` 证明了标量捕获的快照
语义——`base` 改成 `0` 后 `snap(1)` 仍是 `101`。

## 10.5 方法引用

`obj.method` 生成一个 method bridge，**env = 接收者**，因此 `this` 被绑定，可作
值传递或直接调用（见 §6 迁移表）：

```sloth
{{#include examples/method_ref.sl}}
```

```mlir
{{#include examples/method_ref.mlir}}
```

## 10.6 当前限制

- 泛型 / 可变参 / `extern` / 跨模块函数**不能**作为一等函数值（需要实例化或导入签名）。
- 方法引用命中 **trait 方法**时走静态解析而非虚分派。
- 嵌套闭包无法捕获外层 lambda 的形参/捕获（`examples/arc/known/`）。
