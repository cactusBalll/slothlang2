# 26. 协程（fiber）

sloth 2.0 通过**原生有栈协程**恢复 1.0 的 `fiber` 能力：单线程 1:m、协作式调度、
无抢占。协程以**编译器内建模块 `fiber.*`** 提供，无新关键字、无新语法；每个协程
拥有独立的 `mmap` 栈（默认 256 KiB + 保护页），切换只保存/恢复 ABI 被调用者保存
寄存器与栈指针。

类型系统新增引用类型 `Fiber<Y>`：`Y` 是 `resume`/`yield` 双向共用的单一载荷类型。

## 26.1 API

| 函数 | 签名 | 语义 |
| --- | --- | --- |
| `fiber.create(f, init)` | `((Y) -> unit, Y) -> Fiber<Y>` | 新建协程，入口闭包 `f`，`init` 为入口参数 |
| `fiber.create_with(f, init, n)` | `((Y) -> unit, Y, int) -> Fiber<Y>` | 同上，自定义栈字节数 |
| `fiber.resume(f, v)` | `(Fiber<Y>, Y) -> Y?` | 恢复；`yield` 出值→`Y`，完成/出错→`nil` |
| `fiber.yield(v)` | `(Y) -> Y` | 挂起本协程，返回下次 `resume` 传入的值 |
| `fiber.transfer(f, v)` | `(Fiber<Y>, Y) -> Y?` | 对称切换，不触碰 `prev` 链 |
| `fiber.error(msg)` | `(str) -> unit` | 置 Error、打印诊断并切回 `prev` |
| `fiber.check(f)` | `(Fiber<Y>) -> bool` | 是否处于 Error |
| `fiber.resumable(f)` | `(Fiber<Y>) -> bool` | New/Suspended 可恢复 |
| `fiber.cancel(f)` | `(Fiber<Y>) -> unit` | 协作式取消挂起协程 |

状态机与 1.0 一致：`New →resume→ Running →yield→ Suspended →resume→ …→ Done`，
出错走 `Error`。

## 26.2 示例

```rust
func main(): unit {
    let f = fiber.create(|init: int| -> unit {
        var i = 0;
        while (i < 10) {
            i = fiber.yield(i);          // 交出 i，收回 resume 的值
            i = i + 1;
            if (i > 5) {
                fiber.error("i exceeded 5");
            }
        }
    }, 0);

    var cnt = 0;
    while (not fiber.check(f)) {
        cnt = cnt + 3;
        let got = fiber.resume(f, cnt);  // Y?：完成/出错时为 nil
        print("got i from fiber: ${got ?: -1}");
    }
}
```

`Y` 为值类型（`int`/`float`/`bool`）时，`Y?` 结果走一词 payload 盒，因此「yield 出
的值 `0`」与「完成 `nil`」不会混淆；`Y` 为引用类型时直接复用句柄词。

## 26.3 载荷与 ARC

载荷按**普通 borrowed 实参**递交，由运行时 `retain` 接管并在接收侧交付；因此协程
挂起栈帧中已 `retain` 的引用由计数**自然保活**，无需任何根枚举——这也是 ARC 相对
1.0 GC 架构对协程的决定性简化。

```rust
func churn(n: int): unit {
    var i = 0;
    while i < n {
        let f = fiber.create(|init: str| -> unit {
            let a = fiber.yield("a${init}");
            let b = fiber.yield("b${init}");
        }, "seed${i}");
        var k = 0;
        while k < 3 { let g = fiber.resume(f, "x${k}"); k = k + 1; }
        i = i + 1;
    }
}
```

驱动至 `Done` 后，协程对象、入口闭包与所有载荷引用会精确回落到 `sloth_rc_live()`
基线。

## 26.4 取消与限制

- `fiber.cancel(f)` 对 Suspended 协程置取消标志并注入哨兵；其下一次 `fiber.yield`
  经 `setjmp`/`longjmp` 回到协程入口，栈被回收。**出栈前会结算所有在册的局部引用槽**
  （局部槽在声明/退出处逐帧登记，仅协程上下文生效），因此取消不泄漏引用。
- **弃置挂起协程**：一个 Suspended 的 `Fiber` 计数归零时，debug 构建在析构处
  panic（`abandoned suspended fiber`）以暴露问题；release 构建在析构时结算在册
  局部槽、回收对象与栈。协程仍应驱动至 `Done`/`Error`，或显式 `fiber.cancel` 收尾。
- 入口函数受一等函数值既有规则限制（泛型/可变参/`extern`/跨模块函数需闭包包装）。
- **固定栈 + 保护页**：栈溢出触发 `SIGSEGV`，运行时信号处理器打印
  `fiber stack overflow` 并终止；深递归用 `fiber.create_with` 调大栈。
- 仅覆盖 x86_64 SysV 与 aarch64 AAPCS64。

## 26.5 JIT 与 AOT

切换发生在原生栈指针层面，与被切换代码的出身（ORC JIT 或 AOT 目标文件）无关，
两种模式行为一致。示例见 `examples/fiber/`，端到端断言见
`crates/slothc/tests/spec/105_fiber.sl` 与 `106_fiber_ownership.sl`。
