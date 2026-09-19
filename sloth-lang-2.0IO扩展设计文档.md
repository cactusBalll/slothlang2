# sloth-lang 2.0 I/O 扩展设计文档

## 统一 I/O 事件队列 + Fiber/Thread 服务器

| 项目 | 内容 |
| --- | --- |
| 文档版本 | v1.0（与实现对齐稿） |
| 文档日期 | 2026-09-20 |
| 上游文档 | 《sloth-lang 2.0 设计文档》v1.2、《协程扩展设计文档》v1.0、《多线程扩展设计文档》v1.0 |
| 参照实现 | 本仓库 `crates/sloth-rt/src/{bytes,time,net,event}.rs`、`lib/sloth/{io,net,event,http}.slt`、`examples/net/` |
| 文档状态 | 设计评审稿 |

---

## 1. 概述

### 1.1 定位与背景

sloth2 的运行时在此前已具备：确定性 ARC（主文档 §5.1）、有栈协程 `Fiber<Y>`（协程扩展 CE）、共享内存线程与同步原语（多线程扩展 TH）。但**没有任何 I/O 能力**：既无 socket/syscall 面，也无事件循环，因此无法承载网络服务。本扩展补齐这一层：

1. **机制层（Rust / `sloth-rt`）**：把 `select`/`poll`/`epoll`/`kqueue`/`io_uring` 五种多路复用机制统一到一个极小的 `sloth_ev_*` C-ABI 之后，并提供字节缓冲、单调时钟、socket/sockaddr 原语。
2. **策略层（sloth / `lib/sloth`）**：**I/O 事件队列本身用 slothlang2 实现**——`EventLoop` 持有后端句柄、`Fiber<Wait>` 任务槽表、就绪队列与定时器链表；服务器（TCP/UDP/HTTP）在队列之上以 Fiber 与 Thread 组织。

这与项目「机制进 rt、策略用 sloth、不新增语法」的既有哲学一致（对照 §5.3 容器方法、TE 张量内核、CE/TH 内建模块）。

### 1.2 设计目标

1. **五机制统一抽象**：一套 `add/mod/del/wait` 接口覆盖 select/poll/epoll/kqueue/io_uring；后端可运行时探测、可逐个对照测试。
2. **零新关键字**：能力以 `extern type` / `extern func` 形式暴露（与 `fs.slt` 的 mmap 面同款）；事件队列与服务器全部为标准库 `.slt`。
3. **Fiber 与 Thread 双模型**：同一套事件队列支撑「每线程一个 event loop + SO_REUSEPORT」的 fiber 服务器，以及阻塞式 thread-per-connection 对照实现。
4. **HTTP/1.1 最小可用**：请求行 + 头 + Content-Length 体、方法/路径路由、keep-alive，无 TLS/chunked/HTTP2。
5. **双后端一致**：JIT（ORC）与 AOT（`slothc build`）下行为一致，新 rt 符号经 `libsloth_rt.so` 解析。
6. **所有权安全**：Fiber 挂起栈靠 ARC 计数保活（协程扩展 §4.3），I/O 载荷跨切换边界走既有转移协议；不引入根扫描。

### 1.3 非目标

- 不实现 kqueue 后端（kind/ABI 保留，`available(kqueue)` 恒假，待 BSD 环境；见 §6.6）；
- 不实现 TLS、HTTP/2、WebSocket、chunked transfer-encoding；
- 不实现跨线程的协程迁移（Fiber 线程封闭，TH §5.4 不变）；
- 不实现 DNS 解析、连接池、异步文件 I/O（本扩展聚焦 socket 事件）；
- 不做 data-race 静态检测（与 TH 一致，竞争属 UB，见 TH §6）。

---

## 2. 现状分析：阻塞点清单

