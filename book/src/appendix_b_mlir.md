# 附录 B. MLIR 参考

## B.1 模块结构

每个编译单元发射为一个 `module @<名字> { ... }`（多模块程序会把所有被导入模块
合并进根 `module`），内容按顺序为：

1. `llvm.mlir.global private constant @sloth_tynm_N("...")`：`any`/`type_name`
   运行时类型描述符用到的名字字符串常数；
2. `memref.global @sloth_anyd_N : memref<11xi64>`：每个被装箱静态类型一份结构化
   `TypeDesc` 全局 cell（由 `@sloth_<模块>__anyinit` 初始化）；
3. **固定运行时前导**：约 200 条 `func.func private @__sloth_*` 声明（见 §B.2）；
4. `memref.global @sloth_<模块>_g_<名字> : memref<1xi64> = dense<0> {mutable}`：
   模块级全局变量的 cell；
5. 注入的 prelude 定义（`print`、`StrChars`，以及自举的 `@__sloth_arr_*` /
   `@__sloth_map_*` / `@__sloth_range_*` / `@__sloth_box_*`）与用户函数 / 类方法 /
   泛型实例 / `extern` 声明；
6. `@sloth_<模块>__anyinit`：TypeDesc 初值；`@sloth_<模块>__ginit`：顶层
   `var`/`let` 初值写入 cell；
7. `@sloth_<模块>`：脚本入口（根模块为 `@sloth_main`），带
   `attributes {llvm.emit_c_interface}`。

命名约定：

| 形态 | 例 |
| --- | --- |
| 顶层函数 | `sloth_<模块>__<函数>` |
| 类方法 | `sloth_<模块>_<类>__<方法>` |
| 泛型实例方法 | `sloth_<模块>_<类>_<类型实参>__<方法>`（如 `sloth_main_Box_int__set`） |
| 全局 cell | `sloth_<模块>_g_<名字>` |
| 运行时/prelude ABI | `__sloth_<域>_<函数>`（保留前缀） |

## B.2 固定运行时前导

前导由 codegen 的 `rt_decls()` / `obj_rt_decls()` 与编译器注入的
`lib/prelude/abi.slt`（`include_str!` 嵌入，唯一声明点）拼成。所有符号带**保留
前缀 `__sloth_`**，按功能分组：

- **ARC**：`__sloth_rc_retain` / `__sloth_rc_release`（词参返回词）、`__sloth_rc_live` /
  `__sloth_rc_drops`（零参）；
- **弱引用**：`__sloth_weak_new` / `__sloth_weak_upgrade` / `__sloth_weak_release`；
- **值型 optional 盒**：`__sloth_box_new` / `__sloth_box_get`；
- **`any` 顶层类型**：`__sloth_any_from` / `_desc` / `_word` / `_kind` / `_cls_id` /
  `_ref` / `_retain` / `_is` / `_type_id` / `_type_name`，以及运行时渲染器
  `__sloth_rt_write`（`any` → `str`）与 `__sloth_rt_puts`（`str` → stdout + 换行）；
- **字符串**：`__sloth_str_intern`（历史命名，不驻留）/ `_push` / `_finish` /
  `_pushp` / `_push_i|_f|_b|_opt` / `_len` / `_clen` / `_char` / `_codepoint` /
  `_concat` / `_eq` / `_cmp` / `_find` / `_slice` / `_byte` / `_of_byte`；
- **裸内存/rc 分配**（自举容器实现的下层接口）：`__sloth_rt_alloc` / `__sloth_free`、
  `__sloth_mem_load` / `_store` / `_copy`、`__sloth_rc_new`（带 `__dispose__` 析构钩子）；
- **对象/虚表**：`__sloth_obj_new`（第三参为类的死亡级联地址）/ `_field` / `_set_field` /
  `_cls_id` / `_vtable` / `_set_vtable` / `_type_name`、`__sloth_cls_info` / `_name`、
  `__sloth_closure_new`、`__sloth_vt_new` / `_set` / `_get`；
