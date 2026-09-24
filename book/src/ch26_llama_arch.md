# 26. Llama 模型架构与算子详解

第 25 章从**语言扩展**的角度讲了 sloth2 如何用 `Tensor` 类型、`tensor.*` 内建和
`linalg` 通道跑通 llama2.c。本章反过来，从**模型本身**出发，说明 Llama 架构由
哪些算子组成、每个算子的数学定义是什么，以及这些公式在 sloth2 中落到哪段
sloth 代码、哪条 MLIR / 运行时实现。

参考实现以 llama2.c 的 `run.c` 与 `examples/llama/llama.slt` 为准：两者对同一
checkpoint 的 greedy 输出逐字节一致（见 §25.10），因此本章的每个公式都能在
`examples/llama/llama.slt` 里找到一一对应的代码。

## 26.1 架构总览

Llama / Llama-2 是**解码器-only 的 Transformer**（decoder-only），由 `n_layers`
个结构相同的块堆叠而成，每块包含一个**自注意力**子层和一个**前馈（FFN）**子层，
两者都套在**预归一化 + 残差**结构里。与原始 Transformer 的关键差异：

| 设计 | Llama 选择 | 原始 Transformer |
| --- | --- | --- |
| 归一化 | **RMSNorm**，且为 **pre-norm**（归一化在子层入口） | LayerNorm + post-norm |
| 位置编码 | **RoPE**，作用在 Q/K 上，随位置旋转 | 正弦绝对编码 / 可学习编码 |
| 注意力 | **GQA**（`n_kv_heads ≤ n_heads`，多 Q 头共享一组 K/V） | MHA（头数相同） |
| FFN 激活 | **SwiGLU**（门控） | ReLU / GELU |
| 偏置 | 全部 `bias = false` | 多数线性层带 bias |
| 输出头 | 常与 embedding **权重共享**（tied） | 独立线性层 |

配置（`Config`，对应 checkpoint 头部的 7 个 `i32`）：

```text
dim          d       隐藏维（残差流宽度）
hidden_dim   h       FFN 中间维（门控支路宽度）
n_layers     L       层数
n_heads      H       Q 头数
n_kv_heads   H_kv    K/V 头数（GQA，H_kv ≤ H）
vocab_size   V       词表大小
seq_len      S       最大上下文长度
```

由配置派生的量（对应源码里的 `head_size` / `kv_dim` / `kv_mul`）：

$$
d_h = \frac{d}{H}, \qquad
d_{kv} = \frac{d\\,H_{kv}}{H}, \qquad
r = \frac{H}{H_{kv}}
$$

单 token、单层的计算图（`pos` 为当前位置）：

```text
                    ┌─────────────────────────────┐
 token ──embed──►   x ──rms_att──► xb ──wq/wk/wv──► q,k,v
                    │                              │
                    │                         RoPE(q,k,pos)
                    │                              │
                    │                    写 KV cache[l][pos]
                    │                              │
                    │        ┌── 多头注意力（GQA, softmax）──┐
                    │        ▼                              │
                    x ◄── + ── wo·attn ─────────────────────┘   (残差 1)
                    │
                    x ──rms_ffn──► xbf ──w1,w3──► silu(h1)*h3 ──w2──► ff
                    x ◄── + ◄──────────────────────────────────────     (残差 2)
```

整个模型（`forward`），第 `l` 层、位置 `p`：

$$
\begin{aligned}
x_0 &= E[t] \\\\
a_l &= \mathrm{Attn}\\!\left(\mathrm{RMSNorm}(x_{l-1};\\, g_{\text{att}}^{(l)});\\, p\right) \\\\
u_l &= x_{l-1} + W_o^{(l)}\\, a_l \\\\
w_l &= \mathrm{RMSNorm}(u_l;\\, g_{\text{ffn}}^{(l)}) \\\\
x_l &= u_l + W_2^{(l)}\\!\left(\mathrm{SiLU}(W_1^{(l)} w_l) \odot W_3^{(l)} w_l\right)
\end{aligned}
$$