| # | 位置 | 现状 | I/O 扩展的处理 |
| --- | --- | --- | --- |
| 1 | `sloth-rt` 组件 | 无 net/bytes/clock/event | 新增四个模块（§5） |
| 2 | 词面 ABI | 无 tag 单 i64（PLAN §14），引用=裸指针 | 字节缓冲/句柄以不透明指针词传递，无需新编码 |
| 3 | 引用判定 | `extern type` 被 `is_ref` 判为非 rc | I/O 句柄（`Bytes`/`EvLoop`/`EvBuf`/`SockAddr`）正应如此：codegen 不插 retain/release，生命周期由显式 `*_free` 管理 |
| 4 | Fiber | 有栈、协作式、线程封闭 | 事件队列在单线程内以 `fiber.resume/yield` 驱动，无需 rt 改动 |
| 5 | Thread | `thread.spawn` + Channel/Mutex/Atomic | 多线程服务器：每线程自带 event loop；同步计数用 `AtomicInt` |
| 6 | 分配器 | malloc 基确定性链 | `Bytes` 用 malloc/realloc + 显式 free；事件句柄同理 |
| 7 | 字符串 | 不可变、无池 | HTTP 解析用 `str_slice`/`str_find`/`str_of_byte`；接收缓冲用可变 `Bytes` |
| 8 | 编译器内建识别 | `fiber.*`/`thread.*`/`tensor.*` 在 call site 识别 | 本扩展**不新增内建**，全部走 extern，前端/发射器零结构改动 |

**结论**：阻塞点集中在 rt 机制缺失；策略层完全可由现有 sloth 表达（类、泛型、Fiber、Thread、Map、闭包）。因此本扩展是「rt 扩展 + 标准库」，不触碰 EBNF/类型系统主干。

---

## 3. 方案选型

| 决策点 | 备选 | 选定 | 理由 |
| --- | --- | --- | --- |
| API 表面 | A 编译器内建模块（如 `net.*`） / B `extern func` + `.slt` / C 混合 | **B** | fs/tensor 已证 `extern func` 足以暴露 syscall；内建需动前端与 Ty，收益仅语法糖 |
| 事件队列位置 | A Rust 实现 ready 队列+回调 / B sloth 实现队列+策略 | **B** | 「用 slothlang2 实现事件队列」是本扩展的显式目标；rt 只做机制（wait/ctl） |
| io_uring 接入 | A 链接 liburing / B 裸 syscall + mmap | **B** | 不引入新 Cargo 依赖与构建期头文件依赖；`io_uring_setup/enter` 经 `libc::syscall` |
| 服务器模型 | A 纯 fiber / B 纯 thread / C 两者 | **C** | Fiber 为高效主线；阻塞 thread-per-connection 作为对照与低复杂度备选 |
| HTTP | A 最小 / B 扩展 / C HTTP/1.0 | **A** | 覆盖压测与示例所需；chunked/query 解码留待后续 |

---

## 4. 语言层设计

### 4.1 无新关键字、无新语法

I/O 能力**不**进入 EBNF。用户可见面：

```rust
import "sloth/io.slt";     // Bytes / EvLoop / EvBuf / 后端常量 / 字符串工具
import "sloth/net.slt";    // TcpListener / TcpStream / UdpSocket / SockAddr
import "sloth/event.slt";  // EventLoop / Wait / serve 辅助
import "sloth/http.slt";   // Request / Response / Router / serve
```

类型与函数全部来自 `extern`（§4.2）与 `pub class`/`pub func`（§4.3）。

### 4.2 不透明类型（`extern type`）

| 类型 | 含义 | 生命周期 |
| --- | --- | --- |
| `Bytes` | 可增长字节缓冲（malloc + realloc） | 显式 `bytes_free` |
| `EvLoop` | 事件后端句柄（epoll fd / io_uring ring / 注册表 + eventfd） | `ev_free` |
| `EvBuf` | 等待结果输出缓冲（token/revents 数组） | `evbuf_free` |
| `SockAddr` | `sockaddr_storage` + 长度 | `addr_free` |

这些是 `extern type`，`register_import` 会将其登记进 `extern_types`，`is_ref` 返回 false，故 codegen **不会**对它们发射 retain/release（PLAN §14 去 tag 迁移引入的修正）。这是「原始 C 指针不得被 ARC 触碰」这一硬约束的直接应用。

### 4.3 标准库类与函数

- `net.slt`：`class TcpListener`、`class TcpStream`、`class UdpSocket`；地址函数 `addr_new/set/ip/port/free`；构造 `tcp_listen`（非阻塞，可 reuseport）、`tcp_listen_blocking`、`tcp_connect`、`tcp_connect_blocking`、`udp_bind`；常量 `af_inet()`/`sock_stream()` 等。
- `event.slt`：`class Wait`、`class EventLoop`；辅助 `wait_read/write/accept/sleep`、`read_some`、`write_all`、`write_str_all`、`accept_blocking`、`await_*`。
- `http.slt`：`class Request`/`Response`/`Router`；`parse_request`、`reason`、`text`/`html`/`not_found`、`serve`/`serve_n`。

