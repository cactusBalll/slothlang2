# sloth2 张量扩展：与当前实现对**齐**的实现方案

> **去 tag 迁移（2026-09-19，PLAN §14）**：词面已由 LSB tag 编码改为**无 tag
> 单个 i64**（引用=裸指针、int=原生 i64、float=原生 f64 位模式、bool=0/1、
> nil=0）。本文中凡涉「tagged rc 词 / f63 / f62 / `shrsi 1` 解码 / `enc_i`」
> 的表述均为迁移前记录；现行实现见主设计文档 §2.6/§5.1 与 `PLAN` §14。

**基准文档**：`sloth-lang-2.0张量扩展设计文档.md`（v1.0 草案，2026-09-14）
**对齐目标**：在不改变文档验收目标（单机 fp32 跑通 llama2.c `run.c` 推理）的前提下，把文档中**与当前实现冲突**的机制（自定义 dialect、GC、`.slt` 假设）替换为**当前代码库真实可行**的机制。
**已定档**：① 走**真 MLIR `linalg` 通道**（非 rt 内核回退）；② 交付**完整路线图** TE-P0…；③ 采用**真 `.slt` 文件 + 标准库搜索路径**。

---

## 0. 结论摘要

当前编译器与文档的假设有 **3 处根本错位**，方案的全部设计都围绕消解它们：

1. **无自定义 dialect，且无 `tensor`/`linalg`/`scf`/`math` 发射**。文档 §5.1 的 `!sloth.tensor` / `sloth.tensor_alloc / …` 不存在，且项目已**定案放弃** sloth dialect（`book/src/appendix_a_deviations.md` A.1）。→ 张量 lowering **只用标准 dialect**：`tensor` / `linalg` / `memref` / `scf` / `arith` / `math` / `func` / `cf`。
2. **无 GC**。实现是 **ARC + 带内隐藏头**（`crates/sloth-rt/src/rc.rs`，`Hdr` = 6 词 / 48 字节）。文档 §3.1/§5.3.2 的「GC 堆指针 / 数据区免扫描 / mmap 外部根」全部作废——但**因无扫描器，文档想要的「数据区不参与扫描」自动成立**（同 `arrays.rs` 的 `calloc` 元素缓冲）。
3. **值模型是「一切皆为 tagged `i64` 词」**（`util.rs::memref_cell_ty` 恒 `memref<1xi64>`，`mlir_ret_ty` 恒 `i64`）。所有内建/引用/容器都以 tag 位在运行时区分，**没有 MLIR 类型化槽**。→ Tensor 的 ABI 边界必须沿用「一个 tagged rc 词」，把 `linalg` 计算**限制在表达式内部**，边界处物化回词（详见 D1/D2）。

配套：文档 §4.4 的 `.slt`、§5.2 的双通道、§3.2/§3.3 的索引视图/复合赋值/位运算**方向正确**，但落点需改写（D5/D6、TE-P0）。

---

## 1. 对齐差异表（假设 → 实现 → 定案）

| 文档位置 | 文档假设 | 当前实现 | 本方案定案 |
| --- | --- | --- | --- |
| §3.1 内存表示 | GC 堆指针；对象头含 shape/strides/数据指针/所有者；数据区免扫描 | ARC：`rc_addr` 带内头；`Array` = 固定 3 词 rc 头 `[len,cap,buf]` + **非追踪** `calloc` 元素缓冲 | Tensor = **同构的 rc 词**：固定头 `[flags, ndim, shape_ptr, stride_ptr, data_ptr, owner, data_len]`（owner=视图父句柄，保活；0=自持） |
| §3.1 元素精度 | `Tensor<float,R>`=f64，逐点用 float | `float` 实为 f63，**算术路径** f62（appendix A.2.1） | 首版元素 `float`；文档 §4.4「零拷贝 `view_as_f32`」**不成立**：checkpoint 是 f32，须**加宽转换**（D7） |
| §3.1 泛型 rank | rank 作整数常量类型参数 | `Ty` 无整型常量参数；单态化帧是 `HashMap<String,TyId>` | 新增 `Ty::Tensor(TyId, u32)`，**独立通道**，绝不进 `tp_subst`（文档 §8.3 风险 4 的原意） |
| §3.4/§5.1 dialect | 自定义 `sloth.tensor_*` / `!sloth.tensor` | 无自定义 dialect（定案 A.1），仅标准 dialect 文本发射 | 只用标准 dialect；算子语义由 IR 文本 + rt 助手表达（D8） |
| §5.2 双通道 | tensor 值语义→linalg→one-shot bufferize；可变→memref 直降 | 无 linalg/bufferize；无 `scf`；`memref`→llvm 已通 | 保留双通道（D2），但**边界物化点 = 语句/表达式**，避免全局逃逸分析 |
| §5.1 GC 风险 | `GC_malloc_atomic` + mmap 外部根 | 无 GC；泄漏即泄漏、不崩溃 | 删除该风险；改为「描述符 rc 头 + data 缓冲双区，owner 保活」 |
| §2 数学 | MLIR `math` dialect intrinsic | 仅 `sloth_extern_floor/powf`（`externs.rs`） | 发射 `math.*` + 接 `convert-math-to-llvm`；另加标量 libm rt 面（D6） |
| §6/§4.2 stdlib | `import "sloth/tensor.slt"` 等 | **无 `.slt` 文件、无搜索路径**；stdlib=编译器注入源码（`Result`/`Entry`） | 新增 `lib/sloth/*.slt` + `resolve_program` 搜索路径（D5） |
| §4.4 checkpoint | `sloth_mmap` extern + 零拷贝 f32 视图 | `extern func`/`extern type` 机制齐备；无 mmap | 新增 `sloth-rt/src/mmap.rs`（D7） |
| §3.3 复合赋值/位运算 | 新增 `+= -=` 与 `& | ^ << >> ~` | **均不存在**；`& ^ ~` 当前直接词法报错 | TE-P0 前置补齐（含 `>>` 与泛型收尾的歧义处理） |

---

## 2. 架构决策

### D1. Tensor 的 ABI = 一个 tagged rc 词（与 `Array` 同构）
- `Tensor<T,R>` 在槽/形参/返回/字段里**仍是 1 词**（`is_ref` = true）。`util.rs::mlir_ret_ty` / `memref_cell_ty` **不改**。这是把张量塞进现有一趟发射器、零 ABI 地震的关键。
- rc 负载（rank R 静态，布局固定）：
  `[flags(元素种类/是否视图), ndim, shape_ptr, stride_ptr, data_ptr, owner, data_len]`
  - `shape_ptr`/`stride_ptr`：非追踪 `i64[R]` 数组；
  - `data_ptr`：非追踪对齐缓冲（`aligned_alloc`/`calloc`）**或 mmap 区间**；
  - `owner`：视图张量持有父句柄（`retain`）→ 存储保活；`0` 表示自持 data 缓冲。