$$
\mathrm{logits} = W_{\text{cls}}\\,\mathrm{RMSNorm}(x_L;\\, g_{\text{final}})
$$

下面逐个算子展开。约定：\\( \odot \\) 为逐元素乘，\\( \cdot \\) 为矩阵/向量乘，
向量默认列向量，权重按行主序存储；`[n]` 表示长度为 `n` 的向量。

## 26.2 通用约定：权重布局与精度

**权重平面。** checkpoint 在文件头 `28` 字节之后是一整块连续的 `f32` 权重。
`Weights.__init__`（`llama.slt:90`）按 `run.c` 的顺序把这块平面切成若干段，只记录
**元素偏移**，真正取用时再用 reshape 视图切出张量：

```text
off 0            token_embedding : (V, d)           E
off_att          rms_att_weight  : (L, d)           g_att(l)
off_wq           wq              : (L, d, d)        W_q(l)
off_wk           wk              : (L, d, kv_dim)   W_k(l)
off_wv           wv              : (L, d, kv_dim)   W_v(l)
off_wo           wo              : (L, d, d)        W_o(l)
off_ffn          rms_ffn_weight  : (L, d)           g_ffn(l)
off_w1           w1              : (L, h, d)        W_1(l)
off_w2           w2              : (L, d, h)        W_2(l)
off_w3           w3              : (L, h, d)        W_3(l)
off_final        rms_final_weight: (d)              g_final
                 freq_cis        : (S, head_size/2) × 2  （旧版遗留，跳过）
off_wcls         wcls            : (V, d)，tied 时复用 off 0
```

**MatVec 的 `run.c` 约定。** `run.c` 的 `matmul(xout, x, w, n, d)` 把参数 `w` 当作
**`d` 行 `n` 列**、`x` 是长度 `n` 的列向量：

$$
W \in \mathbb{R}^{d \times n},\quad x \in \mathbb{R}^{n}
\\;\Longrightarrow\\;
Wx \in \mathbb{R}^{d}
$$

因此 `wk`/`wv` 是 `(kv_dim, d)`、`w1`/`w3` 是 `(h, d)`、`w2` 是 `(d, h)`——转置关系
在 `Weights.wk` 等访问器里已经吸收（`llama.slt:153`）。

**精度。** checkpoint 是 `f32`，但 sloth2 的 `Tensor<float, R>` 元素是 `f64`
编码。`load_weights` 用 `view_as_f32`（→ `__sloth_tensor_from_f32_ptr`）**一次性加宽**
整块权重为 `f64` 张量（`tensors.rs:516`），此后全部算术在 `f64` 满精度下进行。

## 26.3 算子一：Token Embedding（查表）

**数学。** 设嵌入矩阵 \\( E \in \mathbb{R}^{V \times d} \\)，输入 token id 为
\\( t \\)，则

$$
x = E[t], \qquad x \in \mathbb{R}^{d}
$$

这是一个纯查表操作，没有算术。

**sloth 实现。** `forward` 的第一行把第 `t` 行**拷贝**进残差流：

```sloth
s.x[0..dim] = w.token_embedding_table()[token];
```

`token_embedding_table()` 是 `matrix_view(data, 0, V, d)`，即把扁平权重视为
`(V, d)`；`[token]` 生成降 rank 的 rank-1 视图（零拷贝）；左侧 `s.x[0..dim]` 是保
rank 切片视图。赋值落成 `__sloth_tensor_copy_into`，把这一行元素拷进 `x` 的缓冲区
（`tensors.rs:248`），MLIR 见 §25.9.1。之所以是拷贝而非别名，是因为 `x` 后面要作为
残差流被原地累加，不能与只读权重共享存储。

## 26.4 算子二：RMSNorm

**数学。** Root-Mean-Square Normalization 对输入 \\( x \in \mathbb{R}^{d} \\)、
可学习缩放 \\( g \in \mathbb{R}^{d} \\) 计算

$$
y_i = g_i \cdot \frac{x_i}{\sqrt{\dfrac{1}{d}\displaystyle\sum_{j=1}^{d} x_j^2 + \varepsilon}}
$$

