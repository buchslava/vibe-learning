# Java / Python / Rust — Mental Model Map

One-page orientation. Details live in linked chapters.

## Execution and memory

| Topic | Java | Python | Rust |
|-------|------|--------|------|
| Run model | JVM bytecode | Interpreter / VM | Native binary |
| Memory | GC, heap objects | GC + refcounts | Ownership, stack + heap |
| Null | `null` | `None` | `Option<T>` |
| Exceptions | `throw` / `catch` | `raise` | `Result` + `panic!` |
| Immutability default | fields optional | variables rebind | `let` immutable |

→ [Ch 1 Paradigm](../chapters/01_paradigm_shift.md), [Ch 3 Ownership](../chapters/03_ownership_borrowing.md)

## Types and polymorphism

| Topic | Java | Python | Rust |
|-------|------|--------|------|
| Classes | `class` + inheritance | `class`, duck typing | `struct` + `impl`, no inheritance |
| Interfaces | `interface` | informal protocol | `trait` |
| Generics | type erasure + bounds | hints optional | monomorphized, zero-cost |
| Runtime polymorphism | virtual methods | duck typing | `dyn Trait` or generics |

→ [Ch 6 Traits](../chapters/06_structs_traits_generics.md)

## Collections and loops

| Topic | Java | Python | Rust |
|-------|------|--------|------|
| Dynamic array | `ArrayList` | `list` | `Vec<T>` |
| Map | `HashMap` | `dict` | `HashMap<K,V>` |
| Loop style | indexed for | `for x in` | `for x in iter` / iterators |
| Comprehensions | streams (Java 8+) | list comp | iterator adapters |

→ [Ch 8 Collections](../chapters/08_collections_iterators.md)

## Concurrency

| Topic | Java | Python | Rust |
|-------|------|--------|------|
| Threads | `Thread`, executors | `threading` (GIL) | `std::thread` |
| Shared state | `synchronized`, locks | locks, GIL limits | `Mutex`, `Arc`, atomics |
| Async | virtual threads, CompletableFuture | `asyncio` | `async`/`await` + Tokio |
| Data races | possible at runtime | possible (C ext) | prevented in safe code |

→ [Ch 10–12](../chapters/10_multithreading.md)

## I/O and systems

| Topic | Java | Python | Rust |
|-------|------|--------|------|
| Files | `Files`, streams | `open()` | `File` + `Read`/`Write` traits |
| Subprocess | `ProcessBuilder` | `subprocess` | `Command` |
| Binary data | `ByteBuffer` | `bytes`, `struct` | `u8`, slices, `to_be_bytes` |

→ [Ch 15–16](../chapters/15_io_processes_bits.md)

## Package management

| Java | Python | Rust |
|------|--------|------|
| Maven / Gradle | pip / poetry | Cargo + crates.io |

→ [Ch 2 Toolchain](../chapters/02_toolchain_and_types.md)

## Habits to unlearn

1. **Shared mutable aliasing** — default to one owner; borrow briefly.
2. **Null checks everywhere** — use `Option` and `match`.
3. **Catch-all exceptions** — use `Result` and typed errors.
4. **Clone by habit** — borrow first; clone when semantics require a copy.
5. **Ignore return values** — `Result` must be used or explicitly dropped with intent.

## AI Afterparty

Use [AI_PROMPT_INDEX.md](AI_PROMPT_INDEX.md) with this map: ask the model to translate a Java or Python snippet you know into idiomatic Rust using the right column.
