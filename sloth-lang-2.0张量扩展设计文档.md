# sloth-lang 2.0 张量计算扩展设计文档

## 基于 MLIR linalg / tensor dialect 的张量计算支持

| 项目 | 内容 |
| --- | --- |
| 文档版本 | v1.0（草案） |
| 文档日期 | 2026-09-14 |
| 上游文档 | 《sloth-lang 2.0 设计文档》v1.0（下称「主文档」） |
| 验收基准 | **完整实现 karpathy/llama2.c（run.c）的 fp32 推理** |
| 文档状态 | 设计评审稿 |

---

## 1. 概述

### 1.1 定位

本文档是主文档的**计算扩展**分册：在不改动 sloth2 核心语法骨架的前提下，新增内建张量类型、张量算子标准库与对应的 MLIR lowering 路径（`tensor`/`linalg` dialect），使 sloth2 具备可用的数值计算能力。

### 1.2 验收目标

**语言能力下限：用 sloth2 完整实现 llama2.c `run.c` 的全部推理功能**，包括：

1. `forward()`：Llama-2 架构单 token 前向传播（embedding 查表、RMSNorm、QKV 投影、RoPE、多头注意力 + KV cache、SwiGLU FFN、残差连接、logits 输出）；
2. `sample()`：greedy argmax、temperature + softmax、multinomial、top-p（nucleus）四种采样；
3. checkpoint 加载：llama2.c `.bin` 格式（mmap，fp32，含 shared weights 的 vocab_size 负值约定）；
4. BPE 分词器（encode/decode）——纯字符串/数组逻辑，验证基础语言能力，不依赖张量扩展，但纳入端到端验收。

### 1.3 非目标

- 训练/反向传播（无 autograd）；
- 量化推理（runq.c 的 int8 Q8_0 路径列为后续扩展，需 `int8` 元素类型支持）；
- GPU/accelerator offload（linalg 的 GPU lowering 列为后续工作）；
- 多线程并行（run.c 的 OpenMP 加速对应主文档「线程留待 3.0」的决策；首版对标 `make run` 单线程性能）。

### 1.4 设计约束

- 遵循主文档类型系统：静态强类型、泛型单态化、`T?` 可空、`Hashable` 等；
- 张量相关高层 IR 使用 MLIR **`tensor` dialect**（SSA 值语义）与 **`linalg` dialect**（结构化算子），经 one-shot bufferization 落到 `memref`，最终走主文档 §4.3.3 的 `llvm` dialect 路径；
- 不引入新的顶层语法结构（不新增关键字），张量能力以「内建类型 + 标准库模块」形式提供。

---

## 2. 需求分析：run.c 推理算子 → 语言能力映射

对 run.c 全文（约 700 行）的推理路径逐项分析，提取语言能力需求：

| run.c 代码位置 | 计算内容 | 需要的语言能力 | 扩展覆盖 |
| --- | --- | --- | --- |
| `forward` embedding 拷贝 | `memcpy(x, table + token*dim, dim)` | 2D 张量按行取 1D 视图、视图拷贝 | `t[i]` 降维视图 + `copy_into` |
| `rmsnorm` | 平方和归约 → 逐元素乘加 | 归约（sum）、逐元素乘、`sqrt` | `linalg.reduce` + `linalg.generic` |
| `matmul` | W(d,n) @ x(n) → out(d)，全模型热点 | matvec、浮点乘加 | `linalg.vecmat`（权重布局行主序 (d,n)） |
| RoPE | 按位置 `pos` 与索引 `i` 计算 `cos/sin`，旋转向量对 | 逐索引计算（index-aware 元素级 op）、`sin`/`cos`/`pow` | `linalg.generic`（indexing map 携带索引）+ math intrinsic |
| KV cache | `key_cache[l][pos] = k`，注意力读历史行 | 3D 张量切片视图、**视图原地写** | 可变视图 + `memref.subview` |
| 注意力打分 | q·k 点积、除以 `sqrt(head_size)`、对 `0..=pos` 动态长度 softmax | 动态形状归约、点积、softmax | `linalg.dot` / reduce + generic |
| 注意力加权和 | `xb += a[t] * v[t]` 循环 | 逐元素乘累加 | `linalg.generic`（融合后等价 axpy） |
| 残差连接 | `x[i] += xb[i]` | 原地复合赋值 | `+=`（新增 `AddAssign` 语法糖） |
| SwiGLU | `silu(h1) * h3` 逐元素 | `expf`、逐元素乘 | `linalg.generic` + `math.exp` |
| `softmax` | max 归约 → exp → sum 归约 → 除 | max/sum 归约 | `linalg.reduce` ×2 + generic |
| `sample_argmax` | 概率最大值下标 | 携带下标的 max 归约 | `linalg.reduce`（index-carrying） |
| `sample_mult` / `sample_topp` | CDF 扫描、按概率排序、截断 | 排序（带下标）、前缀扫描、RNG | 标准库 `sort_by` + 手写循环 + xorshift |
| `random_u32/f32` | xorshift 位运算 | **整数位运算符**（`& \\| ^ << >>`，`int` 限定） | 本次扩展新增 |
| `read_checkpoint` | mmap + 指针算术切分权重 | mmap FFI、字节缓冲按 f32 视图化 | `fs.mmap` extern + `view_as_f32` |
| Tokenizer encode/decode | 字符串切分、`qsort`/`bsearch`、UTF-8 处理 | 字符串/数组/排序/哈希表（`Map`） | 主文档基础能力 |