- dtor：释放 shape/stride 数组 → `owner!=0` 则 `release(owner)`，否则 `free(data_ptr)`。抄 `arrays.rs::arr_dtor` 的写法。
- 视图写即原张量写：`data_ptr` 共享，`t[i]` / `t[a..b]` 只改 `data_ptr/offset` 与 shape/stride。

### D2. 双通道，但边界 = 语句/表达式（免全局逃逸分析）
- **通道 A（函数式中间量）**：张量表达式内部走 `tensor`/`linalg` 值语义（`tensor.empty` / `linalg.generic` / `linalg.reduce` / `linalg.matmul` / `linalg.dot`），经 `one-shot-bufferize` 落到 `memref`。融合（rmsnorm/softmax/SwiGLU/RoPE）在**同一表达式**内由 `linalg-fuse-elementwise-ops` 完成。
- **通道 B（可变/视图/权重/KV cache/accumulator）**：描述符词 → `memref`（`sloth_tensor_basis` 取 rank-1 基 + `memref.reinterpret_cast` 用运行期 dim/stride 转成 `memref<?x…xf64>`）→ `memref.subview` / `memref.store`，直降 `memref→llvm`，**不做 tensor 值语义、不引入拷贝**。
- **物化规则**：张量表达式的结果一旦要**绑定/赋值/返回/传参/存字段**，就在该语句边界物化为描述符词（通道 A 结果 → `memref.extract_aligned_pointer_as_index` + `sloth_tensor_adopt(ptr, dims…)`）。这替代文档 §5.3.1 的「类型检查期逃逸标记」，在一趟发射器内**局部可判**。
- 这保留了文档「融合的收益」与「KV cache 零拷贝」两个核心诉求。

### D3. 类型系统
- `ty.rs`：`Ty::Tensor(TyId /*elem*/, u32 /*rank*/)`；补 `fmt_ty`（`tensor:{elem}:{rank}`）、`ty_name`（`tensor`）、`subst`（递归 elem，rank 不变）。
- `ast.rs`：`SimpleType::Tensor(Box<Type>, u32)`。
- `parser.rs::ty_base`：新增 `"Tensor"` 臂，`'<' type ',' INT '>'`（第二个参数直接读 `Tok::Int`，**不走 `self.ty()`**）。
- `tybind.rs`：`ty_of_simple`/`ty_named` 臂；`is_ref(Tensor)=true`；`surface_compat` 要求 **elem 与 rank 完全一致**；elem 仅允许 `float`（首版）/`int`，否则诊断。**R 不进 `tp_subst`/mangle 帧**（按文档 §8.3 风险 4 的「单独通道」执行）。
- 可选：新增预定义 bound `Numeric`（`is_predef_trait`/`satisfies_bound`）接受 `I64|F64`，供 `Tensor<T,R>` 元素约束；或首版直接在 `ty_of` 里硬校验。

### D4. 语法前置（TE-P0）
- `+=` / `-=`：lexer 加 `PlusEq`/`MinusEq`；`ast.rs` 加 `StmtNode::AssignOp { target, op, value }`；parser `stmt` 分支。
  codegen：目标表面为 **Tensor** 时走**原地**通道（`add_into` / `sub_into`，无中间分配）；否则降级为 `target = target OP value`（**目标只求值一次**，复用既有 `eval_assign_container`）。
- 位运算（仅 `int`）：lexer 加 `Amp, Caret, Tilde, Shl, Shr`；`ast.rs` 扩展运算符枚举；parser 优先级按 C：`| < ^ < & < shift < +/-`（新增 `P_BOR/P_BXOR/P_BAND/P_SHIFT`，插在 `P_CMP` 与 `P_ADD` 之间）；`unary()` 加 `~`。
  codegen：`dec_i → arith.andi/ori/xori/shli/shrsi → enc_i`；`~x` = `xori(dec, -1)`；float 操作数**报类型错**。
- **`>>` 与泛型收尾冲突（必须处理）**：当前 `Map<int,Array<int>>` 依赖 `Gt Gt`。加贪婪 `Shr` 后，`ty_base` 在期望 `Gt` 时须把 `Shr` **拆成两个 `>`**（消费 `Shr` 并置「待补一个 `Gt`」标志）。`Shl` 无此对手，可直接贪婪。

### D5. stdlib `.slt` + 搜索路径
- 新增仓库级 `lib/sloth/`：`tensor.slt`、`random.slt`、`fs.slt`、`llama.slt`。
- `resolve_program`（`irgen/mod.rs`）解析顺序：**导入文件所在目录 → `$SLOTH_STDLIB` → `<exe>/../lib` → `<CARGO_MANIFEST_DIR>/../../lib`**，使 `import "sloth/tensor.slt"` 命中 `lib/sloth/tensor.slt`。`slothc/src/main.rs` 的 `imports` 判定与 `base` 计算同步。
- **分工**：需要 `linalg` 发射的核（`matvec/matmul/rmsnorm/softmax/dot/argmax/silu/rope`）是**编译器内建**（`emit_call` 识别 → 张量 lowering），`tensor.slt` 提供泛型包装与便利函数；**纯 sloth** 件（xorshift、`sort_desc_index`、`cumsum`、tokenizer 的 `Map`/排序逻辑）写在 `.slt` 里。与 `Result`/`Entry` 的「注入源码调用内建」模式一致。

### D6. 数学
- 张量核内发射 `math.exp/sqrt/sin/cos/pow`（`math` dialect 已注册加载），并接 `convert-math-to-llvm` 到两条管线。
- 另加 `sloth-rt/src/math.rs` 标量 C-ABI：`sloth_rt_sqrt/exp/sin/cos/pow/tan`，供 sloth 源码里的 `float_sqrt` 等标量调用（文档 §6 用到）。
- **修正文档 §5.4 的精度声明**：f63/f62 下逐算子误差远大于 fp32 与 IEEE 之差，§8.1「相对误差 < 1e-4」仍可达（fp32 24 位尾数，f62 下加宽后误差 ~2^-62），但措辞须改。

### D7. mmap / checkpoint（含对文档 §4.4 的硬修正）
- 新增 `sloth-rt/src/mmap.rs`：`sloth_mmap(path) -> ptr 词`、`sloth_mmap_len`。
- `extern type ByteBuffer;` + `extern func` 暴露（机制已具备，`externs.rs` 有样例）。
- **修正**：`view_as_f32(offset,len): Tensor<float,1>` 的「零拷贝」**不成立**——文件里是 f32，张量元素是 f64。首版走 **`sloth_tensor_from_f32_ptr(ptr, len)`（rt 内 f32→f64 加宽拷贝，一次）**；2× 内存代价。若日后要真零拷贝，需引入独立 `Tensor<f32,R>` 元素类型（列入后续，非本路线图）。
- `read_config`（7 个 i32 + `vocab_size` 负值约定）写 `fs.slt`（可调 mmap + 指针助手）。