其中 \\( \varepsilon = 10^{-5} \\)。与 LayerNorm 的差别是**不减均值**：Llama 只按
均方根做缩放，因此省掉一次均值归约，在大模型里更便宜。注意缩放 \\( g_i \\) 是对
**归一化后**的值逐元素相乘。

**sloth 实现。** 内建 `tensor.rmsnorm(x, w)` 由发射器 `emit_tensor_rmsnorm` 展开
（`irgen/tensor.rs:1122`），分两步：

1. **平方和归约**：用一个 `scf.for` 携带**寄存器累加器**（不分配中间张量、不在循环
   内 `memref.alloca`）：

   ```mlir
   %ss = scf.for %i = 0 to %n step 1 iter_args(%acc = 0.0) -> (f64) {
     %xi = memref.load %x[%i] : memref<?xf64, ...>
     %sq = arith.mulf %xi, %xi : f64
     %s  = arith.addf %acc, %sq : f64
     scf.yield %s : f64
   }
   ```

2. **标量 epilogue + 融合缩放**：先算出

   $$
   \mu_2 = \frac{1}{n}\sum_{i=0}^{n-1} x_i^2, \qquad
   \mathrm{inv} = \frac{1}{\sqrt{\mu_2 + \varepsilon}}, \qquad
   y_i = x_i \cdot \mathrm{inv} \cdot w_i
   $$

   再用一个 `linalg.generic` 把 `x * inv * w` 融成单趟逐元素写（`irgen/tensor.rs:1189`）：

   ```mlir
   linalg.generic {indexing_maps = [...], iterator_types = ["parallel"]}
     ins(%x, %w) outs(%out) {
   ^bb0(%x: f64, %w: f64, %o: f64):
     %t = arith.mulf %x, %inv : f64
     %r = arith.mulf %t, %w  : f64
     linalg.yield %r : f64
   }
   ```

   同一个 \\( \mathrm{inv} \\) 是循环外算好的标量，被 `linalg.generic` 捕获为常量，
   故整个 RMSNorm 只有一次读、一次写。

**调用点。** `forward` 里有三处：注意力入口 `tensor.rmsnorm(s.x, w.rms_att(l))`、
FFN 入口 `tensor.rmsnorm(s.x, w.rms_ffn(l))`、最终 `tensor.rmsnorm(s.x, w.rms_final())`。

## 26.5 算子三：线性投影 MatVec

**数学。** 给定 \\( W \in \mathbb{R}^{m \times n} \\)、\\( x \in \mathbb{R}^{n} \\)：

$$
y_i = \sum_{j=1}^{n} W_{ij}\\, x_j, \qquad i = 1,\dots,m
$$

即 \\( y = Wx \\)。在 Llama 里同一个算子用于 Q/K/V/O 投影、FFN 的 `w1/w3/w2`、以及
分类头。矩阵元素数是全模型的热点（\\( O(mn) \\) 访存受限）。

**sloth 实现。** 内建 `tensor.matvec(w: Tensor<float,2>, x: Tensor<float,1>)` 走
**通道 B**（`irgen/tensor.rs:475`）：

1. 运行期断言 `dim(w,1) == dim(x,0)`（`__sloth_tensor_dim_eq`）；
2. 用 `__sloth_tensor_basis_f64` 把张量句柄变成 rank-1 memref 描述符；
3. 用 `memref.reinterpret_cast` 带上**运行期**的 size/stride 变成 `memref<?x?xf64>`；
4. 发 `linalg.matvec`，由 LLVM `-O3` 完成向量化/分块。

```mlir
%wm = memref.reinterpret_cast %wb to offset: [0],
        sizes: [%dw0, %dw1], strides: [%sw0, %sw1] : ... to memref<?x?xf64, ...>
linalg.matvec ins(%wm, %xm : ...) outs(%ym : ...)
```

完整 MLIR 见 §25.9.2。之所以用运行期 size/stride：权重是从扁平平面切出的 reshape
视图，层偏移在编译期未知；`reinterpret_cast` 正好把「编译期未知、运行期已知」的
形状交给 `linalg`。同形逐元素算子 `add/sub/mul/div`、`matmul` 走同一套通道。

