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

## 14.2 下标：按字节取单字节 / 取字节区间

`str` 的下标是**按字节（byte）**的（UTF-8 原始字节，不是 Unicode 字符）：

```sloth
let s = "héllo";
s[0];        // 104        (int, 0..255) — 'h' 的字节
s[1];        // 195        é 的首字节 0xC3
s[2];        // 169        é 的续字节 0xA9
s[1..3];     // "é"        [1,3) 字节切片，新分配的 str
s[1..=3];    // "él"       [1,3] 闭区间字节切片
s[1..1];     // ""         空切片
```

- `s[i]` 返回 `int`（0..255）。`i` 必须是 `int`；越界在运行时 panic
  （`str byte index ... out of bounds`）。
- `s[a..b]` / `s[a..=b]` 返回**新分配**的 `str`（字节 `[a,b)` / `[a,b]`）；
  边界非法（负、倒置、越界）在运行时 panic（`str slice ...`）。
- **不是按字符**：多字节字符的中间字节可以被单独取出，可能得到非法 UTF-8 的
  切片——这是刻意的低层视图（供 tokenizer、协议解析等使用）。按字符的视图见
  `chars()` 与 `for c in s`。
- **`Array<T>` 不支持区间切片**：`a[1..3]` 不是数组切片（数组只支持单下标
  `a[i]` / `a[i] = v`，见 §12），编译器会报
  `Array does not support range slicing`。区间切片是 `str` 独有的下标语法；需要
  子数组时用显式循环 / 复制。

## 14.3 `chars()`：惰性 UTF-8 码点迭代器

`s.chars()`（等价 `chars(s)`）把字符串包装为一个**惰性迭代器**：每个元素是一个
字符（UTF-8 解码后的 Unicode scalar）的**码点**，类型为 `int`（固定 4 字节足以
表示任一码点）：

```sloth
let t = "aé中";
for c in t.chars() {
    // c 依次为 97 / 233 / 20013
}
// 也可作为一等迭代器值显式驱动协议：
let it = t.chars();
it.next();   // 97
it.next();   // 233
it.next();   // 20013
it.next();   // nil
```

- `chars()` 惰性求值、不预先物化整个字符串；底层用 `sloth_str_clen`（字符数）
  与 `sloth_str_codepoint`（第 i 个码点）。
- 与 `for c in s` 的区别：`for c in s` 每轮产出一个**单字符 `str` 子串**；
  `chars()` 每轮产出一个**码点 `int`**，且可作为迭代器值传递 / 手动 `next()`。
- 迭代器协议（`iter()`/`next(): int?`）见 §16。

## 14.4 示例

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
- 字节下标 / 切片走 `sloth_str_byte` / `sloth_str_slice`；`chars()` 走
  `sloth_str_clen` + `sloth_str_codepoint`（注入的 `StrChars` 类实现
  `iter()`/`next(): int?`，惰性产出码点）。
