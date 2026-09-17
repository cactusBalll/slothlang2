# sloth-lang 2.0 设计文档

## 静态强类型化与 MLIR 编译基础设施改造

| 项目 | 内容 |
| --- | --- |
| 文档版本 | v1.0（草案） |
| 文档日期 | 2026-09-14 |
| 基准版本 | 《sloth-lang 语言参考手册》2026-09-08 版 |
| 文档状态 | 设计评审稿 |
| 前置决策 | **移除用户态协程（fiber）特性** |

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
5. **可实现的运行时**：移除 fiber 后，运行时收敛为 GC、字符串池、容器、IO/FFI 四个组件。

### 1.3 非目标

- 不追求与 sloth-lang 1.0 的源代码兼容（语义差异见 §10）；
- 不实现线程/并行（fiber 移除后语言为单线程模型，线程留待 3.0 评估）；
- 不实现宏系统、异步/await、类型类（type class）等高阶特性；
- 首版不提供增量编译与 IDE 工具链（语言服务器列为后续工作）。

### 1.4 术语

| 术语 | 含义 |
| --- | --- |
| sloth2 | 本文档描述的 sloth-lang 2.0 |
| 单态化（Monomorphization） | 编译期为每个泛型实例生成具体类型副本 |
| `dyn Trait` | 动态分派的对象类型（trait object） |
| statepoint | LLVM 用于精确 GC 的栈上根定位机制 |
| dialect | MLIR 中自定义类型与操作的扩展包 |

---

## 2. 类型系统

### 2.1 类型全集

| 类别 | 类型 | 说明 |
| --- | --- | --- |
| 单元 | `unit` | 空类型，替代 1.0 中"无返回值函数隐式返回 nil" |
| 布尔 | `bool` | `true` / `false`，**取消** 1.0 的"任意值隐式转 bool" |
| 数值 | `int`（i64）、`float`（f64） | 新增 `int`。1.0 仅有 f64，但静态语言中数组索引、取余、位语义需要整数；f64 定名为 `float`，不做 1.0 兼容保留 |
| 字符串 | `str` | UTF-8 不可变字符串，保留 string interning |
| 范围 | `range` | 由 `..`、`..=` 构造，元素类型 `int` |
| 数组 | `Array<T>` | 同构动态数组，替代 1.0 的异构数组 |
| 字典 | `Map<K, V>` | 键类型 `K` 必须实现 `Hashable` trait（见 §2.5），值类型 `V` |
| 可空 | `T?` | 可选类型，`nil` 字面量只能赋给 `T?` |
| 函数 | `(A, B) -> R` | 函数与闭包的一等类型 |
| 类 | `class C ...` | 用户定义引用类型，单继承 |
| 接口 | `trait T ...` | 结构化能力的静态抽象，类可 `impl` 多个 trait |
| 动态对象 | `dyn Trait` | 运行时多态的唯一出口（虚表 + 数据指针） |

关键决策说明：

- **`T?` 替代通用 `nil`**：1.0 中任何类型都可为 `nil` 是运行时错误的主要来源（对 `nil` 的运算报错）。sloth2 中只有显式声明为 `T?` 的类型可持有 `nil`，使用值前必须判空（`if` 条件中的 `is not nil` 收窄或 `?.` 语法，见 §3.6）。
- **取消隐式布尔转换**：`while (x)` 要求 `x: bool`。1.0 的"Nil 转 false、其余转 true"规则与强类型冲突，且与 `T?` 判空习惯重叠。
- **无隐式数值转换**：`int` 与 `float` 之间不隐式转换，提供显式内置函数 `int(x)` / `float(x)`。算术运算两侧类型必须一致。

### 2.2 类型推断

采用**局部类型推断**（类比 Rust，而非全局 Hindley-Milner）：

- 变量声明 `var x = expr;` 可省略标注，类型取自初始化表达式；
- **以下位置类型标注强制**：函数参数、函数返回值（无标注且函数体仅单个 `return` 时可推断）、类字段、泛型无法从参数推断的类型参数；
- `var x: int = 1;` 显式标注与推断冲突时以标注为准做子类型/可空性检查。