关键结论：

1. **张量 rank 需求 ≤ 3**（权重最高 3D：`(layer, dim, dim)`；KV cache 3D；激活 1D/2D）；
2. **所有形状在运行时从 Config 读取**（dim、hidden_dim、n_layers、n_heads 等来自 checkpoint 头部）——张量类型必须支持**动态形状**；
3. **KV cache 要求视图原地写**——纯值语义（每次复制）不可接受，必须有可变视图；
4. 元素类型仅需 `float`（f32/f64 权衡见 §3.1），但类型设计保留泛型以兼容未来 int8 量化；
5. 热点是 matvec（`W(d,n) @ x(n)`），性能取决于 linalg lowering 质量（向量化/分块/融合），这正是接入 linalg 的收益点。

---

## 3. 语言层设计

### 3.1 张量类型

```rust
Tensor<T, R>
```

- `T`：元素类型，约束 `T: Numeric`（首版实例化 `float`；`int` 可用；int8 待量化阶段）；
- `R`：**rank（维度数），编译期常量**，作为类型参数（如 `Tensor<float, 2>`）。这是对主文档泛型系统的受控扩展——类型参数允许出现**整数字面量常量**（const generic 的极简形式，仅允许非负整数字面量，不允许常量表达式），专供张量 rank 使用；
- **形状（shape）动态**：`Tensor<float, 2>` 的实际行列数运行期确定（llama2.c 的 dim/hidden_dim 均来自 checkpoint），形状错误在运行时报 `panic`；rank 错误**编译期**报；
- 为什么 rank 静态而 shape 动态：rank 参与几乎所有算子的类型检查（matvec 要求 `Tensor<T,2> × Tensor<T,1>`），静态 rank 能拦住绝大多数形状类 bug；shape 静态化（`Tensor<float, 512, 288>`）对从运行时配置驱动的推理程序没有收益，且会使泛型系统复杂度爆炸，明确不做。

**内存表示**（补入主文档 §2.6 引用类型表）：

| 类型 | 表示 |
| --- | --- |
| `Tensor<T, R>` | GC 堆指针。对象头：类型描述符 + rank + `shape[R]` + `strides[R]` + 数据区指针 + 数据区所有者引用。数据区为大块无指针内存（对齐分配，**免 GC 扫描**），可被多个视图张量共享（所有者引用保活） |

> 元素精度说明：llama2.c 使用 fp32，而 sloth2 的 `float` 是 f64。首版张量元素一律用 `float`（f64），精度上界优于 fp32、数值与 run.c 的差异在容差内（验收见 §8）；f32 元素类型作为存储/带宽优化列入后续（届时 `Tensor<f32, R>` 为独立元素类型）。

### 3.2 索引、切片与视图

```rust
let row: Tensor<float, 1> = table[token];   // R>1 时 t[i] 返回 rank R-1 的视图
let wq_l: Tensor<float, 2> = wq[l];         // (layer, dim, dim) 第 l 层 → (dim, dim)
let k_hist = kc[l][0..=pos];                // 第 0 维范围切片，rank 不变，view
let head_q = q[h * head_size .. (h+1) * head_size];  // 1D 子段
```

