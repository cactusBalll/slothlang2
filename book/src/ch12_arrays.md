# 12. 数组 Array

## 12.1 基本操作

```sloth
var a = [1, 2, 3];
a.len();        // 或 len(a)
a[0];           // 索引读
a[0] = 10;      // 索引写
a.push(4);      // 尾部追加
a.pop();        // 尾部弹出（返回被移除元素）
```

- `Array<T>` 同构；元素类型取字面量各元素类型的**最近公共祖先**（类字面量会
  上转为公共基类）。
- 空字面量 `[]` 需要**上下文类型**：`var a: Array<float> = [];`。否则默认
  `Array<int>`。
- 越界访问在发射期插入边界守卫，命中走运行时 panic（`sloth_panic_oob`）。
- 元素赋值有词面检查：`a[0] = "x"`（`a: Array<int>`）报
  `type mismatch in array element assignment`。
- **区间切片（新副本）**：`a[lo..hi]` 取 `[lo, hi)`、`a[lo..=hi]` 取闭区间，
  返回一个**新分配的 `Array<T>` 副本**（不是共享视图）；写入副本不影响原数组。
  边界非法（负、倒置、越界）在运行时 panic（`array slice ... out of bounds`）。
  切片读可继续下标 / 再切片（`a[2..5][0]`、`a[1..4][0..2]`）。
- **切片赋值**：`a[lo..hi] = src` 把另一个 `Array<T>`（或数组字面量）按元素写回
  原数组的该区间；`src` 长度必须等于区间长度，否则运行时 panic
  （`array slice assignment length ... does not match target length ...`）。
  右值不是数组是编译期诊断（`array slice assignment expects a matching Array<T>`）。

## 12.2 稳定句柄（重要语义）

运行时数组的 rc 头固定 3 词 `[len, cap, buf]`，元素存放在**独立的非追踪缓冲**中；
增长只 `realloc` 缓冲，**句柄永不移动**。因此：

- `var b = a; a.push(...)` 后 `b` 能看到增长；
- 把数组传给函数，被调函数 `push` 后调用方句柄依旧有效；
- 闭包捕获的数组、嵌套容器 `g[0].push(...)` 均成立。

（这一点修复了早期"增长即搬迁、破坏别名"的缺陷。）

## 12.3 示例

```sloth
{{#include examples/arrays.sl}}
```

```mlir
{{#include examples/arrays.mlir}}
```

对应运行时符号：`sloth_arr_new`（建数组，参数是元素种类 `k`，用于区分值/引用
元素的级联释放）、`sloth_arr_push`/`sloth_arr_pop`/`sloth_arr_get`/`sloth_arr_set`/
`sloth_arr_len`、`sloth_arr_slice`/`sloth_arr_slice_set`（区间读副本 / 区间写）。
字面量 `[1,2,3]` 发射为 `sloth_arr_new_k` + 逐元素 `sloth_arr_set`。
`sloth_rc_retain`/`sloth_rc_release` 出现在把引用元素写入/别名绑定时。

> 自举：这些符号由注入的 `lib/prelude/containers.slt` 用 sloth 自身实现
> （`libsloth_rt.so` 只提供裸分配/字级内存/rc）。数组头 `[len,cap,buf]` 与
> 稳定句柄语义见附录 B.2。

## 12.4 标准库 `sloth/array.slt`

`import "sloth/array.slt";` 提供一组常用的 `Array<T>` 操作（纯 Sloth，构建在语言
内建的下标 / 区间切片 / `push` / `pop` / `len` 之上）。**所有函数都是自由函数**
（`Array<T>` 是内建类型，不能通过 `impl` 挂方法），命名统一用 `array_` 前缀。

索引与越界约定：

- 下标从 0 开始；`array_index_of` / `array_last_index_of` / `array_binary_search*`
  未命中返回 `-1`；
- “钳制”族（`array_take` / `array_drop` / `array_slice` / `array_get_or` /
  `array_insert`）对越界参数**钳制到边界、永不 panic**；内建 `a[i]` / `a[lo..hi]`
  仍是会 panic 的下标形式；`array_remove_at` 与 `a[i]` 一致，越界 panic。

排序约定：

- `T: Comparable` 的 `array_sort` / `array_min` / `array_max` / `array_is_sorted` /
  `array_binary_search` 使用 `<` / `>`：`int` / `float` 为内容比较，类走
  `__lt__` / `__gt__`；
- 当前语言的 `str` `<` 是**句柄**比较，因此字符串排序请用 `array_sort_str` /
  `array_min_str` / `array_max_str` / `array_binary_search_str`（按字节字典序，
  底层 `__sloth_str_cmp`），或用 `array_sort_by` 传入 `sloth/str.slt` 的 `str_cmp`；
- `array_sort_by` 接收三路比较器 `(T, T) -> int`：负 = 左前，0 = 相等，正 = 右前
  （与 `str_cmp` 同约定）。排序**不稳定**。

高阶函数接收函数值；作为谓词 / 比较器的 lambda **必须标注返回类型**
（如 `|x: int| -> bool { … }`），未标注的 lambda 体默认按 `int` 推断。

| 分类 | 函数 |
| --- | --- |
| 查询 | `array_is_empty` `array_index_of` `array_last_index_of` `array_contains` `array_count` `array_count_if` `array_any` `array_all` `array_first` `array_last` `array_get_or` |
| 副本 / 切片 | `array_copy` `array_take` `array_drop` `array_slice` |
| 组合 | `array_concat` `array_extend` `array_repeat` `array_reverse` `array_flatten` |
| 变换 | `array_map` `array_map_indexed` `array_filter` `array_filter_map` `array_flat_map` `array_unique` |
| 聚合 | `array_fold` `array_sum_int` `array_sum_float` `array_min` `array_max` `array_min_by` `array_max_by` `array_min_str` `array_max_str` |
| 原地修改 | `array_insert` `array_remove_at` `array_remove` `array_remove_all` `array_clear` `array_swap` `array_reverse_in_place` |
| 排序 / 查找 | `array_sort` `array_sort_desc` `array_sort_by` `array_sort_str` `array_is_sorted` `array_binary_search` `array_binary_search_by` `array_binary_search_str` |
| 相等 / 生成 | `array_equals` `array_iota` `array_range` `array_range_step` |

```sloth
import "sloth/array.slt";

func main(): unit {
    var xs = [3, 1, 4, 1, 5];
    array_sort(xs);                                            // [1, 1, 3, 4, 5]
    print(array_unique(xs));                                   // [1, 3, 4, 5]
    print(array_map(xs, |x: int| -> int { return x * 2; }));   // [2, 2, 6, 8, 10]
    print(array_filter(xs, |x: int| -> bool { return x > 2; })); // [3, 4, 5]
    print(array_fold(xs, 0, |a: int, b: int| -> int { return a + b; })); // 14
    let f = array_first(xs);
    if f is not nil {
        print(f);                                              // 1
    }
    // 字符串按字节字典序：用专用 helpers（`str` 的 `<` 是句柄比较）
    var names = ["banana", "apple", "cherry"];
    array_sort_str(names);                                     // [apple, banana, cherry]
}
```

> 说明：`array_filter_map` 的 `(T) -> U?` 会把 `nil` 结果丢弃；`array_min` /
> `array_max` / `array_first` / `array_last` 等空数组返回 `nil`，调用侧用
> `is not nil` 收窄（§15.2）。