### 2.3 泛型

- 函数与类型均可泛型：`func map<T, R>(arr: Array<T>, f: (T) -> R): Array<R>`；
- 泛型参数可带 trait 约束：`func sort<T: Comparable>(arr: Array<T>)`；
- 实现策略：**单态化**（编译期为每个具体类型实例生成代码副本）。相比字典传递，单态化生成代码更快、对 MLIR 优化更友好，代价是代码膨胀，可接受。

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

- 运算符重载改为实现预定义 trait（见 §3.4）；
- `dyn Trait` 提供运行时多态：`var s: dyn Speaker = Fish();`；
- trait 无字段、无构造器，方法可提供默认实现；
- 预定义基础 trait 包括：`Hashable`（可哈希，`Map<K, V>` 的键约束，含 `hash(): int` 方法）、`Equatable`、`Comparable`、`Display`。`int`、`float`、`bool`、`str`、`range` 内置实现 `Hashable`；用户类型显式 `impl Hashable` 后即可作为 `Map` 的键类型。`Hashable` 类型必须同时实现 `Equatable`（哈希相等性契约）。

### 2.6 值类型、引用类型与内存表示

类型按内存表示明确划分为两类（与 §4.3.1 的 MLIR 类型映射一一对应）：

**值类型**——直接内联存储，局部变量分配在原生栈上，赋值/传参为复制语义：

| 类型 | 表示 |
| --- | --- |
| `unit` | 零大小，无表示 |
| `bool` | `i1` |
| `int` | `i64` |
| `float` | `f64` |
| `range` | 两个 `i64`（下界、上界；开闭性由上界编码区分） |

**引用类型**——GC 堆分配，变量持有指针，赋值/传参为引用语义：

| 类型 | 表示 |
| --- | --- |
| `str` | 字符串池 Entry 指针（interned，相等比较即指针比较） |
| `Array<T>` / `Map<K, V>` | 指向 GC 堆容器对象的指针，元素按单态化后的具体类型内联存储（无装箱） |
| 类实例 | GC 堆对象指针，对象头含类型描述符（见 §5.1） |
| 闭包 `(A) -> R` | 环境指针 + 函数指针，共两个机器字 |
| `dyn Trait` | 数据指针 + 虚表指针（fat pointer），共两个机器字 |

**可空类型 `T?` 的表示：**

- **`T` 为引用类型**：`T?` 复用指针表示，`nil` 即空指针（null pointer optimization），`is not nil` 编译为一次指针判零，零额外开销；
- **`T` 为值类型**：`T?` 采用带标签布局 `{ payload: T, has_value: i1 }`。例如 `int?` 占 16 字节（8 字节有效载荷 + 1 字节标志 + 对齐填充），`bool?` 占 2 字节；
- 不允许嵌套可空：`T??` 编译期报错，避免多层标签歧义。

**实现表示（定案）**：上表为语言语义表示；运行时统一采用 §5.1 的**带 tag 单 i64 词面**——引用 `ptr|1`、`int` 63 bit（`v<<1`）、`float` f63（`(bits&!1)>>1`，牺牲尾数 LSB）、`bool` `0/2`。由此 `int` 收窄为 63 bit、`float` 损失 1 ULP，属有意取舍（换取掩码/平行函数族消除）。

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
| fiber | `fiber.*` 扩展函数 | **全部移除** |
| 匿名函数 | `\\|a, b\\| { ... }` | 保留，参数类型可推断或标注 `\\|a: int\\| -> int { ... }` |
| 管道 `\\|>` | 动态调用单参函数 | 保留为语法糖，编译期解析类型 |
| 字符串插值 | `"${expr}"` | 保留，编译期展开 |
| map 字面量 | `@("k": v)` | 保留 `@(k: v)`，类型 `Map<K, V>`，K 需实现 `Hashable`，K/V 取各键值公共类型 |
| 关键字 | 19 个 | 新增 `pub` `trait` `impl` `dyn` `int` `float` `str` `bool`，移除（无 fiber 关键字） |

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