### 4.4 限制

- `extern func` 的 `int` 实参为裸 i64（PLAN §14：rt 不再 `dec_i`）；`str` 传句柄词；返回标量由 codegen 重新编码（去 tag 后为恒等）。
- 句柄不是 ARC 对象，忘记 `*_free` 退化为泄漏（与主文档 §5.1「漏插 release 退化为滞留」一致）。
- 受一等函数规则约束，`serve` 的 handler 以非泛型闭包/具名函数提供。

---

## 5. 运行时设计（`crates/sloth-rt/src/`）

### 5.1 模块与职责

| 模块 | 导出（节选） | 说明 |
| --- | --- | --- |
| `bytes.rs` | `sloth_bytes_new/len/cap/ensure/set_len/get/set/fill/append/copy_from_str/to_str/as_str/free` | 可变字节缓冲；recv/HTTP 帧定界的热数据留在 rt |
| `time.rs` | `sloth_now_ms`、`sloth_sleep_ms` | `CLOCK_MONOTONIC`；超时/keep-alive 用 |
| `net.rs` | `sloth_net_socket/close/shutdown/set_nonblocking/set_blocking/set_reuse{addr,port}/set_nodelay/bind/listen/accept/connect/recv/send/send_str/recvfrom/sendto/peer_addr/local_port`、`sloth_addr_*`、`sloth_io_errno/would_block/conn_closed` | 非阻塞为默认（`SOCK_NONBLOCK|SOCK_CLOEXEC`） |
| `event.rs` | `sloth_ev_available/backend_name/new/free/wakeup/ctl/poll`、`sloth_evbuf_*` | 五机制统一后端 |
| `strings.rs`（增补） | `sloth_str_find`、`sloth_str_starts_with` | HTTP 分帧/路由 |

### 5.2 错误约定

所有可能失败的 net/event 调用返回 `-errno`（或非负 fd/计数）；sloth 侧以 `r < 0` 判定，并用 `io.would_block(r)`（匹配 `EAGAIN/EWOULDBLOCK/EINPROGRESS`）与 `io.conn_closed(r)`（`ECONNRESET/ECONNABORTED/EPIPE/ENOTCONN`）分类。`EINTR` 在 wait 层被吸收为「0 个事件」，避免伪错误。

### 5.3 统一事件 ABI

```
kind     : SELECT=0 POLL=1 EPOLL=2 KQUEUE=3 IOURING=4
op       : ADD=1 MOD=2 DEL=3
interest : READ=1 WRITE=2   revents 追加 ERR=4 HUP=8

EvLoop* sloth_ev_new(kind)                 // 0 = 不支持/失败
i64     sloth_ev_ctl(ev, op, fd, events, token)
i64     sloth_ev_poll(ev, timeout_ms, EvBuf*)   // 返回就绪数或 -errno
i64     sloth_ev_wakeup(ev)                // eventfd 唤醒阻塞中的 wait
EvBuf*  sloth_evbuf_new(cap)               // token/revents 输出
```

`token` 是不透明 i64，由 sloth 侧编码 `{slot, generation}`（§7.3）；rt 不改写、只回传。`sloth_ev_poll` 的超时语义：`-1` 表示无限等待（select 后端据此传 `NULL timeval`，见 §12 风险 #1）。

---

## 6. 后端实现（`event.rs`）

### 6.1 注册表与就绪集

`EvLoop` 内含：`kind`、后端 fd（epoll fd / kqueue fd，存于 `epfd`）、`wake_fd`（eventfd）、`regs: Vec<Reg{fd,events,token,gen}>`、`by_token: HashMap<token,index>`。poll/select/io_uring 依赖注册表在每次 wait 时重建/重挂；epoll 由内核保管且把 token 写入 `epoll_data`。

### 6.2 select

每次 wait 用自实现的 128 字节 `fd_set` 位图（16×u64，与 64 位 Linux `libc::fd_set` 布局一致）重建读/写集合，`select(maxfd+1, ...)`。限制 `FD_SETSIZE=1024`（固有）。超时 `<0` 时传 `NULL`。

### 6.3 poll

