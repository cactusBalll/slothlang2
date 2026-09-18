# sloth-lang 2.0 设计文档

## 静态强类型化与 MLIR 编译基础设施改造

| 项目 | 内容 |
| --- | --- |
| 文档版本 | v1.1（与实现对齐稿） |
| 文档日期 | 2026-09-18 |
| 基准版本 | v1.0 设计评审稿（2026-09-14） |
| 文档状态 | **已与当前实现对齐**；原 v1.0 中已被实现废弃的表述就地改写，并在段首以 `【已过时·v1.0】` 注明原设计与现行替代 |
| 前置决策 | **有栈协程（fiber）以 `Fiber<Y>` 扩展恢复**（原 v1.0 的「移除 fiber」已随 ARC 改向与原生 ABI 推翻，见 `sloth-lang-2.0协程扩展设计文档.md`） |

> **一致性说明**
>
> 本文修订自 v1.0 设计稿，正文已改写为**当前实现**（`crates/sloth-frontend`、`crates/sloth-codegen`、`crates/sloth-rt`、`slothc`）。
> 与 v1.0 不一致处就地标记 `【已过时·v1.0】`；实现偏差的完整清单另见
> `book/src/appendix_a_deviations.md`，滚动状态见 `PLAN-2026-09-15.md`，
> 张量扩展的独立设计见 `sloth-lang-2.0张量扩展设计文档.md`。

---

## 1. 概述

### 1.1 背景

sloth-lang 1.0 是一种基于解释执行的动态类型脚本语言，其实现架构为：

- 不依赖外部工具的递归下降分析器，one-pass 解析并直接生成字节码；
- 基于栈的虚拟机解释执行字节码；
- 所有值装箱为 `Value`（Tagged Union），类型检查推迟到运行时；
- 简单的标记-清理（Mark-Sweep）垃圾回收；
- 通过裸指针操作虚拟机栈的宿主语言扩展接口。

该架构简洁但存在性能瓶颈（约比 Python 慢 1x–1.5x）、类型安全缺失、语义不一致（基础类型不是类、`is` 无法判断基础类型）等问题。

### 1.2 设计目标

sloth-lang 2.0（下称 sloth2）的总体目标：

1. **静态强类型**：所有类型在编译期确定或可推断；消除隐式类型变化；动态能力收敛到显式的 `dyn` 机制；
2. **AOT 编译**：基于 MLIR 构建中端与后端，经 LLVM 生成原生机器码，性能目标为数量级提升（对标同类 MLIR/LLVM 语言）；
3. **语义一致性**：基础类型与类实例遵守统一的类型规则，消除 1.0 中"基础类型不是类"等不一致；
4. **语法延续**：保留 C-like 语法外壳与核心惯用法（管道运算符、字符串插值、迭代器 for 循环、运算符重载），降低迁移成本；
5. **可实现的运行时**：运行时收敛为 ARC 引用计数（含 `Weak<T>`）、字符串池、容器、IO/FFI、张量/mmap 与有栈协程（fiber）栈切换六个组件。

### 1.3 非目标

- 不追求与 sloth-lang 1.0 的源代码兼容（语义差异见 §10）；
- 不实现线程/并行（语言为单线程模型；协程为线程内 1:m 复用，线程留待 3.0 评估）；
- 不实现宏系统、异步/await、类型类（type class）等高阶特性；
- 首版不提供增量编译与 IDE 工具链（语言服务器列为后续工作）。

### 1.4 术语

| 术语 | 含义 |
| --- | --- |
| sloth2 | 本文档描述的 sloth-lang 2.0 |
| 单态化（Monomorphization） | 编译期为每个泛型实例生成具体类型副本 |
| `dyn Trait` | 动态分派的对象类型（trait object） |
| ARC | 引用计数所有权；`retain`/`release` 插入点在发射期静态确定，配合 `Weak<T>` 破环（见 §5.1） |
| 词面（word plane） | 一切 SSA 词/槽/字段/容器元素的统一表示：带 tag 的单 i64（见 §2.6） |
| dialect | MLIR 中自定义类型与操作的扩展包（**本实现不使用自定义 dialect**，见 §4.3） |
| ~~statepoint~~ | ~~LLVM 用于精确 GC 的栈上根定位机制~~ `【已过时·v1.0】` 已弃用 GC，无需栈图/statepoint |

---

## 2. 类型系统

### 2.1 类型全集

| 类别 | 类型 | 说明 |
| --- | --- | --- |
| 单元 | `unit` | 空类型，替代 1.0 中"无返回值函数隐式返回 nil" |
| 布尔 | `bool` | `true` / `false`，**取消** 1.0 的"任意值隐式转 bool" |
| 数值 | `int`（**63 bit**）、`float`（**f63**） | 新增 `int`。1.0 仅有 f64，但静态语言中数组索引、取余、位语义需要整数；`float` 为降精度 f64（见 §2.6）。`【已过时·v1.0】` 原为 `int`=i64、`float`=f64 |
| 字符串 | `str` | UTF-8 不可变字符串，保留 string interning |
| 范围 | `range` | 由 `..`、`..=` 构造，元素类型 `int` |
| 数组 | `Array<T>` | 同构动态数组，替代 1.0 的异构数组 |
| 字典 | `Map<K, V>` | 键类型 `K` 必须实现 `Hashable` trait（见 §2.5），值类型 `V` |
| 可空 | `T?` | 可选类型，`nil` 字面量只能赋给 `T?` |
| 函数 | `(A, B) -> R` | 函数与闭包的一等类型 |
| 类 | `class C ...` | 用户定义引用类型，单继承 |
| 接口 | `trait T ...` | 结构化能力的静态抽象，类可 `impl` 多个 trait |
| 动态对象 | `dyn Trait` | 运行时多态的唯一出口（单字段虚表指针 + class-id，见 §2.6） |
| 弱引用 | `Weak<T>` | ARC 破环用的弱引用盒；`upgrade()` 返回 `T?`（见 §5.1） |
| 张量 | `Tensor<T, R>` | 元素类型 `T`（`float`/`int`）+ 静态秩 `R`（见 §5.6） |
| 协程 | `Fiber<Y>` | 有栈协程句柄，载荷类型 `Y`；`fiber.*` 内建模块（见协程扩展设计文档 §3） |

关键决策说明：

- **`T?` 替代通用 `nil`**：1.0 中任何类型都可为 `nil` 是运行时错误的主要来源（对 `nil` 的运算报错）。sloth2 中只有显式声明为 `T?` 的类型可持有 `nil`，使用值前必须判空（`if` 条件中的 `is not nil` 收窄或 `?.` 语法，见 §3.6）。
- **取消隐式布尔转换**：`while (x)` 要求 `x: bool`。1.0 的"Nil 转 false、其余转 true"规则与强类型冲突，且与 `T?` 判空习惯重叠。
- **无隐式数值转换**：`int` 与 `float` 之间不隐式转换，提供显式内置函数 `int(x)` / `float(x)`。算术运算两侧类型必须一致。
- **统一词面表示**：所有类型在运行期统一为带 tag 的单 i64 词（引用 `ptr|1`、`int` 63 bit、`float` f63、`bool` `0/2`、`nil` `0`），这是实现层 ABI，见 §2.6 与 §5.1。`int` 的 63 bit 与 `float` 的尾数损失是这一取舍的直接结果。

