# PLAN：sloth 自定义 dialect 迁移（方案 C + A2）

状态：**完成**（2026-09-23）
决策：**先完成方案 C（还文本债），再执行 A2（C++/TableGen 自定义 dialect）**；正式**重开** PLAN-2026-09-15 §3.2 / 附录 A 中「无 sloth dialect」的冻结定案。

---

## 0. 背景与动机

现状：codegen 以文本拼接直发 MLIR 标准 dialect（`func/arith/cf/memref/llvm/scf/linalg/math`，608 处 `fw.op`），parse 时才验证；存在两处**文本改写债**：

| 债项 | 位置 | 本质 |
| --- | --- | --- |
| `normalize_indices` | `irgen/mod.rs:80-134` | 发射时把用作 memref 下标的 `arith.constant 0` 错标成 `i64`，事后全文改写为 `index` |
| `rename_plain_calls` | `irgen/mod.rs:264-286`，用于 `func.rs:186-195` | 发射裸 `call @`，在 `llvm.func` 体内外拼写不一致，事后全文改写 |

原设计 §4.3 曾规划自定义 `sloth` dialect，实现时放弃并归档为定案（`book/src/appendix_a_deviations.md:11`、`PLAN-2026-09-15.md` §3.2）。本次重开，目标：

1. **方案 C**：就地修发射缺陷，删除两处文本改写，还清工程债；
2. **方案 A2**：引入 C++/TableGen 定义的**语义层** `sloth` dialect（接受 C++/cmake 侧车构建），承载语言语义 op（首批：ARC `retain`/`release`），在 parse 后**单点 lowering** 到标准 dialect，再进入现有管线；
3. 保留全部标准 dialect 发射打底（**不做方案 B**：不重定义 `arith/cf/memref` 等）。

---

## 1. 范围与非目标

### 在范围内
- 方案 C：index 常量发射类型修正、`func.call` 统一拼写、删除两处文本改写。
- 方案 A2：
  - `crates/sloth-codegen/dialect/`：CMake + TableGen + C++ 实现；
  - `build.rs` 集成（cmake 构建静态库，链入 `sloth-codegen`）；
  - C API：`slothRegisterDialect`（注册）+ `slothLowerModule`（greedy pattern lowering）；
  - 首批语义 op：`sloth.rc_retain`、`sloth.rc_release`（覆盖发射点：`fnwalk.rs`×7、`tybind.rs`×2、`fiber.rs`×1）；
  - JIT 与 AOT 双路径接入 lowering + `sloth.` 泄漏闸门；
  - 关闭 `allowUnregisteredDialects`；smoke 测试改用已注册 op。
- 文档：本 PLAN；重开冻结定案（附录 A.1、PLAN §4.1/§4.3 行、§3.2）。
- 测试：全量 `cargo test`；book golden 再生（`book/gen-mlir.sh`）。

### 非目标（明确不做）
- 不重定义标准 dialect（方案 B）。
- 不迁移 `arith/cf/memref/llvm/scf/linalg/math` 发射。
- 不引入 melior 等 Rust MLIR 框架（方案 D）。
- 首批不做 `sloth.rt.call`/`sloth.word.*`/fiber-track op（架构就绪后可增量追加）。
- 不改变运行时 ABI、pass 管线（`pipeline.rs`）与 AOT 外部工具链。

---

## 2. 方案 C 设计（先行）

### 2.1 index 常量
- 凡 `arith.constant 0` 的 SSA 值**仅用作** `memref.load/store` 下标处，发射即写 `: index`（对照：`fnwalk.rs` 已正确的 `rc_release_loop_body`/`pop_scope` 等）。
- 重点修复点（现错标 `i64` 且被 `normalize_indices` 掩盖）：
  - `FnWalk::assign`（`fnwalk.rs:304-313`）：`z` 用作 store 下标；
  - `func.rs:99`：参数槽 store 的 `zi`；
  - 其余以测试失败驱动逐点修复（判据：去掉改写后 MLIR parse/verify 报 index 类型错）。
- 删除 `normalize_indices` / `normalize_chunk`，及全部调用（`mod.rs:143,161`、`pass.rs:52,91`）。

### 2.2 call 拼写
- 统一发射 **`func.call @`**（`func.func` 与 `llvm.func` 体均合法）；`llvm.call`（间接调用）不动。
- 全量替换 `irgen/*.rs` 中裸 `call @` 格式串；AOT wrapper（`slothc/main.rs:98`）同步。
- 删除 `rename_plain_calls` 及 `func.rs:186-195` 调用。

