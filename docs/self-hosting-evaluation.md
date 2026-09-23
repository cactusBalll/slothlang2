# slothlang2 运行时自举评估

日期：2026-09-23
范围：Array/Map 自举落地后的运行时收缩，以及其余模块转为 sloth 实现的前景。

## 0. 本次已落地：Array / Map 自举

- 新增裸运行时面（`crates/sloth-rt/src/mem.rs`、`rc.rs`、`panics.rs`）：
  - 未跟踪内存：`sloth_rt_alloc` / `sloth_free` / `sloth_mem_load` / `sloth_mem_store` / `sloth_mem_copy`
  - rc 核心：`sloth_rc_new(nbytes, aux, __dispose__ 地址)` / `sloth_rc_retain` / `sloth_rc_release` / `sloth_weak_*`
  - panic：`sloth_panic_oob` / `sloth_panic_pop` / `sloth_panic_nokey`
- 新增析构魔法机制：强计数归零时 rc 调用注册的 `__dispose__(payload, aux)`（C-ABI）。
  容器在 `sloth_rc_new` 时经 `fn_addr(sloth_arr_dispose / sloth_map_dispose)` 安装。
- `lib/prelude/containers.slt`（约 340 行 sloth）实现全部容器算法：容量增长、
  稳定句柄、线性探测、0.75 负载扩容、mix64/FNV-1a、对象键缓存哈希、
  `keys()`/`values()`、death cascade。编译期注入根模块一次。
- 生成代码的调用点仍是裸符号 `@sloth_arr_*` / `@sloth_map_*`，但符号由注入的
  sloth 定义提供；`libsloth_rt.so` 不再导出它们（Rust 参考实现 `arrays.rs`/`maps.rs`
  已删除，`any` 渲染器与 `tensor.copy_from_array` 改直读文档化布局）。
- 布局冻结不变：数组 `[len,cap,buf]`、Map `[cap,used,kflag,buckets]`、
  slot `[used,key,value,hash]`（`any` 渲染器与 `tensor.copy_from_array` 依赖）。

验证：codegen 208/208、spec 全绿、`examples/{diff,arc,llama,tensor}` 通过
（llama greedy 40 与 run.c 逐字节一致）。

## 1. 评估标准

一个模块可自举，当且仅当它满足：

1. **无原生机制依赖**：不需要原子指令、汇编上下文切换、OS 系统调用、C 库
   算法（dtoa/libm）、或编译器内部 ABI（对象布局、闭包桥）。
2. **可由裸原语表达**：仅用 `alloc/free`、字级 `mem_load/store/copy`、位运算、
   整数环绕算术、`extern` 到薄 C 包装即可。
3. **有收益**：自举能减少 runtime 代码/语义重复，或把「策略」从 Rust 移到 sloth。

## 2. 建议迁移（收益高、阻塞小）

| 模块 | 现状 | 迁移方式 | 阻塞 | 难度 | 优先级 |
|---|---|---|---|---|---|
| **ranges** `pack/lo/hi` | `ranges.rs` 位打包 | 纯位运算，可注入 prelude 或直接内联 | 无 | ★ | 高 |
| **boxopt** | `boxopt.rs` 读写各一 | sloth 函数 + `sloth_rc_new` | 无 | ★ | 高 |
| **bytes** grow/append/copy/fill | 逻辑半在 Rust | 字节级访问可扩为 `mem_load8/store8`，或复用字级（对齐读改写） | 需字节原语 | ★★ | 中 |
| **str 操作族** `concat/eq/cmp/slice/find/starts_with` | `strings.rs` 主体 | 基于 `sloth_str_byte`/`_len` + `sloth_str_intern` + 裸 alloc；`StrT` 布局文档化 | 需 `str_intern` 裸指针构造（已有） | ★★★ | 中 |
| **int/bool → str**、简单 print 装饰 | `console.rs` / `strings.rs` push_i/push_b | 数位循环纯 sloth；**float 格式化留 rt** | 无 | ★★ | 中 |
| **dyn hash/to_str（int/str/bool 路径）** | `builtins.rs` | 逻辑迁 sloth，dispatch 仍由 codegen | 需 dyn 反射面 | ★★★ | 低 |