### 2.2 类型推断

> `【已过时·v1.0】` 原文描述“局部类型推断 + 强制标注（含单 `return` 推断返回）+ 标注优先的子类型检查”。现行实现为**单趟融合推断**，无独立类型检查阶段。

采用**局部、单趟**推断（解析后由发射器在同一趟内完成符号收集、推断、约束检查与单态化）：

- 变量声明 `var x = expr;` / `let x = expr;` 可省略标注，类型取自初始化表达式；
- 函数参数可省略标注（由调用面/lambda 上下文推断）；**函数返回值不做推断**：省略返回标注即视为 `unit`（`return expr;` 在 `unit` 函数中静默丢弃，不报错）；
- 泛型实例化可为“返回值驱动”：`let r: T = f(...)` 的注解或无注解 `Result<T,E>` 构造器上下文可回填类型实参；
- 声明面与赋值面做结构化词类检查（`surface_compat`）：int/str/bool/类/Array/Map 形状、子类→父类链、`dyn`、`nil` 宽松匹配；**不承诺跨语句的流敏感推断**（`if`/`else` 的类型收窄除外，见 §2.4/§3.6）。

### 2.3 泛型

- 函数与类型均可泛型：`func map<T, R>(arr: Array<T>, f: (T) -> R): Array<R>`；
- 泛型参数可带 trait 约束：`func sort<T: Comparable>(arr: Array<T>)`；
- 实现策略：**单态化**（编译期为每个具体类型实例生成代码副本），函数与泛型类同样处理。相比字典传递，单态化生成代码更快、对 MLIR 优化更友好，代价是代码膨胀，可接受；
- 实例可由参数推断，也可由返回值驱动（`let r: T = ...`、Result 构造器上下文）；无法推断时报诊断。**泛型/可变参/`extern`/跨模块函数不能作为一等函数值**（见 §3.3）。

### 2.4 类与继承

保留 1.0 的 class 模型并静态化：

- **单继承**：`class Cat: Mammal { ... }`，字段与方法解析在编译期完成；
- **虚方法分派**：通过基类引用调用方法时经虚表动态分派；编译期可确定具体类型时消虚（devirtualization 交给 MLIR 内联 pass 配合已知类型信息）；
- **构造器**：`__init__` 保留，实例化 `Cat(...)` 编译期检查参数匹配；子类构造器必须显式调用 `super.__init__(...)`（与 1.0 一致，但改为编译期强制检查）；
- **`this` / `super`**：语义不变，类型化；
- **`is` 运算符**：保留为类型测试，编译期校验两侧类型的可比性（无交集的测试直接编译期报错，消除 1.0 "基础类型不能用 is"的不一致——现在所有类型都可测试）；结果为 `bool`；在 `if (x is C)` 条件分支内对 `x` 做类型收窄（flow typing）。

### 2.5 Trait（接口）

引入 trait 解决 1.0 的鸭子类型问题——1.0 示例中 `Fish` 因"碰巧实现了 `say()`"而可被多态调用，sloth2 中必须显式声明能力：

```rust
trait Speaker {
    func say(): unit;
}

class Fish impl Speaker {
    func say(): unit { print("glub\n"); }
}

func announce(s: Speaker): unit { s.say(); }
```

- 运算符重载由类实现预定义的**魔术方法名**（`__add__`/`__eq__`/`__index__` 等，见 §3.4）；
- `dyn Trait` 提供运行时多态：`var s: dyn Speaker = Fish();`；
- trait 无字段、无构造器，方法可提供默认实现；
- 预定义基础 trait 包括：`Hashable`、`Equatable`、`Comparable`、`Display`。`int`、`float`、`bool`、`str`、`range` 内置可哈希；用户类型实现 `__hash__` 方法族后即可作为 `Map` 的键类型（无 `__hash__` 时回退为指针恒等，并给出编译期提示）。`Hashable` 类型必须同时满足 `Equatable` 契约（编译期联带校验）；
- **迭代协议是结构化的**：类型只要提供 `iter()`/`next()` 即可用于 `for`，不要求显式 `impl Iterable`（见 §3.5）。

### 2.6 值类型、引用类型与内存表示

> `【已过时·v1.0】` 原文为“值类型直接内联到原生栈 / 引用类型 GC 堆分配 / `int`=i64、`float`=f64 / `range` 双 i64 / `T?` 用 `{payload, has_value}` 标签布局 / `dyn` 两字 fat pointer”。现行实现**统一为带 tag 的单 i64 词面**，引用由 ARC 管理；以下为现行表示。

**统一词面（word plane）**——所有 SSA 词、局部槽、对象字段、容器元素、闭包捕获与内存词均为带 tag 的单个 i64（编解码常量见 `crates/sloth-codegen/src/irgen/util.rs`、`crates/sloth-rt/src/rc.rs`）：

| 面 | 编码 | 解码 |
| --- | --- | --- |
| 引用句柄 | `ptr \| 1`（payload 16 对齐，bit0 恒空） | `w & !1` |
| `int` | `v << 1`（**收窄为 63 bit**，环绕语义） | 算术右移 1 |
| `float` | 字面量 `(bits & !1) >> 1`；**运行时算术 `(bits & ~2) >> 1`**（**最多丢 2 个尾数 LSB**） | `w << 1` 后 bitcast |
| `bool` | `0` / `2` | `!= 0` |
| `nil` | `0` | `0` |

**语言语义归类**（仅用于类型检查与所有权，不再对应不同的机器布局）：

- `unit` 零大小；数值/布尔/引用在机器层都是词；
- **引用类型**（`str`、`Array<T>`、`Map<K,V>`、类实例、闭包、`dyn`、`range`、`Tensor<T,R>`、`Weak<T>`、`Fiber<Y>`、值型 optional 盒）由 ARC 管理，变量持有句柄词：
  - `str`：字符串池 Entry 指针词（interned，相等比较即指针比较）；
  - `Array<T>` / `Map<K,V>`：稳定句柄对象，元素按单态化后的具体类型内联存储；数组增长只替换独立的数据缓冲，句柄不移动（`crates/sloth-rt/src/arrays.rs`）；
  - 类实例：ARC 对象，RC 计数**带内**藏在对象头 `Hdr` 中（见 §5.1），头部含 class-id/类型信息；
  - 闭包 `(A) -> R`：2 词 ARC 对象 `{ tagged fnptr, env }`（`lambda.rs`/`closure.rs`）；
  - `dyn Trait`：**单字段虚表指针 + class-id**（运行时等价于两字 fat pointer）；
  - `range`：rc 双词盒 `{ lo, hi }`（`crates/sloth-rt/src/ranges.rs`）；
  - `Tensor<T,R>`：ARC 描述符（7 词 payload，见 §5.6）；
  - `Fiber<Y>`：ARC 对象，payload 为 `[state, prev, inbox, stack_base, stack_size, ctx, entry, init, jmp_buf, cancel]`，`entry` 持有入口闭包；独立 `mmap` 栈 + 保护页（见协程扩展设计文档 §3.1/§4）。

**可空类型 `T?` 的表示：**

