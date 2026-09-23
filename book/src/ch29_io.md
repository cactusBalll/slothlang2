# 29. I/O 扩展：事件队列与网络

在 TH 扩展（第 28 章）之后，sloth2 已具备确定性 ARC、有栈协程与共享内存线程，
但**没有任何 I/O 能力**：既无 socket 面，也无事件循环，因此无法承载网络服务。
本章对应 **IO 扩展**：补齐从 syscall 到 TCP/UDP/HTTP 服务器的完整一层。

设计上延续项目的既有哲学——**机制进 rt，策略用 sloth，不新增语法**：Rust 运行时
把五种多路复用机制统一到极小的 C-ABI 之后；**事件队列本身用 slothlang2 实现**；
服务器在队列之上以 Fiber 与 Thread 组织。

## 29.1 分层：机制（Rust）与策略（sloth）

| 层 | 位置 | 职责 |
| --- | --- | --- |
| 机制 | `crates/sloth-rt/src/{bytes,time,net,event}.rs` | 字节缓冲、单调时钟、socket/sockaddr、五后端统一等待 |
| 策略 | `lib/sloth/{io,net,event,http}.slt` | 事件队列（reactor）、阻塞风格 API、TCP/UDP/HTTP 服务器 |

这与既有分层一致（对照 §5.3 容器方法、张量内核、CE/TH 内建）。之所以**不**把网络
做成 `net.*` 编译器内建：`fs`/`tensor` 已证明 `extern func` 足以暴露 syscall，
内建只会动前端与 `Ty` 却只换来语法糖；而事件队列用 sloth 写，正是本扩展的显式
目标。

## 29.2 不透明类型与所有权

I/O 句柄以 `extern type` 暴露：

| 类型 | 含义 | 释放 |
| --- | --- | --- |
| `Bytes` | 可增长字节缓冲（`malloc` + `realloc`） | `bytes_free` |
| `EvLoop` | 事件后端句柄（epoll fd / io_uring ring / 注册表） | `ev_free` |
| `EvBuf` | `ev_poll` 的 token/revents 输出缓冲 | `evbuf_free` |
| `SockAddr` | `sockaddr_storage` + 长度 | `addr_free` |

`register_import` 把它们登记进 `extern_types`，`is_ref` 返回 false，因此 codegen
**不会**发射 `retain`/`release`——这正是「原始 C 指针不得被 ARC 触碰」这条硬约束
的直接应用（去 tag 迁移后由 `PLAN §14` 的修正保证）。代价是句柄生命周期须显式
管理：忘记 `*_free` 退化为**泄漏**而非悬垂，与「漏插 release 退化为滞留」一致。

## 29.3 统一事件 ABI

```
kind     : SELECT=0 POLL=1 EPOLL=2 KQUEUE=3 IOURING=4
op       : ADD=1 MOD=2 DEL=3
interest : READ=1 WRITE=2     revents 追加 ERR=4 HUP=8

EvLoop* sloth_ev_new(kind)                     // 0 = 不支持/失败
i64     sloth_ev_ctl(ev, op, fd, events, token)
i64     sloth_ev_poll(ev, timeout_ms, EvBuf*)  // 就绪数或 -errno
i64     sloth_ev_wakeup(ev)                    // eventfd 唤醒阻塞 wait
```

`token` 是不透明 i64，由 sloth 侧编码、rt 只回传。所有可能失败的 net/event 调用
返回 `-errno`（或非负 fd/计数），sloth 侧用 `would_block(r)`
（`EAGAIN`/`EWOULDBLOCK`/`EINPROGRESS`）与 `conn_closed(r)`
（`ECONNRESET`/`EPIPE`/…）分类；`EINTR` 在 wait 层被吸收为「0 个事件」。超时
`-1` 表示无限等待。

## 29.4 五后端的实现原理

所有实现都在 `crates/sloth-rt/src/event.rs`，共享同一份循环状态与注册表；后端
差异只体现在 `sloth_ev_ctl`（如何把 fd 挂进内核/登记表）与 `sloth_ev_poll`
（如何取回就绪集）这两个调用上。