## 26.6 算子四：RoPE（旋转位置编码）

**数学。** RoPE 通过**按位置旋转** Q/K 的每一对分量注入相对位置信息。对位置
\\( p \\)、头内第 \\( j \\) 对分量 \\( (x_{2j}, x_{2j+1}) \\)：

$$
\theta_j = p \cdot \mathrm{base}^{\\,-2j/d_h}, \qquad
\mathrm{base} = 10^4, \qquad j = 0,\dots,\frac{d_h}{2}-1
$$

$$
\begin{aligned}
x_{2j}^{\prime} &= x_{2j}\cos\theta_j - x_{2j+1}\sin\theta_j \\\\
x_{2j+1}^{\prime} &= x_{2j}\sin\theta_j + x_{2j+1}\cos\theta_j
\end{aligned}
$$

写成旋转矩阵即

$$
\begin{aligned}
\begin{pmatrix} x_{2j}^{\prime} \\\\ x_{2j+1}^{\prime} \end{pmatrix}
&= \begin{pmatrix} \cos\theta_j & -\sin\theta_j \\\\ \sin\theta_j & \cos\theta_j \end{pmatrix}
\begin{pmatrix} x_{2j} \\\\ x_{2j+1} \end{pmatrix}
\end{aligned}
$$

旋转的几何性质使 \\( q_m \\) 与 \\( k_n \\) 的内积只依赖相对位移 \\( m-n \\)，这正是
「相对位置」的来源。**只有 Q 和 K 旋转，V 与输出投影不旋转。**

**与 HF Llama 的细节差异。** HuggingFace Llama 把 \\( d_h \\) 维向量对半拆成
\\( (x_j, x_{j+d_h/2}) \\) 旋转；llama2.c 则按**相邻对** \\( (x_{2j}, x_{2j+1}) \\)
旋转。两者是同一旋转矩阵的**通道重排**，对训练好的权重等价。

**sloth 实现。** 没有走 `linalg`，而是一段纯 sloth 标量循环（`llama.slt:226`），
逐对就地旋转：

```sloth
func rope(q, k, pos, head_size, kv_dim, dim): unit {
    var i = 0;
    while i < dim {
        let head_dim = i % head_size;
        let freq = 1.0 / float_pow(10000.0, float(head_dim) / float(head_size));
        let val  = float(pos) * freq;
        let fcr  = float_cos(val);
        let fci  = float_sin(val);
        let v0 = q[i];
        let v1 = q[i + 1];
        q[i]     = v0 * fcr - v1 * fci;
        q[i + 1] = v0 * fci + v1 * fcr;
        if i < kv_dim {            // K 只旋转前 kv_dim（GQA 每头一次）
            let k0 = k[i];
            let k1 = k[i + 1];
            k[i]     = k0 * fcr - k1 * fci;
            k[i + 1] = k0 * fci + k1 * fcr;
        }
        i = i + 2;
    }
}
```

这里 `head_dim = i % head_size` 且 `i` 步长 2，故
\\( \mathrm{freq} = \mathrm{base}^{-2j/d_h} \\)，与上面的公式一致。
`float_pow/cos/sin` 是标量 libm 内建（`math.rs`）。由于每步只有相邻依赖、无归约，
标量循环对编译器完全友好，且直接复用 `float_pow` 而不必为索引敏感的旋转写
`linalg.generic`。

## 26.7 算子五：缩放点积注意力（含 KV cache 与 GQA）

**数学。** 对第 \\( l \\) 层、第 \\( h \\) 个 Q 头，查询 \\( q_h \in \mathbb{R}^{d_h} \\)，
历史键值 \\( \lbrace k_t, v_t \rbrace \\)（\\( t = 0,\dots,p \\)）：

$$
s_t = \frac{q_h \cdot k_t}{\sqrt{d_h}}, \qquad
\alpha_t = \frac{e^{s_t}}{\displaystyle\sum_{j=0}^{p} e^{s_j}}, \qquad
o_h = \sum_{t=0}^{p} \alpha_t\\, v_t
$$

