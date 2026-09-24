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
- **不支持区间切片**：数组只支持单下标 `a[i]` / `a[i] = v`；`a[1..3]` **不是**
  数组切片，编译器会给出诊断（`Array does not support range slicing`）。区间切片是
  `str` 独有的下标语法（`s[a..b]` / `s[a..=b]`，见 §14.2）；需要子数组时请用
  显式循环 / 复制。

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
`sloth_arr_len`。字面量 `[1,2,3]` 发射为 `sloth_arr_new_k` + 逐元素 `sloth_arr_set`。
`sloth_rc_retain`/`sloth_rc_release` 出现在把引用元素写入/别名绑定时。

> 自举：这些符号由注入的 `lib/prelude/containers.slt` 用 sloth 自身实现
> （`libsloth_rt.so` 只提供裸分配/字级内存/rc）。数组头 `[len,cap,buf]` 与
> 稳定句柄语义见附录 B.2。