### 2.3 验收
- `cargo test -p sloth-codegen` 全绿；
- 生成 IR 中不再出现「`0 : i64` 作 memref 下标」与裸 `call @`（`llvm.call` 除外）；
- book golden 再生，diff 仅反映 `func.call` 拼写与（若有）下标类型显式化。

---

## 3. 方案 A2 设计（C++/TableGen 语义层 dialect）

### 3.1 目录与构建

```
crates/sloth-codegen/
  build.rs                    # cmake configure+build + 链接指令
  dialect/
    CMakeLists.txt            # find_package(MLIR), tablegen, STATIC lib
    include/Sloth/SlothDialect.td
    include/Sloth/SlothOps.td
    include/Sloth/SlothCAPI.h
    lib/SlothDialect.cpp      # dialect 定义 + register hook
    lib/SlothOps.cpp          # op 定义（含 .inc include）
    lib/SlothLowering.cpp     # greedy patterns → func.call
    lib/SlothCAPI.cpp         # extern "C" 入口
```

构建要点：
- 环境已具备：`cmake`/`ninja`、`/usr/lib/llvm-21`（headers、`mlir-tblgen`、`lib/cmake/mlir`、`libMLIR*.a`）。
- CMake 侧**只编译不链 MLIR**（产出 `libsloth_dialect.a` 仅含本库 .o），MLIR 符号由 `mlir-sys` 既有静态链接解析——避免重复链接。
- `build.rs`：`cmake -G Ninja` → `ninja` → `cargo:rustc-link-search` + `cargo:rustc-link-lib=static=sloth_dialect` + `cargo:rustc-link-lib=stdc++`。
- 编译标志对齐 MLIR（经 `AddMLIR`/`MLIR_INCLUDE_DIRS`）；与 `mlir-sys` 同用 `MLIR_SYS_210_PREFIX=/usr/lib/llvm-21`。

### 3.2 Dialect / Ops（TableGen）

- Dialect：`name = "sloth"`，`cppNamespace = "::sloth"`。
- 首批 op（i64 词面 ABI，与运行时 `@sloth_rc_*` 一一对应）：

| Op | 语法 | 下游 |
| --- | --- | --- |
| `sloth.rc_retain` | `%r = sloth.rc_retain %h : i64` | `func.call @sloth_rc_retain(%h) : (i64) -> i64` |
| `sloth.rc_release` | `sloth.rc_release %h : i64`（无结果） | `func.call @sloth_rc_release(%h) : (i64) -> i64`（丢弃返回值） |

- 运行时符号声明仍由既有 `rt_decls()`（`module.rs:253-254`）提供，dialect 不重复声明。

### 3.3 C API 与接入点

```c
void slothRegisterDialect(MlirDialectRegistry registry);
MlirLogicalResult slothLowerModule(MlirModule module);
```

| 路径 | 接入 |
| --- | --- |
| `Context::new` | `mlirRegisterAllDialects` 之后调用 `slothRegisterDialect`；**`allowUnregisteredDialects=false`** |
| JIT（`run_src`/`run_src_multimod`） | `Op::parse` 成功后立即 `slothLowerModule`，失败即错；随后 `run_llvm_pipeline` |
| AOT（`build_mode_r`） | 写 `/tmp/opencode/app.mlir` **之前**：parse → `slothLowerModule` → print 写盘；确保 `mlir-opt` 只见标准 dialect |
| 泄漏闸门 | lowering 后断言打印文本不含 `sloth.`（op 前缀；符号 `@sloth_*` 不受影响） |

Lowering 实现：`applyPatternsAndFoldGreedily` + 两个 `OpRewritePattern`；模式未覆盖的残余 `sloth.*` 使 `slothLowerModule` 返回 failure（防静默泄漏）。

### 3.4 发射面改动（首批 ARC）

| 文件 | 改动 |
| --- | --- |
| `fnwalk.rs` | 7× `call @sloth_rc_release(...)` → `sloth.rc_release %h : i64` |
| `tybind.rs` | `emit_rc_release`/`emit_rc_retain` helper 改发 sloth op |
| `fiber.rs` | 1× release 改发 sloth op |
| 其余 `@sloth_*` 调用 | **保持 `func.call`**（首批不迁） |

`compile_to_ir` 输出可含 `sloth.*`；对外保证「lowering 后无泄漏」的是 JIT/AOT 运行路径与（如需要）`slothc ir` 可选不 lower——**定案：`slothc ir` 输出 lowering 后的标准 dialect**（与 book golden、`mlir-opt` 兼容一致），另在测试内直接断言 lowering 前的 IR 含 `sloth.rc_*`。

### 3.5 测试策略

