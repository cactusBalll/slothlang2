# sloth-lang 2.0 有栈协程扩展设计文档

> **去 tag 迁移（2026-09-19，PLAN §14）**：词面已改为**无 tag 单个 i64**（引用=
> 裸指针、int=原生 i64、float=原生 f64 位、bool=0/1、nil=0）。本文中 `ptr|1`、
> 「带 tag 的 i64 词」、「tagged fnptr」、「63 bit」等表述均为迁移前记录；
> `FiberObj` 现含 `eref` 位以区分值/引用载荷（见主设计文档 §2.6）。

## 基于原生栈切换的 stackful coroutine（fiber 2.0）

| 项目 | 内容 |
| --- | --- |
| 文档版本 | v1.0（草案） |
| 文档日期 | 2026-09-18 |
| 上游文档 | 《sloth-lang 2.0 设计文档》v1.1（与实现对齐稿，下称「主文档」） |
| 语义基准 | sloth-lang 1.0 手册 §3.5 用户态协程（fiber） |
| 文档状态 | 设计评审稿 |

---

## 1. 概述

### 1.1 定位与背景

主文档 v1.0 的前置决策「移除用户态协程（fiber）」基于两点阻碍：**fiber 与 LLVM 无栈协程模型的错配**、**fiber 栈与 GC 根扫描的交互**。v1.1 架构改向后，这两点前提已不成立：

1. **ARC 替代 GC**（主文档 §5.1）：对象存活由引用计数决定，`retain`/`release` 插入点在发射期静态确定。**挂起协程原生栈帧中持有的引用，其计数早已 +1，无需任何栈扫描/栈图机制**——有栈协程与 ARC 天然兼容；
2. **直发标准 dialect + 原生 ABI**（主文档 §4.3）：所有函数就是普通原生函数，局部槽是 `memref<1xi64>` alloca，调用约定与宿主一致。有栈协程的实现不需要 LLVM coroutine intrinsic 的语义配合——**运行时分配独立栈 + 汇编上下文切换**即可，协程代码与非协程代码的机器码完全相同。

因此本扩展以**原生有栈协程**恢复 1.0 的 fiber 能力：1:m 模型、协作式调度、无抢占，语义对齐 1.0 手册 §3.5，类型化后纳入 sloth2 类型系统。

### 1.2 设计目标

1. **语义对齐 1.0**：`create/resume/yield/error/check/resumable/transfer` 七函数 API 完整恢复（类型化改写）；
2. **零新关键字**：延续 1.0 哲学，协程能力以**内建模块 `fiber`** 形式提供（与 `arr.len()` 同款：表面是普通调用，发射器识别后直接发射运行时调用，见 §3.3）；
3. **双后端可用**：JIT（ORC ExecutionEngine）与 AOT（clang -O3）下行为一致；
4. **与 ARC 所有权协议无缝衔接**：跨切换边界的载荷词按「转移」语义交接（§4.3），不破坏 §5.1.1 的八条协议规则。

### 1.3 非目标

- 不实现抢占式调度、多线程 fiber 迁移（单线程模型不变，线程仍留待 3.0）；
- 不提供内建调度器/事件循环（`fiber.transfer` 足够用户自实现，同 1.0）；
- 不支持 Windows（首版仅 x86_64 SysV 与 aarch64 AAPCS64 两个 ABI 的切换汇编）；
- 不改变错误模型：协程内 panic 仍然终止进程（§4.4）。

---

## 2. 语义设计：与 1.0 fiber 的对照

### 2.1 状态机

与 1.0 完全一致的协程状态：

```
New ──resume──► Running ──yield──► Suspended ──resume──► Running ──► （函数返回）Done
                    │
                    ├─ fiber.error() ─► Error
                    └─ panic ─►（进程终止，见 §4.4）
```

- `resume` 回到协程并将控制转移给它；协程 `yield` 或返回后，控制回到 **`prev` 链上的前一个协程**（1.0 语义：主协程的 prev 为空，主协程结束即进程结束——sloth2 中 `@sloth_main` 即主协程）；
- `fiber.transfer` 直接转移控制、**不触碰 prev 链**，供自定义调度器使用；
- 1.0 中 yield/resume 的动态 `Value` 载荷，在 sloth2 中类型化为**单类型载荷 `Y`**（见 §3.1）。

### 2.2 与 1.0 API 的对照