按注册表构造 `pollfd[]`（首个为 wake_fd，token 哨兵 `i64::MIN`），`poll(..., timeout_ms)`，逐项把 `revents` 映射回 READ/WRITE/ERR/HUP。

### 6.4 epoll

`epoll_create1(CLOEXEC)`；`epoll_ctl` ADD/MOD/DEL，`events = 兴趣 | EPOLLERR|EPOLLHUP|EPOLLRDHUP`，token 存 `epoll_event.u64`。`epoll_wait` 就绪分批取；wake token 被识别并 `read(eventfd)` 清零。

### 6.5 io_uring（Linux，裸 syscall）

- `io_uring_setup(entries)` 取 `io_uring_params`，分别 `mmap` SQ ring / CQ ring / SQE 数组（`IORING_OFF_*`）。
- 注册表每个 fd 提交一条**一次性** `IORING_OP_POLL_ADD`；`user_data = (gen<<32)|token`。就绪的 CQE `res` 是 poll 掩码，映射回 READ/WRITE/ERR/HUP 后按 `gen` 校验是否仍是最新注册，若是则**重挂**一条新 `POLL_ADD`（gen+1）。
- 等待用 `poll(ring_fd, POLLIN, timeout_ms)`（内核 ≥5.5 起 io_uring fd 可 poll），再排空 CQ；提交用 `io_uring_enter(to_submit, 0, 0)`。
- 不可用环境（容器/旧内核）在 `sloth_ev_available(IOURING)` 探测 `io_uring_setup` 失败后返回假，`EventLoop` 不选用。
- 陈旧等待（DEL 后未触发）由 gen 校验忽略；不主动 `POLL_REMOVE`，fd 关闭时其 poll 完成并被忽略。

### 6.6 kqueue（保留）

`EV_KQUEUE` 的 kind 与 API 位保留，但首版 `sloth_ev_available` 对 kqueue 恒返回假（避免「广告了后端却在 wait 失败」）。BSD 后端只需新增 `poll_kqueue` 并放开探测，无需改动 sloth 侧。

---

## 7. 事件队列（`lib/sloth/event.slt`）

### 7.1 任务与等待描述

任务 = `Fiber<Wait>`。当一个操作会阻塞时，任务 `fiber.yield(W)` 交出一个 `Wait`：

| kind | 含义 |
| --- | --- |
| 0 | 不等待，立即重排（yield-now） |
| 1 | 可读（read） |
| 2 | 可写（write） |
| 3 | 可接受（accept） |
| 4 | 睡眠到 `deadline` |

`class Wait { kind, fd, deadline }`；`EventLoop` 是唯一的消费者，任务侧只经 `wait_read/wait_write/wait_accept/wait_sleep` 构造。

### 7.2 reactor 协议

`EventLoop.run()`：

```
while running:
    if active == 0: return                 // 无任务即退出
    if ready 为空:
        tmo = next_timeout()               // 最小 deadline 差，-1=无限
        n = io.ev_poll(ev, tmo, evbuf)     // 阻塞
        collect_ready()                    // token 解码 + alive 校验
        fire_timers()                      // 到期任务入就绪队列
    while ready 非空:
        tok = ready.pop(); resume_slot(tok)
```

`resume_slot(tok)`：先注销该槽上一次注册的 fd（`DEL`），`fiber.resume(f, Wait.none())`；返回 `nil` = 任务结束（释放槽），否则按返回的 `Wait` 注册 fd（`ADD`）或加入定时器链表。

### 7.3 槽回收与 token 代际

槽位复用会带来「陈旧事件/定时器唤醒新任务」的风险。token 编码 `slot*65536 + (gen%65536)`，每次分配 `slot_gen[slot]++`；`alive(tok)` 校验 `live[slot]==1 && gen 匹配`。就绪事件、定时器、`resume` 前均过 `alive`，回收槽不会被旧事件误唤醒。io_uring 的 `user_data` 亦携带 gen，双保险。

### 7.4 定时器

`timer_at[]`/`timer_tok[]` 线性表（最小堆为后续优化）。`next_timeout` 取最小；`fire_timers` 把到期且 `alive` 的 token 入就绪队列。HTTP keep-alive 的每连接超时用该机制。

### 7.5 阻塞风格的 fiber API

`read_some`/`write_all`/`write_str_all`/`accept_blocking` 内部循环：非阻塞调用 → `would_block` → `fiber.yield(对应 Wait)`。于是业务代码是顺序直写，事件循环保持非阻塞（经典 stackful-coroutine reactor，无回调反转/无 async-await）。

