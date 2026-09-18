# 24. 内建函数

内建函数由发射器在调用点直接识别（不是语言层函数）。当前集合：

| 内建 | 签名 | 说明 |
| --- | --- | --- |
| `print(x)` | `(T) -> unit` | 按类型分派到 `sloth_rt_print_*`；类需 `impl Display` |
| `len(x)` | `(str\|Array\|Map) -> int` | `str` 为**字节数** |
| `int(x)` | `(float\|int?…) -> int` | 向零截断；`int(str)` 不支持（MVP） |
| `float(x)` | `(int\|float?…) -> float` | `float(str)` 不支持（MVP） |
| `keys(m)` | `(Map<K,V>) -> Array<K>` | 新数组（owned） |
| `values(m)` | `(Map<K,V>) -> Array<V>` | 新数组（owned） |
| `sloth_rc_live()` | `() -> int` | ARC 存活计数 |
| `sloth_rc_drops()` | `() -> int` | ARC 累计析构数 |

方法形式的等价物：`a.len()`、`s.len()`、`m.len()`；`w.upgrade()`（`Weak<T>` → `T?`）。

`print` / `${}` 对值型 optional 走 nil-aware 面（`sloth_rt_print_opt` /
`sloth_str_push_opt`），因此 `nil` 打印为 `nil`，而盒中的 `0`/`false` 正常显示。

## 24.1 示例

```sloth
{{#include examples/builtins.sl}}
```

```mlir
{{#include examples/builtins.mlir}}
```

`print` 按实参类型选择 `sloth_rt_print_i64` / `_f64` / `_bool` / `_str`；
`len` 按实参类型选择 `sloth_str_len` / `sloth_arr_len` / `sloth_map_len`；
`int(float)` 发射 `fptosi`（先解码-截断-再编码），`float(int)` 发射 `sitofp`。