\\( 1/\sqrt{d_h} \\) 是缩放因子：点积随维数增大而方差增大，缩放使 softmax 的输入
保持稳定。**因果掩码**不需要显式实现——KV cache 里只有 \\( t \le p \\) 的键，未来的
键尚未写入，因此天然只看历史。

**GQA（Grouped-Query Attention）。** 设 \\( H \\) 个 Q 头、\\( H_{kv} \\) 个 KV 头，
\\( r = H / H_{kv} \\)。第 \\( h \\) 个 Q 头映射到 KV 头 \\( g(h) \\)：

$$
g(h) = \left\lfloor \frac{h}{r} \right\rfloor
$$

即相邻 \\( r \\) 个 Q 头共享同一组 K/V。\\( H_{kv} = H \\) 退化为 MHA，\\( H_{kv} = 1 \\)
为 MQA。GQA 在不显著损失质量的前提下把 KV cache 与 K/V 投影缩小到 \\( H_{kv}/H \\)。

**KV cache。** 自回归解码每步只前进一步，若每步重算历史 K/V 则复杂度
\\( O(p^2) \\)；缓存把每层每位置的 K/V 存下来，使单步注意力为 \\( O(p) \\)、整段
生成为 \\( O(p^2) \\) 但每步只需一次投影。cache 形状 \\( (L, S, d_{kv}) \\)。写入是
**零拷贝视图赋值**：

```sloth
s.key_cache[l][pos] = kt;      // 降 rank 视图 + copy_into
s.value_cache[l][pos] = vt;
```

**sloth 实现。** 一个 Q 头一轮（`llama.slt:269`）：

```sloth
var h = 0;
while h < cfg.n_heads {
    let kh = h / kv_mul;
    let qh = q[h * head_size .. (h + 1) * head_size];
    let att_h = s.att[h];
    var t = 0;
    while t <= pos {
        let k_t = s.key_cache[l][t][kh * head_size .. (kh + 1) * head_size];
        att_h[t] = tensor.dot(qh, k_t) / float_sqrt(float(head_size));
        t = t + 1;
    }
    tensor.softmax_into(att_h[0..=pos]);          // 动态长度子段原地 softmax
    let xb_h = xb[h * head_size .. (h + 1) * head_size];
    tensor.fill_zero(xb_h);
    t = 0;
    while t <= pos {
        let v_t = s.value_cache[l][t][kh * head_size .. (kh + 1) * head_size];
        tensor.add_scaled_into(xb_h, v_t, att_h[t]);   // xb_h += α_t · v_t
        t = t + 1;
    }
    h = h + 1;
}
```

逐算子对应：

| 公式步骤 | sloth 算子 | 实现 |
| --- | --- | --- |
| \\( q_h \cdot k_t \\) | `tensor.dot(qh, k_t)` | `scf.for` 寄存器累加器 |
| \\( /\sqrt{d_h} \\) | `float_sqrt` + `/` | 标量 libm |
| \\( \alpha_t = \mathrm{softmax}(s_0..s_p) \\) | `tensor.softmax_into(att_h[0..=pos])` | 三段 `scf.for`（§26.8） |
| \\( \sum_t \alpha_t v_t \\) | `tensor.add_scaled_into(xb_h, v_t, α_t)` | 融合 `linalg.generic` |
| KV cache 写 | 视图赋值 | `__sloth_tensor_copy_into` |

`att_h = s.att[h]` 是长度为 `S` 的暂存行，`att_h[0..=pos]` 只把**当前有效长度**
交给 softmax，因此每步的 softmax 是定长 `pos+1` 的子段；`fill_zero(xb_h)` 清掉上一
token 的注意力输出后，再按步累加。`add_scaled_into` 的循环体是
\\( \mathrm{dst} \mathrel{+}= \mathrm{scale} \cdot \mathrm{src} \\)
（`irgen/tensor.rs:1301`）：