### 29.4.1 共享骨架：`EvLoop`、注册表与 eventfd 唤醒

```rust
struct Reg { fd: i32, events: i64, token: i64, gen: u32 }

pub struct EvLoop {
    kind: i64,
    epfd: i32,                      // epoll/kqueue fd（-1 表示不用）
    wake_fd: i32,                   // eventfd：跨线程唤醒阻塞 wait
    regs: Vec<Reg>,                 // poll/select/io_uring 依赖的注册表
    by_token: HashMap<i64, usize>,  // token -> regs 下标
    #[cfg(target_os = "linux")]
    ring: Option<Box<IoUring>>,
}
```

`poll`/`select`/`io_uring` 没有内核侧的长久注册态，因此把 `{fd, events, token, gen}`
保存在 `regs` 里，每次 wait 时重建/重挂；`epoll` 由内核保管，`regs` 仍保留用于
`gen` 校验与 io_uring 重挂。`wake_fd` 是一个非阻塞 `eventfd`，被作为
哨兵 `TOKEN_WAKE = i64::MIN` 注册进每个后端；`shutdown()` 对 fd 写 1 字节即可
打断任意后端的阻塞 wait：

```rust
pub extern "C" fn sloth_ev_wakeup(h: i64) -> i64 {
    let ev = &*(h as *const EvLoop);
    let v: u64 = 1;
    let _ = libc::write(ev.wake_fd, &v as *const u64 as *const libc::c_void, 8);
    0
}
```

三个入口的分发也非常薄：`sloth_ev_ctl` 对 epoll 直接转发 `epoll_ctl`，对注册表
后端维护 `regs`/`by_token`（io_uring 额外 `arm`）；`sloth_ev_poll` 按 `kind`
调用对应 `poll_*`：

```rust
pub extern "C" fn sloth_ev_poll(h: i64, timeout_ms: i64, out: i64) -> i64 {
    let ev = match unsafe { loop_of(h) } {
        Some(e) => e,
        None => return -(libc::EINVAL as i64),
    };
    ...
    match ev.kind {
        EV_EPOLL   => poll_epoll(ev, timeout_ms, out),
        EV_POLL    => poll_poll(ev, timeout_ms, out),
        EV_SELECT  => poll_select(ev, timeout_ms, out),
        EV_IOURING => poll_iouring(ev, timeout_ms, out),
        _ => -(libc::ENOSYS as i64),
    }
}
```

`EvBuf` 是 `sloth_ev_poll` 的输出缓冲（`cap/count/tokens/revents`），rt 只负责
把 `(token, revents)` 追加进去，再由 sloth 侧解码。

### 29.4.2 select：自建 `fd_set` 位图

`select` 每次 wait 都要重建读/写集合，且 `FD_SETSIZE=1024`。这里用 16×`u64`
的 `FdSet` 镜像 `libc::fd_set` 的 64 位 Linux 布局，避免依赖 `FD_SET` 宏：

```rust
#[repr(C)]
struct FdSet { bits: [u64; 16] }            // 1024 位
impl FdSet {
    fn set(&mut self, fd: i32) { let f = fd as usize; self.bits[f/64] |= 1u64 << (f%64); }
    fn isset(&self, fd: i32) -> bool { let f = fd as usize; self.bits[f/64] & (1u64 << (f%64)) != 0 }
}
```