- 函数重载：首版**不支持**同名重载（降低解析复杂度，后续版本评估）。

### 3.4 运算符重载

由"魔术方法"改为实现预定义 trait，解析在编译期完成：

| 运算符 | trait | 方法签名 |
| --- | --- | --- |
| `+` | `Add<Rhs, Out>` | `func __add__(rhs: Rhs): Out` |
| `-` | `Sub<Rhs, Out>` | `__sub__` |
| `*` | `Mul<Rhs, Out>` | `__mul__` |
| `/` | `Div<Rhs, Out>` | `__div__` |
| `%` | `Mod<Rhs, Out>` | `__mod__` |
| 一元 `-` | `Neg<Out>` | `func __neg__(): Out` |
| `> >= < <=` | `Comparable` | `__gt__ __ge__ __lt__ __le__` |
| `== !=` | `Equatable` | `__eq__ __ne__` |
| `[]` / `[]=` | `Indexable<Idx, V>` | `__index__(idx: Idx): V` / `__assign__(idx: Idx, val: V)` |

1.0 的 `+` 拼接字符串/数组语义保留为标准库对 `str`、`Array<T>` 的内置 `Add` 实现。

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
import "sloth/sloth_lib/func_tool.slt";
import "./geometry.slt" as geo;

let p = geo.Vec2(1, 2);
```

- 编译期解析模块依赖图，循环依赖报错；
- 被导入模块中只有 `pub` 声明可见（1.0 的"顶层全部自动导出"废弃）；
- 模块缓存：同一编译单元内每个模块只编译一次，等价于 1.0 "模块只执行一次"的语义；
- 不再隐式启动新协程加载模块（1.0 依赖 fiber 的加载机制随 fiber 一并移除）。

### 3.8 字符串插值与管道运算符

- 字符串插值 `"hello ${expr}"` 保留，要求 `expr` 的类型实现标准库 trait `Display`（`int`、`float`、`bool`、`str` 内置实现；类可实现）；编译期展开为拼接或格式化缓冲调用，类型错误在编译期报告（1.0 为运行时行为）；
- 管道运算符 `|>` 保留为纯语法糖：`x |> f` ≡ `f(x)`，`x |> f(a, b)` ≡ `f(a, b, x)`（左操作数作为**最后一个**实参，以适配 `map(f)` 柯里化风格——此条与 1.0 行为兼容，1.0 中 `functool.map` 返回单参闭包）；编译期检查右操作数为可调用类型且参数匹配。

### 3.9 EBNF 文法（sloth2 完整定义）

```ebnf
prog            ::= ( import_decl | stmt | decl )*

import_decl     ::= 'import' STRING ( 'as' IDENTIFIER )? ';'

decl            ::= 'pub'? ( var_decl | let_decl | func_decl | class_decl | trait_decl )

var_decl        ::= 'var' IDENTIFIER ( ':' type )? '=' expr ';'
let_decl        ::= 'let' IDENTIFIER ( ':' type )? '=' expr ';'

func_decl       ::= 'func' IDENTIFIER type_params? '(' param_list ')' ( ':' type )? block
type_params     ::= '<' IDENTIFIER ( ':' IDENTIFIER )? ( ',' IDENTIFIER ( ':' IDENTIFIER )? )* '>'
param_list      ::= ( param ( ',' param )* ( ',' variadic_param )? | variadic_param )?
param           ::= IDENTIFIER ( ':' type )?
variadic_param  ::= IDENTIFIER '...' ':' 'Array' '<' type '>'   // 仅可位于参数列表末尾

class_decl      ::= 'class' IDENTIFIER type_params? ( ':' IDENTIFIER )?
                    ( 'impl' IDENTIFIER ( ',' IDENTIFIER )* )?
                    '{' ( 'pub'? ( field_decl | func_decl ) )* '}'
