# 24. 内建函数

内建函数由发射器在调用点直接识别（不是语言层函数）。当前集合：

| 内建 | 签名 | 说明 |
| --- | --- | --- |
| `len(x)` | `(str\|Array\|Map) -> int` | `str` 为**字节数** |
| `int(x)` | `(float\|float?…) -> int` | 向零截断；`int(str)` 不支持（MVP） |
| `float(x)` | `(int\|int?…) -> float` | `uint` 用无符号提升；`float(str)` 不支持（MVP） |
| `int8(x)`/`int16`/`int32`/`uint`/`uint8`/… | `(整数\|float) -> 定宽整数` | 截断到目标宽度（环绕）；见 §5.6 |
| `keys(m)` | `(Map<K,V>) -> Array<K>` | 新数组（owned） |
| `values(m)` | `(Map<K,V>) -> Array<V>` | 新数组（owned） |
| `chars(s)` | `(str) -> StrChars` | 惰性码点迭代器（`next(): int?`，见 §14.3） |
| `typeid(x)` / `type_name(x)` | `(ref\|any) -> int\|str` | 运行时类型身份 |
| `sloth_rc_live()` | `() -> int` | ARC 存活计数 |
| `sloth_rc_drops()` | `() -> int` | ARC 累计析构数 |

> 上表 `sloth_rc_live()` / `sloth_rc_drops()` 是**源码层**名字（发射器识别为内建）；
> 它们生成的调用是 `@__sloth_rc_live` / `@__sloth_rc_drops`。

`print` **不再是编译器内建**：它由编译器自动注入的 Sloth prelude 实现
（源码形态在 `lib/prelude/print.slt`，经 `include_str!` 嵌入），基于运行时
`any` 写入面：

```sloth
extern func __sloth_rt_write(v: any): str;   // 按运行时类型渲染为 str（递归容器）
extern func __sloth_rt_puts(v: str): unit;   // 仅打印一个 str（追加换行）
pub func print(v: any): unit { __sloth_rt_puts(__sloth_rt_write(v)); }
```

任意值传入 `print`/`${}` 时会隐式装箱为 `any`（见 §5.1、§24.2），因此
`print` 可打印数组/Map/嵌套容器/类实例，`${}` 插值也统一走
`__sloth_rt_write`（不再有逐类型的 `__sloth_str_push_*` 分派）。

方法形式的等价物：`a.len()`、`s.len()`、`m.len()`；`s.chars()`（等价 `chars(s)`，
惰性码点迭代器）；`w.upgrade()`（`Weak<T>` → `T?`）。

值型 optional（`int?` 等）经 `any` 装箱后仍是 nil-aware：`nil` 打印为 `nil`，
盒中的 `0`/`false` 正常显示。

## 24.1 示例

```sloth
{{#include examples/builtins.sl}}
```

```mlir
{{#include examples/builtins.mlir}}
```

## 24.2 `any` 与运行时渲染

`any` 是运行时类型化的顶层引用类型：单 word，`0` = `nil`，否则指向一个 rc 盒
`{ TypeDesc*, word }`。每个静态类型在编译期发射一份结构化 `TypeDesc`
（含 `Array<T>`/`Map<K,V>`/`T?` 的递归元素描述符），因此 `write` 可以：
标量按原生格式，`str` 原样，数组 `[a, b]`，Map `{k: v}`，`T?` nil→`nil`，
类实例有 `impl Display` 时调用 `to_str`，否则回退打印类名。

`x is T` 在 `any` 上做**运行时**判定（标量按 kind、类按运行时 class id 链），
命中后可在分支内把 `x` 收窄为具体类型。

## 24.3 `typeid` / `type_name` 与引用类型运行时身份

`typeid(x) -> int` 与 `type_name(x) -> str` 覆盖所有**引用类型**，粒度为单态：

- 类实例与 `dyn`：经对象的 `ObjInfo`（构造时由 `@__sloth_cls_name` 注册名字）取
  **最派生**的具体类，`@__sloth_obj_cls_id` / `@__sloth_obj_type_name` 解析；
- 其余引用类型（`str`/`Array`/`Map`/`Tensor`/`Fiber`/`Channel`/`Weak`/`range`/
  闭包/`any` 等）：编译期常量 id 与名字。`TYPEID_BASE = 2^40`，与类 id 空间不
  相交；
- 可空的引用类型自动解包，`nil` 的 `type_name` 为 `"nil"`、`typeid` 为 `0`；
- 裸值类型（`int`/`float`/`bool`）调用是编译期诊断（v1.1 的「值可调用」语义）。

## 24.4 内建模块

除上表调用点内建外，`fiber.*`、`tensor.*`（张量扩展）与 `thread.*` / `channel.*`
/ `mutex.*` / `atomic.*`（多线程扩展）是发射器识别的**内建模块**，详见 §25、§27、
§28。

