# 28. 多线程扩展（thread）

主文档 v1.1 曾把「线程 / 并行」列为非目标（fiber 移除后语言被定义为单线程模型，
线程留待 3.0）。本章对应 **TH 扩展**：在不改动语言核心语法的前提下，为 sloth2
补齐 OS 级多线程——共享内存模型、原子化 ARC、结构化同步原语
（`JoinHandle` / `Channel` / `Mutex` / `AtomicInt`），并与有栈协程组合成
「每线程 1:m 协程」的并发图景。

线程能力以**内建模块 `thread` / `channel` / `mutex` / `atomic`** 提供（与 `fiber.*`、
`tensor.*` 同款调用点识别机制），因此没有新关键字、没有新语法。

## 28.1 背景与方案选型

多线程要解决的核心问题不是「如何起线程」，而是**共享对象能否安全释放**。盘点
v1.1 实现中内嵌的单线程假设，真正的硬改动只有一处：

| 位置 | 单线程现状 | 多线程问题 | 处理 |
| --- | --- | --- | --- |
| **ARC 计数**（`Hdr.cnt`） | `retain`/`release` 普通读写 | 并发增减 → 计数错乱 → 悬垂/滞留 | **原子化**（§28.2，核心） |
| `Weak<T>` 弱链 | 目标归零沿链置零 | `upgrade` 与析构竞态 | CAS upgrade（§28.2） |
| 诊断计数 `rc_live`/`rc_drops` | 全局普通计数 | 并发抖动 | 原子计数，断言改为「join 后归零」 |
| 容器 / 张量 / 模块全局 | 无内部同步 | 并发读写缓冲/字段 | 不隐式加锁；文档化为数据竞争（UB），安全用法走 `Mutex`/`Channel` |
| Fiber `prev` 链 | 隐式全局 | 跨线程 resume 栈 → 灾难 | **线程封闭** + `owner_tid` 校验（§28.5） |

方案选型上否决了 Erlang 式**隔离堆 + 消息传递**：sloth2 没有 move 语义与深拷贝
基础设施，而张量权重是 GB 级 `mmap`，按值传递与大模型场景根本冲突。最终选定
**共享内存 + 原子 ARC + 同步原语**（与 C++/Go 同族）：内存安全（不悬垂）由运行时
保证，并发正确性（无竞争）由程序员用原语保证。

## 28.2 原子 ARC（核心改动）

`retain` / `release` 改为标准 `shared_ptr` 内存序：

- `retain`：`fetch_add(1, Relaxed)`——值词（`0`）走惰性 no-op 快路径，热路径
  开销与单线程相同；
- `release`：`fetch_sub(1, Release)`；归零时以 `Acquire` fence 收尾，随后递归
  析构、排空弱链、`free`；
- `Weak<T>` 的 `upgrade()` 改为 **CAS 循环**：读计数，若 `>0` 则
  `compare_exchange(cnt, cnt+1)`，成功即升级；目标已归零则失败返回 `nil`。

原子化**无条件启用**（单一运行时，避免双构建变体），发射器**零改动**：`retain` /
`release` 的调用面不变，全部差异封装在 `sloth-rt` 内。去 tag 迁移（附录 A）后
`int` 已是原生 i64，与 64-bit 原子槽天然一致，不再有 63-bit 的语义缝隙。

## 28.3 线程 API

```rust
let h: JoinHandle<int> = thread.spawn(|seed: int| -> int {
    var acc = 0;
    for (var i: 0..1000) { acc = acc + i; }
    return acc;
}, 42);

let r: int = h.join();          // 阻塞取回返回值
```

| 函数 | 签名 | 语义 |
| --- | --- | --- |
| `thread.spawn` | `((T) -> R, T) -> JoinHandle<R>` | 创建 OS 线程（`pthread`），立即执行 |
| `JoinHandle<R>.join` | `() -> R` | 阻塞至结束取回结果；重复 join / join 已 detach 者 → panic |
| `JoinHandle<R>.detach` | `() -> unit` | 放弃 join 权，线程成自由线程 |
| `thread.current_id` | `() -> int` | 当前线程标识（诊断/测试） |
| `thread.yield_now` | `() -> unit` | `sched_yield` 让出 |

实现要点：

- 入口闭包与 `arg` 走与 fiber 相同的**统一桥 ABI** `(env, arg) -> i64`；
- `JoinHandle` 本身是 ARC 对象，内含 join 状态、结果槽与 `Condvar`。`spawn` 会
  retain 入口闭包与引用型参数，worker 持有一份**自引用**，因此调用方立即丢弃
  句柄也不会让 worker 写坏已释放的状态；
- 结果以 **owned +1** 交给 `join`；detach 或弃置时由句柄析构函数释放；
- **弃置纪律**：既未 join 也未 detach 的句柄计数归零时，debug 构建在析构处
  panic（`abandoned thread handle`），release 构建隐式 detach——与协程
  「弃置泄漏退化为滞留」的哲学一致。

## 28.4 Channel / Mutex / AtomicInt

| 类型 | 构造 | 方法 |
| --- | --- | --- |
| `Channel<T>` | `channel.new<T>(capacity)`（`0` = 无界） | `send(v)`、`recv() -> T?`、`close()` |
| `Mutex` | `mutex.new()` | `lock()`、`unlock()`、`try_lock()`、`with(\|g\| …)` |
| `AtomicInt` | `atomic.new(init)` | `load`、`store`、`add`、`sub`、`cas(old,new) -> bool` |

