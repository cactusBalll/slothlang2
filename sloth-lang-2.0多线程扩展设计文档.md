# sloth-lang 2.0 多线程扩展设计文档

## 共享内存线程模型 + 原子 ARC + 结构化同步原语

| 项目 | 内容 |
| --- | --- |
| 文档版本 | v1.0（草案） |
| 文档日期 | 2026-09-18 |
| 上游文档 | 《sloth-lang 2.0 设计文档》v1.1（下称「主文档」）、《协程扩展设计文档》v1.0、《张量扩展设计文档》v1.0 |
| 文档状态 | 设计评审稿 |

---

## 1. 概述

### 1.1 定位

主文档 v1.1 的非目标写明「不实现线程/并行（fiber 移除后语言为单线程模型，线程留待 3.0 评估）」。本扩展将该评估提前落地：在**不改动语言核心语法**的前提下，为 sloth2 增加 OS 级多线程能力——共享内存模型、原子化 ARC、结构化同步原语（`JoinHandle`/`Channel`/`Mutex`），并与已设计的有栈协程扩展组合成「每线程 1:m 协程」的完整并发图景。

### 1.2 设计目标

1. **内存安全底线**：跨线程共享对象不得悬垂——引用计数原子化后，任何线程释放的对象在所有持有者间保持有效；数据竞争（data race）不属于内存安全问题，文档化为 UB（见 §6 内存模型）；
2. **零新关键字**：线程能力以**内建模块 `thread`** 提供（与 `fiber.*`、`arr.len()` 同款编译器识别机制）；
3. **与协程正交组合**：fiber 线程封闭、不可迁移；每线程独立 fiber 集，用户可在其上自建调度；
4. **性能目标**：并行 matvec/注意力对标 llama2.c 的 OpenMP 路径（`make runomp`），补齐张量扩展留下的单线程性能差距；
5. **双后端一致**：JIT 与 AOT 行为一致。

### 1.3 非目标

- 不实现结构化并发（scoped thread）——无生命周期系统支撑借用跨越，线程语义为 spawn/join 的自由线程；
- 不实现 async/await、绿色线程运行时（协程扩展已覆盖协作式并发）；
- 不提供线程间异常传播（线程内 panic 终止进程，与错误模型一致）；
- 不追求数据竞争的静态检测（无所有权/借用系统，此能力超出本语言特性面，见 §9 风险 #1）。

---

## 2. 现状分析：单线程假设的阻塞点清单

逐项盘点 v1.1 实现中内嵌的单线程假设，这是本扩展的工作集：

| # | 位置 | 现状 | 多线程下的问题 | 处理方案 |
| --- | --- | --- | --- | --- |
| 1 | **ARC 计数**（`sloth-rt/src/rc.rs`，`Hdr.cnt`） | `retain`/`release` 为普通读写（首检 tag0 惰性 no-op） | 两线程并发 retain/release 同一对象 → 计数错乱 → 悬垂或滞留 | **原子化**（§5.1），本扩展核心改动 |
| 2 | **字符串**（§5.2） | **无字符串池**：每个构造点（字面量/拼接/切片）都新分配，`intern_bytes` 不做去重 | 无共享池 → 无并发 intern 竞争 | 无需改动（§5.2） |
| 3 | **`Weak<T>` 弱链**（§5.1，`weak_head` 侵入式链） | 目标归零时沿链置 `target=0` | `upgrade()` 与目标析构竞态 → 悬垂 | CAS upgrade 协议（§5.3） |
| 4 | **模块全局变量**（§3.7，`ginit`） | 进程级全局槽，`@sloth_main` 启动时初始化 | 全局可变状态跨线程共享无保护 | 文档化 + 建议 `Mutex` 封装（§5.6） |
| 5 | **容器**（`arrays.rs` 稳定句柄、Map 哈希表） | 增长替换数据缓冲，句柄不动 | 并发 push/读 → 读到被替换的缓冲 | 不提供隐式同步；文档化为数据竞争（UB），安全用法走 `Mutex`/`Channel` |
| 6 | **张量**（§5.6，非追踪数据缓冲 + owner 保活） | 数据区 calloc 免追踪，视图靠 owner 引用保活 | owner 释放与视图使用跨线程竞态 | owner 即 ARC 对象，原子化后自然安全；**并发写同一数据区**文档化为数据竞争 |
| 7 | **Fiber**（协程扩展） | 栈切换上下文、prev 链为隐式全局 | 跨线程 resume 同一线程的栈 → 灾难性 UB | **线程封闭**：Fiber 记录 owner tid，跨线程操作 panic（§5.4） |
| 8 | **诊断计数器**（`sloth_rc_live`/`sloth_rc_drops`） | 全局计数 | 多线程 churn 时计数抖动 | 改为原子计数；测试断言改为「全部 join 后归零」 |
| 9 | **分配器** | malloc 基确定性链 | glibc malloc 线程安全，无问题 | 无需改动 |
| 10 | **JIT**（ORC ExecutionEngine） | 编译在启动期完成 | 运行期不编译，无线程安全问题 | 无需改动（并发编译不在特性面） |
| 11 | **extern/FFI** | 普通原生调用 | 宿主函数的线程安全由宿主负责 | 文档化 |