- **`T` 为引用类型**：复用句柄词，`nil` 即词 `0`；`is not nil` 编译为一次判零；
- **`T` 为值类型**（`int?`/`float?`/`bool?`）：**一词 payload 盒**（`sloth_box_new`/`sloth_box_get`，盒本身由 ARC 跟踪，归零即 `free`）；槽仍为 1 词，`nil` 仍为词 `0`，因此“值 0”与 `nil` 不再混淆（值 0 是合法盒句柄）；`【已过时·v1.0】` 原为 `{payload, has_value}` 两字标签布局；
- 不允许嵌套可空：`T??` 编译期报错（parser 专用诊断）。

**浮点精度说明**：`float` 是降精度 f64——字面量丢 1 个尾数 LSB，而**运行时算术路径的掩码为 `-3`（`~2`）**，比字面量多丢 1 位，二者不一致，属已记录的实现不一致（详见 `book/src/appendix_a_deviations.md` §A.2.1）。写浮点数值代码时应假定约 f62 精度。

---

## 3. 语法设计

### 3.1 相对 1.0 的语法变更总览

| 变更 | 1.0 | 2.0 |
| --- | --- | --- |
| 类型标注 | 无 | `var x: int`、`func f(a: int): bool` |
| 模块导入 | 运行时函数 `import("path")` | 编译期声明 `import "path.slt";` |
| 可空 | 任意值可为 `nil` | `T?`，`nil` 仅属可空类型 |
| 可变参数 | `...` + `va_arg()` | 类型化可变参数 `func f(xs...: Array<int>)`，实参由编译器自动收集为数组 |
| 顶层导出 | 全部自动导出 | `pub` 关键字显式导出 |
| trait | 无 | `trait` / `impl` / `dyn` |
| fiber | `fiber.*` 扩展函数（动态） | **类型化恢复**：内建模块 `fiber.*` + `Fiber<Y>`（见协程扩展设计文档 §2.2） |
| 匿名函数 | `\\|a, b\\| { ... }` | 保留，参数类型可推断或标注 `\\|a: int\\| -> int { ... }` |
| 管道 `\\|>` | 动态调用单参函数 | 保留为语法糖，编译期解析类型 |
| 字符串插值 | `"${expr}"` | 保留，编译期展开 |
| map 字面量 | `@("k": v)` | 保留 `@(k: v)`，类型 `Map<K, V>`，K 需实现 `Hashable`，K/V 取各键值公共类型 |
| 逻辑运算 | `and or not` | 保留，另接受同义 `&&` `\|\|` |
| 复合赋值 | 无 | 新增 `+=` `-=`（`crates/sloth-frontend/src/lexer.rs` `PlusEq`/`MinusEq`） |
| 位运算 | 无 | 新增 int-only `& \| ^ << >> ~`，优先级见 §3.9 |
| 返回标注 | `:` | `:` 或 `->` 均可 |
| for 循环 | `for (var x: expr)` | 另接受 `for x in expr` |
| 外部函数/类型 | 无 | 新增 `extern func ...;` / `extern type Foo;`（见 §5.4） |
| 显式泛型调用 | 无 | 新增 `f<A, B>(args)` |
| 张量类型 | 无 | 新增 `Tensor<T, R>`、`Weak<T>`（见 §5.6/§5.1） |
| 关键字 | 19 个 | **25 个保留字**：`and or not true false for var let if else while func nil return class super this break continue is pub trait impl dyn as`；`int float bool str unit range Array Map Weak Tensor Fiber dyn extern` 为上下文关键字，仅在类型/声明位置有特殊含义（见 §3.9） |

### 3.2 变量与常量

```javascript
var x: int = 1;      // 可变，显式标注
var y = 2.0;         // 可变，推断为 float
let z = "hi";        // 不可变绑定（新增 let，变量本身不可再赋值）
```

- 新增 `let` 表达不可变绑定，鼓励默认使用 `let`；`var` 保留可变语义；
- 最外层声明仍需 `pub` 才会被其他模块可见（见 §3.7）；
- 遮蔽规则与 1.0 一致：内层块级作用域可遮蔽外层同名变量。

### 3.3 函数

```rust
func add(a: int, b: int): int {
    return a + b;
}

// 泛型
func map<T, R>(arr: Array<T>, f: (T) -> R): Array<R> {
    var ret: Array<R> = [];
    for (var elem: arr) {
        ret.push(f(elem));
    }
    return ret;
}

// 匿名函数（表达式，类型可推断）
let double = |x: int| -> int { return x * 2; };
let inc = |x| { return x + 1; };   // 参数类型由上下文推断
```

- `return` 在 `unit` 函数中可省略表达式；
- **类型化可变参数**：参数列表末尾允许一个 `xs...: Array<T>` 形式的可变参数，调用处逐个传参 `f(1, 2, 3)`，由编译器自动收集为数组，等价于显式传 `f([1, 2, 3])`；函数体内它就是普通的 `Array<T>`，不再需要 `va_arg()`：

```rust
func add_all(xs...: Array<int>): int {
    var ret = 0;
    for (var i: xs) { ret = ret + i; }
    return ret;
}
print(add_all(1, 2, 3, 4, 5));   // 15
```

- 函数重载：首版**不支持**同名重载（降低解析复杂度，后续版本评估）；
- 返回标注可用 `:` 或 `->`；省略即 `unit`（不做返回类型推断，见 §2.2）；
- 函数作为一等值已支持（具名函数取值、函数型参数/返回、IIFE、方法引用）；但**泛型 / 可变参 / `extern` / 跨模块函数不能作值**（见 §2.3）。

### 3.4 运算符重载

> `【已过时·v1.0】` 原文描述“实现参数化 trait `Add<Rhs, Out>`/`Sub`/…”。现行实现**不做 trait 参数化**，而是由类实现预定义**魔术方法名**，解析在编译期完成。

| 运算符 | 方法签名 |
| --- | --- |
| `+` | `func __add__(rhs): Out` |
| `-` | `func __sub__(rhs): Out` |
| `*` | `func __mul__(rhs): Out` |
| `/` | `func __div__(rhs): Out` |
| `%` | `func __mod__(rhs): Out` |
| 一元 `-` | `func __neg__(): Out` |
| `> >= < <=` | `__gt__ __ge__ __lt__ __le__` |
| `== !=` | `__eq__ __ne__` |
| `[]` / `[]=` | `__index__(idx): V` / `__assign__(idx, val)` |

- 1.0 的 `+` 拼接字符串/数组语义保留为对 `str`、`Array<T>` 的内置实现；
- 类族若缺少对应比较重载，比较改编译期报错（不再静默退化为词面比较）；
- **位运算 `& \| ^ << >> ~` 是 int 内置运算，不可重载**；
- **复合赋值 `+=` `-=`** 由 `a op= b` 展开为 `a = a op b`（仅这两种，不引入其它复合赋值）。

### 3.5 控制流与迭代

- `while` / `if-else` / `break` / `continue` 语法与 1.0 相同，条件表达式类型必须为 `bool`；
- `for (var x: expr)` 基于**类型化迭代协议**：

```rust
trait Iterator<T> {
    func next(): T?;          // 穷尽返回 nil（替代 1.0 的 __next__ 返回 nil 约定，语义不变但类型化）
}
trait Iterable<T> {
    func iter(): Iterator<T>;
}
```

