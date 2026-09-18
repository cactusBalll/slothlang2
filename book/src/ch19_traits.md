# 19. Trait 与 dyn

## 19.1 声明与实现

```sloth
trait Speaker {
    func name(): str;                        // 必需方法
    func say(): str { return "I am ${this.name()}"; }  // 默认方法体
}

class Cat impl Speaker {
    func name(): str { return "cat"; }
}
class Dog impl Speaker, Display { ... }      // 可 impl 多个 trait
```

- trait 无字段、无构造器；方法可给**默认实现**，默认体里对 `this` 的方法调用是
  **虚**的（子类 override 生效）。
- 类必须实现所有无默认体的方法，否则报 `class `X` does not satisfy trait `Y``。

## 19.2 `dyn Trait`

`dyn Trait` 是运行时多态的唯一出口：

```sloth
var xs: Array<dyn Speaker> = [Cat(), Dog()];
for x in xs { print(x.say()); }
func announce(s: dyn Speaker): str { return s.say(); }
```

表示上，`dyn` 对象携带**虚表指针**（单字段，而非设计文档的"数据指针 + 虚表指针"
两字 fat pointer；运行时等价，见附录 A）。调用经虚表槽
（`sloth_vt_get`）；每条 (trait, 方法) 有一个槽；缺实现落到
`sloth_panic_noimpl`。

## 19.3 预定义 trait

| trait | 方法 | 用途 |
| --- | --- | --- |
| `Display` | `to_str(): str` | `print(x)`、`"${x}"` 插值 |
| `Hashable` | `__hash__(): int` | `Map<K,_>` 的键约束（**必须同时 `impl Equatable`**） |
| `Equatable` | `__eq__(other)`: bool | `==` / `!=` |
| `Comparable` | `__lt__`/`__le__`/`__gt__`/`__ge__` | 比较族 |

`int`/`float`/`bool`/`str`/`range` 内置实现 `Hashable`/`Display`。用户类若不
`impl Display`，对它 `print` 或插值报 `requires trait bound `Display``。

## 19.4 示例

```sloth
{{#include examples/traits.sl}}
```

```mlir
{{#include examples/traits.mlir}}
```

要点：`Cat`/`Dog` 的虚表在全局以 `memref.global @sloth_main_g_vtb_*` 形式构建
（`sloth_vt_new` + `sloth_vt_set`），实例化时 `sloth_obj_set_vtable` 绑上；
`x.say()`（`dyn`）先 `sloth_obj_vtable` 取表再 `sloth_vt_get` 取槽调用。
`announce` 的形参 `s: dyn Speaker` 也是带虚表的引用词。`"${Dog()}"` 走
`to_str()` 后拼接。