| 1.0（动态） | 2.0（类型化） | 语义变化 |
| --- | --- | --- |
| `fiber.create(func, argv…)` | `fiber.create(f: (Y) -> unit, init: Y): Fiber<Y>` | 可变参数入口收敛为**单个载荷参数** `init`；入口函数签名固定为 `(Y) -> unit` |
| `fiber.resume(fiber, v)` | `fiber.resume(f: Fiber<Y>, v: Y): Y?` | 返回 `Y?`：协程 yield 出值→`Y`；协程**完成/出错**→`nil`（替代 1.0 返回 Nil 的约定，类型化为可空） |
| `fiber.yield(v)` | `fiber.yield(v: Y): Y` | 挂起本协程，返回下次 resume 传入的值 |
| `fiber.error()` | `fiber.error(msg: str): unit` | 标记 Error 并回到 prev；`msg` 打印诊断（1.0 无参数，扩展为带消息） |
| `fiber.check(fiber)` | `fiber.check(f: Fiber<Y>): bool` | 不变（是否处于 Error） |
| `fiber.resumable(fiber)` | `fiber.resumable(f: Fiber<Y>): bool` | 不变（New/Suspended 可恢复） |
| `fiber.transfer(fiber)` | `fiber.transfer(f: Fiber<Y>, v: Y): Y?` | 不变（对称协程原语，不触碰 prev） |

关键类型化决策：**yield 与 resume 的载荷共用同一类型 `Y`**。1.0 的双向动态值在静态化后若拆成 `Y`（出）/`R`（入）两个类型参数，会迫使几乎所有调用点显式双标注（`fiber.yield` 的返回类型无法从局部推断——发射器单趟推断的约束，主文档 §2.2）。单类型 `Y` 是表达力与实现复杂度的平衡点；需要双向不同类型时用 `Either` 风格的标准库类装箱。

---

## 3. 语言层设计

### 3.1 新类型 `Fiber<Y>`

- 引用类型，ARC 管理，词面 ABI 中为普通句柄词（`ptr|1`），补入主文档 §2.1 类型全集与 §2.6 引用类型表；
- 运行时对象布局（payload）：`[state, prev, stack_base, stack_size, ctx, entry_closure, yield_slot, owner_flags]`，其中 `ctx` 为切换时保存的寄存器上下文区，`entry_closure` 持有入口闭包（`{tagged fnptr, env}` 两词 ARC 对象，主文档 §4.2.5）保活；
- `Fiber<Y>` 参与泛型单态化：不同 `Y` 的 `fiber.create` 各自实例化，运行时只关心词面宽度（恒为 i64），**运行时无泛型**。

### 3.2 使用示例

1.0 手册 §3.5 的协程示例改写为 sloth2：

```rust
// 内建模块，无需 import
pub func main(): unit {
    let f = fiber.create(|init: int| -> unit {
        var i = 0;
        var v = init;                    // 首次 yield 前的输入
        while (i < 10) {
            v = fiber.yield(i);          // 交出 i，收回 resume 的值
            print("v from main fiber: ${v}\n");
            i = i + 1;
            if (i > 5) {
                print("error in fiber\n");
                fiber.error("i exceeded 5");
            }
        }
    }, 0);

    var cnt = 0;
    while (!fiber.check(f)) {
        cnt = cnt + 3;
        print("resumable: ${fiber.resumable(f)}\n");
        let got = fiber.resume(f, cnt);  // Y?：完成/出错时为 nil
        print("got i from fiber: ${got ?: -1}\n");
    }
}
```

输出序列与 1.0 手册 §3.5 完全一致（`resumable/got/v from main fiber/error in fiber`，最后一轮 `got` 为 `nil` 经 `?:` 显示为 -1——1.0 输出 `Nil`）。

### 3.3 调用面与编译器识别

- **无新关键字、无新语法**；`Fiber` 加入上下文关键字清单（仅类型位置）；
- `fiber.*` 七个函数为**编译器内建**（发射期识别，直接发射 `@sloth_fiber_*` 运行时调用），与主文档 §5.3「`arr.len()` → `@sloth_arr_len`」同款机制。**不**以 `lib/sloth/fiber.slt` 普通标准库形式提供，原因：`resume`/`yield` 的载荷需要所有权转移的特殊插桩（§4.3），普通函数调用面的「形参 borrowed」规则无法满足；
- `Fiber<Y>` 类型进入 EBNF 的 `type_base` 产生式（§7 回写项）。