- `str`（按字符 `str`）、`Array<T>`、`Map<K, V>`（元素为 `Entry<K, V>` 记录类型，含 `key`/`value` 字段）、`range`（元素 `int`）内置实现 `Iterable`；
- 用户类型的 for 循环 = 实现 `Iterable<T>`，替代 1.0 的 `__iter__`/`__next__` 魔术方法。

### 3.6 可空类型操作

```rust
var name: str? = nil;
if (name is not nil) {
    print(name);        // 分支内收窄为 str
}
let n = name ?: "anon";  // 空合并运算符（新增）
```

`?.` 可选链留待后续版本，首版以 `is not nil` 收窄 + `?:` 覆盖主要场景。

### 3.7 模块系统

`import` 从运行时函数改为**编译期声明**（这是静态化的硬性要求：无法对运行时字符串路径做类型检查）：

```rust
import "sloth/tensor.slt";
import "./geometry.slt" as geo;

let p = geo.Vec2(1, 2);
```

- **路径解析顺序**（D5，`crates/sloth-codegen/src/irgen/mod.rs::find_import`）：
  1. 导入者文件所在目录 `dir.join(rel)`；
  2. 环境变量 `$SLOTH_STDLIB` 根目录；
  3. `<当前可执行文件目录>/../lib`；
  4. 开发树 `<repo>/lib`（`CARGO_MANIFEST_DIR/../../lib`）。
  因此 `import "sloth/tensor.slt"` 在源码树内与安装后都可解析；
- 被导入模块中只有 `pub` 声明可见（1.0 的"顶层全部自动导出"废弃）；
- 循环依赖**在编译期检测并报 `circular import`**（DFS 栈比对规范化路径，`mod.rs::resolve_program`）；同一模块在 `done` 集合中去重，只装配/编译一次，等价于 1.0 "模块只执行一次"；
- 模块级全局变量由 `sloth_<mod>__ginit()` 初始化，`@sloth_main` 入口先调用各依赖的 `ginit`；
- 模块加载为编译期装配，不经协程启动（1.0 依赖 fiber 的隐式加载协程机制不再需要）；
- 标准库以真实 `.slt` 文件提供于 `lib/sloth/`：`tensor.slt`、`random.slt`、`fs.slt`、`tokenizer.slt`、`llama.slt`（见 §5.6）。

### 3.8 字符串插值与管道运算符

- 字符串插值 `"hello ${expr}"` 保留，要求 `expr` 的类型实现标准库 trait `Display`（`int`、`float`、`bool`、`str` 内置实现；类可实现）；编译期展开为拼接或格式化缓冲调用，类型错误在编译期报告（1.0 为运行时行为）；
- 管道运算符 `|>` 保留为纯语法糖：`x |> f` ≡ `f(x)`，`x |> f(a, b)` ≡ `f(a, b, x)`（左操作数作为**最后一个**实参，以适配 `map(f)` 柯里化风格——此条与 1.0 行为兼容，1.0 中 `functool.map` 返回单参闭包）；编译期检查右操作数为可调用类型且参数匹配。

### 3.9 EBNF 文法（sloth2 完整定义）

> `【已过时·v1.0】` 原 EBNF 缺 `extern`、位运算、复合赋值、`Weak<T>`/`Tensor<T,R>`、`for x in`、`->` 返回标注，且优先级表不完整。下列文法直接提炼自 `crates/sloth-frontend` 的 lexer/parser，为**当前实现**。

```ebnf
prog            ::= ( import_decl | decl | stmt )*

import_decl     ::= 'import' STRING ( 'as' IDENT )? ';'

decl            ::= 'pub'? ( func_decl | var_let_decl | class_decl
                           | trait_decl | extern_decl )

var_let_decl    ::= ( 'var' | 'let' ) IDENT ( ':' type )? '=' expr ';'

func_decl       ::= 'func' IDENT type_params? '(' params ')' ret_ann? block
ret_ann         ::= ( ':' | '->' ) type
type_params     ::= '<' IDENT ( ':' IDENT )? ( ',' IDENT ( ':' IDENT )? )* '>'
params          ::= ( param ( ',' param )* ( ',' variadic )? | variadic )?
param           ::= IDENT ( ':' type )?
variadic        ::= IDENT '...' ':' 'Array' '<' type '>'      (* 仅可位于参数列表末尾 *)

extern_decl     ::= 'extern' 'type' IDENT ';'
                  | 'extern' 'func' IDENT '(' params ')' ret_ann? ';'
                    (* 无泛型、无可变参、参数必须有类型 *)

class_decl      ::= 'class' IDENT type_params? ( ':' IDENT )?
                    ( 'impl' IDENT ( ',' IDENT )* )?
                    '{' class_item* '}'
class_item      ::= 'pub'? ( field_decl | func_decl )
field_decl      ::= ( 'var' | 'let' ) IDENT ':' type ( '=' expr )? ';'

trait_decl      ::= 'trait' IDENT '{' trait_method* '}'
trait_method    ::= 'func' IDENT '(' params ')' ':' type ( block | ';' )
                                                    (* block = 默认实现 *)

type            ::= type_base '?'?                    (* T?? 报错 *)
type_base       ::= 'unit' | 'int' | 'float' | 'bool' | 'str' | 'range'
                  | 'Array' '<' type '>'
                  | 'Map' '<' type ',' type '>'
                  | 'Weak' '<' type '>'
                  | 'Fiber' '<' type '>'
                  | 'Tensor' '<' ( 'int' | 'float' ) ',' INT '>'
                    (* 秩字面量语法范围 0..=8，语义限 1..=3 *)
                  | 'dyn' IDENT
                  | '(' ( type ( ',' type )* )? ')' '->' type
                  | IDENT type_args?
type_args       ::= '<' type ( ',' type )* '>'

block           ::= '{' stmt* '}'
stmt            ::= block
                  | var_let_decl
                  | if_stmt | while_stmt | for_stmt
                  | 'return' expr? ';'
                  | 'break' ';' | 'continue' ';'
                  | assign_stmt | expr_stmt

if_stmt         ::= 'if' ( '(' expr ')' | expr ) stmt ( 'else' stmt )?
while_stmt      ::= 'while' ( '(' expr ')' | expr ) stmt
for_stmt        ::= 'for' ( '(' 'var' IDENT ':' expr ')' | IDENT 'in' expr ) stmt
assign_stmt     ::= assignable ( '=' | '+=' | '-=' ) expr ';'
assignable      ::= IDENT ( '.' IDENT | '[' expr ']' )*
expr_stmt       ::= expr ';'

(* 表达式按 Pratt 解析，优先级 低 -> 高 *)
expr            ::= pipe
pipe            ::= elvis ( '|>' elvis )*                       (* 左结合 *)
elvis           ::= or ( '?:' elvis )?                          (* 右结合 *)
or              ::= and ( ( 'or' | '||' ) and )*
and             ::= cmp ( ( 'and' | '&&' ) cmp )*
cmp             ::= bor ( ( '==' | '!=' | '<' | '>' | '<=' | '>='
                          | 'is' | 'is' 'not' ) bor )?
bor             ::= bxor ( '|' bxor )*
bxor            ::= band ( '^' band )*
band            ::= shift ( '&' shift )*
shift           ::= range ( ( '<<' | '>>' ) range )*
range           ::= add ( ( '..' | '..=' ) add )?
add             ::= mul ( ( '+' | '-' ) mul )*
mul             ::= unary ( ( '*' | '/' | '%' ) unary )*
unary           ::= ( 'not' | '-' | '~' ) unary | postfix
postfix         ::= primary ( '(' args? ')'
                          | '[' expr ']'
                          | '.' IDENT
                          | '<' type ( ',' type )* '>' '(' args? ')' )*
primary         ::= INT | FLOAT | STRING | 'true' | 'false' | 'nil'
                  | IDENT | 'this' | 'super'
                  | list | map | lambda | '(' expr ')'
args            ::= expr ( ',' expr )*
list            ::= '[' ( expr ( ',' expr )* )? ']'
map             ::= '@' '(' ( expr ':' expr ( ',' expr ':' expr )* )? ')'
lambda          ::= ( '||' | '|' params? '|' ) ( '->' type )? block
```