```mlir
linalg.generic {iterator_types = ["parallel"]} ins(%src) outs(%dst) {
^bb0(%s: f64, %d: f64):
  %m = arith.mulf %scale, %s : f64
  %r = arith.addf %d, %m : f64
  linalg.yield %r : f64
}
```

## 26.8 算子六：Softmax

**数学。** 对向量 \\( z \in \mathbb{R}^{n} \\)：

$$
p_i = \frac{\exp(z_i)}{\displaystyle\sum_{j=1}^{n}\exp(z_j)}
$$

直接实现会在 \\( |z_i| \\) 较大时上溢。数值稳定版本先减去最大值：

$$
m = \max_j z_j, \qquad
p_i = \frac{\exp(z_i - m)}{\displaystyle\sum_{j=1}^{n}\exp(z_j - m)}
$$

因为 softmax 对输入平移不变（分子分母同乘 \\( e^{-m} \\)），结果不变，但指数自变量
\\( \le 0 \\)，永不上溢、下溢为 0 也无妨（分母至少有一个 1）。

**sloth 实现。** 内建 `tensor.softmax(x)` / `tensor.softmax_into(x)` 展开为**三趟**
`scf.for`（`irgen/tensor.rs:1197`）：

```text
pass 1:  m    = max_i x[i]                 // 初值 f64::MIN（有限哨兵）
pass 2:  out[i] = exp(x[i] − m)            // 同时 Σ 累加得到分母
pass 3:  out[i] = out[i] / Σ
```

第二、三趟直接在输出缓冲上原地读写；`softmax_into` 时输出就是输入，省掉一次分配。
注意力里作用在 `att_h[0..=pos]` 这个**运行期长度**的子段上——发射器用
`__sloth_tensor_dim` 取运行期长度，因此支持动态长度归约。

## 26.9 算子七：SwiGLU 前馈网络

**数学。** Llama 的 FFN 是门控的 SwiGLU。设
\\( W_1, W_3 \in \mathbb{R}^{h \times d} \\)、\\( W_2 \in \mathbb{R}^{d \times h} \\)，
输入 \\( x \in \mathbb{R}^{d} \\)：

$$
h_1 = W_1 x, \qquad h_3 = W_3 x, \qquad
\mathrm{FFN}(x) = W_2\big(\mathrm{SiLU}(h_1) \odot h_3\big)
$$

其中 SiLU（又称 swish）：

$$
\mathrm{SiLU}(z) = z\\,\sigma(z) = \frac{z}{1 + e^{-z}}
$$

门控让网络可以按输入选择性地放行 \\( h_3 \\) 的通道。Llama 通常把 \\( h \\) 取成
\\( 8d/3 \\) 附近（本 checkpoint 直接写在配置里）。

**sloth 实现。** 三段：

```sloth
let xbf = tensor.rmsnorm(s.x, w.rms_ffn(l));
let hb  = tensor.matvec(w.w1(l), xbf);     // h_1
let hb2 = tensor.matvec(w.w3(l), xbf);     // h_3
tensor.silu_mul_into(hb, hb2);             // hb = silu(hb) * hb2
tensor.add_into(s.x, tensor.matvec(w.w2(l), hb));
```

`silu_mul_into` 在一个 `linalg.generic` 里融合完成 `silu(h1) * h3`，SiLU 的
`negf / exp / addf / divf` 直接内联进逐元素体（`irgen/tensor.rs:1053`）：

```mlir
^bb0(%a: f64, %b: f64, %o: f64):
  %neg = arith.negf %a : f64
  %e   = math.exp %neg : f64
  %den = arith.addf %c1, %e : f64
  %s   = arith.divf %a, %den : f64
  %r   = arith.mulf %s, %b : f64
  linalg.yield %r : f64
```

`math.exp` 经 `convert-math-to-llvm` 落成 LLVM `exp` intrinsic。

## 26.10 算子八：残差连接与输出投影

**数学。** 预归一化 + 残差的两个子层：

$$
x \leftarrow x + W_o\\,\mathrm{Attn}\big(\mathrm{RMSNorm}(x;\\, g_{\text{att}})\big)
$$

