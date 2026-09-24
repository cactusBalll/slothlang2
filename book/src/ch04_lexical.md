# 4. 词法、字面量与字符串插值

## 4.1 注释与词法外壳

```sloth
// 行注释
/* 块注释，可跨行 */
```

- 标识符：`[A-Za-z_][A-Za-z0-9_]*`。
- 保留字（不可作标识符）：`and or not true false for var let if else while func
  nil return class super this break continue is pub trait impl dyn as`。
- `int` / `float` / `bool` / `str` / `unit` / `range` / `Array` / `Map`，以及
  定宽整数类型名（`int8`/`int16`/`int32`、`uint`/`uint8`/`uint16`/`uint32`/`uint64`
  及其别名 `i8`/`u8`/…）只在**类型位置**有意义，作为普通标识符可用。
- `&&` / `||` 是 `and` / `or` 的同义词。

## 4.2 字面量

| 形态 | 例子 | 类型 |
| --- | --- | --- |
| 十进制整数 | `42`、`4611686018427387903` | `int`（64-bit） |
| 无符号整数字面量 | `5u`、`18446744073709551615` | `uint`（64-bit） |
| 十进制浮点 | `3.5` | `float` |
| 科学计数法 | `1e3`、`2.5e-1`、`2E+4` | `float` |
| 布尔 | `true` / `false` | `bool` |
| 字符串 | `"..."` | `str` |
| 空 | `nil` | 仅属 `T?` |

`int` 在词面就是原生 i64，**取值范围是完整 64 bit**；超出范围的字面量按环绕语义
截断。整数字面量可加后缀 `u`/`U`（如 `5u`）强制为无符号 `uint`；超出 `i64::MAX`
的十进制字面量（如 `18446744073709551615`）本身也只能是 `uint`。`float` 是原生
IEEE f64（满精度），详见 §5。

## 4.3 字符串：转义、插值、内联打包

- 支持的转义：`\n \t \r \" \\`。
- 插值 `${expr}` 与 `print(x)` 统一走运行时渲染器 `__sloth_rt_write`（见 §24.2）：
  `int`/`float`/`bool`/`str`/`range` 与数组/Map 等容器都有内置渲染；用户类若
  `impl Display` 则调用其 `to_str()`，否则回退为类名（不报错，见 §19.3）。
- 字面量在 MLIR 中按 **8 字节一组打包成 `i64` 常量**，经 `__sloth_str_push` 写入
  字符串构建器，最后由 `__sloth_str_finish` 分配出最终 `str`（不做驻留）。

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

字面量在 MLIR 里就是词面常量：`42` 直接是 `arith.constant 42 : i64`；`3.5` 是其
IEEE-754 位模式 `arith.constant 4613937818241073152 : i64`；`true` 是 `1`。`print`
把每个值经 `__sloth_any_from(desc, word)` 装箱为 `any` 再渲染，因此这里看不到
浮点运算——需要标量运算时才有 `llvm.bitcast : i64 to f64` / `f64 to i64`（见 §5.5
的 wordplane 示例）。词面**没有 tag 编解码**（去 tag 迁移后不再有
`arith.shli`/`shrsi`/`andi` 掩码）。
