# 25. 张量扩展与 llama2.c 推理

本章对应张量扩展专项（TE-P4）的验收目标：**在 sloth2 上从 checkpoint 加载到
文本生成，单机跑通 llama2.c `run.c` 推理**。llama2.c 是一个纯 C 的 Llama-2
推理实现，本章说明 sloth2 的 `Tensor` 类型、`tensor.*`/`math` 内建、张量标准
库，以及 `lib/sloth/llama.slt` 对 `run.c` 的移植。

## 25.1 总览

| 层 | 位置 | 作用 |
| --- | --- | --- |
| 语言类型 | `Ty::Tensor(TyId, u32)` | `Tensor<float, R>` / `Tensor<int, R>`，rank 编译期常量 |
| 编译器内建 | 发射器识别 `tensor.*` / `float_*` 调用 | 直接发 `linalg`/`scf`/`math` IR |
| 运行时 | `crates/sloth-rt/src/{tensors,mmap,strings,math}.rs` | 张量描述符、mmap、字符串面、标量 libm |
| 标准库 | `lib/sloth/{tensor,random,fs,tokenizer,llama}.slt` | 纯 sloth 的包装与推理实现 |
| 用例 | `examples/llama/` | tiny 差分 + stories42M 生成 |

`tensor.*` 不是普通函数，而是**调用点识别的内建**（与 `print`/`len` 同类）。
运算符写在 `.slt` 里，需要 `linalg` lowering 的核由发射器生成 IR。

## 25.2 `Tensor<T, R>` 与两条 lowering 通道

`Tensor<T, R>` 的元素 `T` 仅允许 `float`/`int`，rank `R` 支持 1..=3。在槽
位 / 形参 / 返回 / 字段里它仍是**一个 tagged rc 词**（与 `Array` 同构），负载
为 7 词：

```
[flags, ndim, shape_ptr, stride_ptr, data_ptr, owner, total]
```

- `flags` bit0 = 元素种类（0=int，1=float），bit1 = 是否视图；
- `shape_ptr`/`stride_ptr` 是 `malloc` 的 `i64[ndim]`；行主序，8 字节元素；
- `data_ptr` 是当前张量的元素基址，视图指进父张量缓冲（**写视图即写父张量**）；
- `owner` 是视图持有并 `retain` 的父句柄（`0` = 自持数据缓冲）。

因为元素是 `float`（f64 编码）而 checkpoint 是 f32，加载时**一次性加宽**为
f64 张量（见 §25.8）。

计算 lowering 有两条通道：

- **通道 A（值语义）**：`linalg.generic`/`linalg.matvec`/`linalg.matmul` 等，
  经 `one-shot-bufferize` 落 `memref`；同表达式内的元素级算子由
  `linalg-fuse-elementwise-ops` 融合。
- **通道 B（可变/视图）**：描述符词 → `sloth_tensor_basis_*`（rank-1 memref
  描述符）→ `memref.reinterpret_cast`（运行期 dim/stride 转 `memref<?x…>`）
  → `memref.subview`/`memref.store`，直接落 `memref→llvm`，**不做
  bufferization、不引入拷贝**。KV cache、权重视图、`add_into` 走这条。

## 25.3 `tensor.*` 内建

| 内建 | 语义 |
| --- | --- |
| `tensor.zeros([d0, d1, …])` | 按目标类型分配零张量（rank 1..3） |
| `tensor.from_array(arr, [d0, …])` | 从 `Array<T>` 拷贝构造 |
| `tensor.fill_zero(t)` | 原地清零 |
| `tensor.matvec(w: T2, x: T1) -> T1` | `W @ x`（float，通道 B/linalg） |
| `tensor.matmul(a: T2, b: T2) -> T2` | `A @ B`（float） |
| `tensor.dot(a: T1, b: T1) -> T` | 点积（`scf.for` 寄存器累加） |
| `tensor.sum(t: T1) -> T` | 求和 |
| `tensor.add/sub/mul/div(a, b)` | 同形逐元素（分配新张量） |
| `tensor.add_into(dst, src)` | 原地 `dst += src` |
| `tensor.add_scaled_into(dst, src, a)` | 原地 `dst += a*src`（融合） |
| `tensor.div_scalar_into(dst, s)` | 原地 `dst /= s`（融合） |
| `tensor.rmsnorm(x, w)` | `w * x / sqrt(mean(x^2)+1e-5)`（融合） |
| `tensor.softmax(x)` / `tensor.softmax_into(x)` | 数值稳定 softmax（原地开关） |
| `tensor.exp/sqrt/sin/cos/tan(x)` | 元素级 `math.*` |
| `tensor.silu(x)` / `tensor.silu_mul_into(a, b)` | SwiGLU（后者原地 `a = silu(a)*b`） |