### 7.6 事件队列的退出

`active`（存活任务数）归零时 `run` 返回；`shutdown()` 置 `running=0` 并 `ev_wakeup` 打断阻塞 wait。`serve_n` 接受固定连接数后返回，配合 handler fiber 使进程自然收敛，适合 CI。

---

## 8. 服务器模型

### 8.1 Fiber（主线，每线程一个 loop）

```
listener = tcp_listen(host, port, backlog, reuseport)
loop = EventLoop(kind)
loop.spawn(serve(loop, listener, handler))   // acceptor fiber
loop.run()
```

- acceptor fiber `accept_blocking` 后 `loop.spawn` 一个连接 fiber；
- 多核扩展：N 个线程各自 `EventLoop` + **SO_REUSEPORT** 同端口监听，内核分散连接，无共享锁；
- 跨线程仅经 `thread.*`/Channel/AtomicInt；Fiber 不跨线程（TH §5.4）。

### 8.2 Thread（阻塞对照）

`net.slt` 提供 `tcp_listen_blocking` / `tcp_connect_blocking` / `set_blocking`：accept 返回的 fd 置阻塞，`thread.spawn` 每连接一个 handler thread；主线程跑客户端。用于低复杂度场景与对照基准（`examples/net/tcp_echo_threads.sl`）。

### 8.3 UDP

`udp_bind` + `try_recv_from`/`send_to`；UDP 服务器在 event loop 上 `await_readable` 后收包回发，天然每 loop 一 socket。

### 8.4 HTTP/1.1

- `read_request`：非阻塞读入 `Bytes`，找到 `\r\n\r\n` 后按 `Content-Length` 决定请求体是否读齐；
- `parse_request`：请求行（method/path/version，path 保 query）、头（键小写、值 trim）、体；
- `Router`：`Map<str, (Request)->Response>`，键 `"METHOD route"`；`dispatch` 迭代匹配，未命中 404；
- `Response.serialize`：补 Content-Length/Connection，拼接状态行+头+体；
- keep-alive：HTTP/1.1 且 `Connection != close` 时循环处理下一请求，否则响应后关连接；
- `serve`/`serve_n`：acceptor 循环 + 每连接 fiber。

---

## 9. 内存与所有权

- `Bytes`/`EvLoop`/`EvBuf`/`SockAddr` 为 `extern type`，非 ARC；由显式 `*_free` 释放。事件循环对象与 socket 对象为普通 sloth 类（ARC），其字段持有外部句柄但不参与引用计数——因此**连接关闭路径必须释放 `Bytes` 并 `close` fd**，`EventLoop.close` 释放后端句柄与 evbuf。
- Fiber 挂起栈中持有的 sloth 引用早已 retain（协程扩展 §4.3），event loop 无需根枚举。
- 跨 `fiber.yield`/`resume` 的 `Wait` 载荷走既有转移协议，rc 计数在 churn 下回落基线（spec `106_fiber_ownership` 同款断言）。

---

## 10. 编译器影响

| 组件 | 改动 |
| --- | --- |
| `sloth-frontend` | **无** |
| `sloth-codegen/ty` | **无**（无新 Ty；extern type 走既有 `extern_types`） |
| `sloth-codegen/irgen` | **无新内建**；仅修三处与标准库可用性相关的真 bug（见下） |
| pass 管线 | 无新增 pass |

**过程中修掉的三处 codegen 真 bug**（均带回归面，非本扩展的语法改动）：

1. `keys(Map<K,V>)` 结果元素类型恒为 `int`（`values` 已按 V 推导）→ 按 `K` 推导。原因：本项目 Map 迭代依赖 `Entry<K,V>`，而 `Entry` 预注入仅发生在根模块，导入模块不可用，标准库只能经 `keys()` 迭代。
2. lambda 捕获误判**模块限定调用**：`event.sleep(...)`/`io.bytes_new(...)` 的模块名被当作待捕获变量 → `lambda_caps` 纳入 `init_mods`/`mod_alias`。
3. lambda 捕获漏扫**字符串插值** `${expr}` 中的自由变量 → `walk_ids_expr` 增 `ExprNode::Str`（`StrPart::ExprAst`）分支。

---

## 11. 风险与阻碍评估