```rust
fn poll_select(ev: &mut EvLoop, timeout_ms: i64, out: i64) -> i64 {
    let mut rfds = FdSet::zero();
    let mut wfds = FdSet::zero();
    let mut maxfd = ev.wake_fd;
    rfds.set(ev.wake_fd);
    for r in &ev.regs {
        if r.fd < 0 || r.fd as usize >= libc::FD_SETSIZE { continue; }  // 固有上限
        if r.events & R_READ  != 0 { rfds.set(r.fd); }
        if r.events & R_WRITE != 0 { wfds.set(r.fd); }
        if r.fd > maxfd { maxfd = r.fd; }
    }
    // 无限超时（-1）必须传 NULL：负 timeval 会被 select 判 EINVAL
    let mut tv = libc::timeval { tv_sec: timeout_ms.max(0)/1000,
                                 tv_usec: (timeout_ms.max(0)%1000)*1000 };
    let tvp = if timeout_ms < 0 { std::ptr::null_mut() } else { &mut tv as *mut _ };
    let r = libc::select(maxfd + 1,
                         &mut rfds as *mut FdSet as *mut libc::fd_set,
                         &mut wfds as *mut FdSet as *mut libc::fd_set,
                         std::ptr::null_mut(), tvp);
    ...
}
```

取回时遍历 `regs`，查位图得到 READ/WRITE。注意 select 的“无限超时传 `NULL`”
是过程中修掉的真 bug（风险 #1）。

### 29.4.3 poll：按注册表构造 `pollfd[]`

```rust
fn poll_poll(ev: &mut EvLoop, timeout_ms: i64, out: i64) -> i64 {
    let mut pfds: Vec<libc::pollfd> = Vec::with_capacity(ev.regs.len() + 1);
    let mut tokens: Vec<i64> = Vec::with_capacity(ev.regs.len() + 1);
    pfds.push(libc::pollfd { fd: ev.wake_fd, events: libc::POLLIN, revents: 0 });
    tokens.push(TOKEN_WAKE);                       // 哨兵：唤醒项
    for r in &ev.regs {
        let mut pe = 0i16;
        if r.events & R_READ  != 0 { pe |= libc::POLLIN;  }
        if r.events & R_WRITE != 0 { pe |= libc::POLLOUT; }
        pfds.push(libc::pollfd { fd: r.fd, events: pe, revents: 0 });
        tokens.push(r.token);
    }
    let r = libc::poll(pfds.as_mut_ptr(), pfds.len() as libc::nfds_t, timeout_ms as libc::c_int);
    ...
    // 逐项把 revents 映射回 READ/WRITE/ERR/HUP，token 从并行的 tokens[] 取回
}
```

`poll` 与 `select` 的区别只在于用数组而非位图，因此没有 1024 上限，但每次
wait 是 O(注册数)。

### 29.4.4 epoll：内核保管状态，token 进 `epoll_data`

epoll 是 Linux 主线：注册一次后由内核维护兴趣集，`token` 直接写进
`epoll_event.u64`，wait 无需重建集合。兴趣/事件位映射：

```rust
fn map_interest(events: i64) -> i32 {          // 我方位 -> EPOLL*
    let mut m = 0;
    if events & R_READ  != 0 { m |= libc::EPOLLIN;  }
    if events & R_WRITE != 0 { m |= libc::EPOLLOUT; }
    m
}
fn unmap_interest(revents: i32) -> i64 {       // EPOLL* -> 我方位
    let mut r = 0i64;
    if revents & (libc::EPOLLIN | libc::EPOLLPRI) != 0 { r |= R_READ; }
    if revents & libc::EPOLLOUT != 0 { r |= R_WRITE; }
    if revents & libc::EPOLLERR != 0 { r |= R_ERR; }
    if revents & (libc::EPOLLHUP | libc::EPOLLRDHUP) != 0 { r |= R_HUP; }
    r
}
```

`ctl` 把 ADD/MOD/DEL 转发给 `epoll_ctl`，并在兴趣位上强制附加
`EPOLLERR|EPOLLHUP|EPOLLRDHUP`（错误与对端关闭必须能被观察到）：

```rust
let mut e = libc::epoll_event {
    events: (map_interest(events) | libc::EPOLLERR | libc::EPOLLHUP | libc::EPOLLRDHUP) as u32,
    u64: token as u64,
};
libc::epoll_ctl(ev.epfd, eop, fd as i32, &mut e);
```

创建时顺带把 `wake_fd` 以 `TOKEN_WAKE` 注册进去，wait 用 `epoll_wait`，遇到
wake token 就 `read(eventfd)` 清零后跳过：

