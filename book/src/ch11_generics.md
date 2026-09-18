# 11. 泛型与单态化

## 11.1 泛型函数与泛型类

```sloth
func first<T>(xs: Array<T>): T { return xs[0]; }
func twice<T>(x: T): Array<T> { return [x, x]; }

class Box<T> {
    var v: T;
    func set(x: T) { this.v = x; }
    func get(): T { return this.v; }
}
```

- 类型参数可带 trait 约束：`func maxi<T: Comparable>(a: T, b: T): T { ... }`。
  约束在实例化时**强制校验**，不满足报
  `type argument `X` does not satisfy trait bound `Y``。
- 调用处类型实参通常由实参推断；也可显式给出：`first<int>(a)`。
- 泛型类实例化写作 `Box<int>()`、`Box<float>()`。

## 11.2 实现策略：单态化

编译期为每个具体类型实例生成一份代码副本（`sloth_main__first` 的 `int`/`str`
实例等），而不是字典传递。收益是生成代码更快、对 MLIR 优化更友好；代价是代码
膨胀。实例按"泛型定义 + 类型实参"缓存，避免重复。

`Result<T, E>` 就是一个由编译器自动注入的 stdlib 泛型类（见 §17）。

## 11.3 示例

```sloth
{{#include examples/generics.sl}}
```

```mlir
{{#include examples/generics.mlir}}
```

可以看到 `first` 被实例化为两份函数：`@sloth_main__first_int`（`T=int`）与
`@sloth_main__first_str`（`T=str`），各自在体内直接对 `i64` 词操作；`Box<int>`
的方法也以 `T=int` 帧实例化为 `@sloth_main_Box_int__set`/`__get`。`first(a)` 与
`first<int>(a)` 命中同一实例。