| # | 风险 | 等级 | 说明与对策 |
| --- | --- | --- | --- |
| 1 | 后端超时表示不一致 | 中（已修） | select 的 `timeval` 不接受负值：`-1` 无限等待必须传 `NULL`，否则 `EINVAL`。已用「就绪即返回」的无限超时用例覆盖四后端 |
| 2 | io_uring 环境受限 | 中 | 容器/旧内核可能禁用。`available` 探测 + `EventLoop` 不选用；已实测本机 `available==1` 并通过 echo/HTTP |
| 3 | io_uring 一次性 poll 的重挂/陈旧完成 | 中 | `user_data` 打包 gen，CQE 按 gen 校验；DEL 不主动 cancel，靠 gen 忽略；fd 关闭触发完成并被忽略 |
| 4 | select `FD_SETSIZE=1024` | 低 | 固有上限，文档化；高并发用 epoll/io_uring |
| 5 | extern 句柄生命周期 | 中 | 非 ARC，需显式 free；连接路径统一 free；漏 free 退化为泄漏不悬垂 |
| 6 | `Serve` 阻塞退出 | 低 | `active` 归零自然退出；`serve_n`/`shutdown` 收敛；测试用 `serve_n` 避免弃置挂起 Fiber（debug 下 Fiber dtor 会 panic） |
| 7 | kqueue 未实现 | 低 | 明确报告 unavailable，不广告不可用后端；ABI 预留 |
| 8 | 无 TLS/HTTP2 | 低 | 非目标；面向内网/压测/示例 |

---

## 12. 验收与测试

### 12.1 分层测试

- **rt 单元/集成**（`crates/sloth-rt/tests/net_smoke.rs`，9 用例）：四后端 socketpair 可读/可写、TCP loopback accept/connect/send/recv、UDP echo、字节缓冲、地址解析、eventfd 唤醒打断阻塞 wait、无限超时（`-1`）就绪即返回、后端命名/可用性。
- **spec**（`crates/slothc/tests/spec/`）：`119_net_api`（无网络：bytes/clock/后端常量/地址/字符串工具）、`120_event_queue`（定时器顺序 + 四后端 fiber TCP echo + `sloth_rc_live` 回落）、`121_http_parse`（解析/响应/路由 + 四后端端到端 200）。
- **示例**（`examples/net/run.sh`）：`event_backends`、`tcp_echo_fiber`、`udp_echo`、`http_server`、`tcp_echo_threads`（thread-per-connection）全 PASS。

### 12.2 双后端

`slothc run`（JIT）与 `slothc build`（AOT，clang -O3 链接 `libsloth_rt.so`）均验证（udp_echo / tcp_echo_threads AOT 通过）。

### 12.3 基线

`cargo test --workspace`：codegen 207 + frontend 20 + rt（3 单元 + mt_smoke 5 + net_smoke 9 + rt_smoke 5）+ spec 2 runner 全绿；`examples/net/run.sh` 全 PASS。本机四后端（select/poll/epoll/io_uring）逐项通过，`available(kqueue)==false`。

---

## 13. 路线图（IO-P0…IO-P5，均已落地）

| 阶段 | 内容 | 验收 |
| --- | --- | --- |
| **IO-P0** | rt：bytes/time/net + event(select/poll/epoll) + EvBuf；`strings::find` | net_smoke 后端就绪/TCP/UDP/字节/地址 |
| **IO-P1** | io_uring 后端（裸 syscall）+ kqueue 保留/可用性/命名 | 四后端矩阵；kqueue 在 Linux 报 unavailable |
| **IO-P2** | `io.slt`、`net.slt` | spec 119；loopback echo |
| **IO-P3** | `event.slt` 事件队列 + 定时器 + fiber reactor | spec 120（定时器顺序、fiber echo、rc 回落） |
| **IO-P4** | `http.slt` 解析/路由/服务器 | spec 121 + examples/net |
| **IO-P5** | examples/net + run.sh + 文档 | 五示例 JIT/AOT 全 PASS |

**后续（非本版）**：kqueue 后端；定时器最小堆；HTTP chunked 与 query/percent 解码；TLS（需引入密码库，另立扩展）；io_uring 由「就绪」升级为「直接收发」（`IORING_OP_RECV/SEND`）；`SLOTH_STDLIB` 安装布局与文档。

---

## 14. 与主文档的变更清单（回写项）