```rust
let n = libc::epoll_wait(ev.epfd, evs.as_mut_ptr(), evs.len() as i32, timeout_ms as i32);
for i in 0..n as usize {
    let token = evs[i].u64 as i64;
    if token == TOKEN_WAKE { drain_eventfd(ev.wake_fd); continue; }
    evbuf_push(buf, token, unmap_interest(evs[i].events as i32));
}
```

`EINTR` 统一吸收为「0 个事件」，避免把信号打断暴露给 sloth 层。

### 29.4.5 io_uring：裸 syscall + mmap 环

io_uring 不链接 `liburing`，而是直接
`io_uring_setup` / `io_uring_enter` 并通过三次 `mmap` 建立 SQ ring、CQ ring 与
SQE 数组（偏移由 `io_uring_params` 给出）：

```rust
let fd = libc::syscall(libc::SYS_io_uring_setup, entries as c_long, &mut params as *mut _);
let sq_sz  = (params.sq_off.array + params.sq_entries * 4) as usize;
let cq_sz  = (params.cq_off.cqes  + params.cq_entries * 16) as usize;
let sqes_sz = params.sq_entries as usize * 64;
let map = |len, off| libc::mmap(std::ptr::null_mut(), len, libc::PROT_READ|libc::PROT_WRITE,
                                libc::MAP_SHARED|libc::MAP_POPULATE, fd, off);
```

入队一条 SQE 需要写 SQE 数组、把索引写进 SQ array，再推进 tail；提交用
`io_uring_enter`（只提交不等待）：

```rust
unsafe fn queue(&mut self, sqe: &Sqe) -> bool {
    let tail = *self.sq_tail();
    if tail.wrapping_sub(*self.sq_head()) >= self.params.sq_entries { return false; }
    let idx = tail & self.sq_mask();
    std::ptr::write(self.sqes.add(idx as usize * 64) as *mut Sqe, *sqe);
    *self.sq_array().add(idx as usize) = idx;
    *self.sq_tail() = tail.wrapping_add(1);
    true
}
unsafe fn flush(&mut self) {
    let to_submit = *self.sq_tail() - self.submitted_tail;
    if to_submit != 0 {
        libc::syscall(libc::SYS_io_uring_enter, self.fd, to_submit, 0, 0, 0, 0);
        self.submitted_tail = *self.sq_tail();
    }
}
```

由于 `IORING_OP_POLL_ADD` 是**一次性**的，注册一个 fd 就是提交一条
`POLL_ADD`，并把 `(gen, token)` 打包进 `user_data`：

```rust
let sqe = Sqe {
    opcode: IORING_OP_POLL_ADD,           // = 6
    fd,
    poll32_events: pe,                    // POLLIN/POLLOUT
    user_data: pack(gen, token),          // (gen<<32)|(token as u32)
    ..Default::default()
};
queue(&sqe); flush();                     // 环满时先 flush 再重试
```

等待：ring fd 自内核 ≥5.5 起可被 `poll`，因此先排空已完成的 CQ，再用
`poll(ring_fd, POLLIN, timeout)` 等新完成。CQE 的 `res` 是 poll 掩码，按
`gen` 校验是否仍是最新注册；命中则产出事件并**重挂**（gen+1）：

```rust
unsafe fn drain(&mut self, buf: &mut EvBuf, regs: &[Reg]) {
    loop {
        let head = *self.cq_head();
        if head == *self.cq_tail() { break; }
        let cqe = std::ptr::read(self.cqes().add((head & self.cq_mask()) as usize));
        *self.cq_head() = head.wrapping_add(1);
        if cqe.res < 0 { continue; }                 // -ECANCELED/-EBADF：陈旧完成，忽略
        let (gen, token) = unpack(cqe.user_data);
        let re = /* cqe.res 掩码 -> READ/WRITE/ERR/HUP */;
        match regs.iter().find(|r| r.token == token) {
            Some(r) if r.gen == gen => {
                evbuf_push(buf, token, re);
                self.arm(r.fd, r.events, token, gen.wrapping_add(1).max(1));  // 重挂
            }
            _ => {}                                  // DEL 后的陈旧完成
        }
    }
}
```