field_decl      ::= ('var' | 'let') IDENTIFIER ':' type ';'

trait_decl      ::= 'trait' IDENTIFIER '{' func_decl* '}'

type            ::= 'int' | 'float' | 'bool' | 'str' | 'unit' | 'range'
                  | 'Array' '<' type '>'
                  | 'Map' '<' type ',' type '>'
                  | '(' ( type ( ',' type )* )? ')' '->' type
                  | 'dyn' IDENTIFIER
                  | IDENTIFIER type_args?
                  | type '?'
type_args       ::= '<' type ( ',' type )* '>'

stmt            ::= expr_stmt
                  | while_stmt
                  | for_stmt
                  | if_stmt
                  | break_stmt
                  | continue_stmt
                  | return_stmt
                  | assignment_stmt

block           ::= '{' stmt* '}'
expr_stmt       ::= expr ';'
while_stmt      ::= 'while' '(' expr ')' block
for_stmt        ::= 'for' '(' 'var' IDENTIFIER ':' expr ')' block
if_stmt         ::= 'if' '(' expr ')' block ( 'else' block )?
break_stmt      ::= 'break' ';'
continue_stmt   ::= 'continue' ';'
return_stmt     ::= 'return' expr? ';'
assignment_stmt ::= assignable '=' expr ';'
assignable      ::= IDENTIFIER ( '.' IDENTIFIER | '[' expr ']' )*

expr            ::= literal
                  | lambda_expr
                  | list_expr
                  | map_expr
                  | IDENTIFIER
                  | expr '(' ( expr ( ',' expr )* )? ')'
                  | expr '[' expr ']'
                  | expr '.' IDENTIFIER
                  | expr binop expr
                  | unop expr
                  | '(' expr ')'

literal         ::= INT | FLOAT | STRING | 'true' | 'false' | 'nil'
lambda_expr     ::= '|' param_list '|' ( '->' type )? block
list_expr       ::= '[' ( expr ( ',' expr )* )? ']'
map_expr        ::= '@' '(' ( entry_pair ( ',' entry_pair )* )? ')'
entry_pair      ::= expr ':' expr                 // 键表达式类型须实现 Hashable

binop           ::= '+' | '-' | '*' | '/' | '%'
                  | '>' | '<' | '>=' | '<=' | '==' | '!='
                  | 'and' | 'or' | '..' | '..=' | 'is' ( 'not' )? | '|>' | '?:'
unop            ::= 'not' | '-'
```

关键字全集（27 个）：`and or not true false for var let if else while func nil return class super this break continue is pub trait impl dyn as`（`int float bool str unit range Array Map dyn` 为上下文关键字，仅在类型位置保留，可作标识符使用以兼容旧代码）。

---

## 4. 编译器架构

### 4.1 总体流水线

```javascript
源代码 (.slt)
   │
   ▼
[1] 词法分析 Lexer ──────────► Token 流
   │
   ▼
[2] 语法分析 Parser ─────────► AST（递归下降，复用 1.0 语法规则）
   │
   ▼
[3] 名称解析 ────────────────► 作用域/模块符号表，循环依赖检查
   │
   ▼
[4] 类型检查/推断 ───────────► 带类型标注的 AST（Typed AST）
   │      ├─ 局部类型推断
   │      ├─ trait 约束求解
   │      ├─ 闭包捕获分析（替代 1.0 的运行时 UpValue Close）
   │      └─ 类型收窄（flow typing）
   ▼
[5] 单态化 ──────────────────► 泛型函数/类型展开为具体实例
   │
   ▼
[6] MLIR 生成 ───────────────► sloth dialect 高层 IR
   │
   ▼
[7] Dialect Lowering ────────► func / scf / cf / arith / llvm dialects
   │
   ▼
[8] MLIR 优化 pass ──────────► 内联、CSE、DCE、消虚、循环优化
   │
   ▼
[9] LLVM 后端 ───────────────► 目标机器码 / 目标文件
   │
   ▼
