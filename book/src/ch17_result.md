# 17. 错误模型：Result

首版**没有异常**。可恢复错误约定用返回 `T?` 或标准库的 `Result<T, E>`。

## 17.1 `Result<T, E>`

编译器为每个模块自动注入一个 stdlib 泛型类（源码形态）：

```sloth
class Result<T, E> {
    var ok: bool = false;
    var v: T;
    var e: E;
    func is_ok(): bool { return this.ok; }
    func unwrap(): T {          // 失败时 panic
        if this.ok { return this.v; }
        sloth_panic_unwrap();   // 运行时通道（打印并退出）
        return this.v;
    }
    func err(): E { return this.e; }
}
```

- 构造器是**上下文类型驱动**的：`ok(x)` / `err(x)` 需要目标类型可判定为某个
  `Result<_,_>` 实例——来自 `let/var` 的显式标注、函数返回注解、或赋值目标的容器
  值类型。无注解的 `let x = ok(5);` 报
  `ctor `ok` requires a declared Result target`。
- `unwrap()` 在 `ok == false` 时走运行时 `sloth_panic_unwrap`（打印并退出）。
- `err()` 直接返回载荷（类型 `E`）。

## 17.2 返回位置

函数返回注解提供上下文，可直接 `return ok(...)` / `return err(...)`：

```sloth
func parse(x: int): Result<int, str> {
    if x < 0 { return err("neg"); }
    return ok(x * 2);
}
```

## 17.3 示例

```sloth
{{#include examples/result.sl}}
```

```mlir
{{#include examples/result.mlir}}
```

`Result<int,str>` 与 `Result<float,int>` 各被单态化为独立类，其方法发射为
`@sloth_main_Result_int_str__is_ok` 等。`ok`/`err` 构造在调用点内联为
`sloth_obj_new` + `sloth_obj_set_field`（字段 `ok`/`v`/`e`）；
`unwrap` 的非 ok 分支调用 `sloth_panic_unwrap`。