**词法要点**：`||` 空参数 lambda / `||` 逻辑或、`|` lambda 起始 / 位或、`|>` 管道、`?:` Elvis 均为独立 token；`<<`/`>>` 在解析期由相邻的两个 `Lt`/`Gt` 合并，以便 `Map<int, Array<int>>` 的泛型闭合不受影响；`?.` 已被词法识别（`QuestionDot`）但 parser 明确拒绝（可选链推迟）。

关键字全集（**25 个保留字**）：`and or not true false for var let if else while func nil return class super this break continue is pub trait impl dyn as`。上下文关键字（仅在类型/声明位置有特殊含义，可作标识符）：`int float bool str unit range Array Map Weak Tensor dyn extern`。

---

## 4. 编译器架构

### 4.1 总体流水线

> `【已过时·v1.0】` 原文为“10 阶段、含独立 [3] 名称解析 / [4] 类型检查推断 / [5] 单态化 / [6] sloth dialect 高层 IR”。现行实现把 [3][4][5] 融合进**单趟发射器**，且**不使用自定义 dialect**，直接发射标准 dialect。

```javascript
源代码 (.slt)
   │
   ▼
[1] 词法分析 Lexer ──────────► Token 流            (sloth-frontend/src/lexer.rs)
   │
   ▼
[2] 语法分析 Parser ─────────► AST（递归下降）      (sloth-frontend/src/parser.rs)
   │
   ▼
[3] 单趟发射器（融合） ───────► 文本 MLIR          (sloth-codegen/src/irgen/)
   │      ├─ 符号收集/作用域解析（collect）
   │      ├─ 局部推断 + 结构化词类检查（tybind）
   │      ├─ 单态化 + 类型收窄 + 所有权插桩（fnwalk/expr/stmt）
   │      └─ 批量诊断（不做 fail-fast）
   ▼
[4] MLIR pass 管线 ──────────► 标准 dialect 降级   (sloth-codegen/src/pipeline.rs)
   │      canonicalize → cse → one-shot-bufferize
   │      → linalg-fuse-elementwise-ops → convert-linalg-to-loops
   │      → convert-scf-to-cf → convert-math-to-llvm
   │      → convert-func/arith/index/cf-to-llvm
   │      → finalize-memref-to-llvm → reconcile-unrealized-casts
   ▼
[5] 后端（二选一）
   │      ├─ run（JIT）：MLIR ExecutionEngine（ORC），invokePacked("sloth_main")
   │      └─ build（AOT）：mlir-opt → mlir-translate --mlir-to-llvmir
   │                       → clang -O3 app.ll libsloth_rt.so -o out
   ▼
[6] 链接 libsloth_rt（Rust 编译的静态/动态库）────► 可执行文件
```

**与 1.0 的本质区别**：1.0 是 one-pass 直出字节码；sloth2 前端新增 AST，但不设独立名称解析/类型检查阶段，而是在发射 MLIR 的同一趟内完成（见 `irgen/mod.rs` 模块注释与 `book/src/appendix_a_deviations.md` §A.1）。词法与文法规则、测试用例最大化复用。

### 4.2 前端要点

> `【已过时·v1.0】` 原文声称“两遍类型检查 + 显式名称解析阶段 + 闭包捕获结构体/逃逸分析”。现行实现为单趟融合，闭包为统一的 2 词对象。

1. **AST**：`sloth-frontend/src/ast.rs`；节点携带源码位置（诊断用）。类型由 `ty.rs` 的 `Ty`/`TyId` 注册表在发射期建立，不回填 AST 类型槽；
2. **名称解析与依赖装配**：作为发射的一部分；`import` 由 `irgen::resolve_program` 递归装配，DFS 栈检测 `circular import`，`done` 集合去重（**无独立依赖图/拓扑排序阶段**）；
3. **类型检查/推断**：单趟——收集符号/规划函数与 vtable、推断局部类型、做结构化 `surface_compat` 检查、求解 trait 约束、执行 `is` 收窄；允许前向引用与递归函数/类；
4. **单态化**：以“泛型定义 + 具体类型实参”为键缓存实例（函数与类），可由参数或返回值驱动；
5. **闭包**：统一为 2 词 ARC 对象 `{ tagged fnptr, env }`，捕获的标量按值快照、引用按引用共享（不做逃逸性/结构体布局分析）；
6. **错误诊断**：所有类型错误携带源码位置（行:列）与期望/实际类型，一次编译尽可能报多个错误（batch diagnostics，不做 fail-fast）。消息为英文。

### 4.3 MLIR 生成（无自定义 dialect）

> `【已过时·v1.0】` 原文设计了自定义 `sloth` dialect（`!sloth.*` 类型、`sloth.gc_alloc`/`sloth.string_literal` 等操作、三轮 lowering、`gc.strategy` 标注）。**该 dialect 未实现**：当前直接发射标准 dialect，运行时能力以 `func.func private @sloth_*` C-ABI 调用表达（完整前导清单见 `book/src/appendix_b_mlir.md` §B.2）。

#### 4.3.1 类型映射（词面 ABI）

所有值（`int`/`float`/`bool`、引用、optional、闭包、`dyn`、张量）在 SSA 层统一为 `i64` 词；局部变量/可变槽是 `memref<1xi64>` 的 alloca；函数签名参数与返回均为 `i64`。`float` 不直接以 MLIR `f64` 出境，只在算子内部 `dec_f`/`enc_f` 转换。张量算子额外进入张量通道（`tensor`/`linalg`/`memref`/`scf`/`math`，见 §5.6）。`【已过时·v1.0】` 原为 `int`/`float`/`bool` 直接映射 `i64`/`f64`/`i1` 且“不再装箱”。

#### 4.3.2 标准 dialect 与运行时调用（对照原 dialect 表）

