# 21. 模块系统

## 21.1 编译期 import

```sloth
import "modules_lib.sl" as lib;   // 可带别名
import "util.sl";                 // 不带别名：符号直接进入当前作用域
```

- `import` 是**编译期声明**（不是运行时函数）：编译器解析依赖并在同一模块里发射
  被导入模块的函数/类/全局。
- 只有 `pub` 声明对外可见；访问非 `pub` 符号报
  ``X` is private to its module (not `pub`)`。
- 每个模块只编译一次；被导入模块的全局初值在其
  `@sloth_<模块>__ginit` 里，由入口在用户体之前统一调用。
- 出现 `import` 时，`slothc check/ir/run/build` 自动切换到多模块编译。

## 21.2 跨模块访问

- 函数：`lib.double(21)`（别名限定）。
- 全局：读 `lib.counter`，写 `lib.bump()`（被调函数体内的写路由回它**自己模块**的
  cell，因此跨模块写也一致）。
- 类：可经别名构造/引用（`M.Counter(2)`）。

## 21.3 示例

被导入模块：

```sloth
{{#include examples/modules_lib.sl}}
```

主模块：

```sloth
{{#include examples/modules_main.sl}}
```

对应 MLIR（注意两个模块的函数都发射在这一个 `module @main` 里，被导入模块的函数
以 `<模块名>__<函数名>` 命名）：

```mlir
{{#include examples/modules_main.mlir}}
```

可看到 `@sloth_modules_lib__double`、`@sloth_modules_lib__hidden`（非 pub 也发射，
但不在符号表可见面）与 `@sloth_modules_lib__ginit`。