### 3.4 限制

- 入口函数受限于一等函数值的既有规则（主文档 §3.3）：**泛型/可变参/`extern`/跨模块函数不能直接作入口**，用非泛型闭包包装即可（闭包捕获语义不变：标量按值快照、引用按引用共享）；
- **协程内不允许的操作**：无语言级禁止项。张量算子、容器、闭包、类方法均可使用——全部状态要么在协程自己的栈上（随栈保存），要么在 ARC 堆上（计数保活）；
- `Fiber<Y>` 不可作为 `Map` 键（不实现 `Hashable`，回退指针恒等，编译期给提示，与主文档 §2.5 一致）。

---

## 4. 运行时设计（`crates/sloth-rt/src/fiber.rs`）

### 4.1 栈分配与上下文切换

- **独立栈**：`mmap` 分配，默认 256 KiB（`fiber.create_with(f, init, stack_bytes)` 可调），尾部一页 `mprotect(PROT_NONE)` 作为**保护页**——栈溢出触发 SIGSEGV，由运行时信号处理器转为带诊断的进程终止（与错误模型一致，不做栈增长，理由见 §4.5）；
- **上下文**：仅保存被调用者保存寄存器与栈指针（SysV x86_64：`rsp/rbp/rbx/r12–r15`；AAPCS64：`x19–x30/sp/lr`），自研汇编 `sloth_fiber_switch(save: *mut Ctx, restore: *const Ctx, payload: i64) -> i64`，约百行/架构；不保存浮点/向量 callee-saved 之外的寄存器（ABI 保证调用者保存寄存器无需保留）；
- **入口 trampoline**：首次 resume 时 switch 到以 `fiber_trampoline` 为返回地址的新栈帧；trampoline 从 `Fiber` 对象取出入口闭包，拆 `{fnptr, env}` 经 bridge 调用（复用主文档 §4.3.2 闭包调用机制），`init` 词作为实参；入口函数返回 → 标记 Done → 切回 prev；
- **实现选型**：自研汇编而非引入 `corosensei` 等依赖——切换面积极小，自研可控且无外部依赖（与项目「无解析器依赖」的简洁哲学一致）。

### 4.2 与词面 ABI 的关系

- yield/resume 的载荷就是带 tag 的 i64 词，经 `sloth_fiber_switch` 的 `payload` 参数/返回值原样穿过切换边界，**编解码零特殊处理**；
- 协程的局部槽（`memref<1xi64>` alloca）位于其**自己的栈**上，挂起期间该栈整体保留——alloca 语义天然正确，无需任何编译器改动；
- **栈内指针不外泄的不变量**：sloth2 中词面值、容器元素、闭包捕获、张量数据均为堆对象或按值词，不存在指向原生栈帧的逃逸指针（发射器不产生 `alloca` 地址的对外暴露）——因此固定栈 + 整体切换是安全的，无需栈复制/栈增长。

### 4.3 与 ARC 所有权协议的衔接（核心设计点）

主文档 §5.1.1 的协议中，函数形参为 **borrowed（+0）**——若把 `resume(f, v)` 当普通函数，`v` 的生命周期终结于 resume 返回，但协程恢复后要长期使用它。因此：

1. **跨边界载荷按「转移」语义**：发射器对 `fiber.resume`/`fiber.yield`/`fiber.transfer`/`fiber.create` 的载荷实参做特殊插桩——owned 临时量**转移**给运行时（注销临时登记、不插 release），等效于协议规则 6 的「转移」；若实参是借用值（槽/字段/形参），先物化一次 `retain` 再转移（等效规则 4 的返回面物化）；
2. **边界接收侧**：`fiber.yield` 的返回值、入口函数的 `init` 形参，发射为 **owned 临时量**（规则 5 的「调用结果即 owned」），后续生命周期完全走既有协议；
3. **挂起栈的保活**：挂起协程栈帧中的 owned 槽早已 `retain`（规则 2），对象计数非零，ARC 自然保活——**无需任何根枚举**。这正是 ARC 架构相对 1.0 GC 架构对协程的决定性简化（1.0 需要 Fiber.stack 作为 GC 根显式扫描）；
4. **Fiber 对象自身**：ARC 管理；`prev` 持有 prev 协程的强引用（主协程为哨兵静态对象，不计数）；
5. **已知妥协：弃置泄漏**。若一个 Suspended 状态的 `Fiber` 计数归零（不再被 resume），其栈帧中 retain 的引用**无法级联释放**（释放代码在各函数的作用域退出路径上，而控制流永远不会再回到那些帧）。对策：
   - debug 构建：rt 在 Fiber 析构时检测 `state == Suspended` → `@sloth_panic_*` 报「abandoned suspended fiber」，强制测试期暴露；
   - release 构建：回收 Fiber 对象与栈 mmap，栈上引用的对象**滞留**（泄漏但不悬垂，与主文档 §5.1「漏插 release 退化为滞留」的失败哲学一致）；
   - 文档要求：协程应驱动至 Done/Error；长期存活的可挂起任务用 `fiber.cancel(f)`（§4.4）显式收尾。