$$
x \leftarrow x + \mathrm{FFN}\big(\mathrm{RMSNorm}(x;\\, g_{\text{ffn}})\big)
$$

残差把梯度的恒等通路留给深层网络，也让残差流 \\( x \\) 成为贯穿全模型的**唯一主
状态**。

**sloth 实现。** 输出投影是一次 `matvec`，残差是**原地** `add_into`：

```sloth
let xb2 = tensor.matvec(w.wo(l), xb);
tensor.add_into(s.x, xb2);                 // s.x += W_o · attn
...
let xb3 = tensor.matvec(w.w2(l), hb);
tensor.add_into(s.x, xb3);                 // s.x += W_2 · swiglu
```

`add_into(dst, src)` 是原地 `linalg.generic`（\\( \mathrm{dst} = \mathrm{dst} + \mathrm{src} \\)，
见 §25.9.3），不分配新张量。注意 `xb` 既是注意力算子的输出、又被 Q/K/V/O 投影复用，
残差累加放在最后，避免污染后续投影的输入。

## 26.11 算子九：最终归一化与分类头

**数学。** 最后一层之后：

$$
x \leftarrow \mathrm{RMSNorm}(x;\\, g_{\text{final}}), \qquad
\mathrm{logits} = W_{\text{cls}}\\, x
$$

\\( W_{\text{cls}} \in \mathbb{R}^{V \times d} \\) 是分类头。若 checkpoint 头部
`vocab_size` 为负，表示 **tied 权重**（`config_shared` 返回真），此时 \\( W_{\text{cls}} \\)
直接复用 \\( E \\)（偏移 0）；否则读文件末尾的独立分类头（`off_wcls`）。这就是
`load_config` 里对负值取相反数的含义（`llama.slt:44`）。

**sloth 实现。**

```sloth
let xf = tensor.rmsnorm(s.x, w.rms_final());
s.logits = tensor.matvec(w.wcls(), xf);
```

`logits` 是 `RunState` 里预分配的 `(V,)` 张量，采样器直接原地消费它。

## 26.12 从 logits 到 token：采样

前向得到 \\( \mathrm{logits} \in \mathbb{R}^{V} \\) 后，采样器把它变成下一个 token。
`sample` 与 `run.c` 的四种模式一一对应（`llama.slt:395`）。

**Greedy（温度 0）。** 取 argmax：

$$
\hat{t} = \arg\max_i z_i, \qquad z = \mathrm{logits}
$$

```sloth
if temperature == 0.0 { return argmax(logits, cfg.vocab_size); }
```

**Temperature + Softmax。** 温度 \\( T > 0 \\) 先缩放再 softmax：

$$
p_i = \frac{\exp(z_i / T)}{\displaystyle\sum_{j} \exp(z_j / T)}, \qquad z = \mathrm{logits}
$$

```sloth
tensor.div_scalar_into(logits, temperature);
tensor.softmax_into(logits);
```

\\( T \to 0 \\) 趋近 one-hot（greedy），\\( T \to \infty \\) 趋近均匀分布。

**Multinomial。** 用逆变换采样：取 \\( \mathrm{coin} \sim U(0,1) \\)，沿累积分布找
第一个 \\( c_i = \sum_{j \le i} p_j > \mathrm{coin} \\) 的 \\( i \\)：

```sloth
func sample_mult(x, n, coin): int {
    var cdf = 0.0; var i = 0;
    while i < n { cdf = cdf + x[i]; if coin < cdf { return i; } i = i + 1; }
    return n - 1;
}
```

**Top-p（nucleus）。** `sample_topp` 保留累积概率达 `topp` 的最小集合：先按
\\( \mathrm{cutoff} = (1-\mathrm{topp})/(n-1) \\) 裁剪低概率项，再手写插入排序
**降序**，累计到超过 `topp` 截断，最后在截断集合上按 CDF 用
\\( \mathrm{coin}\cdot\mathrm{cumulative} \\) 采样（`llama.slt:340`）。随机数来自
`XorShift`（`lib/sloth/random.slt`），greedy 模式不消耗随机数，因此逐 token 对齐；
温度采样因 RNG 实现不同暂未逐位对齐（§25.8）。

