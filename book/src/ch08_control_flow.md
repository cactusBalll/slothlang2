# 8. 控制流

## 8.1 语句

```sloth
if cond { ... } else { ... }     // 也可写 if (cond) { ... }
while cond { ... }               // 也可写 while (cond) { ... }
for x in iterable { ... }        // 形式 A：for x in expr
for (var x: iterable) { ... }    // 形式 B：for (var x: expr)
break;
continue;
return expr?;
```

- `if` / `while` 的**条件必须是 `bool`**，否则报 `type mismatch in condition`。
- `if` / `while` 接受带括号与不带括号两种写法（兼容 1.0 风格）。
- `for` 的两种写法等价，`for` 变量在循环体作用域内新声明。
- `break` / `continue` 作用于最近的循环。

## 8.2 循环与降级

- `while`/`for` 降级为 `cf.br` / `cf.cond_br` 的基本块控制流。
- `for` 基于**迭代协议**（§16）：`str`（按字符）、`Array<T>`、`Map<K,V>`
  （元素 `Entry<K,V>`）、`range`（元素 `int`）内置实现；用户类型实现
  `Iterable`/`Iterator`。
- 循环体内的引用临时量在每轮结束（含 `continue` / `break` 路径）被 ARC 结算
  （§23）。

## 8.3 示例

```sloth
{{#include examples/control.sl}}
```

```mlir
{{#include examples/control.mlir}}
```

可以观察到：`if` 产生 `cf.cond_br` 分叉到 then/else 基本块；`while` 是
"条件块 ↔ 体块" 的回边；`for x in 0..5` 的字面量范围被内联为计数循环——边界是空的
`int` 词、循环变量 `x` 是逐轮自增的栈槽，不做范围盒分配。`for (var c: "ab")` 迭代
字符串，走 `__sloth_str_clen`（字符数）+ `__sloth_str_char`（第 i 个字符）。
