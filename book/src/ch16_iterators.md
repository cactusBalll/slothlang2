# 16. 迭代协议

## 16.1 协议形态

设计文档把迭代协议放在两个 trait 里：

```sloth
trait Iterator<T> { func next(): T?; }
trait Iterable<T> { func iter(): Iterator<T>; }
```

**当前实现是"结构化的"**：自定义类型只要提供 `iter()` 与 `next()` 两个方法即可被
`for` 使用，**不需要显式 `impl Iterable`**：

```sloth
class Range3 {
    var i: int = 0;
    func iter(): Range3 { return this; }   // 返回迭代器（可以是自身）
    func next(): int? {                    // 穷尽时返回 nil
        if this.i >= 3 { return nil; }
        this.i = this.i + 1;
        return this.i - 1;
    }
}
```

## 16.2 内建 Iterable

| 类型 | 元素 |
| --- | --- |
| `Array<T>` | `T` |
| `Map<K,V>` | `Entry<K,V>`（`key`/`val`） |
| `str` | `str`（按 Unicode 字符） |
| `str.chars()` | `int`（UTF-8 解码后的码点，惰性） |
| `range` | `int` |

> `str` 也可用 `s.chars()` 得到一个**惰性码点迭代器**（元素 `int`，见 §14.3），
> 与 `for c in s`（元素 `str`）互补。`Array<T>` **不支持区间切片**：`a[1..3]`
> 不是数组切片（编译期诊断），区间切片是 `str` 独有语法。

## 16.3 所有权

- `iter()` 的结果是 owned：登记在循环作用域，退出时释放。
- `next()` 的结果是 owned：**每轮迭代末**释放（含 `continue` 路径）；`break` 走
  单独的清理块释放当轮已取元素。
- 若 `for` 的 iterable 本身是 owned 临时量（如 `for x in mkarr()`），该临时量从
  语句账本摘下、交给循环持有，在循环结束时释放（不会被循环头的冲刷提前回收）。

## 16.4 示例

```sloth
{{#include examples/iterators.sl}}
```

```mlir
{{#include examples/iterators.mlir}}
```

`for x in r`（自定义类型）发射为 `r.iter()` 取迭代器、每轮 `iter.next()` 判空；
Map 迭代先用 `sloth_map_keys` 取键快照数组，再逐键 `sloth_map_get`/`sloth_map_str_get`
组装 Entry；**字面量** range（如 `0..=3`）被内联为计数循环，只有一等 range 值才走
`sloth_range_lo`/`sloth_range_hi`（见 §7）。