## 3. 部分可迁（需补接口或性能论证）

| 模块 | 可迁部分 | 必须留 rt 的部分 |
|---|---|---|
| **tensors** | `view`/`reshape`/`shape_eq`/`stride` 的索引算术 | `linalg` 内核、`from_f32_ptr`、memref basis |
| **any 渲染器** | Array/Map/对象遍历（重写为 sloth，`print` 已在 prelude） | dyn 描述符构建、`typeid` 分发 |
| **vtable / cls_name** | 查询薄包装 | 表布局（codegen 契约）、对象头 |
| **weak** | API 面已薄 | CAS/自旋锁、`upgrade` 竞态（必须留 rc） |
| **同步 `mutex.with`** | 回调调用已桥接 | `lock/unlock` 原子 |

## 4. 不建议自举（机制层）

| 模块 | 原因 |
|---|---|
| `rc.rs` / 弱引用 | 原子计数、弱链锁、`relocate`、dtor 分发——裸分配边界本身 |
| `fiber.rs` | 栈分配/上下文切换（汇编/`setcontext`） |
| `thread/channel/sync` | OS 线程与原子指令 |
| `net/event/mmap/fs/time` | 系统调用薄层；**策略层已在 sloth**（`event.slt`/`http.slt`/`net.slt`），符合「机制进 rt、策略用 sloth」 |
| `math.rs` | libm 一行委托；整数算法可另迁 |
| float 打印 / dtoa | 精确格式化依赖 C 生态 |
| `panics` 核心 | 进程终止；消息拼接可在 sloth 后调 `sloth_panic` |
| closure 桥 / 类布局 / monomorph 缓存 | 编译器 ABI 契约 |
| tensor 内核 | 设计定案走 MLIR `linalg` |

## 5. 已自举（锚点）

| 功能 | 位置 |
|---|---|
| Array / Map（本次） | `lib/prelude/containers.slt` |
| `print` 前导 | `module.rs` `IO_PRELUDE` |
| `Result<T,E>` / `Entry<K,V>` | `module.rs` 注入 |
| XorShift RNG | `lib/sloth/random.slt` |
| HTTP 字符串 helpers / 路由 | `lib/sloth/http.slt` |
| 事件 reactor | `lib/sloth/event.slt` |
| 排序 / BPE（示例层） | `examples/**` |

## 6. 建议路线

1. **低垂果实**：ranges + boxopt → 注入 prelude（与容器同机制），几乎零风险。
2. **str 操作族 + bytes**：与容器共享 `mem_*`/`str_*` 原语；先补
   `sloth_mem_load8/store8`（或文档化对齐约束），再迁 `concat/eq/cmp/slice/find`。
3. **tensor 索引算术**：把 shape/stride 检查移入 sloth，内核不动。
4. **类 `__dispose__` 通用化 + 对象级联去运行时布局（已完成）**：用户类可声明
   `func __dispose__()`；codegen 为每个类发射 `@...__cascade` 例程（先跑用户逻辑，
   再逐个 `release` 引用字段），其地址经 `sloth_obj_new(cls_id, n_fields, cascade)`
   登记为 header `sdtor`。运行时**不再有 `ObjInfo.refmask` / `sloth_cls_refmask`**，
   布局全部活在生成代码中。支持继承与泛型实例（见 `spec/126_dispose.sl`）。

## 7. 风险

- **性能**：容器热路径的 `mem_load` 是跨 `.so` 调用，MLIR 管线无 inliner，
  仅靠 JIT -O2 / AOT `clang -O3` 模块内联。`examples/diff` 与 llama 实测无回退；
  若后续出现热点，可 AOT 链静态 `libsloth_rt.a` 或 `-flto`。
- **布局耦合**：任何容器布局调整都会波及 `any.rs` 遍历与 `tensor.copy_from_array`；
  布局已文档化并冻结。
- **诊断措辞**：容器 panic 文案由 prelude 选择（`sloth_panic_oob/pop/nokey`），
  需与 spec 期望保持一致。
