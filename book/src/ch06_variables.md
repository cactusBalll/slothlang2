# 6. 变量与常量

## 6.1 `var` 与 `let`

```sloth
var x: int = 1;   // 可变，可显式标注
var y = 2.5;      // 可变，推断为 float
let z = "hi";     // 不可变绑定
```

- `let` 引入**不可变绑定**：对它再赋值是编译错误
  `cannot assign to immutable `z` (declared with `let`)`。
- 两者都可省略类型标注，类型取自初值。
- 内层块可**遮蔽**外层同名变量。
- 赋值面会做词面兼容检查：把一个 `str` 赋给推断为 `int` 的变量报
  `type mismatch in assignment to `x``。
- 复合赋值 `+=` / `-=` 等价于 `x = x op rhs`，且左值只求值一次（`a[i()] += 1`
  只调用一次 `i()`，见 §7.1）。

## 6.2 模块级全局变量

顶层（不在任何函数/类里）的 `var`/`let` 是**模块级全局**：

- 无标注时，类型来自初值表达式；
- 存储在 `memref.global @sloth_<模块>_g_<名字> : memref<1xi64>` 里；
- 初值在模块初始化函数 `@sloth_<模块>__ginit` 中写入，`@sloth_main` 入口先调用它；
- 函数体内对全局的读写走该 cell（见 MLIR 里的 `memref.get_global`/`memref.load`/
  `memref.store`）；
- `pub var` 可被其它模块读写（见 §21）。

## 6.3 示例

```sloth
{{#include examples/variables.sl}}
```

```mlir
{{#include examples/variables.mlir}}
```

观察：

- 顶部两条 `memref.global` 是全局 `g` / `fixed` 的存储；`@sloth_main__ginit` 把
  `10` 与 `3`（去 tag 后的原生 i64 初值）分别 store 进去。
- 局部 `a` / `b` 各自是 `memref.alloca() : memref<1xi64>` 栈槽。
- 对 `str` 局部 `b` 的绑定是经典的 **copy-in / overwrite-out**：
  `__sloth_rc_retain` 存入、随后 `__sloth_rc_release` 释放生产者的临时量；
  作用域退出时对槽本身再 `__sloth_rc_release`（见 §23）。
- 块级遮蔽的内层 `a` 是**另一个** alloca 槽，与外层 `a` 互不影响。
