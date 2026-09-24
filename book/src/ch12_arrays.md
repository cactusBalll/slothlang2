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