1. **单元**：手写含 `sloth.rc_*` 的文本 → parse → lower → 打印断言变为 `sloth_rc_retain` 调用且无 `sloth.`；
2. **发射断言**：新增/调整用例：lowering 前 IR 含 `sloth.rc_retain`（经内部路径），JIT 运行结果不变；
3. **既有面**：207 个 codegen 测试 + 112 spec 文件 + examples e2e（llama 字节级、tensor ≥0.70×）全绿；
4. **smoke**（`pass.rs` `smoke_all`）：`sloth.gc_alloc` → 已注册的 `sloth.rc_release`（unregistered 关闭后旧样例会 parse 失败）；
5. **AOT 闸门**：`build` 路径写盘前 assert 无 `sloth.`。

### 3.6 风险与缓解

| 风险 | 缓解 |
| --- | --- |
| cmake/TableGen 与 cargo 增量构建不同步 | `build.rs` 用 `rerun-if-changed` 覆盖 `.td`/`.cpp`/`CMakeLists.txt` |
| 静态链接符号重复/顺序 | CMake 不链 MLIR；`stdc++` 显式链接；smoke 先行验证最小 C++ 往返 |
| `allowUnregistered=false` 暴露未知 op | 全量 parse 测试；仅 `sloth.*` 为新增依赖 |
| lowering 后 ARC 语义回归 | RC 为等价 lowering（call→call）；spec ARC/类/闭包用例 + examples 压闸 |
| AOT/JIT 分叉 | 单点 `slothLowerModule` 共享；双路径泄漏断言 |
| 文档失真 | 同步附录 A.1、PLAN §4.1/§4.3、§3.2 解冻记录 |

---

## 4. 执行顺序（里程碑）

| # | 任务 | 验收 |
| --- | --- | --- |
| M0 | 本方案写入 `PLAN-sloth-dialect.md` | 文件存在 |
| M1 | 方案 C：修 index 发射 + `func.call` 统一；删两处文本改写 | codegen 测试全绿 |
| M2 | A2 脚手架：dialect/ + build.rs + 最小 op 往返（parse→lower→print） | 单元测试过；`cargo build` 过 |
| M3 | 注册 + JIT/AOT 接入 + 泄漏闸门 + 关 unregistered | 双路径测试过 |
| M4 | 首批 ARC 发射迁移 | 全量 `cargo test` + spec |
| M5 | 文档解冻（附录 A、PLAN §3.2/§4.x）+ book golden 再生 | 文档一致；golden diff 可解释 |

---

## 5. 完成定义（DoD）

- [x] 两处文本改写已删除，发射即正确。
- [x] `sloth` dialect 由 TableGen/C++ 定义并注册；`allowUnregisteredDialects=false`。
- [x] ARC `retain`/`release` 发射为 `sloth.rc_*`，parse 后单点 lower 为 `func.call @sloth_rc_*`。
- [x] JIT 与 AOT 均无 `sloth.` 泄漏到下游（断言覆盖）。
- [x] `cargo test`（workspace）全绿；spec 全绿；examples 关键 e2e 通过。
- [x] 附录 A / PLAN 冻结项已重开并记录理由。
- [x] book golden 已再生且 diff 可解释。

## 6. 落地记录（2026-09-23）

- 方案 C：删除 `normalize_indices`/`normalize_chunk`/`rename_plain_calls`；全量裸 `call @` → `func.call @`；index 常量发射点修正（`fnwalk`/`func`/`expr`/`stmt`/`lambda`）。
- A2：`crates/sloth-codegen/dialect/`（CMake+TableGen+C++，静态库不链 MLIR，符号由 `mlir-sys` 解析）；`build.rs` 集成；`src/dialect.rs` FFI；`Context::new` 注册 + 关 unregistered。
- 单点接入：`run_src`/`run_src_multimod`（parse 后 `lower_parsed`）；`compile_to_ir`/`compile_multimod`（返回前 `lower_text`），AOT 因此天然只见标准 dialect。
- 发射迁移：`tybind.rs::emit_release/emit_retain`、`fnwalk.rs` 7×、`fiber.rs` 1× 改发 `sloth.rc_*`；`@sloth_rc_*` 声明保留（lowering 目标）。
- 测试：`dialect_migration::arc_ops_are_sloth_then_lowered`（发射含 `sloth.rc_release`，lowering 后无 `sloth.` 且有 `@sloth_rc_release`）；`smoke_all` 用已注册 op；workspace 全绿。
- 文档：附录 A.1、`PLAN-2026-09-15.md` §4.x/§3.2 解冻；book golden 再生（25 文件，diff 含方案 C 下标/call 与 lowering 重打印）。