[10] 链接 libsloth_rt ───────► 可执行文件
```

**与 1.0 的本质区别**：1.0 是 one-pass 直出字节码，无前述 [3][4][5] 阶段，也没有 AST。sloth2 前端整体重写，词法与语法规则可复用，解析器骨架可改造复用。

### 4.2 前端要点

1. **AST**：新增。节点携带源码位置（诊断用）与类型槽（供 [4] 回填）；
2. **名称解析**：模块符号表 + 词法作用域栈；`import` 在此阶段完成依赖图构建与拓扑排序，检测循环依赖；
3. **类型检查**：

- 两遍策略：第一遍收集所有顶层声明签名（允许前向引用与递归函数/类——1.0 one-pass 无法做到）；
- 第二遍检查函数体，执行局部推断、trait 约束求解、`is` 收窄；
- 闭包捕获分析：静态确定捕获变量及其逃逸性，直接决定捕获环境结构体的字段布局与内存分配方式（替代 1.0 的 `UpValue::Ref/Closed` 运行时机制）；

4. **单态化**：以"泛型定义 + 具体类型实参"为键缓存实例；递归泛型（如 `f<T>` 调用 `f<Array<T>>`）需实例化深度限制与诊断；
5. **错误诊断**：所有类型错误携带源码位置与期望/实际类型，一次编译尽可能报多个错误（不做 fail-fast）。

### 4.3 sloth MLIR Dialect 设计

#### 4.3.1 类型

```mlir
!sloth.string                      // interned str
!sloth.array<T>                    // GC 托管数组
!sloth.map<K, V>                   // GC 托管字典（K 须实现 Hashable）
!sloth.optional<T>                 // T?
!sloth.range
!sloth.class<@Cat>                 // 类实例引用（GC 托管）
!sloth.closure<(i64) -> i64>       // 闭包（环境指针 + 函数指针）
!sloth.dyn<@Speaker>               // trait object（数据指针 + 虚表指针）
```

`int`/`float`/`bool`/`unit` 直接映射 MLIR 内建类型（`i64`/`f64`/`i1`/无返回值），作为值类型直接分配在原生栈上、**不再装箱**——这是性能收益的根本来源；引用类型与 `T?` 的具体内存布局见 §2.6。

#### 4.3.2 操作（节选）

| 操作 | 语义 | Lowering 目标 |
| --- | --- | --- |
| `sloth.gc_alloc` | GC 堆分配（对象头 + 类型描述符） | 调用运行时 `sloth_gc_alloc` |
| `sloth.string_literal` | interned 字符串字面量 | 运行时字符串池查询/插入 |
| `sloth.string_concat` | 字符串拼接 | 运行时 `sloth_str_concat` |
| `sloth.string_interp` | 插值展开（Display 调用序列） | `scf` + 运行时 |
| `sloth.array_new / push / get / set` | 数组操作（带边界检查） | 运行时 + `arith`（越界 → `sloth.panic`） |
| `sloth.call_indirect` | 闭包/trait object 调用 | `func.call_indirect`（虚表/环境指针拆解） |
| `sloth.call_virtual` | 类虚方法调用 | 虚表加载 + 间接调用 |
| `sloth.type_test` | `is` 测试 | 类型描述符比对（考虑继承链的祖先表） |
| `sloth.closure_create` | 闭包创建（捕获环境打包） | `gc_alloc` + 字段写入 |
| `sloth.iter_begin / iter_next` | 迭代协议 | 内联展开 `Iterable` 实现为 `scf.while` |
| `sloth.panic` | 不可恢复运行时错误 | 运行时 `sloth_panic`（打印 + 退出） |

#### 4.3.3 Lowering 路径

```javascript
sloth dialect
  ├─ 第一轮：迭代/插值/管道展开，闭包转换（closure conversion）
  │     → 剩余 sloth 对象操作
  ├─ 第二轮：对象/虚表/字符串/容器操作
  │     → func, scf, cf, arith + 运行时调用
  └─ 第三轮：--convert-func-to-llvm 等标准 conversion
        → llvm dialect（gc.strategy 标注，见 §5.1）
        → LLVM IR → 机器码