- **Channel** 是 mpmc：有界用环形队列、无界用链表队列，共享一把 mutex + 两个
  condvar（not_full / not_empty）。元素是裸词，`send` 转移一个 owned +1 进队，
  `recv` 交付一个 owned +1（值型可选则包成 payload 盒）；`close` 后 `send` panic，
  `recv` 排空后返回 `nil`。`eref` 记录元素类型是否为引用，值元素不参与级联释放。
- **Mutex** 是不透明 `pthread_mutex_t`；`with` 闭包形式是唯一推荐用法（unlock 在
  闭包返回后自动执行，避免漏解锁）。
- **AtomicInt** 是 `AtomicI64`，所有操作 `SeqCst`。

## 28.5 `Send` 标记与 Fiber 线程封闭

`thread.spawn` 的 `T`/`R` 与 `Channel<T>` 的 `T` 要求内建标记 trait `Send`
（不可用户 `impl`）。绝大多数类型自动满足；**`Fiber<Y>` 不满足**——它是线程封闭
的。`spec/110_diag_fiber_send.sl` 即验证「把 fiber 传入 `thread.spawn`」会得到
编译期 `Send` 诊断。

Fiber 的线程封闭由运行时兜底：每个 `FiberObj` 记录 `owner_tid`，`resume` /
`yield` / `transfer` / `cancel` 入口校验当前 tid，不匹配即 panic。每线程在 TLS
里维护自己的 `CUR`/`MAIN` 哨兵；线程 trampoline 退出时断言本线程的 fiber 集已
全部 `Done`/`Error`，否则 debug 构建 panic（`abandoned fibers on thread exit`）。

## 28.6 内存模型（文档契约）

- **happens-before 建立点**：`thread.spawn`（调用 → 入口）、`join`（退出 →
  返回）、`Channel.send/recv`、`Mutex.lock/unlock`、`AtomicInt` 操作；
- **数据竞争**：两个线程对同一可变对象并发访问、至少一个为写、且无上述同步点
  分隔 → **未定义行为**。编译器与首版运行时不检测；
- **保证**：无论是否竞争，原子 ARC 保证对象不悬垂、计数不损坏；词面读写是单个
  对齐 i64，天然原子，不存在撕裂。竞争的最坏后果是读到中间状态。

## 28.7 编译器影响

| 组件 | 改动 |
| --- | --- |
| `sloth-frontend` | 无（表面都是普通调用；`JoinHandle<R>`/`Channel<T>` 复用泛型类型语法） |
| `sloth-codegen/ty` | 注册 `Ty::JoinHandle(R)` / `Ty::Channel(T)` / `Mutex` / `AtomicInt`；`Send` 自动满足判定 |
| `sloth-codegen/irgen` | 识别 `thread.*` / `channel.*` / `mutex.*` / `atomic.*` → 发射 `@sloth_thread_*` / `@sloth_chan_*` / `@sloth_mutex_*` / `@sloth_atomic_*`；spawn 载荷与 join 结果的转移插桩；`Send` 约束检查 |
| pass 管线 | 无新增 pass |

## 28.8 示例与验收

数据并行 `matvec` 是最能体现内存模型的负载：把 `W(d,n) @ x(n)` 按行分块给 P 个
线程，各自写 `out` 的不相交行区间（无写-写冲突），`w`/`x` 只读共享——这是
「无同步点也合法」的典型形态（`examples/threads/matvec_parallel.sl`）。

```rust
func matvec_parallel(w: Tensor<float,2>, x: Tensor<float,1>, d: int, n: int, p: int): Tensor<float,1> {
    var out: Tensor<float,1> = tensor.zeros([d]);
    let rows = d / p;
    var handles: Array<JoinHandle<int>> = [];
    for (var t: 0..p) {
        let lo = t * rows;
        var hi = lo + rows;
        if (t == p - 1) { hi = d; }
        handles.push(thread.spawn(|unused: int| -> int {
            for (var i: lo..hi) {
                let row: Tensor<float,1> = w[i];
                var acc = 0.0;
                for (var j: 0..n) { acc = acc + row[j] * x[j]; }
                out[i] = acc;
            }
            return 0;
        }, 0));
    }
    for (var h: handles) { h.join(); }
    return out;
}
```

`examples/threads/` 还有 `producer_consumer.sl`（4 生产 × 4 消费 × 10⁴，经有界
`Channel` 汇聚）与 `thread_fiber.sl`（4 线程 × 每线程 8 fiber，经 `Channel`
跨线程交付结果，对应「1:m × N」组合）。

| 用例 | 位置 |
| --- | --- |
| spawn/join（值与 `str` 载荷）、detach、线程身份 | `crates/slothc/tests/spec/107_thread_basic.sl` |
| Channel FIFO / 有界无界 / close→nil | `108_channel.sl` |
| Mutex + AtomicInt 全部表面 | `109_mutex_atomic.sl` |
| `Send` 拒绝跨线程 fiber（诊断） | `110_diag_fiber_send.sl` |
| 并发压力（原子 ARC、channel、mutex 竞速） | `crates/sloth-rt/tests/mt_smoke.rs` |
| 三示例 | `examples/threads/run.sh` |