### D8. 无自定义 dialect 的算子表达
- 不写 ODS/C++ dialect。算子 = **标准 dialect IR 文本**（`linalg.*` + `arith`/`math`/`memref`/`scf`）+ **rt 助手**（`sloth_tensor_*`）的组合。
- 新增 `irgen/tensor.rs` 承载张量 lowering，避免 `expr.rs` 膨胀。

---

## 3. 分阶段路线图

> 每阶段：`cargo test` 全绿（codegen 内联 + `slothc/tests/spec` + `rt_smoke`）+ `cargo fmt --check` 干净，方可提交。

### TE-P0 语法前置（主文档层）
| 项 | 内容 |
| --- | --- |
| 文件 | `lexer.rs`、`ast.rs`、`parser.rs`、`expr.rs`、`stmt.rs` |
| 交付 | `+=`/`-=`；`& | ^ << >>` + `~`；`>>` 泛型拆分 |
| 验收 | spec 新域（位运算表、复合赋值、`Map<int,Array<int>>` 回归）+ irgen 用例 |

### TE-P1 Tensor 类型 + 运行时描述符
| 项 | 内容 |
| --- | --- |
| 前端 | `ty.rs`（`Ty::Tensor`）、`ast.rs`、`parser.rs::ty_base`、`tybind.rs` |
| rt | 新增 `crates/sloth-rt/src/tensors.rs`：`sloth_tensor_new_1/2/3`、`zeros`、`from_array`、`dim/stride`、`view(t, off, len)`、`copy_into`、`tensor_dtor`；`lib.rs` 挂模块 |
| codegen | 索引/切片视图（`t[i]` 降 rank、`t[a..b]`/`t[a..=b]` 保 rank）、视图赋值 = copy_into、`rt_decls()` 加 `sloth_tensor_*` |
| 验收 | 创建/视图读写/越界 panic；spec `tensor_*` 域 + irgen 用例 |

### TE-P2 MLIR `linalg` 通道
| 项 | 内容 |
| --- | --- |
| codegen | 新增 `irgen/tensor.rs`：`sloth_tensor_basis` + `memref.reinterpret_cast`（通道 B）；`linalg.generic`/`reduce`/`dot`/`matmul` + `bufferization.alloc_tensor`（通道 A）；`math.*` |
| 管线 | `jit.rs` + `slothc::build_mode_r` 接入 `one-shot-bufferize`、`linalg-fuse-elementwise-ops`、`convert-linalg-to-loops`、`convert-scf-to-cf`、`convert-math-to-llvm`（抽共享 `pass_args()` 防两处漂移） |
| 算子 | `matvec`/`matmul`/`add/sub/mul/div`/`dot`/`reduce` 族 |
| 验收 | matvec benchmark ≥ `run.c -O3` 的 0.7×（文档 §8.2） |

### TE-P3 融合 + stdlib + checkpoint
| 项 | 内容 |
| --- | --- |
| 融合 | rmsnorm / softmax / SwiGLU / RoPE 标为融合区（`linalg.generic` + `linalg.index`），加速比 ≥1.3× |
| stdlib | `lib/sloth/{tensor,random,fs}.slt` + `resolve_program` 搜索路径 |
| IO | `sloth-rt/src/mmap.rs`、`math.rs`；`fs` 加载 + `read_config` + f32→f64 加宽 |
| 验收 | stories260K 加载成功；融合加速比达标；`.slt` 导入端到端跑通 |

### TE-P4 llama.slt + 采样器 + tokenizer
| 项 | 内容 |
| --- | --- |
| 交付 | `lib/sloth/llama.slt`：`forward`（§6 参考实现落码）、4 种采样（`argmax`/temp+softmax/multinomial/top-p）、BPE tokenizer |
| 验收 | 文档 §8.1：同 seed greedy 200 步 token 序列一致；stories15M 生成连贯文本；chat BOS/EOS 状态机一致 |

### TE-P5 验收与性能
| 项 | 内容 |
| --- | --- |
| 差分 | 复刻 llama2.c `test_all.py`：`examples/llama/` + `run.sh` 对比 `run.c`（stories260K/15M/42M/110M） |
| 性能 | `matvec` 单独 benchmark + 融合开关对照；`--target-cpu=native` 编译开关 |

---

## 4. 逐文件改动清单

**前端 `crates/sloth-frontend/src/`**
- `lexer.rs`：`Tok` 增 `PlusEq,MinusEq,Amp,Caret,Tilde,Shl,Shr`；`punct()` 补配对与单字（`&`/`^`/`~` 当前落 `unexpected character`）。
- `ast.rs`：`SimpleType::Tensor(Box<Type>,u32)`；位运算运算符载体；`StmtNode::AssignOp`。
- `parser.rs`：`ty_base` Tensor 臂 + `Shr→Gt Gt` 拆分；位运算优先级常量 + Pratt 段；`unary()` `~`；`stmt` 复合赋值分支。
- `ty.rs`：`Ty::Tensor`、`fmt_ty`、`ty_name`、`subst`。

**后端 `crates/sloth-codegen/src/`**
- `irgen/tybind.rs`：Tensor 解析、`is_ref`、`surface_compat`、elem/rank 校验、可选 `Numeric` bound。
- `irgen/tensor.rs`（**新增**）：通道 A/B lowering、视图、内建张量算子发射。
- `irgen/expr.rs`：张量表达式入口、索引/范围视图、内建算子识别、位运算 lowering。
- `irgen/stmt.rs`：`AssignOp`（张量原地）、视图赋值/copy_into、let 张量。
- `irgen/util.rs`：`emit_tensor_basis`、运行期 dim/stride 解码（`memref_cell_ty`/`mlir_ret_ty` 不变）。
- `irgen/module.rs`：`rt_decls()` 增 `sloth_tensor_*`；`resolve_program` 加 stdlib 搜索路径。
- `irgen/state.rs`：如需张量算子缓存/内建表。
- `jit.rs`：`run_llvm_pipeline` 接 linalg/scf/math/bufferize；与 `slothc` 共享 pass 列表。
- `pass.rs`：`irgen_*` 测试。

**运行时 `crates/sloth-rt/src/`**
- `tensors.rs`（**新增**）：描述符 + 数据缓冲 + 视图 + dtor + `adopt`/`basis`/`from_f32_ptr` 等。
- `mmap.rs`（**新增**）：`sloth_mmap`/`sloth_mmap_len`。
- `math.rs`（**新增**）：`sloth_rt_sqrt/exp/sin/cos/pow/tan`。
- `lib.rs`：挂载新模块。
- `externs.rs`：保留样例；mmap/math 走正式模块。

