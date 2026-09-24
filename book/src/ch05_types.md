# 5. 类型系统与内存表示

## 5.1 类型全集

| 类别 | 类型 | 说明 |
| --- | --- | --- |
| 单元 | `unit` | 空返回类型 |
| 布尔 | `bool` | 仅 `bool` 可作条件 |
| 数值 | `int`（有符号 64-bit）、`float`（f64） | **无隐式互转**，用 `int()`/`float()` |
| 定宽整数 | `int8`/`int16`/`int32`、`uint`/`uint8`/`uint16`/`uint32`/`uint64`（别名 `i8`/`u8`/…） | 显式转换、按宽度环绕；不同宽度不可直接混算（见 §5.6） |
| 字符串 | `str` | UTF-8，不可变；无 interning，`==` 按内容比较 |
| 范围 | `range` | `a..b` / `a..=b`，元素 `int` |
| 数组 | `Array<T>` | 同构动态数组 |
| 字典 | `Map<K,V>` | `K` 须为 `Hashable` |
| 可空 | `T?` | `nil` 只能赋给 `T?`；不允许 `T??` |
| 函数 | `(A,B) -> R` | 一等值，闭包为 `{ fnptr, env }` |
| 类 | `class C` | 单继承 |
| 接口 | `trait T` | `impl` 多个 |
| 动态对象 | `dyn Trait` | 运行时多态出口 |
| 顶层 | `any` | 运行时类型化的盒引用（`0` = `nil`）；任意值可隐式装箱 |
| 弱引用 | `Weak<T>` | 不增量目标计数；`upgrade()` → `T?`（见 §15） |
| 协程 | `Fiber<Y>` | 有栈协程，`Y` 为 `resume`/`yield` 共用载荷（见 §27） |
| 张量 | `Tensor<T,R>` | `T` ∈ {`int`,`float`}，`R` 编译期常量（语义 1..=3，见 §25） |
| 线程句柄 | `JoinHandle<R>` | OS 线程结果句柄（见 §28） |
| 通道 | `Channel<T>` | 跨线程 mpmc 队列（见 §28） |

`Mutex` / `AtomicInt` 是不透明内建句柄（`pthread_mutex_t` / `AtomicI64`），随
§28 的线程扩展引入。

## 5.2 词面编码（word plane）

**所有** SSA 值、局部槽、字段、容器元素都是一个 **i64 词**。词面无 tag：引用是
裸 payload 指针（`0` = nil），值是其原生位模式。运行期不再靠 LSB 自描述；对象的
ARC 级联策略由 codegen 发射**每类一个 `(payload, aux) -> i64` 级联例程**（登记为
header `sdtor`），其余种类各带编译期标志（数组 `elref`、Map `kflag`、通道/协程
`eref`）。

| 面 | 编码 | 解码 |
| --- | --- | --- |
| 引用句柄 | 裸 payload 指针（`0` = nil） | 直接使用 |
| `int` | 原生 `i64`（**64-bit**，环绕） | 直接使用 |
| 定宽整数 | 原生 `i64`，运算前按声明宽度**截断/环绕** | 直接使用（按有无符号解释） |
| `float` | 原生 `f64` 位模式（bitcast 到 `i64`） | `i64` bitcast 回 `f64` |
| `bool` | `0` / `1` | `!= 0` |
| `nil` | `0` | `0` |

`int` 与 `float` 保持完整 64-bit 精度；算术在 `i64`/`f64` 标量域进行，仅在词面
边界做 bitcast（`int` 无转换，`float` 用 `llvm.bitcast`）。`float` 打印走 Rust
的 `Display`（最短往返），与 C 的 `%g` 不逐字一致。

引用类型的 `rc` 计数**带内隐藏**在对象头里（payload 之下 6 词 `Hdr`），不再使用
全局计数表；`retain`/`release` 只对编译期判定的引用词发射，`0`（nil）为惰性
no-op。值词不会被 `retain`/`release`（它们本就不是 rc 句柄）。

## 5.3 值类型 vs 引用类型

- **值类型**：`unit` / `bool` / `int` / `float` / 定宽整数 / `range`（`range` 在
  实现里是 `{lo,hi}` 盒的引用词）。赋值/传参为复制。