**结论**：阻塞点高度集中——真正的硬改动只有 **#1 原子 ARC**；#2/#3 是运行时局部改造；#4–#7 主要靠「线程封闭 + 文档化 UB + 同步原语」的设计纪律解决。单线程假设没有渗透进 codegen（发射器无全局可变状态、词面 ABI 无线程相关编码），这是 v1.1 架构对多线程友好的关键资产。

---

## 3. 方案选型

| 方案 | 描述 | 评估 |
| --- | --- | --- |
| A. 隔离堆 + 消息传递（Erlang 式） | 每线程独立对象空间，跨线程仅允许深拷贝/移动 | 内存最安全，但 sloth2 无 move 语义与深拷贝基础设施；张量 mmap 权重（GB 级）无法按值传递，与大模型场景根本冲突；**否决** |
| B. 共享内存 + 原子 ARC + 同步原语 | 对象自由共享，RC 原子化，提供 Mutex/Channel | 与现有 ARC/词面 ABI 兼容；数据竞争风险靠文档纪律与原语管理；与 C++/Go 同族，团队心智成本低；**选定** |
| C. 混合（默认隔离 + 显式共享类型） | 引入 `Send`/`Share` 严格划分 | 需要所有权/借用系统支撑，超出特性面；其安全子集以 `Send` 标记 trait 的弱化形式吸收进方案 B（§4.2） |

**选定方案 B 的理由**：与协程扩展「弃置泄漏退化为滞留」的工程哲学一致——内存安全（不悬垂）由运行时保证，并发正确性（无竞争）由程序员用原语保证，编译器不做超出能力面的承诺。

---

## 4. 语言层设计

### 4.1 线程 API（内建模块 `thread`）

```rust
// 创建线程：入口闭包 + 单载荷参数（与 fiber.create 同构）
let h: JoinHandle<int> = thread.spawn(|seed: int| -> int {
    var acc = 0;
    for (var i: 0..1000) { acc = acc + i; }
    return acc;
}, 42);

let r: int = h.join();        // 阻塞等待，取回返回值；重复 join → panic
```

| API | 签名 | 语义 |
| --- | --- | --- |
| `thread.spawn` | `(f: (T) -> R, arg: T): JoinHandle<R>` | 创建 OS 线程（pthread），立即执行；`f` 受限于一等函数值规则（非泛型/非可变参/非 extern/同模块，与 fiber 一致） |
| `JoinHandle<R>.join` | `(): R` | 阻塞至线程结束，取回 `R`；句柄被 join 后失效；线程 panic → 进程终止（不传播） |
| `JoinHandle<R>.detach` | `(): unit` | 放弃 join 权；线程成为自由线程（返回值随线程结束被 release） |
| `thread.current_id` | `(): int` | 当前线程标识（诊断/测试用） |
| `thread.yield_now` | `(): unit` | `sched_yield` 让出（自旋等待等场景） |

未 join 也未 detach 的 `JoinHandle` 计数归零时：debug 构建 panic（强制纪律），release 构建隐式 detach——与协程「弃置泄漏」处理哲学一致。

### 4.2 `Send` 标记约束

引入内建标记 trait `Send`（无方法，不可用户 impl）：

