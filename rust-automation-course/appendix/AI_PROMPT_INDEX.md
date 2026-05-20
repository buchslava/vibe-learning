# AI Prompt Index

All **Afterparty: AI Lego blocks** from the book, numbered for reuse. Paste into any AI assistant after reading the linked chapter.

**How to use:** `P042` → find prompt → open [chapter](#) for context → paste prompt → demand compiler-accurate answers.

---

## Chapter 0 — Preface

| ID | Prompt |
|----|--------|
| P001 | **Learning plan** — I know Java and Python. Based on this preface, build me a 2-week study plan using only this book's chapter list; 45 minutes per day. |
| P002 | **Gap check** — Ask me five quick questions to see if I should skip straight to Chapter 3 (ownership) or read Chapters 1–2 first. |
| P003 | **Prompt practice** — Give me one sample Afterparty-style question about ownership; I will answer; you grade like a Rust teacher. |
| P004 | **Motivation map** — List three real projects (automation, CLI, service) where Rust's ownership model wins over GC; no hype. |
| P005 | **Glossary seed** — Define in one sentence each: ownership, borrow, trait, async, atomics — I will refine after reading Part I. |

## Chapter 1 — Paradigm shift

| ID | Prompt |
|----|--------|
| P006 | **GC quiz** — Quiz me: for 8 snippets, say whether Java GC, Python refcount, or Rust drop applies; then explain Rust's case. |
| P007 | **Move drill** — Give 5 tiny Rust snippets; I predict compile error or ok; you reveal answers with one-line fixes. |
| P008 | **Latency story** — Explain stop-the-world GC pauses vs Rust drop for a 1 kHz control loop — 200 words, no jargon pile-up. |
| P009 | **Java habit** — I wrote `clone()` everywhere in Java; how should I think about `clone()` in Rust? When is `.clone()` idiomatic vs a smell? |
| P010 | **Paradigm essay** — In 150 words: what does "fearless concurrency" mean if I only know Python's GIL? |
| P011 | **Refactor fantasy** — Take this Python class with shared mutable list; sketch the Rust struct + ownership split without full code. |

## Chapter 2 — Toolchain and types

| ID | Prompt |
|----|--------|
| P012 | **Cargo vs pip** — Compare Cargo.toml + Cargo.lock to requirements.txt + venv in 8 bullet points; include reproducibility. |
| P013 | **Type annotate** — Give 5 Rust expressions where inference fails; I add types; you check. |
| P014 | **match warm-up** — Extend my match on HTTP codes to include redirects and server errors; exhaustiveness hints only. |
| P015 | **Java bridge** — Map Java int/long/BigInteger habits to Rust integer types for a finance-lite exercise. |
| P016 | **Playground drill** — Rewrite this Python loop over range(10) as idiomatic Rust `for`; explain `..` vs `..=`. |
| P017 | **env! vs std::env** — When do I use `env!` vs `std::env::var`? Three real scenarios. |

## Chapter 3 — Ownership and borrowing

| ID | Prompt |
|----|--------|
| P018 | **Borrow checker tutor** — I paste compiler errors; you explain the borrow conflict and show the smallest fix. |
| P019 | **Five snippets** — Move vs borrow quiz: 5 code fragments, I label ok/error and why. |
| P020 | **Python port** — This Python function mutates a list passed in; rewrite with `&mut Vec` and explain aliasing rules. |
| P021 | **Java port** — This Java method stores the passed List in a field; show the Rust ownership split (return owned vs `Arc`). |
| P022 | **Slice drill** — Given `&[i32]`, write `first` and `rest` without panicking on empty — use `Option`. |
| P023 | **clone audit** — Review my 30-line Rust snippet; mark unnecessary `.clone()` calls. |

## Chapter 4 — Lifetimes

| ID | Prompt |
|----|--------|
| P024 | **Error archaeology** — I paste a "lifetime may not live long enough" error; walk me through owner vs reference diagram. |
| P025 | **Return type choice** — For API `fn title(book: &Book) -> ???` compare `&str` vs `String` trade-offs for a library. |
| P026 | **Struct lifetime** — Design `ConfigParser` holding `&str` slices into input buffer — when is it sound vs use owned `String`? |
| P027 | **Elision quiz** — Add explicit lifetimes to 4 function signatures where elision fails. |
| P028 | **Java analogy** — Compare Rust lifetimes to Java stack locals vs heap references — 120 words, accurate only. |
| P029 | **Fix mine** — I return `&String` built inside function; show three idiomatic fixes ranked by simplicity. |

## Chapter 5 — Enums and pattern matching

| ID | Prompt |
|----|--------|
| P030 | **Null replacement** — Translate 5 Java methods returning null into `Option` Rust; explain callsite changes. |
| P031 | **Exhaustive match** — I have enum with 4 variants; generate match that compiles; then add variant and show compiler error. |
| P032 | **Result railway** — Chain parse → validate → compute with `?`; I fill blanks, you verify. |
| P033 | **if let vs match** — When is `if let` clearer than `match`? Give 3 contrasting snippets. |
| P034 | **State machine** — Model TCP connection states as enum; methods connect, send, close with illegal transition errors. |
| P035 | **Python Union** — This Python function accepts int \| str; design Rust enum + match without dynamic typing. |

## Chapter 6 — Structs, traits, generics

| ID | Prompt |
|----|--------|
| P036 | **Interface port** — Convert Java interface `Measurable` + two classes to trait + two structs + `impl`. |
| P037 | **Duck typing** — Python function accepts anything with `.read()`; express as trait bound in Rust generic. |
| P038 | **dyn vs impl** — Quiz: 4 scenarios — pick `dyn Trait` or `impl Trait` and justify. |
| P039 | **Default trait methods** — Add default `summary()` on trait; override in one type only. |
| P040 | **Generic bounds** — Fix compiler error: `T` needs `Display + Clone`; minimal bound set. |
| P041 | **OOP myth** — Explain in 100 words why Rust has no inheritance and what you do instead. |

## Chapter 7 — Errors and testing

| ID | Prompt |
|----|--------|
| P042 | **? chain** — Refactor nested match on Results to `?` style; explain each change. |
| P043 | **Error enum design** — Design `AppError` for CLI that reads config + talks serial; variants + `From` impls sketch. |
| P044 | **panic audit** — Mark which of 10 `unwrap()` calls should stay vs become `Result`. |
| P045 | **Test generation** — Write table-driven tests for `parse_port` including edge ports. |
| P046 | **Java exceptions** — Map checked Exception flow to Rust `Result` for file-not-found scenario. |
| P047 | **anyhow vs thiserror** — When would I pick each for automation binary vs library crate? |

## Chapter 8 — Collections and iterators

| ID | Prompt |
|----|--------|
| P048 | **Loop port** — Rewrite this C-style indexed loop as iterator chain; preserve behavior. |
| P049 | **HashMap merge** — Two maps of scores — merge by taking max per key; iterator style. |
| P050 | **Closure capture** — Explain `FnOnce` vs `FnMut` for closure storing `String`. |
| P051 | **collect types** — Why does `collect()` need type hint sometimes? Show turbofish example. |
| P052 | **Windows** — Detect rising edges in `Vec<f64>` with `.windows(2)` — write snippet. |
| P053 | **Performance myth** — Do Rust iterators optimize to loops? When might they not? |

## Chapter 9 — Smart pointers and modules

| ID | Prompt |
|----|--------|
| P054 | **Box why** — When is `Box<[T]>` better than `Vec<T>` on stack semantics? Two cases. |
| P055 | **Rc cycle** — Explain why `Rc` cycles leak memory; contrast Rust with Python cycles. |
| P056 | **Module split** — Split monolithic main.rs into lib + bin; list file tree only, I implement. |
| P057 | **pub audit** — What should be `pub` in a library crate vs kept private? |
| P058 | **Arc Mutex sketch** — Diagram thread-safe cache with Arc<Mutex<HashMap>> — no full code. |
| P059 | **Java heap** — Map Java "everything is reference" to Rust ownership + when Arc applies. |

## Chapter 10 — Multithreading

| ID | Prompt |
|----|--------|
| P060 | **Race quiz** — Which snippets are data races in C++ but rejected by Rust compiler? |
| P061 | **Channel design** — Worker pool with mpsc: I describe throughput; you sketch thread count + channel shape. |
| P062 | **Mutex vs RwLock** — Read-heavy sensor cache — pick primitive and why. |
| P063 | **Send fix** — I try to move `Rc` into thread; show fix with Arc. |
| P064 | **Join panic** — What happens if spawned thread panics? Handle in main. |
| P065 | **Python GIL** — Compare this Python threading example to Rust for same task. |

## Chapter 11 — Atomics

| ID | Prompt |
|----|--------|
| P066 | **Ordering quiz** — For shutdown flag + published config pointer, which orderings? Justify briefly. |
| P067 | **Counter port** — Replace Mutex counter with AtomicUsize; discuss lost updates with Relaxed. |
| P068 | **ABA problem** — Explain ABA in 80 words for compare_exchange — no full queue impl. |
| P069 | **Java AtomicInteger** — Map Java atomic increment to Rust fetch_add snippet. |
| P070 | **When not** — Three cases atomics are the wrong tool; prefer channels or Mutex. |
| P071 | **Fence intuition** — Draw happens-before arrow diagram for Release store + Acquire load. |

## Chapter 12 — Async and Tokio

| ID | Prompt |
|----|--------|
| P072 | **Future diagram** — Draw state machine for async fn with two await points. |
| P073 | **Tokio scaffold** — Generate minimal Tcp echo server skeleton; I fill body. |
| P074 | **select! scenario** — Cancel slow request when fast path returns — outline `select!`. |
| P075 | **async vs thread** — 1000 Modbus polls — argue async vs thread pool for latency. |
| P076 | **blocking fix** — Identify blocking calls in async snippet; suggest `spawn_blocking`. |
| P077 | **Python asyncio** — Map asyncio gather to Tokio join — API comparison table. |

## Chapter 13 — Metaprogramming

| ID | Prompt |
|----|--------|
| P078 | **Macro vs fn** — Rewrite macro as generic fn if possible; when impossible, say why. |
| P079 | **derive need** — List derives I want for config struct loaded from TOML — justify each. |
| P080 | **Hygiene** — Explain macro hygiene in 60 words with `$crate` mention. |
| P081 | **Debug expand** — Walk me through `cargo expand` on derive Debug output (conceptual). |
| P082 | **DSL sketch** — Design tiny `command!` macro for CLI subcommands — tokens only. |
| P083 | **Java annotation** — Map Lombok `@Data` to Rust derive set — what's missing? |

## Chapter 14 — Unsafe

| ID | Prompt |
|----|--------|
| P084 | **Invariant list** — For raw pointer to buffer + length, list 5 invariants safe wrapper must enforce. |
| P085 | **Soundness** — Explain "safe Rust can't cause UB" vs unsafe — one paragraph. |
| P086 | **FFI checklist** — Checklist for calling C library from Rust binary. |
| P087 | **Miri** — What is Miri and when run it relative to unsafe changes? |
| P088 | **Avoid** — Review use case: speed up JSON — unsafe vs simd crate vs algorithm. |
| P089 | **Java JNI** — Compare JNI pitfalls to Rust FFI ownership rules. |

## Chapter 15 — I/O and processes

| ID | Prompt |
|----|--------|
| P090 | **Trait refactor** — Refactor file copy loop to generic `copy<R: Read, W: Write>`; discuss error propagation. |
| P091 | **CSV tool** — Spec for CLI: read two-column CSV, emit `name=value`; I implement with BufRead. |
| P092 | **Packet layout** — Add CRC byte to 4-byte packet; update encode/decode with XOR — show tests. |
| P093 | **Command safety** — Review shell=True style command; rewrite without shell when possible. |
| P094 | **Endian trap** — Quiz: 3 scenarios pick LE vs BE for Modbus-style register. |
| P095 | **Pipeline** — Design `program A \| program B` using only Rust std (two processes, pipe). |

## Chapter 16 — Automation lab

| ID | Prompt |
|----|--------|
| P096 | **Capstone scaffold** — Generate module tree and function signatures for sensor_gateway; no bodies. |
| P097 | **Serial debug** — I get timeout on read; give systematic checklist (baud, cable, permissions). |
| P098 | **Retry policy** — Design exponential backoff for Modbus-style poll errors; Rust pseudocode. |
| P099 | **Log schema** — Propose JSON log lines for sensor events with timestamp and error codes. |
| P100 | **GPIO next step** — After serial works on Pi, outline migration to gpio-cdev for one LED. |
| P101 | **Code review** — I paste capstone main loop; review for panic risks and missing flush. |

---

**Total: 101 prompts** (target ≥80 met).

## By theme

| Theme | IDs |
|-------|-----|
| Ownership / borrow | P006–P011, P018–P023, P024–P029 |
| Types / traits | P030–P041 |
| Errors / tests | P042–P047 |
| Concurrency | P060–P077 |
| Systems / automation | P090–P101 |
| Meta / tooling | P001–P017, P078–P089 |

## See also

- [PLAYGROUND_GUIDE.md](PLAYGROUND_GUIDE.md)
- [JAVA_PYTHON_RUST_MAP.md](JAVA_PYTHON_RUST_MAP.md)
- [CONTENTS.md](../CONTENTS.md)