- **引用类型**：`str` / `Array` / `Map` / 类实例 / 闭包 / `dyn`。赋值/传参为
  引用共享（语义上是别名）。

`T?` 的表示：

- `T` 为引用类型 → 复用句柄，`nil` 即 `0`（空指针优化）。
- `T` 为值类型（`int?`/`float?`/`bool?`）→ 指向一个 rc 跟踪的 payload 盒的引用
  词，`0` = `nil`。因此值 `0` / `0.0` / `false` 是**活盒**，不会与 `nil` 混淆。

`Weak<T>` 是一个独立的 rc 盒，不增加目标计数；目标归零时沿弱链失效，
`upgrade()` 返回 `T?`（死目标为 `nil`）。见 §15。

`any` 是一个值型单 word 的引用类型：`0` = `nil`，否则指向 rc 盒
`{ TypeDesc*, word }`。装箱发生在静态类型到 `any` 的隐式转换处（赋给 `any`
变量/字段、传入 `any` 形参、`print`/`${}`），因此强类型代码的热路径仍然无 tag。
`x is T` 在 `any` 上做运行时判定（标量按 kind、类按运行时 class id 链），
命中后可在分支内收窄为具体类型。见 §24.2。

## 5.4 类型推断

**局部**推断（非全局 Hindley–Milner）：

- `var x = expr;` 的类型取自初值；`let` 同理但绑定不可重赋。
- **必须标注**的位置：函数参数、类字段、（不能从单个 `return` 推出的）返回类型、
  无法从实参推断的泛型参数。
- 显式标注与推断冲突时报 `type mismatch`。

## 5.5 示例：编码如何体现在 MLIR

```sloth
{{#include examples/wordplane.sl}}
```

```mlir
{{#include examples/wordplane.mlir}}
```

对着这段 MLIR 可以读到无 tag 词面的形态：`1 + 2` 直接是 `arith.constant 3 : i64`
（常量折叠）或 `arith.addi`；`float` 只在标量域出现——`llvm.bitcast : i64 to f64`
取出 `f64`、`arith.addf` 相加、`llvm.bitcast : f64 to i64` 收回词面；`3 < 4` 用
`arith.cmpi`，结果经 `arith.extui : i1 to i64` 得到 `bool` 词（`true` = `1`，不再
是去 tag 前的 `2`）；`nil` 就是常量 `0`。

## 5.6 定宽与无符号整数

在默认的 `int`（有符号 64-bit）之外，语言提供一族定宽整数（对齐设计 §3.3 的
位运算扩展）：

| 有符号 | 无符号 | 位宽 |
| --- | --- | --- |
| `int8`（`i8`） | `uint8`（`u8`） | 8 |
| `int16`（`i16`） | `uint16`（`u16`） | 16 |
| `int32`（`i32`） | `uint32`（`u32`） | 32 |
| `int`（唯一名称） | `uint`（`uint64`/`u64`） | 64 |

语义要点：

- **显式转换**：`int8(x)` / `uint32(x)` / `uint(x)` … 把操作数截断到目标宽度
  （按二补码环绕）：`int8(300) == 44`、`uint8(250) + 10 == 4`、`uint(-1)` 是
  `18446744073709551615`。
- **不隐式加宽**：不同定宽操作数混算/比较报
  `type mismatch in arithmetic operand` / `…comparison operand`，需显式转换；
  `int` 字面量会自动采用另一操作数的宽度。
- **有/无符号运算符**：`/`、`%`、`>>`、`< <= > >=` 按操作数的有无符号选择
  `divsi/divui`、`remsi/remui`、`shrsi/shrui`、`slt/ult` 等（见 §7）。
- **`is` 按宽度判定**：`int8(5) is int8` 为 `true`，`is int` 为 `false`；在 `any`
  上同样精确到具体宽度。
- **提升到 float**：`uint`/`uint8`/… 转 `float` 用无符号提升，保留量值
  （`float(uint(-1)) > 1.0e18` 为 `true`）。
- **字面量**：`5u` 强制无符号；超过 `i64::MAX` 的十进制字面量是 `uint`（见 §4.2）。

> 别名只在**类型位置**可用；有符号 64-bit 没有别名，其类型名就是 `int`
> （`int64`/`i64` 不是可声明的类型名）。
