# 13. 字典 Map

## 13.1 基本操作

```sloth
var m = @("a": 1, "b": 2);   // 字面量
m["a"];                       // 读
m["c"] = 3;                   // 写（不存在则插入）
len(m);                       // 或 m.len()
keys(m);                      // Array<K>
values(m);                    // Array<V>
```

- 键类型 `K` 必须实现 `Hashable`。`int`/`float`/`bool`/`str`/`range` 内置实现；
  用户类需 `impl Hashable`（且必须同时 `impl Equatable`，见 §19）。
- **混用键族**（如 `@("a": 1, 2: 3)`）编译错误 `mixed map key types`。
- `float` 键走词面哈希路由（`float` 实现 `Hashable`）。
- 空字面量需要上下文类型：`var m: Map<str,int> = @();`。
- 引用键（如类实例）走内容哈希：相等的键命中同一槽（`__hash__`/`__eq__`）。
- **迭代顺序不保证**（哈希表）。

## 13.2 Entry 迭代

`for (var e: m)` 的元素是 **`Entry<K,V>` 记录**，字段为 `key` 与 `val`：

```sloth
for (var e: m) {
    print("${e.key} = ${e.val}");
}
```

`Entry<K,V>` 由编译器自动注入的 stdlib 泛型类提供。发射端对每次迭代**即时**构造
Entry 对象，因此是"活"的键值对视图。

## 13.3 示例

```sloth
{{#include examples/maps.sl}}
```

```mlir
{{#include examples/maps.mlir}}
```

对应运行时符号：`sloth_map_new`、读 `sloth_map_str_get`（`str` 键专用路由）/
`sloth_map_get` + `sloth_map_get_h`（`int` 等词键，`_h` 变体带预计算哈希）、写
`sloth_map_str_set`/`sloth_map_set`/`sloth_map_set_h`、`sloth_map_len`、以及
`sloth_map_keys`/`sloth_map_values`。注意 `keys()`/`values()` 返回的是**新数组**
（owned 生产者），调用者负责结算。

> 自举：开放寻址 + 线性探测 + FNV/mix64 哈希与扩容逻辑同样在注入的
> `lib/prelude/containers.slt` 中以 sloth 实现；析构经 `__dispose__` 钩子注册。
