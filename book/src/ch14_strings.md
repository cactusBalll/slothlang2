# 14. 字符串 str

## 14.1 语义

- `str` 是 UTF-8、**不可变**、**interned** 的：内容相等的字符串是同一池条目，
  因此 `==` 近似指针比较（实现用 `sloth_str_eq` 走值语义，保证正确）。
- `len(s)` / `s.len()` 返回**字节长度**（不是字符数）：`"héllo".len() == 6`。
- 迭代 `for c in s` 按 **Unicode 字符**（scalar）产出子串，非 ASCII 不会产生非法
  UTF-8 片段（`sloth_str_clen` 数字符、`sloth_str_char` 取第 i 个字符）。
- 拼接用 `+`：`"a" + "b"`，结果为新的 interned 字符串。
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
  `sloth_str_finish` 收口 intern。
- 拼接走 `sloth_str_concat`，结果统一 intern（因此 `"ab" == "a" + "b"` 为真）。
- 逐字符迭代用 `sloth_str_clen`（字符数）+ `sloth_str_char`（第 i 个字符子串），
  每轮产出的是 owned 临时量，在迭代末（含 `break`）结算。