- **自动满足**：`unit`/`int`/`float`/`bool`/`range`、`str`（不可变，无驻留）、`Tensor<T, R>`（数据区无指针，共享语义清晰）、`Fiber<Y>`（禁止——线程封闭，见 §5.4）之外的引用类型**全部满足**；
- `thread.spawn` 的 `T`/`R` 与 `Channel<T>` 的 `T` 要求 `Send`；
- **设计取舍**：不区分「安全共享」与「危险共享」——所有 ARC 对象可跨线程（原子 RC 保内存安全），并发可变性导致的竞争属 §6 的 UB 域。`Send` 的作用是**显式拒绝**线程不安全的类型（首版仅 `Fiber<Y>` 与 `Weak<T>` 的跨线程 `upgrade` 受限场景，见 §5.3），并为未来更严格的所有权扩展预留语法位。

### 4.3 同步原语

| 类型/函数 | 说明 |
| --- | --- |
| `Mutex` | `extern type`（不透明，pthread_mutex）；`mutex.new(): Mutex`、`m.lock(): unit`、`m.unlock(): unit`、`m.try_lock(): bool`；配合 `m.with(|g| { ... })` 闭包形式（unlock 在闭包返回后自动执行，**唯一推荐用法**，避免漏 unlock） |
| `Channel<T>` | 引用类型，泛型；`channel.new<T>(capacity: int): Channel<T>`（capacity=0 为无界）；`send(v: T): unit`（有界满则阻塞）、`recv(): T?`（关闭且空返回 `nil`）、`close(): unit`；mpmc 语义，send/recv 构成 happens-before（§6） |
| `AtomicInt` | `extern type`；`load/store/add/sub/cas`，供无锁计数/标志位（词面 int 语义，原生 i64 / 64 bit） |

### 4.4 EBNF 增量

```ebnf
type_base ::= ... | 'JoinHandle' '<' type '>' | 'Channel' '<' type '>'
```

上下文关键字追加：`JoinHandle Channel`（`Mutex`/`AtomicInt` 为 `extern type`，不经类型语法）。无新保留字。

---

## 5. 运行时设计

### 5.1 原子 ARC（核心改动）

`crates/sloth-rt/src/rc.rs` 改造：

- `retain`：`fetch_add(1, Relaxed)`（tag0 惰性 no-op 快路径保留——值词不触内存，热路径开销与现状相同）；
- `release`：`fetch_sub(1, Release)`；归零判定时 `Acquire` fence，随后析构级联 + 排空弱链 + free（标准 C++ shared_ptr 内存序方案）；
- **性能策略**：原子化**无条件启用**（单一运行时，避免双构建变体的测试矩阵爆炸）。非原子→原子的 retain/release 开销在单线程负载上可测（x86_64 上 lock 前缀指令约 20–40 周期/次），接受基准回归 ≤5%；超标时后续引入 biased RC（线程本地计数 + 共享计数的两级方案）作为优化项，不改变语义；
- 发射器**零改动**：retain/release 调用面不变，全部变化封装在 rt 内。

### 5.2 字符串并发化

- **现状无字符串池**：`str` 是普通 ARC 对象，每个构造点新分配（主文档 §5.2），
  相等与 Map 键按内容比较。因此不存在进程级共享哈希表，**无并发 intern 竞态**，
  本扩展无需改动 `strings.rs` 的分配路径。
- 跨线程传递 `str` 与其它 ARC 对象一致：原子 retain/release 保活，只读共享安全；
  内容相等的两个句柄不可假设同一。

### 5.3 `Weak<T>` 的并发 upgrade

- 目标对象头增加 spin-flag（复用 `aux` 词的一位）；`upgrade()` 采用 CAS 循环：读 `cnt`，若 >0 则 `compare_exchange(cnt, cnt+1)`，成功即升级；若 0 则失败返回 `nil`；
- 析构路径在归零（Acquire）后、排空弱链前置 `target=0`——与 CAS 的内存序配合保证 upgrade 不会复活已析构对象；
- 限制：`Weak<T>` 可跨线程传递，但文档建议仅在明确同步的边界使用。

### 5.4 Fiber 线程封闭