**CLI / 标准库 / 用例**
- `crates/slothc/src/main.rs`：`build_mode_r` pass 列表同步；stdlib 根接线。
- `lib/sloth/{tensor,random,fs,llama}.slt`（**新增**）。
- `crates/slothc/tests/spec/`：新增 tensor/位运算/复合赋值域。
- `examples/tensor/`（matvec/rmsnorm/softmax + benchmark）、`examples/llama/`（+ `run.c` 差分）。

---

## 5. Pass pipeline 变更（两处必须一致）

当前（`jit.rs::run_llvm_pipeline` 与 `slothc::build_mode_r`）：
`canonicalize, cse, convert-func-to-llvm, convert-arith-to-llvm, convert-index-to-llvm, convert-cf-to-llvm, finalize-memref-to-llvm, reconcile-unrealized-casts`

目标顺序（在 `convert-arith-to-llvm` **之前**插入张量段）：
`canonicalize, cse,` **`one-shot-bufferize, linalg-fuse-elementwise-ops, convert-linalg-to-loops, convert-scf-to-cf, convert-math-to-llvm,`** `convert-func-to-llvm, convert-arith-to-llvm, convert-index-to-llvm, convert-cf-to-llvm, finalize-memref-to-llvm, reconcile-unrealized-casts`

**建议**：抽出共享 `fn pass_names() -> &'static [&'static str]`（或 `sloth-codegen` 导出的 pass-args 常量），JIT 与 AOT 各按需转成 C-API pass / CLI 参数，**消除现有的两处硬编码漂移**（本身就是潜在 bug 源）。

---

## 6. 风险

| # | 风险 | 等级 | 对策 |
| --- | --- | --- | --- |
| R1 | **指针→memref 的 ABI**：rt 助手返回 `memref<?xf64>` 需匹配 MLIR 的 memref 描述符 ABI（`{alloc,aligned,offset,size}`，sret） | 高 | 首选 `sloth_tensor_basis(t) -> memref<?xf64>` + `memref.reinterpret_cast`；**先用最小原型验证 sret 约定**；失败则回落为「rt 计算内核」并保留 linalg 接口（文档 §8.3 风险 5 已授权此回退） |
| R2 | `one-shot-bufferize` 引入隐形拷贝，性能崩塌 | 高 | 双通道（D2）；**通道 B 完全绕开 bufferization**；KV cache 路径专项回归 |
| R3 | `>>` 词法贪婪破坏 `Map<int,Array<int>>` 等嵌套泛型 | 中 | `ty_base` 把 `Shr` 拆为两个 `Gt`（D4）；补嵌套泛型回归用例 |
| R4 | ARC 视图生命周期：视图持有 `owner` 但漏插 retain/release → 悬垂或泄漏 | 中 | 视图构造走 `is_ref` 通道（沿用 patch B 四类插入点）；`examples/arc/` 加视图场景 |
| R5 | f32→f64 加宽使权重内存翻倍（7B 级不可行） | 中 | 首版接受；后续引入 `Tensor<f32,R>` 元素类型（文档 §3.1 已列后续项） |
| R6 | f63/f62 精度 vs fp32 采样分叉 | 低 | greedy 路径确定性对齐；采样路径按同 seed 对齐（文档 §8.3 风险 3） |
| R7 | `linalg` 向量化质量不及手写 `-O3` | 中 | `--target-cpu=native`；必要时 matvec 单算子回落手工内核 |

---

## 7. 验收与测试

- **单测**：`cargo test`（codegen `irgen_*` 内联、`slothc/tests/spec/*.sl` + `spec_suite.rs`、`sloth-rt/tests/rt_smoke.rs`）。
- **ARC**：`examples/arc/run.sh` 增张量/视图场景，`sloth_rc_live` 回落基线。
- **数值**：`examples/tensor/` 固定用例（matvec/rmsnorm/softmax）对标手写参考；`examples/llama/run.sh` 复刻 `test_all.py`（同 seed greedy token 序列一致、逐层相对误差 < 1e-4）。
- **边界**：`pos = seq_len-1`（KV 写满）、GQA（`n_kv_heads < n_heads`）、shared/unshared 权重。
- **性能**：matvec benchmark ≥ `run.c -O3` 0.7×；融合开关加速比 ≥1.3×。
- **格式**：`cargo fmt --check`。

---

## 8. 文档回写（实现过程中同步）

1. `sloth-lang-2.0张量扩展设计文档.md`
   - §3.1/§5.3.2：删除 GC/免扫描/外部根，改写为 **ARC 双区（rc 头 + 非追踪 data）**；
   - §4.4：**修正「零拷贝 f32 视图」**为「f32→f64 加宽」，并说明后续 `Tensor<f32,R>`；
   - §5.1：删除自定义 `sloth.tensor_*` / `!sloth.tensor`，改为**标准 dialect 组合**；
   - §5.2：双通道保留，边界改为「语句/表达式物化」；
   - §7 变更清单：同步 `AddAssign/SubAssign`、位运算、`Ty::Tensor` 落点；
   - §8.3 风险：R1（memref ABI）入表，GC 风险删除。
2. `book/src/appendix_a_deviations.md`：新增张量条目（A.2 内存表示、A.5 标准库、A.6 精度）。
3. `PLAN-2026-09-15.md`：新增「张量扩展专项」章节（TE-P0… 进度表）。

---

## 9. 建议的首个可提交切片

**TE-P0 全量 + TE-P1 的 `Ty::Tensor` 与 rt 描述符骨架**：这两步不动 pass pipeline、不动 ABI，风险最低，且能立即用 spec/irgen 覆盖。之后以 **TE-P2 的 R1 原型**（`sloth_tensor_basis` memref ABI 最小验证）为关口——通过则继续 linalg 通道，不通过则按 R1 对策回退 rt 内核并在文档记录。

---

## 10. TE-P0 实施记录（已落地）

**交付**：复合赋值 `+=` / `-=`、整数位运算 `& | ^ << >> ~`，`>>` 与泛型收尾无冲突。

**落点**（与 §4 清单一致）：
- `sloth-frontend/src/lexer.rs`：`Tok` 增 `PlusEq/MinusEq/Amp/Caret/Tilde`；`punct()` 补配对与单字。
- `sloth-frontend/src/ast.rs`：`ArithOp` 增 `BitAnd/BitOr/BitXor/Shl/Shr`；`UnOp` 增 `BitNot`；`StmtNode::AssignOp`。
- `sloth-frontend/src/parser.rs`：优先级常量重编号并插入 `P_BOR/P_BXOR/P_BAND/P_SHIFT`（C 序：`| < ^ < & < shift < +`，且均紧于比较）；Pratt 段新增位运算；`unary()` 增 `~`；`stmt()` 增复合赋值分支。
- `sloth-codegen/src/irgen/expr.rs`：位运算走「解码 → `arith.andi/ori/xori/shli/shrsi` → 编码」，仅 `int`（非 int 报 `bitwise operators require \`int\` operands`）；`~` = `xori(dec, -1)`。
- `sloth-codegen/src/irgen/stmt.rs`：`AssignOp` 脱糖为 `t = t op v`，**索引子表达式先溢出到新建不可变短时槽**（`spill_temp`，沿用 `let` 的所有权规则）→ 读写各一次，副作用索引不被重复求值。
- `sloth-codegen/src/irgen/util.rs`：`path_to_expr`（路径 → 左值表达式）；`stmt_has_super_init` 补臂。
- `sloth-codegen/src/irgen/fnwalk.rs`：`walk_ids_stmt` 补 `AssignOp` 臂。