- **panic**：`__sloth_panic_noimpl` / `__sloth_panic_divzero` / `__sloth_panic_unwrap`
  / `__sloth_panic_oob` / `_nokey` / `_pop` / `_slice` / `_slice_assign`；
- **I/O / 网络**：`__sloth_bytes_*`、`__sloth_now_ms` / `__sloth_sleep_ms`、`__sloth_ev_*` /
  `__sloth_evbuf_*`、`__sloth_async_*`、`__sloth_net_*` / `__sloth_addr_*`、`__sloth_mmap_*`；
- **协程 / 线程**：`__sloth_fiber_*`（含 ref 局部登记用的 `__sloth_fiber_track` /
  `_untrack`）、`__sloth_thread_*`、`__sloth_chan_*`、`__sloth_mutex_*`、`__sloth_atomic_*`；
- **张量**：`__sloth_tensor_*`。

**自举 prelude**：Array/Map/range/值盒的**实现**（算法、扩容、线性探测、
FNV/mix64 哈希、`keys()`/`values()`、以及强计数归零时的 death cascade）全部写在
`lib/prelude/{containers,core}.slt`，编译期注入根模块一次。因此
`@__sloth_arr_*` / `@__sloth_map_*` / `@__sloth_range_*` / `@__sloth_box_*`
在生成的模块里是被**定义**的函数（不是 `private` 外部声明），调用点也只写这些
裸符号。容器析构经通用可重载的 `__dispose__` 钩子（`__sloth_arr_dispose` /
`__sloth_map_dispose`）注册到 rc 头；`libsloth_rt.so` 只保留裸分配、字级内存与
rc 机制。

除自举 prelude 定义的符号外，其余 `__sloth_*` 声明由 `libsloth_rt.so` 导出，程序在
JIT/链接期解析。

**保留命名空间**：`__sloth_` 前缀只允许编译器注入的 prelude 声明。用户模块声明
`__sloth_*` 会报 `reserved symbol ... may only be declared in the prelude`；要直接
调用这些运行时入口，模块需带伪导入 `import "__sloth";`（否则报
`... may only be called from a module that imports "__sloth"`）。

## B.3 完整示例（未剥离前导）

`hello.sl` 的完整模块：

```mlir
{{#include examples/hello_full.mlir}}
```

## B.4 Lowering 管线

`slothc run`（JIT）与 `slothc build`（AOT）都跑同一组 pass（单一真源为
`crates/sloth-codegen/src/pipeline.rs::pass_names`）：

```text
canonicalize
cse
one-shot-bufferize
linalg-fuse-elementwise-ops
convert-linalg-to-loops
convert-scf-to-cf
convert-math-to-llvm
convert-func-to-llvm
convert-arith-to-llvm
convert-index-to-llvm
convert-cf-to-llvm
finalize-memref-to-llvm
reconcile-unrealized-casts
```

张量段（`one-shot-bufferize` … `convert-math-to-llvm`）位于 func/arith 转换之前，
使 `linalg`/`scf`/`math` 在同一次运行内降到 `llvm`；通道 B 算子保持在 `memref`
域、绕开 bufferization（见设计 §5.6）。AOT 的最终优化由链接期 `clang -O3` 完成。

- `run`：把结果模块交给 MLIR ExecutionEngine，`invokePacked("sloth_main")`。
- `build`：把结果写成 `.mlir`，依次调 `mlir-opt`（同上管线）→ `mlir-translate
  --mlir-to-llvmir` → `clang app.ll libsloth_rt.so -o out`。

`sloth.*` → 标准方言的 lowering **不在这条 pass 管线里**，而是 parse 之后立即执行的
**单点转换**（`crates/sloth-codegen/src/dialect.rs`，`slothLowerModule`），因此进入
管线的 IR 已无 `sloth.*`。除此之外发射端已把类型信息与所有权插桩全部显式化，管线里
**没有**其它语言特定的 pass（无消虚 pass）；`canonicalize`/`cse` 只做通用化简。