- `FiberObj` 增加 `owner_tid`；`resume`/`yield`/`transfer`/`cancel` 入口校验当前 tid，不匹配 → `@sloth_panic_*`；
- 每线程一个「主协程哨兵」（TLS），线程 trampoline 启动时建立；线程结束时其 fiber 集必须已全部 Done/Error，否则 debug panic（同协程扩展弃置纪律）；
- **组合形态**：N 个 OS 线程 × 每线程 m 个 fiber，构成 1:m × N 的并发结构；跨线程通信走 `Channel`，不走 fiber 原语。

### 5.5 线程 trampoline 与 TLS

- spawn 时：入口闭包词与 `arg` 词按**转移语义**移交新线程（同协程扩展 §4.3 的插桩：owned 转移/借用物化 retain）；
- trampoline：建立 TLS（主协程哨兵、panic 上下文）→ 调闭包 → 返回值按转移语义写回 `JoinHandle` 的共享槽（带 release-fence 发布）→ 清理 TLS → 线程退出；
- `JoinHandle` 为 ARC 对象：`[tid/ pthread_t, result_slot, state(Running/Done/Joined), mutex+cond]`；`join` 经 condvar 等待，acquire 读 result_slot。

### 5.6 全局变量与模块

- `ginit` 在 `@sloth_main` 开头、任何用户 spawn 之前完成——初始化无竞态；
- 运行期对模块级 `var` 的跨线程并发读写为**数据竞争（UB）**；文档建议以 `Mutex` 封装或改为线程局部（线程局部存储 **不提供**语言级支持，列为后续候选）。

### 5.7 Channel 实现

- 有界：环形缓冲 + 单 mutex + 双 condvar（not_full/not_empty）；无界：链表队列 + 同款锁对；
- 元素为词面 i64（ARC 对象或值词），send 转移所有权、recv 交付 owned——语义与协程载荷转移一致；
- `close` 后 send → panic；recv 排空后返回 `nil`。

---

## 6. 内存模型（文档契约）

- **happens-before 建立点**：`thread.spawn`（调用点 → 新线程入口）、`join`（线程退出 → join 返回）、`Channel.send/recv`、`Mutex.lock/unlock`、`AtomicInt` 操作（默认 seq_cst）；
- **数据竞争**：两个线程对同一可变对象（类字段、Array/Map 元素、张量数据区、模块全局）并发访问且至少一个为写，且无上述同步点分隔 → **未定义行为**。编译器不检测，运行时首版不检测（TSan 兼容构建列为测试基础设施，见 §8）；
- **保证**：无论是否竞争，ARC 原子化保证对象不悬垂、计数不损坏——竞争的最坏后果是读到撕裂的中间状态（非内存安全事件）。词面读写为单 i64，对齐天然原子，不存在词撕裂。

---

## 7. 编译器影响

| 组件 | 改动 |
| --- | --- |
| `sloth-frontend` | 无（`thread.spawn` 等表面是普通调用；`JoinHandle<R>`/`Channel<T>` 复用泛型类型语法） |
| `sloth-codegen/ty` | `Ty::JoinHandle(R)`、`Ty::Channel(T)` 注册；`Send` 标记 trait 的自动满足判定 |
| `sloth-codegen/irgen` | 识别 `thread.*`/`channel.*`/`mutex.*` 内建调用面 → 发射 `@sloth_thread_*`/`@sloth_chan_*`/`@sloth_mutex_*`；spawn 载荷与 join 结果的**转移插桩**（复用协程扩展的同一套辅助函数）；`Send` 约束检查（拒绝 `Fiber<Y>` 等） |
| 单态化 | `spawn`/`Channel<T>` 按类型实参实例化，与既有机制一致 |
| pass 管线 | 无新增 pass |

---

## 8. 验收与测试策略

### 8.1 功能验收

1. **ARC 压力测试**：N∈{2,4,8,16} 线程并发 retain/release 共享对象图 10⁷ 次，全部 join 后 `sloth_rc_live` 回落基线；ASAN/TSan 构建下无报告；
2. **Channel 语义**：mpmc 生产者-消费者（4 生产 × 4 消费 × 10⁶ 消息）无丢失无重复；close 语义、有界阻塞语义用例；
3. **Mutex**：`with` 闭包形式的计数器竞速测试（8 线程 × 10⁶ 次自增结果精确）；
4. **Fiber 封闭性**：跨线程 resume 触发预期 panic；「每线程 fiber 集 + 跨线程 Channel」组合示例正确；
5. **Weak**：多线程并发 upgrade/析构竞速，无悬垂（ASAN）、升级结果要么成功要么 `nil`；
6. **回归**：全部既有单线程 spec/churn/张量/llama 测试在原子 ARC 下通过，性能回归 ≤5%。