标量面：`float_sqrt/exp/sin/cos/tan/floor/pow`（参数必须 `float`）。索引/切片
`t[i]`（rank>1 降 rank 视图）、`t[a..b]`/`t[a..=b]`（保 rank 视图）由 `Tensor`
类型专属支持。形状不匹配在运行期 panic（`sloth_tensor_shape_eq`/
`sloth_tensor_dim_eq`）。

## 25.4 标准库

```
lib/sloth/
├─ tensor.slt      # normalize/sumsq/l2 + matrix_view/cube_view/flatten_view
├─ random.slt      # XorShift（TE-P0 位运算）
├─ fs.slt          # ByteBuffer + mmap 读取 + 字符串/输出助手
├─ tokenizer.slt   # BPE encode/decode
└─ llama.slt       # Config/Weights/RunState + forward + 采样 + generate
```

`fs.slt` 用 `extern type ByteBuffer` 把 mmap 句柄包装成不透明类型，并暴露
`open_file/size/read_i32/read_u8/read_f32/read_str/view_as_f32`。checkpoint
的权重不能直接零拷贝复用 f32 字节，故 `view_as_f32` 走
`sloth_tensor_from_f32_ptr` **加宽拷贝**；`read_*` 支持变长 tokenizer 条目的
非对齐偏移。

`tensor.slt` 的 `matrix_view`/`cube_view`/`flatten_view` 是
`sloth_tensor_reshape{2,3,1}` 的包装：在一块扁平权重张量上按偏移建立连续
rank-2/3 共享视图，从而把 `(layer, dim, dim)` 权重在某一层切成 `(dim, dim)`
喂给 `matvec`。

## 25.5 `forward`：对 `run.c` 的移植

`Weights` 保存**一整块加宽的权重平面** `data: Tensor<float, 1>` 与各段元素
偏移；`wq(l)`/`wk(l)`/… 用 reshape 视图取层。注意 `run.c` 的
`matmul(xout, x, w, n, d)` 把 `w` 当作 **d 行 n 列**，因此
`wk/wv = (kv_dim, dim)`、`w1/w3 = (hidden_dim, dim)`、`w2 = (dim, hidden_dim)`。

`forward` 逐行对应：

```sloth
pub func forward(cfg: Config, w: Weights, s: RunState, token: int, pos: int): unit {
    // 1. embedding 行拷贝（视图 -> 切片 -> copy_into）
    s.x[0..dim] = w.token_embedding_table()[token];

    var l = 0;
    while l < cfg.n_layers {
        // 2. attention rmsnorm + QKV
        let xb = tensor.rmsnorm(s.x, w.rms_att(l));
        let q = tensor.matvec(w.wq(l), xb);
        let kt = tensor.matvec(w.wk(l), xb);
        let vt = tensor.matvec(w.wv(l), xb);
        // 3. RoPE（q 全量、k 前 kv_dim）
        rope(q, kt, pos, head_size, kv_dim, dim);
        // KV cache 原位写（视图赋值 = copy_into）
        s.key_cache[l][pos] = kt;
        s.value_cache[l][pos] = vt;

        // 4. 多头注意力（GQA：每 kv 头服务 kv_mul 个 q 头）
        var h = 0;
        while h < cfg.n_heads {
            let qh = q[h * head_size .. (h + 1) * head_size];
            let att_h = s.att[h];
            var t = 0;
            while t <= pos {
                let k_t = s.key_cache[l][t][kh * head_size .. (kh + 1) * head_size];
                att_h[t] = tensor.dot(qh, k_t) / float_sqrt(float(head_size));
                t = t + 1;
            }
            tensor.softmax_into(att_h[0..=pos]);   // 动态长度子段原地 softmax
            let xb_h = xb[h * head_size .. (h + 1) * head_size];
            tensor.fill_zero(xb_h);
            t = 0;
            while t <= pos {
                let v_t = s.value_cache[l][t][...];
                tensor.add_scaled_into(xb_h, v_t, att_h[t]);
                t = t + 1;
            }
            h = h + 1;
        }

        // 5. 输出投影 + 残差
        tensor.add_into(s.x, tensor.matvec(w.wo(l), xb));

        // 6. FFN：rmsnorm -> w1/w3 -> SwiGLU -> w2 -> 残差
        let xbf = tensor.rmsnorm(s.x, w.rms_ffn(l));
        let hb = tensor.matvec(w.w1(l), xbf);
        let hb2 = tensor.matvec(w.w3(l), xbf);
        tensor.silu_mul_into(hb, hb2);
        tensor.add_into(s.x, tensor.matvec(w.w2(l), hb));
        l = l + 1;
    }

    // 7. 最终 rmsnorm + 分类头
    s.logits = tensor.matvec(w.wcls(), tensor.rmsnorm(s.x, w.rms_final()));
}
```

