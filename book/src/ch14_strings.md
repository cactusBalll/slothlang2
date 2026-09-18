# 14. 字符串 str

## 14.1 语义

- `str` 是 UTF-8、**不可变**的。**不做 interning/池化**：每次构造（字面量、
  拼接、切片、取字符……）都在堆上新分配一个 `StrT`，内容相等的两个字符串是
  **不同对象**。因此 `==` 走 `sloth_str_eq` 的**内容**比较（len + memcmp），
  不能依赖句柄相等；`Map<str, …>` 的键同样按内容哈希/比较。
- `len(s)` / `s.len()` 返回**字节长度**（不是字符数）：`"héllo".len() == 6`。
- 迭代 `for c in s` 按 **Unicode 字符**（scalar）产出子串，非 ASCII 不会产生非法
  UTF-8 片段（`sloth_str_clen` 数字符、`sloth_str_char` 取第 i 个字符）。
- 拼接用 `+`：`"a" + "b"`，结果是一个**新分配**的字符串（同样不共享/去重）。
- 转义与插值见 §4。

## 14.2 示例

```sloth
{{#include examples/strings.sl}}
```

```mlir
{{#include examples/strings.mlir}}
```

要点：

- 字面量按 8 字节打包成 `i64` 常量，经 `sloth_str_push` 入构建器、
  `sloth_str_finish` 收口为一个新分配的 `str`（`sloth_str_intern` 是历史命名，
  并不做驻留）。
- 拼接走 `sloth_str_concat`，结果也是新分配（因此 `"ab" == "a" + "b"` 为真靠的是
  内容比较，而非句柄同一）。
- 逐字符迭代用 `sloth_str_clen`（字符数）+ `sloth_str_char`（第 i 个字符子串），
  每轮产出的是 owned 临时量，在迭代末（含 `break`）结算。
