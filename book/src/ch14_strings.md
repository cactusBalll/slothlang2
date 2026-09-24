# 14. 字符串 str

## 14.1 语义

- `str` 是 UTF-8、**不可变**的。**不做 interning/池化**：每次构造（字面量、
  拼接、切片、取字符……）都在堆上新分配一个 `StrT`，内容相等的两个字符串是
  **不同对象**。因此 `==` 走 `__sloth_str_eq` 的**内容**比较（len + memcmp），
  不能依赖句柄相等；`Map<str, …>` 的键同样按内容哈希/比较。
- `len(s)` / `s.len()` 返回**字节长度**（不是字符数）：`"héllo".len() == 6`。
- 迭代 `for c in s` 按 **Unicode 字符**（scalar）产出子串，非 ASCII 不会产生非法
  UTF-8 片段（`__sloth_str_clen` 数字符、`__sloth_str_char` 取第 i 个字符）。
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
- **与 `Array` 切片的对照**：`str` 下标是**按字节**的；`Array<T>` 另有一套
  **元素级**区间切片 `a[lo..hi]` / `a[lo..=hi]`（返回新副本，见 §12.1），两套
  语法形态一致但语义不同。

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

- `chars()` 惰性求值、不预先物化整个字符串；底层用 `__sloth_str_clen`（字符数）
  与 `__sloth_str_codepoint`（第 i 个码点）。迭代器 `StrChars` 由编译器自动注入
  （源码形态 `lib/prelude/strchars.slt`）。
- 与 `for c in s` 的区别：`for c in s` 每轮产出一个**单字符 `str` 子串**；
  `chars()` 每轮产出一个**码点 `int`**，且可作为迭代器值传递 / 手动 `next()`。
- 迭代器协议（`iter()`/`next(): int?`）见 §16。

## 14.4 标准库 `sloth/str.slt`

`import "sloth/str.slt";` 提供一组常用的字符串处理函数（纯 Sloth，构建在
运行时的 `__sloth_str_*` 与语言内建之上）。**所有函数都是自由函数**（`str` 是
内建类型，不能通过 `impl` 挂方法），命名统一用 `str_` 前缀。

索引约定与核心语言一致：

- **字节面**（与 `len(s)` / `s[i]` / `s[a..b]` 对应）：`str_len`、`str_byte`、
  `str_slice`、`str_find`；
- **字符面**（与 `for c in s` / `s.chars()` 对应，按 Unicode scalar 计数）：
  `str_clen`、`str_char_at`、`str_substr`、`str_take`、`str_drop`、
  `str_index_of`、`str_last_index_of`。

| 分类 | 函数 |
| --- | --- |
| 长度 / 基础 | `str_len` `str_clen` `str_is_empty` `str_eq` `str_cmp` `str_byte` `str_of_byte` `str_slice` `str_find` |
| 查找 / 判定 | `str_starts_with` `str_ends_with` `str_contains` `str_count` `str_index_of` `str_last_index_of` |
| 字符切片 | `str_char_at` `str_substr` `str_take` `str_drop` |
| 修剪 | `str_is_space` `str_trim` `str_trim_start` `str_trim_end` |
| 大小写 | `str_to_upper` `str_to_lower`（ASCII；非 ASCII 原样保留） |
| 字符分类 | `str_is_alpha` `str_is_digit` `str_is_alnum` `str_is_upper` `str_is_lower` `str_is_hex` `str_is_digits` |
| 分割 / 连接 | `str_split` `str_split_lines`（CRLF 感知） `str_join` |
| 替换 / 重复 / 反转 | `str_replace` `str_replace_first` `str_repeat` `str_reverse` |
| 填充 | `str_pad_start` `str_pad_end` |
| 数值解析 | `str_to_int` `str_to_int_or` `str_to_float` |

```sloth
import "sloth/str.slt";

func main(): unit {
    let csv = "  alpha, beta ,gamma  ";
    let parts = str_split(str_trim(csv), ",");   // ["alpha", " beta ", "gamma"]
    for i in 0..parts.len() {
        print(str_trim(parts[i]));
    }
    print(str_join(["a", "b", "c"], "-"));       // a-b-c
    print(str_to_upper("héllo"));                // HéLLO（é 保留）
    print(str_substr("aé中", 1, 2));             // é中（按字符）
    print(str_to_int_or("nope", -1));            // -1
}
```

> 说明：`str_slice` / `str_find` / `str_starts_with` 等与 `sloth/io.slt` 中的同名
> 低层助手语义完全一致（两者同时导入不会产生语义分歧）；字符分类函数按 **ASCII**
> 判定，多字节字符一律不匹配。字符串不可变，每个结果都是新分配（§14.1）。

## 14.5 示例

```sloth
{{#include examples/strings.sl}}
```

```mlir
{{#include examples/strings.mlir}}
```

要点：

- 字面量按 8 字节打包成 `i64` 常量，经 `__sloth_str_push` 入构建器、
  `__sloth_str_finish` 收口为一个新分配的 `str`（`__sloth_str_intern` 是历史命名，
  并不做驻留）。
- 拼接走 `__sloth_str_concat`，结果也是新分配（因此 `"ab" == "a" + "b"` 为真靠的是
  内容比较，而非句柄同一）。
- 逐字符迭代用 `__sloth_str_clen`（字符数）+ `__sloth_str_char`（第 i 个字符子串），
  每轮产出的是 owned 临时量，在迭代末（含 `break`）结算。
- 字节下标 / 切片走 `__sloth_str_byte` / `__sloth_str_slice`；`chars()` 走
  `__sloth_str_clen` + `__sloth_str_codepoint`（注入的 `StrChars` 类实现
  `iter()`/`next(): int?`，惰性产出码点）。