**与 §D4 的偏差（有意）**：未在词法层引入 `<<`/`>>` 记号，而是保留单字 `Lt`/`Gt`，在 **Pratt 循环里把相邻两枚 `Lt`/`Gt` 合并为移位**。这样 `Map<int,Array<int>>`、`Array<Array<int>>` 等嵌套泛型收尾零改动、零风险（若按原方案贪婪词法化 `Shr`，类型解析必须再做拆分）。风险 R3 因此不成立。

**验收**：
- spec 新增 `96_compound_assign.sl`（14 例，含 `data[next_index()] += 5` 断言索引只求值一次）、`97_bitwise.sl`（24 例，含优先级组合与嵌套泛型共存）、`98_diag_bitwise.sl`（3 条诊断）。
- codegen 新增 `irgen_te_p0`：6 用例（目标种类全覆盖 / 位运算 / 移位+泛型 / 浮点位运算诊断 / `~` 诊断 / `let` 不可变诊断）。
- `cargo test`：codegen **181** + frontend 20 + rt 1 + spec 2 runner 全绿；`cargo fmt --check` 干净；`examples/arc/run.sh` 9/9、`examples/diff/run.sh` 8/8 整数差分 MATCH；复合赋值含 str 索引/str 字段的 1000 轮 `sloth_rc_live` 差为 0。

---

## 11. TE-P1 实施记录（已落地）

**交付**：`Tensor<T, R>` 类型 + ARC 描述符运行时 + 索引/切片视图 + 视图赋值（`copy_into`）。ABI 仍是**一个 tagged rc 词**（D1），`util.rs::mlir_ret_ty`/`memref_cell_ty` 未改。

**落点**：
- `sloth-frontend/src/ast.rs`：`SimpleType::Tensor(Box<Type>, u32)`。
- `sloth-frontend/src/parser.rs::ty_base`：`Tensor '<' type ',' INT '>'`，rank 直读 `Tok::Int`（非负，0..=8）；元素仅 `float`/`int`（解析期拒绝其余，带位置）。嵌套泛型收尾无冲突（`>` 仍是单字 `Gt`）。
- `sloth-frontend/src/ty.rs`：`Ty::Tensor(TyId, u32)`；`fmt_ty`（`tensor:{elem}:{rank}`，作后端缓存键）、`ty_name`（`tensor`）、`subst`（递归 elem，rank 不变，**不进 `tp_subst`**）。
- `sloth-codegen/src/irgen/tybind.rs`：`ty_of_simple` Tensor 臂；`is_ref(Tensor)=true`；`surface_compat` 要求 **elem 与 rank 完全一致**；`surface_name` 增 `Tensor<T, R>`。
- `sloth-codegen/src/irgen/tensor.rs`（**新增**）：`tensor_info`、`emit_tensor_new_from_shape`（按 rank 发 `sloth_tensor_new_1/2/3`，dims 从 shape `Array<int>` 读）、`emit_tensor_zeros`/`emit_tensor_from_array`、`emit_tensor_index`（rank>1 降 rank 视图 / rank-1 取标量 / `a..b` 保 rank 切片）、`emit_tensor_view_drop`、`emit_tensor_set1`、`emit_tensor_copy_into`。
- `sloth-codegen/src/irgen/expr.rs`：`ExprNode::Index` 在 obj 求值后优先走 Tensor 路径（避免把 Range 当普通下标）；字段读取臂补 `Ty::Tensor`（此前落 `_ => I64`）；`emit_call` 识别 `tensor.zeros/from_array/fill_zero`。
- `sloth-codegen/src/irgen/stmt.rs`：`eval_assign_container` 中路径中间下标对 rank>1 Tensor 生成视图；最终下标臂：rank-1 = 标量写（`sloth_tensor_set1`），rank>1 = 视图 + `copy_into`；**range 目标**（`t[a..b] = src`）走保 rank 视图 + `copy_into`；`assign_target_type`/`assign_container_type` 补 Tensor。
- `sloth-codegen/src/irgen/module.rs::rt_decls`：挂 `sloth_tensor_*` 声明。
- `sloth-rt/src/tensors.rs`（**新增**）：7 词 payload `[flags, ndim, shape_ptr, stride_ptr, data_ptr, owner, total]`；`sloth_tensor_new_1/2/3`、`view`、`get1`/`set1`、`copy_into`、`copy_from_array`、`rank`/`dim`、`fill_zero`、`tensor_dtor`（释放 shape/stride → owner 则 release 否则 free data）。
- `sloth-rt/src/lib.rs`：挂 `tensors` 模块。

**关键不变式 / 踩坑**：
1. **视图 offset 必须乘 `stride[0]`**：`t[i]` 的 `i` 是 dim0 下标，不是扁平下标。首版漏乘导致 `from_array` 的 2D 读取错位（row1 读到 [2,3,4]），rt_smoke 的 stride 断言 + spec 行覆盖该回归。
2. 视图 `owner` 为 retained 父句柄，写视图即写原张量，dtor 级联 release；rt_smoke 尾断言 `sloth_rc_live` 回基线（无泄漏）。
3. 越界在 rt 侧 `panic_oob("tensor", ...)`；rank 0..>3 的创建在 codegen 侧诊断 `rank N unsupported`。

**验收**：
- spec 新增 `99_tensor.sl`（创建 / 2D-3D 索引链 / rank-1 标量读写 / 行视图共享存储 / `a..b` 切片 / `copy_into` / `from_array` / `int` 元素 / 切片目标赋值）。
- codegen 新增 `irgen_te_p1`：4 用例（视图读写 / 类字段张量 / `zeros` 缺目标诊断 / rank 超限诊断）。
- rt_smoke 新增张量段：shape/stride、视图共享写、保 rank 切片、`copy_into`、`copy_from_array`、释放链回基线。
- `cargo test`：codegen **185** + frontend 20 + rt 1 + spec 2 全绿；`cargo fmt --check` 干净；`examples/arc/run.sh` 9/9、`examples/diff/run.sh` 8/8 整数差分 MATCH。

---

## 12. TE-P2 R1 关口实施记录（已通过）

**结论**：R1 **通过** —— 指针→memref 的 ABI 在多方言 C-API + ORC JIT 路径上可用，TE-P2 按 §D2 双通道继续 `linalg` 路线（不回退 rt 内核）。