`DEL` 不主动 `IORING_OP_POLL_REMOVE`：注册表移除后，陈旧 CQE 会因 `token`/`gen`
不匹配而被忽略；fd 关闭触发的完成同样被忽略。容器/旧内核上
`sloth_ev_available(IOURING)` 先探测 `io_uring_setup`，失败即不选用。

### 29.4.6 kqueue（保留）

`EV_KQUEUE` 的 kind 与 API 位保留，但首版对 kqueue 恒返回假——避免「广告了
后端却在 wait 时失败」。BSD 环境只需补 `poll_kqueue` 并放开探测，sloth 侧与
`EvLoop` 骨架无需改动。

### 29.4.7 后端对照

| 后端 | 注册态 | 每次 wait 开销 | 上限 | 唤醒机制 | 备注 |
| --- | --- | --- | --- | --- | --- |
| select | `regs` 重建位图 | O(注册数) | `FD_SETSIZE=1024` | eventfd 位 | 无限超时须 `NULL` |
| poll | `regs` 重建数组 | O(注册数) | 无 | 哨兵 pollfd | 可移植 |
| epoll | 内核 `epoll_ctl` | O(就绪数) | 无 | wake_fd 注册 | Linux 主线 |
| io_uring | `regs` + 一次性 `POLL_ADD` | O(完成数) | 环大小（64） | ring fd 可 poll | 裸 syscall，可降级 |
| kqueue | — | — | — | — | 保留，Linux 不可用 |

## 29.5 事件队列（`event.slt`）

事件队列是一个经典的 **stackful-coroutine reactor**。任务 = `Fiber<Wait>`：当
一个操作会阻塞时，任务 `fiber.yield(W)` 交出一个 `Wait{kind, fd, deadline}`，
其中 `kind` 为 0 立即重排、1 可读、2 可写、3 accept、4 睡眠到 deadline。
`EventLoop` 是唯一的消费者：

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

`resume_slot(tok)` 先注销该槽上一次注册的 fd（`DEL`），再
`fiber.resume(f, wait_yield())`：返回 `nil` = 任务结束并释放槽；否则按返回的
`Wait` 注册 fd（`ADD`）或加入定时器链表。

**槽回收与 token 代际**：槽位复用会带来「陈旧事件唤醒新任务」的风险。token 编码
`slot*65536 + (gen%65536)`，每次分配 `slot_gen[slot]++`；`alive(tok)` 校验
`live[slot]==1 && gen 匹配`。就绪事件、定时器、resume 前都过 `alive`。io_uring 的
`user_data` 也带 gen，构成双保险。

`read_some` / `write_all` / `write_str_all` / `accept_blocking` 内部循环：非阻塞
调用 → `would_block` → `fiber.yield(对应 Wait)`。于是业务代码是顺序直写，事件
循环全程非阻塞——没有回调反转，也没有 async/await。

## 29.6 服务器模型

**Fiber 主线（每线程一个 loop）**：acceptor fiber 在 `accept_blocking` 后
`loop.spawn` 一个连接 fiber；多核扩展用 N 个线程各自 `EventLoop` +
**SO_REUSEPORT** 同端口监听，由内核分散连接，无共享锁。跨线程只用
`thread.*`/`Channel`/`AtomicInt`，Fiber 绝不跨线程。

**Thread 阻塞对照**：`net.slt` 另提供 `tcp_listen_blocking` /
`tcp_connect_blocking` / `set_blocking`，配合 `thread.spawn` 实现
thread-per-connection（`examples/net/tcp_echo_threads.sl`），用于低复杂度场景与
对照基准。

**UDP**：`udp_bind` + `try_recv_from` / `send_to`，服务器在 loop 上
`await_readable` 后收包回发，天然每 loop 一 socket。