## 26.13 内存复用与前向不变量

`RunState` 一次性分配所有跨层复用的缓冲（`llama.slt:210`），`forward` 全程**零分配**：

| 张量 | 形状 | 用途 |
| --- | --- | --- |
| `x` | `(d)` | 残差流，贯穿全前向 |
| `att` | `(H, S)` | 每头注意力分数暂存 |
| `logits` | `(V)` | 分类头输出，采样器原地消费 |
| `key_cache` | `(L, S, kv_dim)` | KV cache 的 K |
| `value_cache` | `(L, S, kv_dim)` | KV cache 的 V |

三条不变量保证这一点：

1. **权重只读共享**：`Weights.data` 是一整块加宽后的 `f64` 张量，所有 `wq(l)`/`wk(l)`/…
   都是它的 reshape 视图，只读、不拷贝。
2. **KV cache 就地写**：`key_cache[l][pos] = kt` 是视图赋值 → `copy_into`，写进
   cache 缓冲；`__sloth_tensor_view` 只改 shape/stride/data 指针并 retain 父句柄，不拷贝。
3. **累加与归一化就地**：`add_into` / `add_scaled_into` / `softmax_into` /
   `silu_mul_into` / `fill_zero` / `div_scalar_into` 全部原地，避免每 token 的中间分配。

## 26.14 算子—公式—实现总表

| # | 算子 | 数学 | sloth 调用 | 实现 |
| --- | --- | --- | --- | --- |
| 1 | Embedding | \\( x = E[t] \\) | `x[0..d] = E_table()[t]` | 视图 + `copy_into` |
| 2 | RMSNorm | \\( g_i x_i / \sqrt{\mu_2 + \varepsilon} \\) | `tensor.rmsnorm` | `scf.for` 归约 + 融合 generic |
| 3 | 线性投影 | \\( y = Wx \\) | `tensor.matvec` | `linalg.matvec` + `reinterpret_cast` |
| 4 | RoPE | \\( (x_{2j}, x_{2j+1}) \mapsto R(\theta_j)(x_{2j}, x_{2j+1}) \\) | `rope(...)` | 纯 sloth 标量循环 + libm |
| 5 | 注意力打分 | \\( q_h \cdot k_t / \sqrt{d_h} \\) | `tensor.dot` + `float_sqrt` | `scf.for` 寄存器累加 |
| 6 | Softmax | \\( e^{z_i-m} / \sum_j e^{z_j-m} \\) | `tensor.softmax_into` | 三趟 `scf.for`（max/exp+sum/div） |
| 7 | 加权和 | \\( \sum_t \alpha_t v_t \\) | `tensor.add_scaled_into` | 融合 `linalg.generic` |
| 8 | SiLU | \\( z/(1+e^{-z}) \\) | `tensor.silu` / `silu_mul_into` | 融合 `linalg.generic` + `math.exp` |
| 9 | SwiGLU | \\( W_2(\mathrm{SiLU}(W_1 x) \odot W_3 x) \\) | `matvec`×3 + `silu_mul_into` | 同上 |
| 10 | 残差 | \\( x \leftarrow x + \mathrm{sublayer}(x) \\) | `tensor.add_into` | 原地 `linalg.generic` |
| 11 | 分类头 | \\( \mathrm{logits} = W_{\text{cls}} x \\) | `tensor.matvec` | `linalg.matvec` |
| 12 | 采样 | \\( \arg\max \\) / softmax / CDF | `argmax`/`sample_mult`/`sample_topp` | 纯 sloth |

公式与实现的逐条对应关系，就是 `forward` 能被当作 `run.c` 的等价移植来阅读的原因：
每个数学步骤都落在具名算子（`rmsnorm`/`matvec`/`dot`/`softmax_into`/`silu_mul_into`/
`add_scaled_into`/`add_into`）上，而每个算子又只对应一条可核对的 MLIR 或运行时
实现。端到端正确性与性能验收见 §25.10。