### 8.2 性能验收（对标 llama2.c OpenMP）

- **并行 matvec**：将 matvec 按行分块到 P 线程（每线程处理 `d/P` 行，经 `thread.spawn` + join 汇聚），P=物理核数；对标 `run.c` 的 `#pragma omp parallel for`；
- **端到端**：stories42M 推理，单线程 sloth2 为 1.0x，多线程并行 matvec 目标 **≥ 2.5x（4 核）**——OpenMP 版本同规模加速比为参照系；
- 注意 KV cache/激活缓冲为每线程私有（各自 RunState），共享的是只读权重——天然无竞争，是验证本扩展的理想负载。

### 8.3 基础设施

- CI 增加 TSan 构建变体（`clang -fsanitize=thread`）跑 §8.1 全部用例；
- `examples/threads/`：并行 matvec、生产者-消费者、线程+协程组合三个示例。

---

## 9. 风险与阻碍评估

| # | 风险 | 等级 | 说明与对策 |
| --- | --- | --- | --- |
| 1 | **数据竞争 UB 无静态防线** | 高（已接受） | 无所有权/借用系统，无法提供 Rust 级保证，这是方案 B 的固有代价。对策：内存模型文档契约（§6）+ TSan CI + 原语优先的惯用法引导（`Channel`/`Mutex.with`）；在文档中明确「共享可变状态即责任转移给程序员」 |
| 2 | **原子 ARC 性能回归** | 中 | 单线程负载 retain/release 全原子化，基准回归可能超 5%。对策：先测量再优化；预留 biased RC 两级计数方案（不改变语义，仅 rt 内部优化） |
| 3 | **Weak upgrade 竞态的微妙正确性** | 中 | CAS 升级与析构排空弱链的内存序配合易错。对策：直接照搬 C++ `weak_ptr::lock` 的成熟协议；§8.1.5 竞速测试 + TSan |
| 4 | **Fiber 跨线程误用** | 中 | 封闭性靠运行时 tid 校验，误用即 panic——失败显式化，风险可控 |
| 5 | **死锁/锁序** | 低 | 语言不提供锁序分析；`Mutex.with` 单一获取点降低死锁面；文档建议 Channel 优先于共享锁 |
| 6 | **extern 宿主代码的线程安全** | 低 | 宿主函数被多线程并发调用时责任在宿主；文档明示 |
| 7 | ~~词面 63-bit int 与 64-bit 原子槽的语义缝隙~~ | 已消除 | 去 tag 后 `int` 为原生 i64，与 64-bit 原子槽一致（PLAN §14） |

---

## 10. 与主文档及扩展文档的变更清单（回写项）

| 位置 | 变更 |
| --- | --- |
| 主文档 §1.3 非目标 | 删除「不实现线程/并行」，改为「线程支持见多线程扩展设计文档」 |
| 主文档 §2.1 类型全集 | 新增 `JoinHandle<R>`、`Channel<T>` 行 |
| 主文档 §2.5 Trait | 预定义 trait 追加 `Send`（标记 trait，不可用户 impl） |
| 主文档 §2.6 引用类型表 | 新增 `JoinHandle`/`Channel` 行 |
| 主文档 §3.1 变更总览 | 新增「多线程」行；上下文关键字追加 `JoinHandle Channel` |
| 主文档 §3.9 EBNF | `type_base` 追加两个产生式；上下文关键字更新 |
| 主文档 §5.1 ARC | 「retain/release 普通读写」改写为原子操作 + 内存序约定（§5.1）；追加性能策略段 |
| 主文档 §5.2 字符串 | 「单线程无需锁」改写为「无池，无并发 intern 竞态」 |
| 主文档 §5.1.1 协议 | 追加第 10 条：跨线程边界（spawn/join/channel）的转移语义与内存序 |
| 主文档 §5.5 错误模型 | 补充「线程内 panic 终止进程，不跨线程传播」 |
| 主文档 §7 风险表 | 追加本文档 §9 风险 #1–#3 |
| 主文档 §8 路线图 | 追加 TH 阶段（本文档 §11） |
| 协程扩展 §3.4/§5.4 | 「无语言级禁止项」补充例外：`Fiber<Y>` 不满足 `Send`，禁止跨线程传递；线程封闭校验 |
| 张量扩展 §1.3 非目标 | 「多线程并行」项标注：由多线程扩展解除，并行 matvec 验收见该文档 §8.2 |

