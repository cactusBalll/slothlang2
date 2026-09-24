# slothlang2 运行时自举评估

日期：2026-09-24（初版 2026-09-23；本次修订：档位修正、bytes/str 前提澄清、ranges/boxopt 落地）
范围：Array/Map 自举落地后的运行时收缩，其余模块转为 sloth 实现的前景。

## 0. 已落地

### 0.1 Array / Map（2026-09-23）

- 新增裸运行时面（`crates/sloth-rt/src/mem.rs`、`rc.rs`、`panics.rs`）：
  - 未跟踪内存：`sloth_rt_alloc` / `sloth_free` / `sloth_mem_load` / `sloth_mem_store` / `sloth_mem_copy`
  - rc 核心：`sloth_rc_new(nbytes, aux, __dispose__ 地址)` / `sloth_rc_retain` / `sloth_rc_release` / `sloth_weak_*`
  - panic：`sloth_panic_oob` / `sloth_panic_pop` / `sloth_panic_nokey`
- 新增析构魔法机制：强计数归零时 rc 调用注册的 `__dispose__(payload, aux)`（C-ABI）。
  容器在 `sloth_rc_new` 时经 `fn_addr(sloth_arr_dispose / sloth_map_dispose)` 安装。
- `lib/prelude/containers.slt`（约 460 行 sloth）实现全部容器算法：容量增长、
  稳定句柄、线性探测、0.75 负载扩容、mix64/FNV-1a、对象键缓存哈希、
  `keys()`/`values()`、death cascade。编译期注入根模块一次。
- 生成代码的调用点仍是裸符号 `@sloth_arr_*` / `@sloth_map_*`，但符号由注入的
  sloth 定义提供；`libsloth_rt.so` 不再导出它们（Rust 参考实现 `arrays.rs`/`maps.rs`
  已删除，`any` 渲染器与 `tensor.copy_from_array` 改直读文档化布局）。
- 布局冻结不变：数组 `[len,cap,buf]`、Map `[cap,used,kflag,buckets]`、
  slot `[used,key,value,hash]`（`any` 渲染器与 `tensor.copy_from_array` 依赖）。

### 0.2 ranges + boxopt（2026-09-24，本次）

- 新增 `lib/prelude/core.slt`：`sloth_range_pack/lo/hi`（rc 双词盒 `{lo,hi}`）与
  `sloth_box_new/get`（rc 单词 payload 盒）全部改为 sloth 实现，仅依赖
  `sloth_rc_new` + `sloth_mem_load/store`（与容器 prelude 同一套裸原语）。
- codegen：`rt_decls()` 移除这 5 个符号的 `func.func private` 声明；
  prelude 符号登记 `CORE_SYMS` → `fixed_syms`（与 `CONTAINER_SYMS` 同机制）。
- Rust 侧收缩：`ranges.rs` 删除（`any` 渲染器改直读 `{lo,hi}` 文档化布局）；
  `boxopt.rs` 去掉 `#[no_mangle]`，只剩 Rust 内部构造/读取（`fiber.rs`/`channel.rs`
  从 Rust 侧造盒、`any.rs`/`console.rs`/`strings.rs` 直读 payload）——
  这是「构造类原语有 Rust 内部调用者时，只迁 C-ABI 面、布局文档化冻结」的先例。
- 布局冻结新增两契约：range 盒 `[lo,hi]`、box 盒 `[payload]`
  （`any` 渲染器、`sloth_rt_print_opt`、`sloth_str_push_opt` 依赖）。

验证：codegen 212/212、frontend 20、rt 6（含新增 `core_symbols_not_exported`
契约测试）、spec 全绿（≥200 用例）、`examples/{diff,arc,llama,tensor}` 通过
（llama greedy 40 与 run.c 逐字节一致；arc 9/9 压力场景 RC 归零基线）。
`nm -D libsloth_rt.so` 确认 5 个符号不再导出。

## 1. 评估标准

一个模块可自举，当且仅当它满足：

1. **无原生机制依赖**：不需要原子指令、汇编上下文切换、OS 系统调用、C 库
   算法（dtoa/libm）、或编译器内部 ABI（对象布局、闭包桥）。
2. **可由裸原语表达**：仅用 `alloc/free`、字级 `mem_load/store/copy`、位运算、
   整数环绕算术、`extern` 到薄 C 包装即可。
3. **有收益**：自举能减少 runtime 代码/语义重复，或把「策略」从 Rust 移到 sloth。
4. **无 Rust 内部调用者**（新增，或按 0.2 的先例降级处理）：若 Rust 运行时内部
   直调该逻辑（net↔bytes、fiber/channel↔box、any↔builder），则只能迁「C-ABI 面 +
   算法」，构造原语与布局读取留在 rt 并文档化冻结——否则会出现双实现漂移。

## 2. 建议迁移（收益高、阻塞小）