**HTTP/1.1**：`read_request` 非阻塞读入 `Bytes`，以 `\r\n\r\n` 分帧并按
`Content-Length` 判断体是否读齐；`parse_request` 切请求行（method/path/version，
path 保 query）、头（键小写、值 trim）、体；`Router` 是
`Map<str, (Request)->Response>`，键 `"METHOD route"`，未命中 404；
`Response.serialize` 补 Content-Length/Connection；HTTP/1.1 且
`Connection != close` 时循环处理下一请求（keep-alive）。无 TLS/chunked/HTTP2。

## 29.7 计算卸载：fiber → OS 线程

事件循环要求任务不长时间占用 loop 线程，但计算密集任务无法改写为非阻塞。为此
`event.slt` 提供：

```rust
pub func run_blocking<T, R>(loop: EventLoop, entry: (T) -> R, arg: T): R
```

`entry(arg)` 在新建 OS 线程上运行（`thread.spawn`），当前 fiber 立即
`fiber.yield(wait_read(fd))`，loop 继续服务其他 fiber；该线程完成时经
`io.async_signal(fd)`（rt 的独立 eventfd）通知 loop，fiber 被唤醒后
`async_drain/free` 并 `handle.join()` 取回结果（此时线程已结束，join 不阻塞）。
`JoinHandle<R>` 作为 fiber 局部量跨越 yield 存活，因此**无需类型擦除容器**；
`T`/`R` 受 `Send` 约束。

## 29.8 编译器影响

本扩展**不新增内建、不动前端与 `Ty`**（全走 extern）。过程中修掉的 codegen 真 bug：

1. `keys(Map<K,V>)` 结果元素类型恒为 `int` → 按 `K` 推导；
2. lambda 捕获误判**模块限定调用**（`event.sleep(...)` 的模块名被当作变量）；
3. lambda 捕获漏扫**字符串插值** `${expr}` 中的自由变量；
4. 类字段读取丢失内建句柄类型（`JoinHandle`/`Channel`/`Fiber`/`Fn`/`Mutex`/
   `AtomicInt`/`Any`/`Range` 落到 `_ => int`）；
5. lambda 捕获函数值后无法调用（`syn_ty_of` 缺 `Ty::Fn`）；
6. 泛型参数无法从**函数型实参**推断（`unify_tp` 缺 `(Fn, Fn)`，`run_blocking`
   推断不出 `R`）；
7. 泛型单态化时 `tp_mangled` 泄漏进嵌套 lambda 体，不同实例共用同一 lambda；
8. 跨模块泛型函数未单态化（`lib.fn<T,U>(...)` 直呼模板）。

## 29.9 示例与验收

```rust
import "sloth/io.slt";
import "sloth/net.slt";
import "sloth/event.slt";
import "sloth/http.slt";

func main(): unit {
    let loop = EventLoop(kind_epoll());
    let lst = tcp_listen("0.0.0.0", 8080, 1024, false);

    var router = Router();
    router.get("/", |q: Request| -> Response { return html("<h1>sloth</h1>"); });
    router.post("/echo", |q: Request| -> Response { return text(200, q.body); });

    loop.spawn(|w: Wait| -> unit { serve(loop, lst, router); });
    loop.run();
}
```

| 用例 | 位置 |
| --- | --- |
| 无网络：字节缓冲/时钟/后端常量/地址/字符串工具 | `crates/slothc/tests/spec/119_net_api.sl` |
| 定时器顺序 + 四后端 fiber TCP echo + `rc_live` 回落 | `120_event_queue.sl` |
| HTTP 解析/响应/路由 + 四后端端到端 200 | `121_http_parse.sl` |
| 四后端计算卸载（结果正确 + loop 未阻塞） | `122_thread_offload.sl` |
| rt 集成（四后端 socketpair、TCP/UDP、eventfd 唤醒、无限超时） | `crates/sloth-rt/tests/net_smoke.rs` |
| 六示例（含 AOT） | `examples/net/run.sh` |

JIT（`slothc run`）与 AOT（`slothc build`，clang -O3 链接 `libsloth_rt.so`）下行为
一致；本机四后端（select/poll/epoll/io_uring）逐项通过，`available(kqueue)==false`。