规则：

- `t[i]`（`i: int`）：`Tensor<T, R> → Tensor<T, R-1>` 视图（R=1 时返回标量 `T`）；越界运行时报 panic；
- `t[a..b]` / `t[a..=b]`：沿第 0 维切片，`Tensor<T, R> → Tensor<T, R>` 视图；`..`/`..=` 沿用主文档范围运算符；
- **视图共享底层存储，写视图即写原张量**——KV cache 的 `kc[l][pos] = k;` 依赖此语义；
- 视图赋值：`kc[l][pos] = k;` 要求两侧 rank/shape 匹配，语义为元素拷贝（copy_into），不是重绑定；
- 视图是廉价的（只拷贝 header，R ≤ 8 上限，shape/strides 内联在 header 中）。

### 3.3 运算符与广播

- `+ - * /` 对 `Tensor` 重载（实现主文档 §3.4 的 `Add/Sub/Mul/Div` trait）：**NumPy 式广播**的逐元素运算；标量与张量混合运算合法（`t * 0.5`）；
- **新增复合赋值** `+=` `-=`（`AddAssign`/`SubAssign` trait），用于残差连接 `x += xb2;`，语义为原地累加，编译器保证无中间分配；
- 矩阵乘不设运算符（`@` 已被 map 字面量占用），用标准库函数 `matmul`/`matvec`（见 §4），类型签名即形状约束；
- **新增整数位运算符**（仅 `int`，为 xorshift RNG 与底层位处理所需）：`& | ^ << >>` 及一元 `~`。这是主文档运算符表的新增行，优先级与 C 一致。

### 3.4 EBNF 增量（叠加在主文档 §3.9 之上）

```ebnf
// 类型产生式新增
type            ::= ... | 'Tensor' '<' type ',' INT '>'

// 赋值语句扩展（复合赋值）
assignment_stmt ::= assignable ( '=' | '+=' | '-=' ) expr ';'

// 表达式：binop 新增位运算（仅 int）
binop           ::= ... | '&' | '|' | '^' | '<<' | '>>'
unop            ::= ... | '~'
```

无新关键字。`Tensor` 与 `int/float/...` 同为上下文关键字。

---

## 4. 张量标准库（`tensor` 模块）与 linalg 映射

标准库 `sloth/tensor.slt`，算子以泛型函数提供；每个算子标注其 lowering 目标。

### 4.1 构造与 IO

| API | 说明 | lowering |
| --- | --- | --- |
| `tensor.zeros<T, R>(shape: Array<int>): Tensor<T, R>` | 零初始化分配 | `sloth.tensor_alloc` → 运行时 |
| `tensor.from_array<T, R>(data: Array<T>, shape: Array<int>)` | 由数组构造 | 运行时拷贝 |
| `t.reshape(shape: Array<int>): Tensor<T, R2>` | 形状重解释（元素数须一致，零拷贝视图） | `tensor.reshape` / `memref.expand_shape` |
| `t.transpose(): Tensor<T, R>` | 轴交换视图 | `tensor.transpose`（linalg indexing map 交换） |

### 4.2 核心算子（llama2.c 全覆盖）

