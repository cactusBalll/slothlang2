# 20. 运算符重载

运算符重载通过实现**魔术方法**（对应预定义 trait 的方法）完成，解析在编译期：

| 运算符 | 方法 | 备注 |
| --- | --- | --- |
| `+ - * / %` | `__add__` `__sub__` `__mul__` `__div__` `__mod__` | 参数为右操作数，返回结果类型 |
| 一元 `-` | `__neg__` | 无参 |
| `> >= < <=` | `__gt__` `__ge__` `__lt__` `__le__` | 返回 `bool` |
| `== !=` | `__eq__` `__ne__` | 返回 `bool` |
| `x[i]` / `x[i]=v` | `__index__(i)` / `__assign__(i, v)` | Indexable |

规则：

- **缺重载即编译错误**：对没有 `__eq__` 的类比较报
  `comparison `EqEq` on classes `A` and `B` requires a `__eq__` overload`；
  索引一个没有 `__index__` 的对象报 `requires an `__index__` overload`。
- `str` 的 `+`（拼接）、`Array<T>` 的 `+` 是内置语义，不受影响；`int`/`float`/`str`
  的词比较也走内置路径。
- 重载方法体可以复用其它重载/普通方法。

## 20.1 示例

```sloth
{{#include examples/overload.sl}}
```

```mlir
{{#include examples/overload.mlir}}
```

`a + b` 发射为直接方法调用 `@sloth_main_Vec2____add__`（编译期解析，无虚表往返）；
`bag[0] = 7` 调用 `__assign__`，`bag[0]` 调用 `__index__`。