| 位置 | 变更 |
| --- | --- |
| 主文档 §1.2 运行时组件 | 追加 bytes/time/net/event 四模块 |
| 主文档 §2.1 类型全集 | 不新增语言类型；`Bytes`/`EvLoop`/`EvBuf`/`SockAddr` 为 `extern type`（运行时句柄，非类型系统成员） |
| 主文档 §5.4 FFI | 追加「I/O 句柄以 `extern type` 暴露，非 ARC 管理，须显式释放」 |
| 主文档 §8 路线图 | 追加 IO 阶段（本文档 §13） |
| 主文档 §7 风险表 | 追加本文档 §11 |
| 协程扩展 §4.3 | 明确事件循环以 `fiber.resume/yield` 驱动，载荷转移协议不变 |
| 多线程扩展 §5.4 | 多线程服务器 = 每线程独立 EventLoop + SO_REUSEPORT；Fiber 线程封闭不变 |

---

## 附录 A：API 一览

### A.1 io.slt

```
bytes_new/len/cap/ensure/set_len/get/set/fill/append/copy_from_str/to_str/as_str/free
now_ms/sleep_ms/errno/would_block/conn_closed
kind_select/kind_poll/kind_epoll/kind_kqueue/kind_iouring
ev_add/ev_mod/ev_del  ev_read/ev_write/ev_err/ev_hup
backend_available/backend_name/ev_new/ev_free/ev_wakeup/ev_ctl/ev_poll
evbuf_new/free/count/token/events
str_find/str_starts_with/str_slice/str_len/str_byte/str_of_byte/str_cmp
```

### A.2 net.slt

```
af_inet/af_inet6/sock_stream/sock_dgram
addr_new/free/set/ip/port
class TcpListener  { raw_fd/port_num/set_blocking/try_accept/close }
tcp_listen(host,port,backlog,reuseport) -> TcpListener
tcp_listen_blocking(host,port,backlog) -> TcpListener
class TcpStream    { raw_fd/set_blocking/set_nonblocking/set_nodelay/try_read/try_write/write_str/shutdown_both/close }
tcp_connect / tcp_connect_blocking / stream_from_fd
class UdpSocket    { raw_fd/try_recv_from/send_to/close }
udp_bind(host,port,reuseport) -> UdpSocket
local_port(fd) / peer_addr(fd, a)
```

### A.3 event.slt

```
class Wait          { kind, fd, deadline }
wait_read/write/accept/sleep/yield
class EventLoop     { spawn/run/shutdown/close/ok/backend }
await_readable/await_writable/await_accept/sleep
read_some/write_all/write_str_all/accept_blocking
```

### A.4 http.slt

```
class Request   { method, path, version, headers, body; query/route/header }
class Response  { status, body, headers; set_header/have/serialize }
class Router    { get/post/route/dispatch }
parse_request(raw) -> Request?   reason(code)   text/html/not_found
serve(loop, listener, router)    serve_n(loop, listener, router, max)
```

---

## 附录 B：最小 HTTP 服务器示例

```rust
import "sloth/io.slt";
import "sloth/net.slt";
import "sloth/event.slt";
import "sloth/http.slt";

func main(): unit {
    let loop = EventLoop(kind_epoll());
    let lst = tcp_listen("0.0.0.0", 8080, 1024, false);

    var router = Router();
    router.get("/", |q: Request| -> Response {
        return html("<h1>sloth</h1>");
    });
    router.post("/echo", |q: Request| -> Response {
        return text(200, q.body);
    });

    loop.spawn(|w: Wait| -> unit {
        serve(loop, lst, router);          // 每连接一个 fiber，keep-alive
    });
    loop.run();
}
```

## 附录 C：关键运行时结构（Rust 侧草图）

```rust
pub struct EvLoop {
    kind: i64,          // SELECT/POLL/EPOLL/KQUEUE/IOURING
    epfd: i32,          // epoll/kqueue fd（-1 表示不用）
    wake_fd: i32,       // eventfd：跨线程唤醒阻塞 wait
    regs: Vec<Reg>,     // { fd, events, token, gen }（poll/select/uring）
    by_token: HashMap<i64, usize>,
    ring: Option<Box<IoUring>>,   // io_uring（Linux）
}

pub struct EvBuf {      // sloth_ev_poll 的输出
    cap: usize, count: i64,
    tokens: *mut i64, revents: *mut i64,
}
```