| API | 语义 | linalg / tensor 目标 |
| --- | --- | --- |
| `matvec(w: Tensor<float,2>, x: Tensor<float,1>): Tensor<float,1>` | W(d,n)@x(n)→(d,)，**推理第一热点** | `linalg.vecmat`（经转置 map）→ 向量化 |
| `matmul(a: Tensor<float,2>, b: Tensor<float,2>): Tensor<float,2>` | 通用 GEMM（供扩展，run.c 未用） | `linalg.matmul` |
| `add/sub/mul/div(a, b)` | 广播逐元素 | `linalg.generic`（parallel，broadcast map） |
| `add_into(dst, src)` | 原地 `dst += src` | bufferization 后 `linalg.generic`（无分配） |
| `rmsnorm(x, w: Tensor<float,1>): Tensor<float,1>` | 平方和归约 + 缩放逐元素乘 | `linalg.reduce`(sum) + `linalg.generic`，epilogue 融合 |
| `softmax(x: Tensor<float,1>): Tensor<float,1>` | max/exp/sum 三段 | `linalg.reduce`(max,sum) + generic，逐段融合 |
| `rope(q, k: Tensor, pos: int)` | 按索引取 `cos/sin(pos·freq)` 旋转 q/k 对 | `linalg.generic` + `linalg.index`，sin/cos → `math` dialect intrinsic |
| `dot(a, b: Tensor<float,1>): float` | 点积（注意力打分） | `linalg.dot` |
| `argmax(x: Tensor<float,1>): int` | 最大值下标（greedy 采样） | `linalg.reduce`（携带 index 的 max） |
| `embedding_row(table: Tensor<float,2>, i: int)` | 取行视图（= `table[i]`，零拷贝） | `tensor.extract_slice` → `memref.subview` |
| `silu(x)` | `x * sigmoid(x)` 逐元素 | `linalg.generic` + `math.exp` |
| `sort_desc_index(x: Tensor<float,1>): Array<int>` | 按值降序返回下标（top-p 用） | 标准库实现（比较 + 下标对排序），不经 linalg |
| `cumsum(x)` / 前缀扫描 | top-p CDF | 首版标准库顺序实现，标注后续可 lowering 到 `linalg.scan` 类 op |

### 4.3 随机数（采样）

xorshift32/xorshift64 用 §3.3 新增的整数位运算**纯 sloth2 实现**（标准库 `random` 模块），与 run.c 的 `random_u32`/`random_f32` 算法一致，便于数值对齐测试；不作为语言内建。

### 4.4 checkpoint 加载（`fs` 模块 + extern）

```rust
extern func sloth_mmap(path: str): ByteBuffer;   // extern type，映射 llama2.c 的 mmap
```

- `ByteBuffer` 为 `extern type`（主文档 §5.4 的不透明类型）；
- `buf.read_config(): Array<int>` 读取头部 7 个 i32（复刻 run.c 的 Config 结构及 **vocab_size 负值 = 权重不共享**的约定）；
- `buf.view_as_f32(offset: int, len: int): Tensor<float, 1>` 零拷贝视图化，再经 `reshape` 得到各权重矩阵——完整复刻 `memory_map_weights` 的指针切分逻辑，但以类型安全的方式表达（offset/len 越界 panic）。

---

## 5. MLIR 集成设计

### 5.1 Dialect 扩展

主文档 §4.3.1 的 sloth dialect 新增：

```mlir
!sloth.tensor<f64, 2>              // rank 静态、shape 动态的张量引用（GC 托管 header）

// 高层 op（携带 GC/布局语义）
sloth.tensor_alloc                 // 分配 header + 数据区
sloth.tensor_slice                 // 视图（subview 语义）
sloth.tensor_copy_into             // 视图间拷贝
sloth.tensor_store                 // 视图原地写（KC cache）
sloth.tensor_op                    // 结构化算子入口（matvec/rmsnorm/...，携带算子标记）
```

### 5.2 Lowering 路径（张量专用通道）

```javascript
sloth.tensor_op（结构化算子标记）
   │
   ├─ 不可变中间结果 → tensor dialect 值语义（tensor.empty / tensor.extract_slice）
   │        │
   │        ▼
   │   linalg dialect（linalg.vecmat / generic / reduce）
   │        │  ◄── fusion（rmsnorm/softmax/SwiGLU 多段融合）、tile、vectorize
   │        ▼
   │   one-shot bufferization ──────────► memref dialect
   │
   └─ 可变张量/视图（KV cache、residual add_into）→ 直接 memref 语义
        （subview/store，绕过 tensor 值语义，避免无谓拷贝）
   │
   ▼
memref → func/scf/arith → llvm dialect（接主文档 §4.3.3 第三轮）
```

### 5.3 关键设计决策

