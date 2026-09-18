# 23. 内存管理：ARC 所有权协议

运行时**没有 GC**：引用类型走**引用计数 + `Weak<T>` 破环**。分配器是 malloc 基
确定性链（`sloth_rt_alloc`/`realloc`）；对象头带内承载计数；归零即析构级联并
`free`。漏插 `release` 只会退化为**内存滞留**（泄漏），不会悬垂。

## 23.1 所有权状态与规则

任一引用词在任一时点处于二者之一：

- **owned（+1）**：持有者负责恰好一次 `release`；
- **borrowed（+0）**：只借用读取，持有者不得 `release`。

发射器必须维持的不变量：

1. **生产即 owned**：构造器、`Array`/`Map`/闭包/盒字面量、字符串 intern/拼接、
   `keys()`/`values()`、range/Entry 盒等产出 owned 句柄。
2. **持有即 owned**：局部槽、字段、容器元素、闭包捕获写入时 `retain`（copy-in），
   覆盖旧值时 `release`（overwrite-out）；作用域退出释放本层声明的槽。
3. **形参与接收者为 borrowed**：被调函数不得释放形参或 `this`。
4. **返回值为 owned**：引用返回一律交付 +1。返回面三态：**生产者**直接转移其 +1；
   **调用结果**（已 owned）转移；对**借用值**（槽/字段/形参）先 `retain` 再交付。
5. **调用结果即 owned 临时量**：直接调用、方法调用、虚分派、dyn 分派、函数值调用
   的引用结果均登记为 owned 临时量（`extern` C 函数除外，C 所有权自负）。
6. **转移**：owned 临时量被接管（绑定、字段/元素写、`return`）时**不再 retain**，
   所有权直接转移并注销临时登记。
7. **临时量回收**：未被接管的 owned 临时量在**语句结束**由发射器 `release`；
   在条件终止（`cjump`）与 CFG 分裂（虚分派/短路/分支 merge）处于**当前块内**冲刷，
   避免跨 merge 引用非支配 SSA 值。
8. **`Weak<T>`**：弱持有不增目标计数；闭包捕获标量按值快照、引用按引用共享。

**主要插入点**：绑定/赋值、字段与元素写、作用域退出、`return`、语句级临时量冲刷、
条件/merge 冲刷，以及容器迭代协议（`iter()` 结果 owned 于循环退出释放、`next()`
结果 owned 于每轮迭代末释放，`break`/`continue` 路径同样覆盖）。

## 23.2 可观测性

运行时暴露两个零参内置用于断言计数收敛：

- `sloth_rc_live()`：当前存活引用数；
- `sloth_rc_drops()`：累计析构数。

采样点应放在 **caller 侧**（helper 内采样会把 `str` 实参字面量的 owned +1 误读为
泄漏）。

## 23.3 示例

```sloth
{{#include examples/arc.sl}}
```

```mlir
{{#include examples/arc.mlir}}
```

在 MLIR 里，`churn` 的每一轮对 `h` 的字段读取之后，循环体末尾能看到作用域结算：
`sloth_rc_release` 依次释放在本层声明的槽。`main` 里 `sloth_rc_live() == base` 为
`true` 说明没有泄漏。

## 23.4 已知残余

- **强引用环按设计泄漏**（ARC 的固有代价），应用 `Weak<T>` 打破（§15.4）。
- 少数边界观察项（如某些容器死亡路径、Map 覆盖时旧值的所有权语义）属已知残余，
  不影响常规使用。