| v1.0 设想操作 | 现行实现 |
| --- | --- |
| `sloth.gc_alloc` | `call @sloth_obj_new` / `@sloth_arr_new` / ……（malloc 基确定性分配 + ARC，见 §5.1） |
| `sloth.string_literal` | 8 字节打包 `i64` 常量 + `@sloth_str_push` / `@sloth_str_finish`（intern） |
| `sloth.string_concat` | `@sloth_str_concat` |
| `sloth.string_interp` | 逐段 `str_push` / `str_pushp` / `str_push_i\|_f\|_b` + `str_finish` |
| `sloth.array_new / push / get / set` | `@sloth_arr_new` / `_get` / `_set` / `_push` / `_pop`（越界/除零 → `@sloth_panic_*`） |
| `sloth.call_indirect` | 闭包 `{fnptr, env}` 拆解后经 `llvm.call`（每目标生成 bridge） |
| `sloth.call_virtual` | 对象头虚表指针 + class-id 分支/间接调用 |
| `sloth.type_test` | class-id / 运行时活跃判定 |
| `sloth.closure_create` | `@sloth_closure_new` + env 字段写入 |
| `sloth.iter_begin / iter_next` | 结构化 `iter()`/`next()` 协议，展开发射到 `cf`/`scf` |
| `sloth.panic` | `@sloth_panic_divzero` / `@sloth_panic_unwrap` / `@sloth_panic_noimpl` |

#### 4.3.3 Lowering 路径

```text
发射端（irgen）直接产出标准 dialect（func / arith / cf / memref；张量追加 linalg / scf / math / tensor）
  → pipeline.rs 的统一 pass 列表（见 §4.1 [4]）
  → llvm dialect
  → JIT（ORC ExecutionEngine）或 AOT（mlir-translate → clang -O3）
```

#### 4.3.4 可直接复用的 MLIR 优化

- `canonicalize` / `cse`：通用化简；**无语言特定 pass**（所有权与类型信息已在发射端显式化）；
- 张量通道：`one-shot-bufferize`、`linalg-fuse-elementwise-ops`、`convert-linalg-to-loops`、`convert-math-to-llvm`（见 §5.6）；
- 数值代码（词面 i64 + 算子内部 f64）进入 `arith` 优化管线；
- AOT 链接期由 `clang -O3` 完成最终优化（`examples/tensor` 基准 matvec ≈0.9x、fusion ≈1.7x vs gcc -O3）。

---

## 5. 运行时设计（libsloth_rt）

运行时由 ARC 引用计数（含 `Weak<T>`）、字符串池、容器、IO/FFI、张量/mmap 与有栈协程（fiber，`crates/sloth-rt/src/fiber.rs`，含 `sloth_fiber_switch_asm` 汇编切换）等组件构成，以 Rust 实现并编译为静态/动态库随程序链接。

### 5.1 内存管理：引用计数（ARC）

**决策**：不采用 GC（1.0 的 mark & sweep 与早期 Boehm 保守式兜底均已移除）。引用类型统一走**引用计数 + `Weak<T>` 弱引用破环**。理由：全部代码由本编译器发射，`retain`/`release` 插入点可在发射期静态确定，无需栈图/statepoint；确定性回收贴合单线程模型。分配器为 malloc 基确定性链（`sloth_rt_alloc/realloc`），对象头带内承载计数；归零即析构级联并 `free`，漏插 `release` 退化为内存滞留而非悬垂。

**词面编码（word plane，与 §2.6/§4.3.1 对应）**：所有 SSA 词、槽、字段、容器元素、内存词均为带 tag 的单 i64：

| 面 | 编码 | 解码 |
| --- | --- | --- |
| 引用句柄 | `ptr \| 1`（payload 16 对齐，bit0 恒空） | `w & !1` |
| `int` | `v << 1`（收窄为 63 bit，环绕语义） | 算术右移 1 |
| `float` | `(bits & !1) >> 1`（**f63**，牺牲尾数 LSB） | `w << 1` 后 bitcast |
| `bool` | `0` / `2` | `!= 0` |
| `nil` | `0` | `0` |

**对象头 RC（带内）**：每个堆对象在 payload 之下内联 6 词 `Hdr = [cnt, size, dtor, aux, weak_head, pad]`（48 字节）。`retain`/`release` 首检 tag（tag0 惰性 no-op，不触内存）；归零 → 跑 dtor 级联 → 排空侵入式 weak 链（置 `target=0`）→ `free(header+payload)`。rt 内部 metadata（数组 len/cap、map kflag、vtable 容量等）保持裸 i64，不入 tag 词面。

#### 5.1.1 ARC 所有权协议（ownership protocol）

**所有权状态**：任一引用词在任一时刻处于二者之一：

- **owned（+1）**：持有者负责恰好一次 `release`；
- **borrowed（+0）**：仅借用读取，持有者不得 `release`。

**协议规则**（发射器必须逐条维持，构成安全性不变量）：

1. **生产即 owned**：构造器、`Array`/`Map`/闭包/box 字面量、字符串 intern/拼接、`keys()`/`values()`、range/Entry 盒等产出对象的操作，交付一个 owned 句柄。
2. **持有即 owned**：局部槽、对象字段、容器元素、闭包捕获在写入时 `retain`（copy-in），覆盖旧值时 `release`（overwrite-out）；作用域退出释放本层声明的槽。
3. **形参与接收者为 borrowed**：被调函数不得释放形参或 `this`。
4. **返回值为 owned**：引用类型返回值一律向调用者交付 +1。返回面三态处理：**生产者**（dangling）直接转移其 +1；**调用结果**（已 owned）转移；**对槽/字段/参数等借用值**在返回前物化一次 `retain` 再交付。非引用返回释放未使用的生产者。
5. **调用结果即 owned 临时量**：直接调用、方法调用、虚分派（`emit_class_virtual_call`）与 dyn 分派的引用结果，均登记为 owned 临时量。
6. **转移（transfer）**：owned 临时量被持有者接管（绑定局部/全局、字段/容器写、`return`）时**不再 retain**，所有权直接转移；接管点必须注销临时登记。
7. **临时量回收**：owned 临时量若在语句结束前未被接管，由发射器在**语句结束**插入 `release`；在条件终止（`cjump`）与 CFG 分裂（虚分派/短路/分支 merge）处，临时量于**当前块内**冲刷，避免跨 merge 引用非支配 SSA 值。
8. **`Weak<T>`**：弱持有不增加目标计数（弱盒本身参与 rc）；目标归零时沿弱链失效，`upgrade()` 返回 `T?`（死目标为 `nil`）。闭包捕获的标量按值快照、引用类型按引用共享（捕获即 retain）。
9. **跨协程边界载荷**：`fiber.create/resume/yield/transfer` 的载荷按普通 borrowed 实参递交，接收侧由运行时 `retain` 接管（`sloth_fiber_*`）；`yield`/`resume` 的引用型返回值交付 +1（同规则 4/5）。协程挂起栈帧中已 `retain` 的引用由计数自然保活，无需根枚举（详见协程扩展设计文档 §4.3）。

**插入点汇总**：绑定/赋值、字段与元素写、作用域退出、`return`、语句级临时量冲刷、条件分支前与 merge 处的冲刷，以及容器迭代协议——`iter()` 结果为 owned（循环退出时释放），`next()` 结果为 owned（每轮迭代末释放，`continue` 路径同样覆盖）。

**诊断**：rt 暴露 `sloth_rc_live`/`sloth_rc_drops` 零参内置，供 spec/churn 断言计数收敛。