关键点是把 `run.c` 的**指针原地写**映射为**视图赋值**：`key_cache[l][pos] = kt`
是对降维视图调用 `copy_into`，因此 KV cache 零拷贝；注意量的 `_into` 算子
（`add_into`/`add_scaled_into`/`softmax_into`/`silu_mul_into`/`fill_zero`）同样
原地，避免每步分配。

## 25.6 采样器

与 `run.c` 四种模式一一对应：

```sloth
pub func sample(cfg, logits, temperature, topp, rng): int {
    if temperature == 0.0 {
        return argmax(logits, cfg.vocab_size);       // greedy
    }
    tensor.div_scalar_into(logits, temperature);
    tensor.softmax_into(logits);
    let coin = rng.next_f32();                        // xorshift
    if topp <= 0.0 or topp >= 1.0 {
        return sample_mult(logits, cfg.vocab_size, coin);       // multinomial
    }
    return sample_topp(logits, cfg.vocab_size, topp, coin);     // nucleus
}
```

`sample_topp` 先按 `(1-topp)/(n-1)` 裁剪候选，手写插入排序降序，累计到 `topp`
截断，再按 CDF 采样（`Array<int>` + `Array<float>`，无 `sort_by`）。

## 25.7 BPE tokenizer

`tokenizer.slt` 移植 `run.c` 的 `encode`/`decode`：

- `load_tokenizer` 用 mmap 读 `tokenizer.bin`：`[max_token_length: i32]` 后每个
  词条 `[score: f32][len: i32][bytes]`；
- 词表按字节序排序一次（`str_cmp` + 手写 quicksort），`str_lookup` 二分；
- `encode`：BOS(1) + dummy 空格前缀，按 Unicode 码点查表，缺失走**字节回退**
  （`byte+3`），然后反复合并「词表内得分最高」的相邻对；
- `decode`：跟随 BOS 时去掉前导空格，`<0xNN>` 词条还原为原始字节；
- `safe_write` 跳过单字节不可打印控制符（对应 `run.c` 的 `safe_printf`）。

## 25.8 与 `run.c` 的差异与注意

| 项 | `run.c` | sloth2 | 影响 |
| --- | --- | --- | --- |
| 权重精度 | 直接读 f32 | f32 → f64 **加宽拷贝** | 内存 ×2；数值更精确 |
| 算术精度 | IEEE f32 | f63 存储 / f62 算术（见附录 A.2.1） | greedy 下确定性对齐 |
| 整数 | i64 | **63 bit** | RNG 需注意溢出 |
| RNG | 64-bit `xorshift64*` | 31-bit `XorShift` | greedy 无 RNG，逐 token 对齐；temperature 采样暂未逐位对齐 |
| 显存/内存 | 权重 mmap | 权重加宽后常驻 | 42M ≈ 500MB（167MB 映射 + 334MB f64） |

## 25.9 运行与验收

```bash
cargo build
# 单模型生成（当前目录需有 model.bin / tokenizer.bin）
slothc build examples/llama/main.sl /tmp/llama.bin && /tmp/llama.bin
# 差分 + 真实模型
bash examples/llama/run.sh
```

`examples/llama/run.sh` 做两件事：

1. `gen_tiny.py` 生成两个确定性 tiny checkpoint（shared / unshared，含 GQA）
   与合成 32000 词表 tokenizer，跑 greedy 40 步与 `run.c` 黄金输出**逐字节**
   比对；
2. 若存在 `examples/res/stories42M.bin` 与 `tokenizer.bin`，跑 stories42M 并与
   `run.c` 对照。

实测：

- tiny shared / unshared、**stories42M** 的 greedy 40 步 token 序列均与
  `run.c` **完全一致**；
- stories42M 单次 40 步约 **2.1s**，`run.c -O3` 约 1.1s（≈ **0.52×**，目标
  ≥0.5×）。

llama2.c 验收要点（生成 `Once upon a time`）：

```text
Once upon a time, there was a little girl named Lily. She loved to play
outside in the sunshine. One day, she saw a big, yellow flower in the garden. It
```

至此，sloth2 从 checkpoint 加载到文本生成无需外部依赖（除 libm/libc/系统
mmap）。
