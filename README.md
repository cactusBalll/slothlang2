# sloth-lang 2.0 (slothlang2)

[English](README.md) | [简体中文](README.CN.md)

A statically typed, AOT-compiled systems language built on a **Rust frontend +
MLIR/LLVM backend**. `sloth-lang 2.0` (sloth2) is a full rewrite of the
`sloth-lang 1.0` tree-walking interpreter: the bytecode VM, the fully boxed
value representation and the mark-and-sweep GC are gone, replaced by a static
type system, an MLIR pipeline, native code generation and automatic reference
counting.

```sloth
func main() {
    let name = "sloth";
    print("hello, ${name}!");
}
```

```sh
$ ./target/debug/slothc run book/src/examples/hello.sl
hello, sloth!
```

## Highlights

- **Static, strong typing** — most types are checked or locally inferred at
  compile time; dynamic behaviour is confined to an explicit `dyn Trait`.
- **AOT / JIT via MLIR + LLVM** — source → MLIR → LLVM IR → native machine
  code, with a `run` (JIT) and a `build` (native executable) path.
- **Untagged single-word value plane** — every SSA value, stack slot, field and
  container element is one `i64`. References are bare pointers, numbers are
  native bit patterns; there is no runtime tag decoding.
- **ARC memory management** — deterministic reference counting plus `Weak<T>`
  for cycle breaking; no tracing collector.
- **C-like syntax with modern ergonomics** — pipe `|>`, string interpolation
  `${expr}`, iterator `for`, operator overloading, generics with
  monomorphization, classes/inheritance and traits.
- **A self-hosted standard library** — `Array`, `Map`, `range`, optional boxes,
  `Result`, `Entry`, the `print` prelude and the `StrChars` iterator are
  implemented in Sloth itself (`lib/prelude/*.slt`) and injected at compile
  time.
- **Extension modules for the hard parts** — tensors, fibers, OS threads and
  an async I/O / networking reactor are layered on the core language through
  ordinary function calls, with no new keywords.

## Architecture

The workspace is split into four crates plus a bootstrap standard library:

| Crate | Responsibility |
| --- | --- |
| `sloth-frontend` | Lexer, recursive-descent + Pratt parser, AST, type representation |
| `sloth-codegen` | Semantic-analysis pass (`sem/`), MLIR emission (`irgen/`), the minimal `sloth` dialect, the LLVM pipeline and JIT (`pass.rs`/`jit.rs`), MLIR C-API wrappers |
| `sloth-rt` | Runtime `libsloth_rt.so`: allocation, ARC/weak references, strings, objects/vtables, panics, plus the C-ABI entry points for I/O, networking, tensors, fibers and threads |
| `slothc` | Command-line driver (`check` / `ir` / `run` / `build`) |

Compilation currently runs as **two explicit passes** after parsing:

```text
source (.sl)
  │  lexer
  ▼
tokens ─► parser ─► AST (Program { imports, decls, stmts })
  │
  │  Pass 1 — sem/ (ModEmitter, check_mode = true)
  │    symbol/class/trait/module collection, type inference & constraint
  │    checking, generic monomorphization, the NodeId type side-table, and the
  │    sole producer of diagnostics
  ▼
  │  Pass 2 — irgen/ (ModEmitter, check_mode = false, diagnostics silent)
  │    replays the side-table, emits ARC/coercion/width conversions and the
  │    MLIR text (`sloth.rc_retain`/`sloth.rc_release` + standard dialects)
  ▼
MLIR ─► single-point lowering of `sloth.*` to `func.call @__sloth_*`
  ▼
MLIR (standard dialects + calls into libsloth_rt / the self-hosted prelude)
  │  canonicalize → cse → one-shot-bufferize → linalg-fuse-elementwise-ops
  │  → convert-linalg-to-loops → convert-scf-to-cf → convert-math-to-llvm
  │  → convert-func-to-llvm → convert-arith-to-llvm → convert-index-to-llvm
  │  → convert-cf-to-llvm → finalize-memref-to-llvm
  │  → reconcile-unrealized-casts
  ▼
LLVM IR ──► JIT (`run`) or object code + clang link (`build`)
```

Key points:

- The `sloth` dialect is a **minimal semantic layer**: only ARC
  `sloth.rc_retain` / `sloth.rc_release` are emitted, then lowered in a single
  step to `func.call @__sloth_rc_*`. No `sloth.*` op survives into the
  externally visible IR.
- The **runtime ABI namespace is reserved** (`__sloth_*`); ordinary modules must
  not declare those symbols.
- Diagnostics are **collected in batches**, not fail-fast.

## Repository layout

| Path | Contents |
| --- | --- |
| `crates/` | The four Rust crates listed above |
| `lib/prelude/` | Compiler-injected standard library (`containers`, `core`, `print`, `result`, `strchars`, `abi`) |
| `lib/sloth/` | Importable standard-library modules (`array`, `str`, `net`, `http`, `event`, `io`, `fs`, `tensor`, `random`) |
| `book/` | The mdBook language guide (Chinese) with generated MLIR for every example |
| `examples/` | Example programs and runners: `arc`, `diff`, `fiber`, `fs`, `llama`, `net`, `tensor`, `threads` |
| `test_workspace/` | Faceted compiler test workspace and the resulting bug reports |
| `docs/` | Design notes (e.g. the runtime self-hosting evaluation) |
| `slothlang2-syntax/` | VS Code syntax-highlighting extension for `.sl` / `.slt` |
| `crates/slothc/tests/spec`, `.../seed` | Regression suites driven by `spec_suite.rs` / `seed_suite.rs` |