**最小原型**（`pass.rs::irgen_te_p2_r1`，2 用例）：
1. `basis_memref_abi`：rt 建 2×3 张量 → `sloth_tensor_basis_f64` → `memref.reinterpret_cast`（用运行期 `dim`/`stride` 喂 sizes/strides）→ `memref.load` 得 6；再 `memref.store` 7 → 经 `sloth_tensor_view` + `sloth_tensor_get1` 读回 7（**写视图即写原张量**在 linalg 通道同样成立）。
2. `target_pass_pipeline_parses`：JIT 侧 `mlirRegisterAllPasses()` + `mlirOpPassManagerAddPipeline("canonicalize,cse,linalg-fuse-elementwise-ops,one-shot-bufferize,convert-linalg-to-loops,convert-scf-to-cf,convert-math-to-llvm")` 解析成功；错误回调下「不存在的 pass」被拒（证明成功非空转）。

**关键 ABI 事实（与 §D1 的 sret 措辞修正）**：
- `func.func private @sloth_tensor_basis(i64) -> memref<?xf64, strided<[?], offset: ?>>` 经 `convert-func-to-llvm` 落为 **按值返回** `!llvm.struct<(ptr, ptr, i64, array<1 x i64>, array<1 x i64>)>`，**不是显式 `sret` 属性**。x86-64 SysV 对这枚 40B 聚合用隐藏返回指针，故 Rust `#[repr(C)] struct { allocated: *mut c_void, aligned: *mut c_void, offset: i64, size: [i64;1], stride: [i64;1] }` **按值返回**即 ABI 匹配（`sloth_tensor_basis_f64/i64`）。
- 描述符约定：`allocated == aligned == data_ptr`、`offset = 0`、`size[0] = total`、`stride[0] = 1`（视图已是连续区段；只切/降 dim0）。
- `memref.reinterpret_cast %flat to offset:[o], sizes:[d0,d1], strides:[s0,s1] : memref<?xf64,strided<[?],offset:?>> to memref<?x?xf64,strided<[?,?],offset:?>>` 落为 GEP 指针/偏移算术，运行期 sizes/strides 正常。
- **C-API 无 `one-shot-bufferize` 单 pass 构造器**，但可由 `mlirOpPassManagerAddPipeline` 文本管线驱动（`mlirRegisterAllPasses` 后）；AOT `slothc build` 侧 `/usr/lib/llvm-21/bin/mlir-opt` 亦有全部目标 pass。两处 pass 列表必须按 §5 抽共享常量。

**落点**：
- `sloth-rt/src/tensors.rs`：`MemRefDesc`（5 字段）+ `sloth_tensor_basis_f64/i64` + `sloth_tensor_stride`。
- `sloth-codegen/src/pass.rs`：`irgen_te_p2_r1` 原型（手工 MLIR 文本驱动，验证 ABI 与管线，不含 sloth 源层）。

**下一步（TE-P2 正式项）**：`irgen/tensor.rs` 的通道 B（`sloth_tensor_basis_*` + `memref.reinterpret_cast`）、通道 A（`tensor.empty`/`linalg.generic`/`reduce`/`dot`/`matmul` + `math.*`）；抽出 `pass_names()` 常量供 JIT（`mlirOpPassManagerAddPipeline`）与 AOT（`mlir-opt`）共用；matvec 达标线 §8.2。

---

## 13. TE-P2 实施记录（已落地）

**交付**：真 MLIR `linalg` 通道 + 统一 pass 管线 + `matvec/matmul/dot/sum/add/sub/mul/div/add_into` 算子族；matvec benchmark 达标。

**落点**：
- `sloth-codegen/src/pipeline.rs`（**新增**）：`pass_names()` 唯一真源，顺序为 `canonicalize, cse` → **`one-shot-bufferize, linalg-fuse-elementwise-ops, convert-linalg-to-loops, convert-scf-to-cf, convert-math-to-llvm`** → `convert-func/arith/index/cf-to-llvm, finalize-memref-to-llvm, reconcile-unrealized-casts`；`pass_pipeline_string()` 供 C-API。
- `sloth-codegen/src/jit.rs`：`run_llvm_pipeline` 改为 `mlirRegisterAllPasses()` + `mlirOpPassManagerAddPipeline(共享字符串)`（`one-shot-bufferize` 无单 pass creator），带丢弃式诊断回调，解析失败返回 `Err`。
- `slothc/src/main.rs::build_mode_r`：从 `pass_names()` 生成 `--<name>` 参数（消除与 JIT 的两处硬编码漂移）；clang 链接加 `-O3`。
- `sloth-codegen/src/irgen/tensor.rs`：通道 B 桥接 —— `emit_tensor_basis`（`sloth_tensor_basis_f64/i64`）、`emit_tensor_memref`（运行期 `dim/stride` + `memref.reinterpret_cast` 到 rank-1/2 strided）、`emit_tensor_alloc_dims`（按 dim 词建张量）；算子 `emit_tensor_matvec/matmul/dot/sum/add_into/binop`；`emit_encode_scalar`（f64/i64 → tagged 词）。
- `sloth-codegen/src/irgen/module.rs::rt_decls`：挂 `sloth_tensor_basis_f64/i64`、`sloth_tensor_stride`。
- `sloth-codegen/src/irgen/expr.rs`：`tensor.matvec/matmul/dot/sum/add/sub/mul/div/add_into` 识别（沿用 `tensor.*` 伪模块面）。

**关键设计/踩坑**：
1. **通道 B 落在 memref 域**（`linalg.*` 直接吃 memref），完全绕开 bufferization（风险 R2），KV cache/视图零拷贝天然成立；通道 A 的 `one-shot-bufferize` 仍在管线里，留给 TE-P3 融合用。
2. **视图靠运行期 stride 正确**：`emit_tensor_memref` 用 `sloth_tensor_dim/stride`（tagged→`shrsi 1`→`index`）喂 `reinterpret_cast`；`w3[l]` 这类 rank-3→rank-2 层视图 matvec 已验证（rt_smoke/spec/irgen 覆盖）。
3. **`dot`/`sum` 用 `scf.for` + 寄存器累加器**，不用 `memref.alloca`：后者若在循环体内会被 `finalize-memref-to-llvm` 落成每轮动态栈分配且只到函数返回才回收，长循环会栈溢出（本方案对 `linalg.dot` 的有意偏离，语义等价且无分配）。10 万轮 `tensor.dot` 回归无栈增长。
4. **命名算子隐式累加**：`linalg.matvec/matmul` 语义为 `out += A*B`，故输出张量必须零初始化（`emit_tensor_alloc_dims` 走 `calloc`）。
5. **元素类型约束**：TE-P2 的 `matvec/matmul` 仅 `float`（诊断），`dot/sum` 兼顾 `int`；`add_into` 原地 `dst += src`（无分配，视图可写）。
6. **运行期形状断言**（设计 §3.1）：算子入口发 `sloth_tensor_shape_eq`（同形元素级/`add_into`）与 `sloth_tensor_dim_eq`（`matvec` 内维 `dim(w,1)==dim(x,0)`、`matmul` `dim(a,1)==dim(b,0)`、`dot` 同长），不匹配即 panic，避免 linalg 越界写坏内存。

