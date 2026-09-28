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
- 类必须实现所有无默认体的方法，否则报 `trait `Y` method `m` not implemented by `X``。

## 19.2 `dyn Trait`

`dyn Trait` 是运行时多态的唯一出口：

```sloth
var xs: Array<dyn Speaker> = [Cat(), Dog()];
for x in xs { print(x.say()); }
func announce(s: dyn Speaker): str { return s.say(); }
```

表示上，`dyn` 对象携带**虚表指针**（单字段，而非设计文档的"数据指针 + 虚表指针"
两字 fat pointer；运行时等价，见附录 A）。调用经虚表槽
（`__sloth_vt_get`）；每条 (trait, 方法) 有一个槽；缺实现落到
`__sloth_panic_noimpl`。

## 19.3 预定义 trait

| trait | 方法 | 用途 |
| --- | --- | --- |
| `Display` | `to_str(): str` | `print(x)`、`"${x}"` 插值 |
| `Hashable` | `__hash__(): int` | `Map<K,_>` 的键约束（**必须同时 `impl Equatable`**） |
| `Equatable` | `__eq__(other)`: bool | `==` / `!=` |
| `Comparable` | `__lt__`/`__le__`/`__gt__`/`__ge__` | 比较族 |

编译器**按名识别**这四组 trait 及其方法签名（例如 `print`/插值按 `to_str`、
`Map` 键按 `__hash__`、`==` 按 `__eq__`），但它们**不是自动注入的声明**：需要
由用户代码或标准库显式 `trait … { … }` 声明（本章示例均如此）。未声明时
`class C impl Display { … }` 会报 `unknown trait Display in impl`。这与附录 A
的偏差说明一致。

`int`/`float`/`bool`/`str`/`range` 内置实现 `Hashable`/`Display`。用户类若不
`impl Display`，对它 `print` 或插值会回退为类名输出（不报错）。

## 19.4 值类型自动装箱

内建值类型 `int`/`float`/`bool` 隐式实现预定义 trait（`Display`/`Equatable`/
`Hashable`/`Comparable`），也可满足**无方法**的 trait（如 `trait Any {}`）。
在这些 trait 的 `dyn` 位置上，值类型会**自动装箱**为一个合成对象：

```sloth
trait Any {}
trait Display { func to_str(): str; }

var a: dyn Any = 42;          // 装箱：class-id + 虚表 + 字段0 = 值词
if a is int {                 // 运行时 class-id 判定
    var n: int = a;           // 收窄并拆箱
    print(n);
}
var d: dyn Display = 7;
print(d);                     // 经虚表 to_str() 打印
```

- 装箱对象：保留的负 class-id、按 (值类型, trait) 惰性构建的虚表、`word2` 存值词；
  本身是普通 rc 对象，死亡级联无需特例（值词永不参与引用释放）。
- 方法桥：codegen 为每个 `(kind, 方法)` 生成薄 `llvm.func`，转发到运行时
  `__sloth_dyn_to_str`/`__sloth_dyn_hash`/`__sloth_dyn_binop`（`crates/sloth-rt/src/builtins.rs`）。
- 只允许值类型进入它**能满足**的 `dyn`：带方法但非预定义的 trait（如
  `trait Speaker { func noise(): str; }`）会把 `var s: dyn Speaker = 42` 报为
  `type mismatch in initializer`。

## 19.5 示例

```sloth
{{#include examples/traits.sl}}
```

```mlir
{{#include examples/traits.mlir}}
```

要点：`Cat`/`Dog` 的虚表在全局以 `memref.global @sloth_main_g_vtb_*` 形式构建
（`__sloth_vt_new` + `__sloth_vt_set`），实例化时 `__sloth_obj_set_vtable` 绑上；
`x.say()`（`dyn`）先 `__sloth_obj_vtable` 取表再 `__sloth_vt_get` 取槽调用。
`announce` 的形参 `s: dyn Speaker` 也是带虚表的引用词。`"${Dog()}"` 走
`to_str()` 后拼接。