The design documents at the repository root (`sloth-lang-2.0设计文档.md` and the
tensor / coroutine / thread / I/O extension documents) are the original Chinese
specifications; the language guide records where the implementation deliberately
diverges.

## Requirements

- **Rust** (nightly; the workspace currently builds with `rustc 1.99.0-nightly`).
- **LLVM/MLIR 21** with its CMake package, plus `clang`, `mlir-opt` and
  `mlir-translate`. The build pins `mlir-sys = 210.0.4` and defaults to
  `/usr/lib/llvm-21` (override with `MLIR_SYS_210_PREFIX`).
- **CMake** (Ninja recommended) to build the C++ `sloth` dialect sidecar.
- A C++ toolchain and `libstdc++`.

The workspace `.cargo/config.toml` sets `MLIR_SYS_210_PREFIX=/usr/lib/llvm-21`.

## Build

```sh
cargo build -p slothc      # produces target/debug/slothc and libsloth_rt.so
```

To build everything (library crates, runtime and driver):

```sh
cargo build
```

For an optimized native runtime, build the runtime in release mode:

```sh
cargo build --release -p sloth-rt
```

## Usage

```text
slothc <check|ir|run|build> file.sl [out]
```

| Mode | Effect |
| --- | --- |
| `check` | Parse and run the semantic-analysis pass only; prints `check ok` or diagnostics to stderr |
| `ir` | Print the generated MLIR text |
| `run` | JIT-compile and execute (`main` entry) |
| `build` | MLIR → LLVM → object → `clang` link into a native executable (default output `sloth_app`) |

Any source containing an `import` automatically switches `check`/`ir`/`run`/`build`
to multi-module compilation.

```sh
./target/debug/slothc check examples/hello2.sl
./target/debug/slothc ir    book/src/examples/hello.sl
./target/debug/slothc run   book/src/examples/hello.sl
./target/debug/slothc build examples/threads/matvec_parallel.sl my_app
```

## Language tour

```sloth
trait Speaker { func say(): unit; }

class Mammal impl Speaker {
    let kind: str;
    func __init__() { this.kind = "Mammal"; }
    func say(): unit { print("Mammal kind is: ${this.kind}\n"); }
}

class Cat: Mammal {
    func __init__() { super.__init__(); this.kind = "Cat"; }
    func say(): unit { print("meow\n"); super.say(); }
}

pub func main(): unit {
    let l: Array<dyn Speaker> = [Cat(), Mammal()];
    for (var m: l) { m.say(); }
}
```

The full type set includes `unit`, `bool`, `int` (64-bit), `float` (f64), the
fixed-width integer family (`int8`/`int16`/`int32`, `uint`/`uint8`/…/`uint64`),
`str`, `range`, `Array<T>`, `Map<K,V>`, `T?`, first-class functions and
closures, classes with single inheritance, traits / `dyn`, `any`, `Weak<T>`,
`Tensor<T,R>`, `Fiber<Y>`, `JoinHandle<R>` and `Channel<T>`.

### Standard library and extension modules

- Importable from `lib/sloth/` via the compile-time module system, e.g.
  `import "sloth/array.slt";` or `import "sloth/str.slt";`.
- **Tensors** (`Tensor<T,R>` + `linalg`) — see `examples/tensor`, `examples/llama`.
- **Fibers** — stackful coroutines (`fiber.create`/`resume`/`yield`) in
  `examples/fiber`.
- **Threads** — `thread.spawn`, `JoinHandle`, `Channel`, `Mutex`, `AtomicInt` in
  `examples/threads`.
- **I/O / networking** — a Sloth-implemented event reactor over `io_uring`/
  `epoll` with TCP/UDP/HTTP helpers in `examples/net` and `lib/sloth/{io,net,http,event}.slt`.

## Testing

```sh
cargo test --workspace
```

The compiler regression suites live under `crates/slothc/tests/`:

- `spec_suite.rs` — spec/behaviour tests (`tests/spec/*.sl`), run through the
  driver.
- `seed_suite.rs` — curated regression tests (`tests/seed/*.sl`) using
  `// expect:`, `// diag:` and `// bug:` directives.

JIT-vs-AOT differential checking is available via
`bash test_workspace/aot_diff.sh`; per-area example runners live at
`examples/*/run.sh` (set `SLOTHC=/path/to/slothc` to point at a built driver).

## Documentation

- **Language guide (mdBook, Chinese):** `book/` — build with
  [`mdbook`](https://rust-lang.github.io/mdBook/):
  ```sh
  mdbook build book     # output in book/book/
  mdbook serve book
  ```
  Every example in the guide ships alongside the MLIR the compiler actually
  generates (`book/gen-mlir.sh` regenerates them).
- **Original design specifications (Chinese):** the `sloth-lang-2.0*.md` files
  at the repository root.
- **Deviations from the design docs:** `book/src/appendix_a_deviations.md`.
- **Runtime self-hosting evaluation:** `docs/self-hosting-evaluation.md`.
- **Test workspace report:** `test_workspace/BUGS.md`.

## Status

The language surface is largely feature-complete: the type system (including
fixed-width/unsigned integers), generic monomorphization, classes / inheritance
/ virtual dispatch, traits and `dyn`, closures and first-class functions,
containers, optionals, `Result`, the iteration protocol, compile-time modules,
FFI and ARC are all implemented, as are the tensor, fiber, thread and I/O
extensions.

Two structural deviations from the original design are intentional and
documented: the compiler is a **two-pass** front end (analysis, then emission,
sharing one inference engine), and the `sloth` dialect is intentionally
**minimal** (ARC only). See `book/src/appendix_a_deviations.md` for the complete
list.

## License

MIT (see the workspace `Cargo.toml`).