**验收**：
- codegen 新增 `irgen_te_p2`：5 用例（算子族运行 / rank-3 层视图 matvec / matvec rank 诊断 / matvec int 诊断 / 元素级 rank 诊断）；总数 **192** + frontend 20 + rt 1 + spec 2 全绿；`cargo fmt --check` 干净。
- spec 新增 `100_tensor_linalg.sl`（matvec / 层视图 matvec / matmul / 加减乘除 / dot / sum / add_into / rank-2 元素级）。
- `examples/tensor/`：`matvec.sl` + `matvec.c` + `run.sh`（512×512、300 轮，取多次最小值）。实测 reference（gcc -O3）48ms、sloth2 AOT（clang -O3）51ms，**ratio 0.94×**（目标 ≥0.70×，§8.2）；cargo/JIT 与 AOT 两路均正确。
- `examples/arc/run.sh` 9/9、`examples/diff/run.sh` 8/8 整数差分 MATCH。

**下一步（TE-P3）**：融合算子（rmsnorm/softmax/SwiGLU/RoPE，`linalg.generic` + `linalg.index` + `math.*`）与融合加速比；`lib/sloth/{tensor,random,fs}.slt` + `resolve_program` 搜索路径；`sloth-rt/src/mmap.rs`、`math.rs`（标量 libm）；checkpoint f32→f64 加宽。

---

## 14. TE-P3 实施记录（融合 + math + stdlib，已落地）

**交付**：融合算子族 + `math` 标量面 + 真 `.slt` 标准库与搜索路径 + 融合加速比达标。

**落点**：
- `sloth-codegen/src/irgen/tensor.rs`：`open_map`/`close_map`（同形 `linalg.generic` 发射）、`fconst`/`fmt_f64`、`emit_tensor_dim_i64`、`emit_same_shape_out`；算子 `emit_tensor_unary`（`exp/sqrt/sin/cos/tan` → `math.*`）、`emit_tensor_silu`、`emit_tensor_silu_mul_into`（SwiGLU 融合）、`emit_tensor_rmsnorm`（1 次 `scf.for` 平方和 + 1 个融合 `x*inv*w` generic）、`emit_tensor_softmax`（`softmax`/`softmax_into`：max→exp+sum→div 三段 `scf.for`，原地开关）、`emit_tensor_add_scaled_into`、`emit_tensor_div_scalar_into`。
- `sloth-codegen/src/irgen/expr.rs`：识别上述 `tensor.*` 算子；`float_sqrt/exp/sin/cos/tan/floor/pow` 标量内建（走 `emit_dec_f`/`emit_enc_f`，非 float 报错）。
- `sloth-rt/src/math.rs`（**新增**）：`sloth_rt_sqrt/exp/sin/cos/tan/pow/floor`（D6）；`lib.rs` 挂载；`rt_decls` 声明。
- `sloth-codegen/src/irgen/mod.rs`：`find_import` + `DEV_LIB`（`<repo>/lib`）实现 D5 搜索序（导入者目录 → `$SLOTH_STDLIB` → `<exe>/../lib` → dev `<repo>/lib`）。
- `lib/sloth/random.slt`（`pub class XorShift`，TE-P0 位运算 RNG）、`lib/sloth/tensor.slt`（`normalize`/`sumsq`/`l2` 纯 sloth 包装）。

**关键踩坑**：
1. **MLIR 浮点字面量必须带小数点**：`1e-5`、`-inf` 均被拒（`expected constant integer or floating point value`），`1.0e-5` 可；故 `fmt_f64` 在指数前补 `.0`，softmax 的 max 初值用有限哨兵 `f64::MIN` 取代 `-inf`。此前全局浮点都是 tagged 整数字面量，未暴露此问题。
2. **`linalg.generic` 的 `ins` 是 `ins(%a, %b : ty, ty)`**（call 风格 `ins(%a : ty, %b : ty)` 解析失败 `expected non-function type`）；且 **零输入 `ins()` 被拒**，`div_scalar_into` 传入 dst 自身作单输入。
3. **导入模块的 `func`/`class` 必须 `pub`** 才能跨模块调用（否则 `unknown`/`is private`）。
4. 融合即「一个 generic 体内多算子 + 少一次中间分配」；`silu_mul_into`/`rmsnorm` 的 epilogue 天然融合。

**验收**：
- spec 新增 `101_stdlib_import.sl`（`.slt` 导入 + RNG 确定性 + `tensor.normalize`/`l2`）、`102_tensor_fused.sl`（math 元素级 / silu / SwiGLU / rmsnorm / softmax / add_scaled / div_scalar / 标量 math）。
- codegen 新增 `irgen_te_p3`：3 用例（融合核运行 / rmsnorm rank 诊断 / 标量 math int 诊断）；总数 **195** + frontend 20 + rt 1 + spec 2 全绿；`cargo fmt --check` 干净。
- `examples/tensor/run.sh` 增融合对照：rmsnorm fused 19ms vs naive 33ms，**fusion speedup 1.74×**（目标 ≥1.30×）；matvec 仍 0.92×。

**下一步（TE-P4）**：`lib/sloth/llama.slt`（`forward` 参考实现 + 4 种采样 + BPE tokenizer）；stories260K/15M 加载与生成；融合开关对照（pass 级）留 TE-P5。

### 14.1 TE-P3 checkpoint IO（已落地）

**交付**：`mmap` 运行时 + `lib/sloth/fs.slt` + f32→f64 加宽 + 端到端加载。

**落点**：
- `sloth-rt/src/mmap.rs`（**新增**）：`sloth_mmap(str) -> ByteBuffer`（`open`+`fstat`+`mmap(PROT_READ, MAP_PRIVATE)`，句柄为 `BufHdr{len,data}`）、`sloth_mmap_len`、`sloth_mmap_i32`（LE，越界 panic）。
- `sloth-rt/src/tensors.rs`：`sloth_tensor_from_f32_ptr(buf, off, n)` 把映射区 f32 加宽拷贝进新 rank-1 f64 张量（D7 的「非零拷贝」硬修正）。
- `lib/sloth/fs.slt`：`extern type ByteBuffer;` + `extern func sloth_mmap/mmap_len/mmap_i32/tensor_from_f32_ptr`，`open_file`/`size`/`read_config`（7 个 i32 头，vocab 负值约定）/`view_as_f32`。
- `examples/fs/`：`gen_fixture.sh`（7×i32 头 + 4×f32 权重）、`load.sl`、`run.sh` 断言 config/长度/权重和。