> 历史缺陷（本协议修复）：owned 临时量此前仅在命名/全局赋值处被接管，方法接收者、`len`/索引、运算符、实参、容器迭代等消费者既不登记也不冲刷，导致每个临时量滞留 +1（见 PLAN §10）。

### 5.2 字符串池

- 保留 1.0 的 string interning：相等比较即指针比较；
- 改动：从"每 VM 实例一个 StringPool"变为**进程级单例**（无 VM 了）；首版单线程无需锁，结构沿用哈希表保证 Entry 唯一。

### 5.3 容器与内建类型方法

- `Array<T>` / `Map<K, V>` 运行时实现（容量增长、哈希），元素布局由单态化后的具体类型决定（无装箱，直接内联存储）；数组采用**稳定句柄**，增长只替换独立数据缓冲（`crates/sloth-rt/src/arrays.rs`）；`Map` 键的哈希与相等比较经单态化 `__hash__`/`__eq__` 路由；
- 基础类型方法（如 `arr.len()`、`str.len()`）由编译器**直接发射运行时调用**（`arr.len()` → `@sloth_arr_len` 等），而非生成 stdlib 泛型函数。`【已过时·v1.0】` 原计划“解析为标准库泛型函数”；直接 rt 调用为定案，功能面等价；
- `Result<T,E>` 与 `Entry<K,V>` 由编译器**自动注入**为标准库类。

### 5.4 FFI 与宿主互操作

1.0 的扩展方式（满足 `fn(&mut Vm, usize, bool)` 签名、手工操作 VM 栈）**废弃**，改为声明式外部函数：

```rust
extern func floor(x: float): float;          // 链接期解析符号
```

- 参数/返回值按 C ABI 或定义的 sloth ABI 传递，编译器自动生成 marshalling；
- `OpaqueData` 由 `extern type`（不透明类型声明）替代，仅能经 extern 函数传递，编译期保证脚本侧无法解引用；
- extern 函数仍是普通原生调用；但宿主帧不在受管协程栈上，`extern func` 体内不得出现 `fiber.yield`（body-less 声明天然满足）；宿主侧异步由宿主自行管理。

### 5.5 错误模型

首版不提供异常机制：

- 编译期：所有类型错误；
- 运行时不可恢复错误（数组越界、`nil` 解引用、整数除零、张量形状不匹配、`Result` unwrap-on-err、断言失败）：调用运行时 `@sloth_panic_noimpl` / `@sloth_panic_divzero` / `@sloth_panic_unwrap` 等，打印诊断并终止进程（`crates/sloth-rt/src/panics.rs`）。`【已过时·v1.0】` 原文写作 `sloth.panic` op；
- 可恢复错误：约定返回 `T?` 或标准库 `Result<T, E>`（以泛型枚举类实现，构造器 `ok()`/`err()`）；
- 协程内 panic 仍终止进程；协程级协作式出错用 `fiber.error(msg)`（置 Error、打印诊断并切回 prev，使该处 `resume` 返回 `nil`）。

### 5.6 张量扩展与标准库（TE-P0–TE-P4）

张量是为“单机 fp32 跑通 llama2.c 推理”引入的受控扩展，独立设计见 `sloth-lang-2.0张量扩展设计文档.md` 与实现方案文档。要点：

- **类型**：`Tensor<T, R>`（元素 `float`/`int`，秩 `R` 为整数字面量；语法 0..=8，语义限 1..=3）。运行时为 ARC 描述符（7 词 payload `[flags, ndim, shape, stride, data, owner, total]`），数据缓冲为**非追踪** `calloc`；视图（reshape / 切片 / 索引）共享存储并靠 `owner` 保活；
- **算子**：`matvec`/`matmul`/`dot`/`sum`/`add`/`sub`/`mul`/`div` 及融合算子 `exp`/`sqrt`/`sin`/`cos`/`tan`/`silu`/`silu_mul_into`/`rmsnorm`/`softmax`/`add_scaled_into`/`div_scalar_into`。走**真 MLIR `linalg` 通道**（`memref.reinterpret_cast` + 运行期 shape/stride，绕开 bufferization），非 rt 内核回退；
- **标准库**：以真实 `.slt` 文件提供于 `lib/sloth/`：`tensor.slt`、`random.slt`、`fs.slt`（`ByteBuffer` + mmap checkpoint IO）、`tokenizer.slt`、`llama.slt`；
- **位运算/复合赋值（TE-P0）**：`& \| ^ << >> ~`（int-only）、`+=`/`-=`，见 §3.4；
- **验收**：tiny / stories42M 与 `run.c` 逐字节一致（greedy）；性能 matvec ≈0.9x、fusion ≈1.7x vs gcc -O3；
- **已知未对齐**：RNG 为 31-bit XorShift，未与 `run.c` 的 64-bit `xorshift64*` 逐位对齐（temp=0 路径不用 RNG，故输出一致）；详见 `book/src/appendix_a_deviations.md` §A.7。

---

## 6. 兼容性说明与迁移指南（1.0 → 2.0）

| 1.0 代码模式 | 2.0 迁移方式 |
| --- | --- |
| `var x = 1; x = "str";` | 拆分变量或显式设计类型；此类代码多数本就是缺陷 |
| 异构数组 `[Cat(), Dog(), Fish()]` | 抽取共同 trait：`Array<dyn Speaker>` |
| 鸭子类型（Fish 实现 say） | 显式 `impl Speaker` |
| `import("path")` 运行时加载 | 编译期 `import "path";`；动态插件场景用 `extern` + 宿主加载器 |
| `func f(...)` + `va_arg()` | 类型化可变参数 `func f(xs...: Array<T>)`（调用写法不变），或显式 `Array<T>` 参数 |
| `nil` 任意赋用 | `T?` + `?:` + `is not nil` 收窄 |
| `if (x)` 真值判断 | `if (x != nil)` / `if (x is not nil)` / 显式 bool 表达式 |
| `fiber.create/resume/yield/...` | 类型化恢复为内建模块 `fiber.*` + `Fiber<Y>`；载荷收敛为单类型 `Y`，`resume` 返回 `Y?`（见协程扩展设计文档 §2.2） |
| 魔术方法 `__iter__`/`__next__` | 结构化提供 `iter()`/`next()` 即可用于 `for`（不要求显式 `impl Iterable`） |
| 魔术方法 `__add__` 等 | 保留魔术方法名（`__add__`/`__eq__`/…，见 §3.4；不做 trait 参数化） |
| 方法引用 `orange.whoami` | 保留，类型为 `() -> unit`，this 绑定语义不变 |
| 字符串插值、管道、范围、map 字面量 | 语法不变，获得编译期类型检查 |

---

## 7. 风险与阻碍评估

