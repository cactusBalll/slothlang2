# 5. 类型系统与内存表示

## 5.1 类型全集

| 类别 | 类型 | 说明 |
| --- | --- | --- |
| 单元 | `unit` | 空返回类型 |
| 布尔 | `bool` | 仅 `bool` 可作条件 |
| 数值 | `int`（63-bit）、`float`（f63） | **无隐式互转**，用 `int()`/`float()` |
| 字符串 | `str` | UTF-8，不可变；无 interning，`==` 按内容比较 |
| 范围 | `range` | `a..b` / `a..=b`，元素 `int` |
| 数组 | `Array<T>` | 同构动态数组 |
| 字典 | `Map<K,V>` | `K` 须为 `Hashable` |
| 可空 | `T?` | `nil` 只能赋给 `T?`；不允许 `T??` |
| 函数 | `(A,B) -> R` | 一等值，闭包为 `{ fnptr, env }` |
| 类 | `class C` | 单继承 |
| 接口 | `trait T` | `impl` 多个 |
| 动态对象 | `dyn Trait` | 运行时多态出口 |

## 5.2 词面编码（word plane）

**所有** SSA 值、局部槽、字段、容器元素都是一个**带 tag 的 `i64`**，tag 是
最低位（LSB）：

| 面 | 编码 | 解码 |
| --- | --- | --- |
| 引用句柄 | `ptr \| 1`（payload 16 对齐，bit0 恒空） | `w & !1` |
| `int` | `v << 1`（**63-bit**，环绕） | 算术右移 1 |
| `float` | 字面量 `(bits & !1) >> 1`；算术发射 `(bits & !3) >> 1` | `w << 1` 后 bitcast |
| `bool` | `0` / `2` | `!= 0` |
| `nil` | `0` | `0` |

由此产生的**语义代价**（均为有意取舍，已定案）：

- `int` 收窄为 63 bit；
- `float` 是"降精度 f64"：词面编码会丢弃尾数低位。编译期**字面量**路径丢 1 位
  （`(bits & ~1) >> 1`），而**运行时算术边界**用 `(bits & ~3) >> 1` 额外清一位，
  于是运算结果相对 IEEE 再被量化一次。实测：字面量 `1.0000000000000004` 原样打印，
  而 `1.0000000000000004 + 0.0` 得 `1`。这是发射器与运行时编码掩码不一致导致的
  （详见[附录 A](appendix_a_deviations.md)）；
- `float` 打印走 Rust 的 `Display`（最短往返），与 C 的 `%g` 不逐字一致。

引用类型的 `rc` 计数**带内隐藏**在对象头里（payload 之下 6 词 `Hdr`），不再使用
全局计数表；`retain`/`release` 先查 tag，tag=0 直接惰性 no-op、不触内存。

## 5.3 值类型 vs 引用类型

- **值类型**：`unit` / `bool` / `int` / `float` / `range`（`range` 在实现里是
  `{lo,hi}` 盒的引用词）。赋值/传参为复制。
- **引用类型**：`str` / `Array` / `Map` / 类实例 / 闭包 / `dyn`。赋值/传参为
  引用共享（语义上是别名）。

`T?` 的表示：

- `T` 为引用类型 → 复用句柄，`nil` 即 `0`（空指针优化）。
- `T` 为值类型（`int?`/`float?`/`bool?`）→ 指向一个 rc 跟踪的 payload 盒的引用
  词，`0` = `nil`。因此值 `0` / `0.0` / `false` 是**活盒**，不会与 `nil` 混淆。

`Weak<T>` 是一个独立的 rc 盒，不增加目标计数；目标归零时沿弱链失效，
`upgrade()` 返回 `T?`（死目标为 `nil`）。见 §15。

## 5.4 类型推断

**局部**推断（非全局 Hindley–Milner）：

- `var x = expr;` 的类型取自初值；`let` 同理但绑定不可重赋。
- **必须标注**的位置：函数参数、类字段、（不能从单个 `return` 推出的）返回类型、
  无法从实参推断的泛型参数。
- 显式标注与推断冲突时报 `type mismatch`。

## 5.5 示例：编码如何体现在 MLIR

```sloth
{{#include examples/wordplane.sl}}
```

```mlir
{{#include examples/wordplane.mlir}}
```

对着这段 MLIR 可以读到整形的完整解码-运算-编码序列：`arith.shrsi` 解出原始
`int`、`arith.addi` 相加、`arith.shli` 重新编码；比较用 `arith.cmpi`，结果经
`arith.extui` 变成 `bool` 词（`true` = `2`）。
