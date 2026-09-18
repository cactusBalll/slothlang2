# 15. 可空类型与 Weak

## 15.1 `T?` 与 `nil`

- 只有显式声明为 `T?` 的类型可持有 `nil`；`nil` 只能赋给 `T?`。
- 不允许嵌套：`int??` 报 `nested optional T?? is not allowed`。
- 表示（见 §5）：
  - 引用型 `T?`：句柄本身，`nil` 即 `0`；
  - 值型 `T?`（`int?`/`float?`/`bool?`）：指向 rc 跟踪 payload 盒的引用词，
    `0` = `nil`。因此盒中的 `0`/`0.0`/`false` 是**合法活值**，与 `nil` 不混淆。

## 15.2 判空与收窄

```sloth
var i: int? = nil;
if i is not nil { print(i); }   // 分支内 i 收窄为 int
if i is nil { ... } else { print(i); }  // else 分支同样收窄
let n = i ?: 7;                 // Elvis：nil 时取 7
```

- 收窄支持 `if` 的 **then 分支**与 **else 分支**。
- 收窄对**标识符**生效；对 `n.next.v` 这类链式表达式不支持——需先取出中间值再
  判空。`?.` 可选链**未实现**（设计明确推迟）。对可空接收者直接取字段会报
  `field `v` on an optional receiver`。
- `int(opt)` / `float(opt)` 会先解盒再转换（nil → `0`）。
- `print` 与 `${}` 对 optional 走 nil-aware 面：`nil` 打印为 `nil`。

## 15.3 示例

```sloth
{{#include examples/optionals.sl}}
```

```mlir
{{#include examples/optionals.mlir}}
```

可观察到的运行时面：值型 optional 用 `sloth_box_new`/`sloth_box_get`；
print/插值对 optional 走 `sloth_rt_print_opt`/`sloth_str_push_opt`（带 kind 标记）；
`is nil` 是对词面是否为 `0` 的判定。

## 15.4 `Weak<T>`

`Weak<T>` 是一个**不增加目标计数**的 rc 盒，用于打破强引用环：

```sloth
class Node {
    var next: Weak<Node> = nil;   // 弱后向边，不参与强环
}
var w: Weak<Node> = node;
var strong = w.upgrade();         // -> Node?（死目标为 nil）
```

- `upgrade()` 是 Receiver 内置：返回 `T?`；`Weak<int>` 会一跳箱化为 `int?`。
- 弱盒自身参与 rc；目标归零时运行时沿侵入式弱链把 `target` 置 `0`，此后所有
  `upgrade()` 返回 `nil`。
- 把**强句柄**存进 `Weak` 元素槽（push/字面量/Map 写）会自动装箱为弱盒。

## 15.5 Weak 示例

```sloth
{{#include examples/weak.sl}}
```

```mlir
{{#include examples/weak.mlir}}
```

`w = tmp` 发射 `sloth_weak_new`（不 retain `tmp`）；`w.upgrade()` 发射
`sloth_weak_upgrade`，返回的 `T?` 再做 nil 判定。目标死亡后 `upgrade()` 得到 `0`。