初版按「原语缺口」分档；本次修订把**实际零原语依赖**的模块提到第一档，
并修正 bytes/str 两处误判。

### 2a. 第一档：零新原语（纯 `rc_new` + word 读写，可立即迁）

| 模块 | 现状 | 迁移方式 | 注意 | 优先级 |
|---|---|---|---|---|
| ~~ranges~~ | 已落地（§0.2） | — | — | — |
| ~~boxopt~~ | 已落地（§0.2） | — | — | — |
| **vtable** `vt_new/set/get`、`obj_set_vtable/vtable` | `vtable.rs` 边界检查 word 读写 | `sloth_rt_alloc` + `mem_load/store`，逐行同构 | 表布局是 codegen 契约，冻结 | 高 |
| **objects** `obj_new/field/set_field`、`cls_info/cls_name`、`type_name` | `objects.rs` | `sloth_rc_new((n+2)*8, n, fn_addr(cascade))` + word 偏移 2 起读写；`type_name` 用已暴露的 `sloth_str_intern` | 迁后 `builtins.rs` 的 `payload()` 改 Rust 直读布局（同 `any.rs` 读容器） | 高 |
| **any** `desc_eq`/`is`/`type_id`/`type_name`（非渲染部分） | `any.rs` 描述符递归 | 描述符是 `i64[11]` 平表，word 读 + 递归；`type_name` 经 `sloth_str_intern` | 与渲染器解耦，可先行 | 中 |

### 2b. 第二档：补 1 个一行原语后可迁

| 模块 | 现状 | 迁移方式 | 缺口（修正） | 优先级 |
|---|---|---|---|---|
| **str 操作族** `concat/eq/cmp/slice/find/starts_with/of_byte` | `strings.rs` 主体 | 新增 `sloth_str_data(s): int`（返回 `StrT.data` 裸指针，一行）后：词级 `mem_copy` 批量搬移 + `sloth_str_intern` 构造 | ~~逐字节原语~~（误判）：逐字节 `sloth_str_byte` 跨 `.so` 循环在 32k 词表 tokenizer 上不可接受；**必须词级批量**，故 `str_data` 是硬前提。**验收：llama tokenizer 吞吐无回退** | 中 |
| **int/bool → str**、简单 print 装饰 | `console.rs` / `strings.rs` push_i/push_b | 数位循环纯 sloth + 词级打包 | 无；**float 格式化留 rt**（dtoa） | 中 |
| **tensor 索引算术** `new/view/reshape/shape_eq/dim_eq/stride/dim` | `tensors.rs` 前半 | 7-word 描述符 word 读写；`tensor_dtor` 改 sloth `__dispose__`（`sloth_rc_new(56,0,fn_addr(...))`，容器已验证） | ① panic 消息需一个 C-ABI `sloth_panic_str(s: str)` 薄入口；② **`get1/set1` 的 float 分支需 `f64↔bits` 原语**（sloth 源语言无 bitcast），int 分支可迁、float 分支留 rt 或补原语 | 中 |

### 2c. 需补字节粒度纪律（本次修正：**不缺原语**）

| 模块 | 现状 | 迁移方式 | 真实阻塞 | 优先级 |
|---|---|---|---|---|
| **bytes** grow/append/get/set/fill | `bytes.rs` 逻辑半在 Rust | **word 读改写（RMW）即可**：`new` 把 cap 抹齐 8 的倍数后所有访问都落在整词内——不需要 `mem_load8/store8`（初版误判）；growth 用 `sloth_rt_alloc`+`mem_copy`+`sloth_free`（或薄 `sloth_realloc` 包装） | ① `net.rs` 是 Rust 内部调用者（`pub(crate)` 直调 `ensure/set_len/len`，且直接摸 `BytesObj.data`）——迁后 Rust/net 侧需重复布局知识或内联；② `copy_from_str/to_str` 依赖 2b 的 `str_data` | 低（先解决 net 耦合） |

## 3. 部分可迁（需补接口或性能论证）

| 模块 | 可迁部分 | 必须留 rt 的部分 |
|---|---|---|
| **any 渲染器** | Array/Map/range/scalar 分支遍历拼接（布局已冻结，sloth 侧本来就有权访问） | `disp` 函数指针调用（`transmute` 后按 `extern "C" fn(i64)->i64` 调用——sloth 无裸地址间接调用）、float 渲染（dtoa）、`StrB` builder（Rust 内部调用者多，见 §1.4） |
| **dyn 内建**（`builtins.rs`） | `dyn_hash` 的 int 路径与 float bits 路径（bits 作为 word 读出后纯整数 mix64）、`dyn_to_str`/`dyn_binop` 的 int/bool 分发 | float 比较的 `as_f64` 解码需 `f64↔bits`；float 渲染留 rt |
| **同步 `mutex.with`** | 回调桥接薄包装 | `lock/unlock` 原子 |
| **weak** | API 面已薄 | CAS/自旋锁、`upgrade` 竞态（必须留 rc） |