1. **双通道**：纯函数式中间结果走 `tensor`→`linalg`→bufferization（最大化优化空间）；KV cache 这类跨迭代存活的可变状态直接走 `memref`（其语义本就是可变 buffer，强行值语义只会引入拷贝）。判定规则：类型检查期标记「跨函数逃逸/被原地写」的张量直接采用 memref 通道；
2. **数据区免扫描**：张量数据区只含 float，GC 位图中标记为无指针区域，大模型权重（7B 级 ~26GB）不参与 GC 根扫描——否则保守式 Boehm GC 的扫描开销不可接受；mmap 来的权重区注册为 GC 的"外部根"，header 持有引用保活；
3. **性能对标**：run.c 单线程 `gcc -O3`（向量化和循环展开）与 `make runfast`（`-Ofast -march=native`）。对应到本设计：`linalg` tile + vectorize pass + LLVM `-O3` 等价物 + `target-cpu=native` 编译选项。预期达到 run.c `-O3` 的同一数量级；matvec 的访存受限特性决定了差距主要来自向量化质量；
4. **融合优先级**：rmsnorm（reduce+scale）、softmax（reduce+exp+reduce+div）、SwiGLU（silu+mul）、RoPE（index+cos/sin+rotate）全部标注为可融合区域，由 linalg fusion pass 合并为单个循环，消除中间张量分配。

### 5.4 数学函数

`exp/sqrt/sin/cos/pow`：优先使用 MLIR `math` dialect（`math.exp`、`math.sqrt`、`math.sin`…），lowering 到 LLVM intrinsic，自动获得向量化版本；精度语义对标 libm f64。

---

## 6. llama2.c 推理的 sloth2 参考实现

以下代码即验收基准的功能规格（节选自 `llama.slt`，语法遵循主文档 + 本扩展）：

```rust
import "sloth/tensor.slt";
import "sloth/fs.slt";
import "sloth/random.slt";

class Config {
    let dim: int;  let hidden_dim: int;  let n_layers: int;
    let n_heads: int;  let n_kv_heads: int;
    let vocab_size: int;  let seq_len: int;
}

class Weights {                       // 全部为零拷贝 mmap 视图
    let token_embedding_table: Tensor<float, 2>;   // (vocab_size, dim)
    let rms_att_weight: Tensor<float, 2>;          // (layer, dim)
    let wq: Tensor<float, 3>;  let wk: Tensor<float, 3>;
    let wv: Tensor<float, 3>;  let wo: Tensor<float, 3>;
    let rms_ffn_weight: Tensor<float, 2>;
    let w1: Tensor<float, 3>;  let w2: Tensor<float, 3>;
    let w3: Tensor<float, 3>;
    let rms_final_weight: Tensor<float, 1>;
    let wcls: Tensor<float, 2>;                    // 可能与 embedding 共享
}

class RunState {
    var x: Tensor<float, 1>;   var xb: Tensor<float, 1>;
    var xb2: Tensor<float, 1>; var hb: Tensor<float, 1>;
    var hb2: Tensor<float, 1>; var q: Tensor<float, 1>;
    var att: Tensor<float, 2>;                      // (n_heads, seq_len)
    var logits: Tensor<float, 1>;
    var key_cache: Tensor<float, 3>;                // (layer, seq_len, kv_dim)
    var value_cache: Tensor<float, 3>;
}

func forward(cfg: Config, w: Weights, s: RunState, token: int, pos: int): unit {
    let dim = cfg.dim;
    let kv_dim = dim * cfg.n_kv_heads / cfg.n_heads;
    let kv_mul = cfg.n_heads / cfg.n_kv_heads;
    let head_size = dim / cfg.n_heads;
    let hidden_dim = cfg.hidden_dim;

    // embedding 行拷贝（零拷贝视图 + copy_into）
    s.x = tensor.copy_of(w.token_embedding_table[token]);

    for (var l: 0..cfg.n_layers) {
        // 1. attention rmsnorm
        s.xb = tensor.rmsnorm(s.x, w.rms_att_weight[l]);

        // 2. qkv 投影（热点 matvec）
        s.q = tensor.matvec(w.wq[l], s.xb);
        let kc_pos = s.key_cache[l][pos];           // 视图
        let vc_pos = s.value_cache[l][pos];
        kc_pos = tensor.matvec(w.wk[l], s.xb);      // 视图原地写 = KV cache 更新
        vc_pos = tensor.matvec(w.wv[l], s.xb);

        // 3. RoPE（q 全量，k 仅前 kv_dim）
        tensor.rope(s.q, pos, head_size);
        tensor.rope(kc_pos, pos, head_size);

        // 4. 多头注意力
        for (var h: 0..cfg.n_heads) {
            let q_h = s.q[h * head_size .. (h + 1) * head_size];
            let att_h = s.att[h];                   // (seq_len,) 视图
            let k_hist = s.key_cache[l][0..=pos];   // (pos+1, kv_dim) 视图
            let kh = h / kv_mul;
            for (var t: 0..=pos) {
                let k_t = k_hist[t][kh * head_size .. (kh + 1) * head_size];
                att_h[t] = tensor.dot(q_h, k_t) / float_sqrt(int_to_float(head_size));
            }
            // softmax（动态长度 0..=pos 子段，原地）
            tensor.softmax_into(att_h[0..=pos]);
            // 加权和写回 xb 的第 h 头
            let xb_h = s.xb[h * head_size .. (h + 1) * head_size];
            tensor.fill_zero(xb_h);
            for (var t: 0..=pos) {
                let v_t = s.value_cache[l][t][kh * head_size .. (kh + 1) * head_size];
                tensor.add_scaled_into(xb_h, v_t, att_h[t]);   // xb_h += a * v_t
            }
        }

        // 5. 输出投影 + 残差
        s.xb2 = tensor.matvec(w.wo[l], s.xb);
        s.x += s.xb2;                               // AddAssign，原地

        // 6. FFN：rmsnorm → w1/w3 → SwiGLU → w2 → 残差
        s.xb = tensor.rmsnorm(s.x, w.rms_ffn_weight[l]);
        s.hb = tensor.matvec(w.w1[l], s.xb);
        s.hb2 = tensor.matvec(w.w3[l], s.xb);
        tensor.silu_mul_into(s.hb, s.hb2);          // hb = silu(hb) * hb2，融合算子
        s.xb = tensor.matvec(w.w2[l], s.hb);
        s.x += s.xb;
    }

    // 7. 最终 rmsnorm + 分类头
    s.x = tensor.rmsnorm(s.x, w.rms_final_weight);
    s.logits = tensor.matvec(w.wcls, s.x);
}

// 采样器：与 run.c 四种模式一一对应
func sample(logits: Tensor<float, 1>, temperature: float, topp: float, rng: XorShift): int {
    if (temperature == 0.0) { return tensor.argmax(logits); }
    tensor.div_scalar_into(logits, temperature);
    let probs = tensor.softmax(logits);
    let coin = rng.next_f32();
    if (topp <= 0.0 or topp >= 1.0) { return tensor.sample_cdf(probs, coin); }
    return tensor.sample_topp(probs, topp, coin);   // 内部使用 sort_desc_index + cumsum
}
```

