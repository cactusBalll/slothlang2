# 4. 词法、字面量与字符串插值

## 4.1 注释与词法外壳

```sloth
// 行注释
/* 块注释，可跨行 */
```

- 标识符：`[A-Za-z_][A-Za-z0-9_]*`。
- 保留字（不可作标识符）：`and or not true false for var let if else while func
  nil return class super this break continue is pub trait impl dyn as`。
- `int` / `float` / `bool` / `str` / `unit` / `range` / `Array` / `Map`
  只在**类型位置**有意义，作为普通标识符可用。
- `&&` / `||` 是 `and` / `or` 的同义词。

## 4.2 字面量

| 形态 | 例子 | 类型 |
| --- | --- | --- |
| 十进制整数 | `42`、`4611686018427387903` | `int`（64-bit） |
| 十进制浮点 | `3.5` | `float` |
| 科学计数法 | `1e3`、`2.5e-1`、`2E+4` | `float` |
| 布尔 | `true` / `false` | `bool` |
| 字符串 | `"..."` | `str` |
| 空 | `nil` | 仅属 `T?` |

`int` 在词面就是原生 i64，**取值范围是完整 64 bit**；超出范围的字面量按环绕语义
截断。`float` 是原生 IEEE f64（满精度），详见 §5。

## 4.3 字符串：转义、插值、内联打包

- 支持的转义：`\n \t \r \" \\`。
- 插值 `${expr}` 在**编译期**展开为"拼接 + `Display`"；`expr` 的类型需实现
  `Display`（`int` / `float` / `bool` / `str` / `range` 内置实现，用户类需
  `impl Display` 提供 `to_str()`，见 §19）。
- 字面量在 MLIR 中按 **8 字节一组打包成 `i64` 常量**，经 `sloth_str_push` 写入
  字符串构建器，最后由 `sloth_str_finish` 分配出最终 `str`（不做驻留）。

## 4.4 示例

```sloth
{{#include examples/literals.sl}}
```

输出：

```text
42
3.5
true
text
1000
0.25
tab	here
quote " inside
1 + 2 = 3
hi sloth, len=5
```

对应 MLIR：

```mlir
{{#include examples/literals.mlir}}
```

在 MLIR 里可以直接看到浮点的 tag 编解码：`arith.shli %c, 1` 再
`llvm.bitcast : i64 to f64` 得到原始 `f64`，运算后
`llvm.bitcast : f64 to i64` 再 `arith.andi %w, -2` 还原词面。