**关键 ABI 修正**：
1. **导入模块的 extern func 符号错配**：`register_import` 把 extern func 也按 sloth 规则 mangled 注册进 `cross_funcs`，调用名 `sloth_fs__…` 与按 raw 名发射的声明不匹配。修：extern func 同时挂进 `self.funcs`（raw 名），调用走 extern ABI 路径（参数 `emit_dec_*`/返回 `emit_enc_*`）；`cross_funcs` 也登记 raw 名。
2. **extern 边界是裸 C ABI**：`int` 实参在调用侧已解码为裸 i64、返回再编码；故 rt 侧 `sloth_mmap_i32`/`sloth_tensor_from_f32_ptr` 的 offset/len **不能再 `dec_i`**（先前的 `dec_i` 是配合错误的 `cross_funcs` 直传路径）。`str` 实参仍以 tagged 词透传（`w_unref` 取 `StrT`）。

**验收**：`examples/fs/run.sh` checkpoint load **OK**（dim=4/hidden=8/vocab=32/len=44/权重 [1,4]/sum=10）；rt_smoke 增 mmap 段（config + 加宽 + `rc_live` 回基线）；codegen 增 `fs_import_mmap`（`.slt` 导入 + mmap 端到端）；总数 **196** + frontend 20 + rt 1 + spec 2 全绿；`cargo fmt --check` 干净；arc 9/9、diff 8/8、tensor matvec 0.92× / fusion 1.74×。


---

## 15. TE-P4 实施记录（llama.slt + 采样器 + BPE tokenizer，已落地）

**交付**：`lib/sloth/{tokenizer,llama}.slt`（完整 llama2.c 推理）+ 4 种采样 + BPE tokenizer + 端到端 checkpoint 加载/生成；单机 fp32 跑通 `run.c` 目标达成。

**落点**：
- `sloth-rt/src/tensors.rs`：`sloth_tensor_reshape1/2/3(t, off, dims…)` 共享存储视图（连续行主 stride，越界 panic），把整块加宽的权重平面切层为 rank-2/3。
- `sloth-rt/src/strings.rs`：`sloth_str_cmp`（字节序词法比较，裸 C 返回）、`sloth_str_byte`、`sloth_str_slice`、`sloth_str_of_byte`、`sloth_rt_write_str`（无换行输出，生成用）。
- `sloth-rt/src/mmap.rs`：`sloth_mmap_u8`、`sloth_mmap_f32`、`sloth_mmap_str`；`i32/f32` 读取改 `read_unaligned`（tokenizer 条目变长、偏移不对齐）。
- `lib/sloth/fs.slt`：新增上述 extern + `read_i32/read_u8/read_f32/read_str/write/byte_at/cmp_str/str_slice/byte_str` pub 包装。
- `lib/sloth/tensor.slt`：`matrix_view`/`cube_view`/`flatten_view`（reshape extern 包装）。
- `lib/sloth/tokenizer.slt`（**新增**）：`Tokenizer`（vocab/scores/sorted）、`load_tokenizer`（mmap 读 tokenizer.bin，qsort 词表）、`str_lookup`（二分）、`decode`（BOS 去前导空格 + `<0xNN>` 原始字节展开）、`safe_write`、`encode`（dummy 前缀 + 按码点查找 + 字节回退 + 贪心分数合并）。
- `lib/sloth/llama.slt`（**新增**）：`Config`/`Weights`/`RunState`；`load_config`（vocab 负值=非共享）、`config_shared`、`load_weights`（一次 f32→f64 加宽 + reshape 切层）；`forward` 逐行对标 run.c（embedding 行拷贝、attention rmsnorm、QKV、RoPE、GQA 多头注意力 + KV cache、输出投影 + 残差、FFN rmsnorm + SwiGLU、最终 rmsnorm + 分类头）；`argmax`/`sample_mult`/`sample_topp`/`sample`（temp=0 greedy，否则 softmax + multinomial/top-p）、`generate`。
- `examples/llama/`：`main.sl`（greedy 40 步）、`gen_tiny.py`（确定性共享/非共享 GQA tiny 模型 + 合成 32000 词表 tokenizer）、`expected_tiny_{shared,unshared}.txt`（run.c 黄金输出）、`run.sh`（tiny 差分 + stories42M 实跑与 run.c 对照）。

**关键修正（编译/codegen，均为通用 bug）**：
1. **嵌套 import 顺序**：`resolve_program` 原先把「模块本身」先于其依赖压栈，导致 `register_import` 在依赖注册前发射模块体 → 跨模块调用全部 `unknown`。改为依赖先入栈（后序）。
2. **导入模块的私有函数**：非 `pub` 函数只进 `hidden`，同模块内调用解析不到。同一模块内回退 `{cur_mod}.{name}` 查找 `cross_funcs`（其他模块仍被 `hidden` 挡住）。
3. **返回 `unit` 的 extern 声明**：声明侧恒发 `-> i64`，调用侧发 `-> ()`，MLIR 校验失败。改为 unit 发 `-> ()`。
4. **AOT 链接缺 `-lm`**：`math.exp` 等 lowering 出的 libm 调用在 `--as-needed` 下未被拉入；`clang` 链接补 `-lm`。
5. **权重矩阵朝向**：run.c `matmul(xout,x,w,n,d)` 把 w 当 `d` 行 `n` 列，故 `wk/wv=(kv_dim,dim)`、`w1/w3=(hidden,dim)`、`w2=(dim,hidden)`（初版按反了，运行期 `dim_eq` 16≠32 暴露）。

**验收**：
- `examples/llama/run.sh`：tiny shared / unshared（含 GQA）greedy 40 步输出与 `run.c` **逐字节一致**；stories42M greedy 40 步与 `run.c` **一致**（`Once upon a time, there was a little girl named Lily. …`），实测 2095ms vs run.c 1089ms ≈ **0.52×**（§8.2 目标 ≥0.5×）。
- codegen 新增 `irgen_te_p4`：4 用例（reshape 视图共享存储 / 嵌套导入调用私有函数 / unit extern / reshape 视图 matvec）；总数 **200** + frontend 20 + rt 1 + spec 2 全绿；`cargo fmt --check` 干净。
- spec 新增 `103_reshape_views.sl`、`104_fs_str_helpers.sl`；rt_smoke 增字符串面 + reshape 视图段。
- 回归：arc 9/9、diff 8/8、fs checkpoint OK、tensor matvec 0.92× / fusion 1.74×。

**遗留（TE-P5）**：`random.slt` 的 xorshift 为 31-bit，temperature 采样与 run.c 的 64-bit `xorshift64*` 未逐位对齐（greedy 确定性对齐已达成）；融合 pass 对照与 `--target-cpu=native` 开关；stories15M/110M 覆盖。
