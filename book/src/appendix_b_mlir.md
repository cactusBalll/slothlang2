# 附录 B. MLIR 参考

## B.1 模块结构

每个编译单元发射为一个 `module @<名字> { ... }`，内容按顺序为：

1. `llvm.mlir.global private constant @sl_strN`：字符串字面量的字节缓冲（若有）；
2. **固定运行时前导**：约 60 条 `func.func private @sloth_*` 声明（见 §B.2）；
3. `memref.global @sloth_<模块>_g_<名字> : memref<1xi64> = dense<0> {mutable}`：
   模块级全局变量的 cell；
4. 用户函数 / 类方法 / 泛型实例 / `extern` 声明；
5. `@sloth_<模块>__ginit`：模块初始化（顶层 `var`/`let` 初值写入 cell）；
6. `@sloth_main`：脚本入口，带 `attributes {llvm.emit_c_interface}`。

命名约定：

| 形态 | 例 |
| --- | --- |
| 顶层函数 | `sloth_<模块>__<函数>` |
| 类方法 | `sloth_<模块>_<类>__<方法>` |
| 泛型实例方法 | `sloth_<模块>_<类>_<类型实参>__<方法>`（如 `sloth_main_Box_int__set`） |
| 全局 cell | `sloth_<模块>_g_<名字>` |

## B.2 固定运行时前导

前导由 `rt_decls()` + `obj_rt_decls()` 拼成，按功能分组：

- **ARC**：`sloth_rc_retain` / `sloth_rc_release`（词参返回词）、`sloth_rc_live` /
  `sloth_rc_drops`（零参）；
- **弱引用**：`sloth_weak_new` / `sloth_weak_upgrade` / `sloth_weak_release`；
- **值型 optional 盒**：`sloth_box_new` / `sloth_box_get`，以及 nil-aware 的
  `sloth_rt_print_opt` / `sloth_str_push_opt`；
- **打印**：`sloth_rt_print_i64` / `_f64` / `_bool` / `_str`；
- **字符串**：`sloth_str_intern` / `_push` / `_finish` / `_pushp` / `_push_i|_f|_b` /
  `_len` / `_clen` / `_char` / `_concat` / `_eq`；
- **range**：`sloth_range_pack` / `_lo` / `_hi`；
- **数组**：`sloth_arr_new` / `_new_k` / `_len` / `_get` / `_set` / `_push` / `_pop`；
- **Map**：`sloth_map_new` / `_len` / `_get` / `_set` / `_get_h` / `_set_h` /
  `_str_get` / `_str_set` / `_keys` / `_values`；
- **对象/虚表**：`sloth_obj_new` / `_field` / `_set_field` / `_cls_id` / `_vtable` /
  `_set_vtable`、`sloth_cls_info` / `_refmask`、`sloth_closure_new`、
  `sloth_vt_new` / `_set` / `_get`；
- **panic**：`sloth_panic_noimpl` / `sloth_panic_divzero` / `sloth_panic_unwrap`
  （后者仅在使用 Result 时出现）。

这些符号由 `libsloth_rt.so` 导出，程序在 JIT/链接期解析。

## B.3 完整示例（未剥离前导）

`hello.sl` 的完整模块：

```mlir
{{#include examples/hello_full.mlir}}
```

## B.4 Lowering 管线

`slothc run`（JIT）与 `slothc build`（AOT）都先跑同一组 pass：

```text
canonicalize
cse
convert-func-to-llvm
convert-arith-to-llvm
convert-index-to-llvm
convert-cf-to-llvm
finalize-memref-to-llvm
reconcile-unrealized-casts
```

- `run`：把结果模块交给 MLIR ExecutionEngine，`invokePacked("sloth_main")`。
- `build`：把结果写成 `.mlir`，依次调 `mlir-opt`（同上管线）→ `mlir-translate
  --mlir-to-llvmir` → `clang app.ll libsloth_rt.so -o out`。

由于发射端已把类型信息与所有权插桩全部显式化，管线里**没有**语言特定的 pass
（无消虚 pass、无 sloth 方言转换）；`canonicalize`/`cse` 只做通用化简。