| # | 风险 | 等级 | 说明与对策 |
| --- | --- | --- | --- |
| 1 | 前端整体重写 | 高 | one-pass 架构无法演进式改造，AST+类型检查必须重写；对策：词法/文法规则与测试用例最大化复用 |
| 2 | 类型系统落地复杂度 | 高 | 手册作者自述"不亚于实现一种语言"；对策：局部推断（非全局 HM）、首版不做函数重载、trait 不带关联类型，严格控制特性面 |
| 3 | ~~精确 GC 工程风险~~ `【已过时·v1.0】` | 已消除 | 原对策为“MVP Boehm → statepoint 精确 GC”。架构改向 **ARC + `Weak<T>`** 后，GC/栈图/statepoint 风险整体消除；新风险转为 ARC 引用环（需 `Weak<T>` 破环）与漏插 `release` 导致的内存滞留 |
| 4 | 单态化代码膨胀与编译速度 | 中 | 泛型实例爆炸；对策：实例缓存 + 后续考虑对引用类型共享实例（类型擦除混合策略） |
| 5 | 动态模块加载能力丢失 | 低 | `import` 编译期化后失去运行时脚本热加载；对策：文档明确，插件场景由宿主 FFI 承接 |
| 6 | 标准库缺口 | 中 | 1.0 标准库本就不完整，2.0 同步建设（容器方法、`Display`、`Result`、张量/llama 等）；对策：标准库以 sloth2 自身编写（`lib/sloth/*.slt`），仅 IO/ARC/张量走运行时 |
| 7 | 弃置挂起协程的 ARC 泄漏 | 低 | 协程局部引用槽逐帧登记（`@sloth_fiber_track`），error/cancel/弃置在栈有效时结算在册槽，不再滞留；对策：debug 构建仍对弃置析构 panic 以暴露纪律问题，文档明示「驱动至 Done/Error」 |
| 8 | ARC 引用环 | 中 | 强引用环按设计泄漏；对策：`Weak<T>` 破环（`sloth_weak_*`），文档明示 |
| 9 | 固定协程栈溢出 | 低 | 默认 256 KiB + 保护页，溢出触发 SIGSEGV 诊断终止（无 1.0 的动态增长）；对策：`fiber.create_with` 调大栈 |
| 10 | 汇编切换可移植性 | 中 | 仅覆盖 x86_64 SysV / aarch64 AAPCS64；对策：接口收敛为 `sloth_fiber_switch_asm` + trampoline 单点 |

> 原方案中最难的两项——fiber 与 LLVM 无栈协程模型的错配、fiber 栈与 GC 根扫描的交互——已随 ARC 改向与原生 ABI **消解**：有栈协程用原生栈切换而非 LLVM coroutine intrinsic，挂起栈中引用由 ARC 计数自然保活、无需根扫描；fiber 因此以原生有栈方案恢复（见协程扩展设计文档）。
>
> `【已过时·v1.0】` 原方案中的 GC 相关风险（Boehm、statepoint、标记-清理）已随 ARC 改向一并消除（见 §5.1）。

---

## 8. 分阶段实施路线图

| 阶段 | 内容 | 验收标准 |
| --- | --- | --- |
| **P0 语言定稿** | 类型系统与语法冻结；编写语言规范测试集（正/负类型用例） | 本文档评审通过；≥200 条规范测试用例 |
| **P1 前端** | Lexer/Parser/AST、名称解析、类型检查（非泛型子集） | 非泛型程序的类型检查通过/报错符合规范 |
| **P2 MLIR 端到端 MVP** | ~~sloth dialect~~ `【已过时·v1.0】` 标准 dialect（标量 + 函数 + 控制流）→ LLVM；~~Boehm GC~~ ARC；`hello world` 级程序原生运行 | 算术/分支/循环/函数程序编译运行，数值正确 |
| **P3 对象与闭包** | class/继承/虚表、trait 与 `dyn`、闭包转换、字符串池 | 1.0 面向对象示例（Cat/Dog/Fish 改写版）运行正确 |
| **P4 泛型与单态化** | 泛型函数/类型、trait 约束、容器泛型化、迭代协议 | `map`/`reduce` 泛型版管道示例运行正确 |
| **P5 模块与标准库** | 编译期 `import`、`pub` 可见性、核心标准库 | 多模块程序编译；标准库自举 |
| **P6 优化与内存** | 消虚/内联/融合调优；~~statepoint 精确 GC 替换 Boehm~~ `【已过时·v1.0】` **ARC 所有权协议 + 全词 tag 化已完成**（见 §5.1/§5.1.1）；张量 linalg 通道与融合算子 | ARC 压力测试（`examples/arc/`）计数回落基线无泄漏；张量 matvec/fusion 相对 gcc -O3 达标 |

里程碑建议：P2 完成即具备持续集成价值（端到端可跑），P4 完成即语言特性完备，可开放试用。
> **执行状态（2026-09-18）**：P0–P6 均已落地或按 ARC 改向定案；另完成张量扩展 TE-P0–TE-P4（见 §5.6）与协程扩展 CE-P0–CE-P2（见协程扩展设计文档 §9：运行时栈切换、类型化 `fiber.*`、ARC 交接与取消）。真实进度与测试规模见 `PLAN-2026-09-15.md`。

---

## 9. 附录

### 9.1 示例：1.0 Hello World 变体的 2.0 版本

```rust
import "sloth/tensor.slt";   // 标准库以真实 .slt 提供于 lib/sloth/，见 §5.6

pub func main(): unit {
    let names = ["Curry", "Dijkstra", "Benjamin", "Hitori", "foo"];
    names |> map(|name| {
        print("Hello , ${name} for 6 times!\n");
        for (var i: 0..=5) {
            print("${name}!");
        }
        print("\n");
    });
}
```

差异仅在于：`import` 为编译期声明；`map` 为泛型函数；顶层以 `pub func main()` 组织。
> `【已过时·v1.0】` 原示例路径 `sloth/sloth_lib/func_tool.slt` 已不存在；现行标准库文件为 `lib/sloth/{tensor,random,fs,tokenizer,llama}.slt`（§5.6）。示例中 `map` 仅作示意，实际需由所导入模块提供（当前标准库未内置 `map`）。

### 9.2 示例：面向对象改写

```rust
trait Speaker { func say(): unit; }

class Mammal impl Speaker {
    let kind: str;
    func __init__() { this.kind = "Mammal"; }
    func say(): unit { print("Mammal kind is: ${this.kind}\n"); }
}

class Cat: Mammal {
    func __init__() { super.__init__(); this.kind = "Cat"; }
    func say(): unit { print("meow\n"); super.say(); }
}

class Fish impl Speaker {          // 不再依赖鸭子类型，显式声明能力
    let kind: str = "Fish";
    func say(): unit { print("Fish kind is: ${this.kind}\n"); }
}

pub func main(): unit {
    let l: Array<dyn Speaker> = [Cat(), Mammal(), Fish()];
    for (var m: l) { m.say(); }
}
```

### 9.3 参考实现技术选型

| 组件 | 选型 | 备注 |
| --- | --- | --- |
| 编译器实现语言 | Rust | 与 1.0 一致，团队经验延续 |
| MLIR 接入 | `mlir-sys`（C API） | 现状；未采用 melior |
| MLIR 工具链 | `mlir-opt` / `mlir-translate` + `clang -O3`（AOT）；ORC JIT（`run`） | 见 §4.1 |
| 内存管理 | **ARC + `Weak<T>`**，malloc 基确定性分配器 | `【已过时·v1.0】` 原为 MVP Boehm、目标版 statepoint + 标记-清理 |
| 张量后端 | 标准 `linalg`/`memref`/`scf`/`math` + 运行时助手 | 真 MLIR 通道，见 §5.6 |
| 构建/包管理 | 暂不涉及，随标准库阶段评估 | — |