---

## 11. 实施路线图（TH 阶段，叠加于 CE 阶段之后）

| 阶段 | 内容 | 验收标准 |
| --- | --- | --- |
| **TH-P0** | 原子 ARC 改造 + 诊断计数器原子化（字符串池分片锁项取消：当前无池，见 §5.2） | 既有全部单线程测试通过；性能回归测量报告（门槛 ≤5%） |
| **TH-P1** | `thread.spawn`/`JoinHandle`/trampoline/TLS；转移插桩；`Send` 检查 | spawn/join 基础用例；跨线程 churn 测试（§8.1.1）通过 |
| **TH-P2** | `Channel<T>`/`Mutex`/`AtomicInt`；Weak CAS upgrade；Fiber 线程封闭校验 | §8.1.2–8.1.5 全部通过；TSan 构建无报告 |
| **TH-P3** | 并行 matvec 与 llama 多线程推理示例；内存模型文档定稿 | §8.2 性能达标；`examples/threads/` 三示例 CI 全绿 |

里程碑：TH-P1 完成即具备最小可用多线程；TH-P3 完成即达成「对标 run.c OpenMP 路径」的性能目标，多线程扩展正式落地。

---

## 附录 A：并行 matvec 参考实现（验收负载）

```rust
// 将 W(d,n) @ x(n) 按行块分给 P 个线程，对应 run.c 的 #pragma omp parallel for
func matvec_parallel(w: Tensor<float, 2>, x: Tensor<float, 1>, d: int, n: int, p: int): Tensor<float, 1> {
    let out = tensor.zeros<float, 1>([d]);
    let rows = d / p;
    var handles: Array<JoinHandle<int>> = [];
    for (var t: 0..p) {
        let lo = t * rows;
        let hi = (t == p - 1) ? d : lo + rows;   // 注意 ?: 返回 int
        handles.push(thread.spawn(|unused: int| -> int {
            // 闭包捕获 w/x/out（引用共享，ARC 原子化保活）
            for (var i: lo..hi) {
                var acc = 0.0;
                for (var j: 0..n) { acc = acc + w[i][j] * x[j]; }
                out[i] = acc;                    // 各行互不重叠：无竞争
            }
            return 0;
        }, 0));
    }
    for (var h: handles) { h.join(); }
    return out;
}
```

> 说明：行块互不重叠，写 `out` 无竞争；`w`/`x` 只读共享——该负载是 §6 内存模型下「无同步点也合法」的特例（无写-写/写-读冲突），也是最常见的数据并行形态。

## 附录 B：线程 + 协程组合形态

```rust
// N 个 worker 线程，每线程内部用 fiber 跑 m 个协作任务；任务产出经 Channel 汇聚
let results = channel.new<int>(0);
for (var t: 0..4) {
    thread.spawn(|tid: int| -> int {
        var fibers: Array<Fiber<int>> = [];
        for (var k: 0..8) {
            fibers.push(fiber.create(|init: int| -> unit {
                var acc = init;
                for (var i: 0..100) {
                    acc = acc + i;
                    fiber.yield(acc);           // 线程内协作让出
                }
                results.send(acc);              // 跨线程通信走 Channel
            }, tid * 1000 + k));
        }
        // 本线程的轮询调度（协程扩展附录 A 的 Scheduler 模式）
        while (fibers.len() > 0) {
            let f = fibers[0];
            if (fiber.resumable(f)) { fiber.resume(f, 0); }
            if (!fiber.resumable(f)) { fibers.remove(0); }
        }
        return 0;
    }, t);
}
```