### 4.4 错误与取消

- **协程内 panic 终止进程**：与主文档 §5.5 一致，不做跨协程异常传播（无异常机制可传播）；`fiber.error(msg)` 是协作式出错的唯一出口，它将协程置为 Error、切回 prev，使该处 `resume` 返回 `nil`；
- **`fiber.cancel(f)`**（新增，1.0 无）：对 Suspended 协程置取消标志并 resume 注入一个哨兵载荷；**注入路径上 `fiber.yield` 检测到取消标志后直接执行 `return`**——控制流沿正常返回路径逐帧走完作用域退出的 `release`，栈被完整拆收。取消是协作式的：只在 yield 点生效。这是「弃置泄漏」的正规出口；
- `fiber.check` 之后才能安全丢弃 Fiber 句柄（debug 构建强制）。

### 4.5 栈大小策略

- 固定栈 + 保护页（§4.1）。**不做**分段栈/栈复制：§4.2 的无栈内指针外泄不变量使复制在技术上可行，但收益低、复杂度高；
- 默认 256 KiB 的依据：sloth2 帧小（局部槽为词宽 alloca，无装箱大对象），llama 级负载的调用深度有限；深递归场景由 `create_with` 调大；
- 溢出 = 保护页 SIGSEGV → 诊断终止。相比 1.0 的 VM 栈 `Vec<Value>` 动态增长是退化，作为已知限制记录在案（§8 风险 #4）。

### 4.6 JIT/AOT 一致性

切换发生在原生栈指针层面，与被切换代码的出身（ORC JIT 或 AOT 目标文件）无关——两者都遵守同一宿主 ABI。`run`（JIT）与 `build`（AOT）模式下 `fiber.*` 行为一致；唯一的后端相关点是 AOT 链接时需保证 `libsloth_rt` 的汇编对象被链接（构建脚本既有流程覆盖）。

---

## 5. 编译器影响

改动集中在发射器，前端（lexer/parser/AST）**零改动**：

| 组件 | 改动 |
| --- | --- |
| `sloth-frontend` | 无（`fiber.create` 等表面是普通成员调用；`Fiber<Y>` 复用泛型类型语法，parser 无需特判） |
| `sloth-codegen/ty` | `Ty::Fiber(Y)` 注册；`Fiber` 上下文关键字识别 |
| `sloth-codegen/irgen` | 识别 `fiber.*` 内建调用面 → 发射 `@sloth_fiber_create/resume/yield/error/check/resumable/transfer/cancel`；载荷词的转移插桩（§4.3.1/4.3.2）；入口闭包的 trampoline 桥接复用既有闭包调用发射 |
| 单态化 | `fiber.create`/`resume` 按 `Y` 实例化（与普通泛型函数同机制）；`Fiber<Y>` 类型实参参与实例键 |
| pass 管线 | **无新增 pass**；协程代码与非协程代码的机器码无任何区别，优化管线（canonicalize/cse/bufferize/linalg 通道…）不受影响 |

值得注意的边界：内联优化对协程**透明安全**——有栈切换保存的是执行位置而非编译期结构，内联/尾调用等变换不改变「切走再切回后从 yield 点继续」的可观察语义。

---

## 6. 与 1.0 行为的差异说明（语义层面）