```

#### 4.3.4 可直接复用的 MLIR 优化

- `-inline`（配合类型已知信息完成消虚）；
- `-cse` / `-canonicalize` / `-sccp`；
- `-loop-invariant-code-motion` 等 `scf`/`affine` 循环优化（`for` 循环 lowering 到 `scf.for` 后自然获得）；
- 数值代码（`int`/`float` 不装箱）直接进入 `arith` 优化管线，这是相对 1.0 虚拟机最大的单项性能来源。

---

## 5. 运行时设计（libsloth_rt）

fiber 移除后，运行时不再有协程栈管理，收敛为四个组件，以 Rust 实现并编译为静态/动态库随程序链接。

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

**插入点汇总**：绑定/赋值、字段与元素写、作用域退出、`return`、语句级临时量冲刷、条件分支前与 merge 处的冲刷，以及容器迭代协议——`iter()` 结果为 owned（循环退出时释放），`next()` 结果为 owned（每轮迭代末释放，`continue` 路径同样覆盖）。

**诊断**：rt 暴露 `sloth_rc_live`/`sloth_rc_drops` 零参内置，供 spec/churn 断言计数收敛。

> 历史缺陷（本协议修复）：owned 临时量此前仅在命名/全局赋值处被接管，方法接收者、`len`/索引、运算符、实参、容器迭代等消费者既不登记也不冲刷，导致每个临时量滞留 +1（见 PLAN §10）。

### 5.2 字符串池

- 保留 1.0 的 string interning：相等比较即指针比较；
- 改动：从"每 VM 实例一个 StringPool"变为**进程级单例**（无 VM 了）；首版单线程无需锁，结构沿用哈希表保证 Entry 唯一。

### 5.3 容器与内建类型方法

- `Array<T>` / `Map<K, V>` 运行时实现（容量增长、哈希），元素布局由单态化后的具体类型决定（无装箱，直接内联存储）；`Map` 键的哈希与相等比较经由 `Hashable`/`Equatable` 的单态化实现调用；
- 基础类型方法（如 `arr.len()`、`str.len()`）由编译器解析为标准库泛型函数——彻底解决 1.0 "基础类型不是类导致方法依赖折衷实现"的不一致，顺带补齐 1.0 缺失的 `len()` 等标准库。

### 5.4 FFI 与宿主互操作

1.0 的扩展方式（满足 `fn(&mut Vm, usize, bool)` 签名、手工操作 VM 栈）**废弃**，改为声明式外部函数：

```rust
extern func floor(x: float): float;          // 链接期解析符号
```

- 参数/返回值按 C ABI 或定义的 sloth ABI 传递，编译器自动生成 marshalling；
- `OpaqueData` 由 `extern type`（不透明类型声明）替代，仅能经 extern 函数传递，编译期保证脚本侧无法解引用；
- 移除 fiber 后无协程栈切换约束，extern 函数就是普通原生调用。

### 5.5 错误模型

首版不提供异常机制：

- 编译期：所有类型错误；
- 运行时不可恢复错误（数组越界、`nil` 解引用、整数除零、断言失败）：`sloth.panic`，打印诊断并终止进程；
- 可恢复错误：约定返回 `T?` 或标准库 `Result<T, E>`（以泛型枚举类实现）。

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
| `fiber.create/resume/yield/...` | **无替代**（特性移除）；并发需求由宿主语言承担 |
| 魔术方法 `__iter__`/`__next__` | 实现 `Iterable<T>`/`Iterator<T>` trait |
| 魔术方法 `__add__` 等 | 实现对应运算符 trait（语义等价，仅组织方式变化） |
| 方法引用 `orange.whoami` | 保留，类型为 `() -> unit`，this 绑定语义不变 |
| 字符串插值、管道、范围、map 字面量 | 语法不变，获得编译期类型检查 |

---

## 7. 风险与阻碍评估

| # | 风险 | 等级 | 说明与对策 |
| --- | --- | --- | --- |
| 1 | 前端整体重写 | 高 | one-pass 架构无法演进式改造，AST+类型检查必须重写；对策：词法/文法规则与测试用例最大化复用 |
| 2 | 类型系统落地复杂度 | 高 | 手册作者自述"不亚于实现一种语言"；对策：局部推断（非全局 HM）、首版不做函数重载、trait 不带关联类型，严格控制特性面 |
| 3 | 精确 GC 工程风险 | 中 | statepoint 重写与 stack map 生成易出隐蔽 bug；对策：MVP 先用 Boehm GC 解耦编译器与 GC 的开发进度 |
| 4 | 单态化代码膨胀与编译速度 | 中 | 泛型实例爆炸；对策：实例缓存 + 后续考虑对引用类型共享实例（类型擦除混合策略） |
| 5 | 动态模块加载能力丢失 | 低 | `import` 编译期化后失去运行时脚本热加载；对策：文档明确，插件场景由宿主 FFI 承接 |
| 6 | 标准库缺口 | 中 | 1.0 标准库本就不完整，2.0 需同步建设（容器方法、`Display`、`Result` 等）；对策：标准库以 sloth2 自身编写（自举验证），仅 IO/GC 走运行时 |
| 7 | fiber 移除的用户影响 | 已接受 | 前置决策，文档与迁移指南明示无替代方案 |

> 原方案中最难的两项——fiber 与 LLVM 无栈协程模型的错配、fiber 栈与 GC 根扫描的交互——随 fiber 移除**整体消除**，这是本次裁剪的最大架构收益。

---

## 8. 分阶段实施路线图

| 阶段 | 内容 | 验收标准 |
| --- | --- | --- |
| **P0 语言定稿** | 类型系统与语法冻结；编写语言规范测试集（正/负类型用例） | 本文档评审通过；≥200 条规范测试用例 |
| **P1 前端** | Lexer/Parser/AST、名称解析、类型检查（非泛型子集） | 非泛型程序的类型检查通过/报错符合规范 |
| **P2 MLIR 端到端 MVP** | sloth dialect（标量 + 函数 + 控制流）→ LLVM；Boehm GC；`hello world` 级程序原生运行 | 算术/分支/循环/函数程序编译运行，数值正确 |
| **P3 对象与闭包** | class/继承/虚表、trait 与 `dyn`、闭包转换、字符串池 | 1.0 面向对象示例（Cat/Dog/Fish 改写版）运行正确 |
| **P4 泛型与单态化** | 泛型函数/类型、trait 约束、容器泛型化、迭代协议 | `map`/`reduce` 泛型版管道示例运行正确 |
| **P5 模块与标准库** | 编译期 `import`、`pub` 可见性、核心标准库 | 多模块程序编译；标准库自举 |
| **P6 优化与精确 GC** | 消虚/内联调优、statepoint 精确 GC 替换 Boehm | 性能基准 vs 1.0 提升 ≥ 10x；GC 压力测试无泄漏/误回收 |

里程碑建议：P2 完成即具备持续集成价值（端到端可跑），P4 完成即语言特性完备，可开放试用。

---

## 9. 附录

### 9.1 示例：1.0 Hello World 变体的 2.0 版本

```rust
import "sloth/sloth_lib/func_tool.slt";

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

差异仅在于：`import` 为编译期声明；`map` 为标准库泛型函数；顶层以 `pub func main()` 组织。输出与 1.0 完全一致。

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
| MLIR 接入 | melior（Rust MLIR 绑定）或 C API | 首版建议 C API，绑定成熟度高 |
| MVP GC | Boehm-Demers-Weiser libgc | 见 §5.1 |
| 目标版 GC | LLVM statepoint + 自研标记-清理 | P6 阶段 |
| 构建/包管理 | 暂不涉及，随标准库阶段评估 | — |