tokenizer 的 encode/decode（BPE 合并循环、`str_lookup` 二分查找）使用主文档的 `str`/`Array`/`Map<str, int>`/排序闭包实现，不含张量代码，从略。

---

## 7. 与主文档的变更清单（回写项）

本扩展对主文档产生的增量修改，实施时需回写：

| 位置 | 变更 |
| --- | --- |
| §2.1 类型全集 | 新增 `Tensor<T, R>` 行 |
| §2.3 泛型 | 补充「类型参数允许整数字面量常量（仅限张量 rank）」的受控扩展条款 |
| §2.6 内存表示 | 引用类型表新增 `Tensor<T, R>` 行（双区结构：header 受 GC 管理，数据区免扫描） |
| §3.1 变更总览 | 新增「张量」「复合赋值」「位运算」三行 |
| §3.4 运算符 trait | 新增 `AddAssign`/`SubAssign`；位运算 trait（`BitAnd` 等，`int` 内置实现） |
| §3.9 EBNF | 应用本文档 §3.4 增量 |
| §4.3.1 dialect 类型 | 新增 `!sloth.tensor<T, R>` |
| §4.3.3 lowering | 新增张量双通道（§5.2） |
| §5.1 GC | 补充「大块无指针数据区免扫描 + mmap 外部根注册」约定 |
| 风险表 | 新增 #8 张量数据区与保守 GC 交互（见 §8.3） |

---

## 8. 验收与测试策略

### 8.1 功能验收（对标 run.c）