| 项 | 1.0 | 2.0（本扩展） |
| --- | --- | --- |
| 载荷类型 | 动态 `Value`，双向任意类型 | 单一类型 `Y`；协程完成/出错时 resume 得 `nil`（`Y?`） |
| 入口参数 | 可变参数 `argv…` | 单载荷 `init: Y` |
| 调度 | 协作式，prev 链 | 不变 |
| `fiber.error()` | 无参 | 带 `msg: str` 诊断 |
| 取消 | 无 | 新增协作式 `fiber.cancel`（yield 点生效，完整拆栈释放 ARC 引用） |
| 栈 | VM 内 `Vec<Value>`，动态增长 | 固定 mmap 栈（默认 256 KiB）+ 保护页 |
| 挂起栈中引用的存活 | GC 以 Fiber.stack 为根扫描 | ARC 计数自然保活，**无根枚举** |
| 调度器 | `fiber.transfer` 自建 | 不变 |
| 模块加载 | 隐式启动协程加载模块 | 无此机制（模块系统为编译期装配，主文档 §3.7） |

---

## 7. 与主文档的变更清单（回写项）

本扩展落地时需回写主文档（v1.1 → v1.2）：

| 位置 | 变更 |
| --- | --- |
| 头部「前置决策」 | 「移除 fiber」修订为「fiber 以**有栈协程扩展**恢复，语义见协程扩展设计文档」 |
| §1.2 目标 5 | 运行时组件清单追加「协程栈切换（fiber）」 |
| §1.3 非目标 | 删去「fiber 移除后」的单线程表述中关于协程的排除义（单线程不变，协程为线程内 1:m 复用） |
| §2.1 类型全集 | 新增 `Fiber<Y>` 行 |
| §2.6 引用类型表 | 新增 `Fiber<Y>`（payload 布局引用本文档 §3.1） |
| §3.1 变更总览 | 「fiber」行改为「1.0 `fiber.*` → 类型化内建模块恢复（`Fiber<Y>`）」；上下文关键字追加 `Fiber` |
| §3.9 EBNF | `type_base` 追加 `'Fiber' '<' type '>'`；上下文关键字追加 `Fiber` |
| §4.2 前端要点 | 追加「`fiber.*` 内建识别与载荷所有权转移插桩」 |
| §4.3.2 对照表 | 追加 `fiber.*` → `@sloth_fiber_*` 运行时调用行 |
| §5 章首语 | 「运行时不再有协程栈管理」改写为组件清单含 fiber |
| §5.1.1 协议 | 追加第 9 条：跨协程边界载荷的转移语义（引用本文档 §4.3） |
| §5.4 FFI | 「移除 fiber 后无协程栈切换约束」一句删除（extern 仍就是普通原生调用——extern 函数内不得调用 `fiber.yield`，因为 extern 帧不在受管栈上，编译期禁止） |
| §5.5 错误模型 | 补充「协程内 panic 终止进程；协程级出错用 `fiber.error`」 |
| §6 迁移指南 | 「fiber 无替代」行替换为本文档 §2.2 对照表 |
| §7 风险表 | #7「fiber 移除的用户影响」移除；追加本文档 §8 的风险 #1–#4 |
| §8 路线图 | 追加 CE 阶段（本文档 §9） |
| 章末按语 | 「fiber 与 LLVM 模型错配、与 GC 根扫描交互随移除而消除」改写为「随 ARC 改向与原生 ABI，两大阻碍已消解，fiber 以原生有栈方案恢复」 |

---

## 8. 风险与阻碍评估

| # | 风险 | 等级 | 说明与对策 |
| --- | --- | --- | --- |
| 1 | **弃置协程的 ARC 泄漏** | 中 | 挂起态 Fiber 被丢弃时栈上 retain 的对象滞留。对策：debug 构建 panic 暴露 + `fiber.cancel` 协作式收尾（§4.3.5/§4.4）；文档明示「驱动至 Done/Error」纪律 |
| 2 | **所有权转移插桩遗漏** | 高 | `resume/yield` 载荷若按普通「形参 borrowed」处理，协程恢复后用到已释放词 → 悬垂。对策：内建调用的插桩与既有协议规则 4/5/6 复用同一发射辅助函数；spec 测试覆盖「载荷经三次以上切换转手后释放」的 churn 用例（`sloth_rc_live` 断言归零） |
| 3 | **汇编切换的可移植性** | 中 | 仅覆盖 x86_64 SysV / aarch64；Windows（MSVC 调用约定不同）与未来架构需新写。对策：接口面收敛为 `sloth_fiber_switch` 单函数 + trampoline，移植点明确 |
| 4 | **固定栈溢出** | 低 | 深递归协程溢出 = 保护页终止，无 1.0 的动态增长。对策：`create_with` 可调栈大小；保护页诊断明确 |
| 5 | **与 extern 的交互** | 低 | extern 函数（宿主代码）中调用 `fiber.yield` 会把宿主帧卷入切换，UB。对策：编译期禁止（extern 函数体内出现 `fiber.yield` 报错）；宿主侧异步由宿主自己管理 |
| 6 | **张量通道交互** | 低 | 张量数据区在堆上、计算无栈内逃逸指针，与协程正交。对策：验收用例中含「协程内跑 llama 前向逐 token yield」的集成测试 |