## 4. 不建议自举（机制层）

| 模块 | 原因 |
|---|---|
| `rc.rs` / 弱引用 | 原子计数、弱链锁、`relocate`、dtor 分发——裸分配边界本身 |
| `fiber.rs` | 栈分配/上下文切换（汇编/`setcontext`） |
| `thread/channel/sync` | OS 线程与原子指令 |
| `net/event/mmap/fs/time` | 系统调用薄层；**策略层已在 sloth**（`event.slt`/`http.slt`/`net.slt`），符合「机制进 rt、策略用 sloth」 |
| `math.rs` | libm 一行委托；整数算法可另迁 |
| float 打印 / dtoa | 精确格式化依赖 C 生态 |
| `panics` 核心 | 进程终止；消息拼接可在 sloth 后调 `sloth_panic_str`（新增薄入口，见 §2b） |
| `str_intern` / `StrB` builder / `console` 构造面 | **Rust 内部调用者**（`any.rs` 渲染、`objects.rs`/`mmap.rs` 构造、`builtins.rs` push_f）——迁 C-ABI 面会造成双实现；算法层（§2b）可迁 |
| closure 桥 / 类布局 / monomorph 缓存 | 编译器 ABI 契约 |
| tensor 内核 / `basis_*` / `from_f32_ptr` | 设计定案走 MLIR `linalg`；memref 描述符按值返回是编译器 ABI |

## 5. 已自举（锚点）

| 功能 | 位置 |
|---|---|
| Array / Map（2026-09-23） | `lib/prelude/containers.slt` |
| range / 值盒 boxopt（2026-09-24） | `lib/prelude/core.slt` |
| `print` 前导 | `module.rs` `IO_PRELUDE` |
| `Result<T,E>` / `Entry<K,V>` | `module.rs` 注入 |
| XorShift RNG | `lib/sloth/random.slt` |
| HTTP 字符串 helpers / 路由 | `lib/sloth/http.slt` |
| 事件 reactor | `lib/sloth/event.slt` |
| 排序 / BPE（示例层） | `examples/**` |

## 6. 建议路线

1. ~~**低垂果实**：ranges + boxopt → 注入 prelude~~（本次已完成，§0.2）。
2. **第一档收尾**：vtable + objects + `any_is/type_id` → 并入 `core.slt`
   （零新原语，与容器/range 同机制；`builtins.payload` 改直读布局）。
3. **str 操作族 + int/bool→str**：先补 `sloth_str_data`（一行），
   迁 `concat/eq/cmp/slice/find`；**以 llama tokenizer 吞吐无回退验收**。
4. **tensor 索引算术**：shape/stride 检查移入 sloth、`tensor_dtor` 改
   sloth `__dispose__`；`get1/set1` float 分支与 `basis`/内核留 rt
   （或补 `f64↔bits` 原语后一并迁）。
5. **类 `__dispose__` 通用化 + 对象级联去运行时布局（已完成）**：用户类可声明
   `func __dispose__()`；codegen 为每个类发射 `@...__cascade` 例程（先跑用户逻辑，
   再逐个 `release` 引用字段），其地址经 `sloth_obj_new(cls_id, n_fields, cascade)`
   登记为 header `sdtor`。运行时**不再有 `ObjInfo.refmask` / `sloth_cls_refmask`**，
   布局全部活在生成代码中。支持继承与泛型实例（见 `spec/126_dispose.sl`）。
6. **bytes / any 渲染器 / dyn**（低优先）：分别等待 net 耦合决策、
   `sloth_call1` 裸地址调用原语、`f64↔bits` 原语。

## 7. 风险

- **性能**：容器热路径的 `mem_load` 是跨 `.so` 调用，MLIR 管线无 inliner，
  仅靠 JIT -O2 / AOT `clang -O3` 模块内联。`examples/diff` 与 llama 实测无回退；
  若后续出现热点，可 AOT 链静态 `libsloth_rt.a` 或 `-flto`。
  **str 操作族是下一个性能敏感点**（tokenizer 逐词 cmp），迁移必须走
  `str_data` + 词级批量，逐字节循环不可接受（§2b）。
- **布局耦合**：容器布局调整波及 `any.rs` 遍历与 `tensor.copy_from_array`；
  本次新增 range/box 两契约（`any` 渲染、`*_print_opt`/`*_push_opt` 依赖）。
  布局已文档化并冻结。
- **双实现漂移**：凡「C-ABI 面迁 sloth、Rust 内部调用者留 rt」的模块
  （0.2 的 boxopt 是首例），布局是唯一契约——改动必须两侧同步并更新本表。
- **诊断措辞**：容器 panic 文案由 prelude 选择（`sloth_panic_oob/pop/nokey`），
  需与 spec 期望保持一致；tensor 迁移同理（需 `sloth_panic_str` 薄入口）。