1. **数值对齐测试**：复刻 llama2.c `test_all.py` 策略——加载 stories260K（约 2MB，最小 checkpoint），sloth2 与 run.c 用相同 seed 各跑 200 步 greedy 采样，**token 序列完全一致**（greedy 下确定性）；temperature 采样模式下比对同 seed xorshift 的输出 token 序列；
2. **逐层数值容差**：fp32（run.c）vs f64（sloth2）的累积误差，逐算子比对中间激活，相对误差 < 1e-4；
3. **模型规模覆盖**：stories260K / 15M / 42M / 110M 四个 checkpoint 全部可加载并生成连贯文本；多轮 chat 模式（`-m chat`）的 BOS/EOS 状态机行为一致；
4. **边界用例**：`pos = seq_len - 1`（KV cache 写满）、GQA（n_kv_heads < n_heads，110M 以外的多查询变体）、shared/unshared 权重两种 checkpoint。

### 8.2 性能验收

- 基准机固定，`run.c -O3` 单线程为 1.0x 基线；sloth2 目标 **≥ 0.5x**（同一数量级内），matvec 内核单独 benchmark ≥ 0.7x；
- 融合开关对照：rmsnorm/softmax/SwiGLU 融合前后加速比 ≥ 1.3x（验证 linalg fusion 生效）。

### 8.3 风险

| # | 风险 | 等级 | 对策 |
| --- | --- | --- | --- |
| 1 | bufferization 引入隐形拷贝，性能崩塌 | 高 | 双通道设计（§5.3.1）；对 KV cache 路径做 memref 直降的专项回归测试 |
| 2 | 保守式 Boehm GC 误把 float 数据区当指针保留 | 中 | 数据区用 `GC_malloc_atomic`（无指针区）分配；mmap 区显式注册外部根 |
| 3 | f64 vs fp32 的精度差异导致采样分叉 | 低 | greedy 路径确定性对齐；采样路径用同 seed 对齐而非逐 float 对齐 |
| 4 | rank 作为整数字面量类型参数的泛型扩展超范围 | 中 | 严格限制为整数字面量、仅 `Tensor` 使用；类型检查器中单独通道处理，不侵入通用单态化 |
| 5 | `linalg.vecmat` 向量化质量不及手写 `-O3` | 中 | 提供 `--target-cpu` 编译开关；必要时 matvec 单算子回落到手工优化运行时内核 |

---

## 9. 实施路线图（叠加在主文档 P0–P6 之后）

| 阶段 | 内容 | 验收 |
| --- | --- | --- |
| **TE-P0** | 整数位运算、复合赋值（主文档层先行项） | 位运算/复合赋值单测通过 |
| **TE-P1** | `Tensor<T, R>` 类型 + header/数据区运行时 + GC 免扫描集成；构造/索引/切片/视图 | 张量创建、视图读写、越界 panic 单测 |
| **TE-P2** | `tensor`/`linalg` lowering 通道 + bufferization；matvec/elementwise/reduce 三族算子 | matvec benchmark 达到基线 0.7x |
| **TE-P3** | 融合 pass（rmsnorm/softmax/SwiGLU/RoPE）；`fs.mmap` 与 checkpoint 加载 | stories260K 加载成功；融合加速比达标 |
| **TE-P4** | `tensor.slt`/`random.slt` 标准库；llama.slt 完整推理 + tokenizer + sampler | §8.1 全部用例通过；stories15M 生成连贯文本 |

里程碑：TE-P4 完成即达成本文档验收目标——sloth2 单机 fp32 完整跑通 llama2.c 推理，从 checkpoint 加载到文本生成无外部依赖（除 libm/libgc/系统 mmap）。

---

## 附录 A：run.c 与 sloth2 实现的结构对照

| run.c | sloth2 | 说明 |
| --- | --- | --- |
| `Config`/`TransformerWeights`/`RunState` 裸指针结构体 | `Config`/`Weights`/`RunState` 类（`let` 字段） | 指针算术切分权重 → 类型安全的 `view_as_f32` + `reshape` |
| `malloc_run_state` calloc 缓冲 | `tensor.zeros` | GC 托管，无显式 free |
| `forward()` 450 行 C | §6 参考实现 | 视图原地写保留 KV cache 零拷贝特性 |
| `#pragma omp parallel for` | 无（单线程，3.0 再评估） | 首版对标 `make run` |
| `Sampler` + xorshift | `random.slt` + `sample()` | 算法逐行对应，便于数值对齐 |
| `Tokenizer` + `qsort`/`bsearch` | `Array.sort_by` / `Map` | 标准库能力 |