> 与 v1.0 评估的对照：原风险「fiber 与 LLVM 无栈协程模型错配」「fiber 栈与 GC 根扫描交互」在新架构下**均不再存在**——前者因不采用 LLVM coroutine intrinsic 而是原生栈切换，后者因 ARC 无需根扫描。本扩展的新风险集中在 ARC 交接纪律（#1/#2），均可由测试与编译期规则控制。

---

## 9. 实施路线图（CE 阶段，叠加于主文档 P0–P6 与 TE-P0–TE-P4 之后）

| 阶段 | 内容 | 验收标准 |
| --- | --- | --- |
| **CE-P0** | 运行时栈/上下文切换：`sloth_fiber_switch`（x86_64 + aarch64）、`Fiber` 对象、trampoline、prev 链 | C 级单测：两个协程交替切换 10⁶ 次无错；保护页溢出诊断正确 |
| **CE-P1** | 发射器内建识别与类型化：`Fiber<Y>` 类型注册、七函数 + `cancel` 的发射、载荷转移插桩 | §3.2 示例运行输出与 1.0 手册逐行一致（`nil` 差异除外） |
| **CE-P2** | ARC 交接与生命周期：churn 测试、`cancel` 拆栈、debug 弃置 panic | 协程引用计数 churn 后 `sloth_rc_live` 回落基线；弃置用例 debug panic / release 仅滞留不悬垂（ASAN 验证） |
| **CE-P3** | 集成与调度器生态：`fiber.transfer` 自定义调度器示例（M:N 不涉线程的 1:m 轮询调度器）、协程+张量集成测试、JIT/AOT 双后端回归 | 调度器示例正确；llama 推理在协程内逐 token yield 且输出与非协程版逐字节一致；CI 双后端全绿 |

里程碑：CE-P1 完成即恢复 1.0 全部协程语义（类型化后）；CE-P2 完成即内存安全可信，可合入主线。

---

## 附录 A：自定义轮询调度器示例（`fiber.transfer` 用法）

```rust
// 1:m 轮询调度器：m 个协程在单线程上轮转，验证 transfer 不触碰 prev 链
class Scheduler {
    var tasks: Array<Fiber<int>> = [];
    var current: int = 0;

    func spawn(f: (int) -> unit): unit {
        this.tasks.push(fiber.create(f, 0));
    }

    func run(): unit {
        while (this.tasks.len() > 0) {
            let f = this.tasks[this.current];
            if (fiber.resumable(f)) {
                fiber.transfer(f, 0);      // 对称切换：协程 yield 后回到本调度器
            }
            if (!fiber.resumable(f)) {
                this.tasks.remove(this.current);
            } else {
                this.current = (this.current + 1) % this.tasks.len();
            }
        }
    }
}
```

> 注：协程侧用 `fiber.yield` 让出时控制沿 prev 链返回；调度器若需严格对称切换（协程间直接互转而不经 prev），协程内可改用对调度器持有的「调度 fiber」做 `fiber.transfer`。1.0 手册的说明同样适用：prev 链语义满足嵌套调用，transfer 语义满足平级轮转。

## 附录 B：关键运行时结构（Rust 侧草图）

```rust
#[repr(C)]
pub struct FiberObj {
    hdr: Hdr,                 // ARC 带内对象头（主文档 §5.1）
    state: i64,               // New/Suspended/Running/Done/Error/CancelRequested
    prev: *mut FiberObj,      // prev 链（强引用语义，主协程为哨兵）
    stack_base: *mut u8,      // mmap 基址（含尾部保护页）
    stack_size: usize,
    ctx: Ctx,                 // 被调用者保存寄存器 + sp
    entry: i64,               // 入口闭包词 {tagged fnptr, env}
    yield_slot: i64,          // 切换载荷的暂存词
    owner_flags: i64,
}

extern "C" {
    fn sloth_fiber_switch(save: *mut Ctx, restore: *const Ctx, payload: i64) -> i64;
}
```
