# AI Prompt Index

All **Afterparty: AI Lego blocks** from the book, numbered for reuse. Paste into any AI assistant after reading the linked chapter.

## How to use Afterparty

Full workflow (starter context, one chat per chapter, verification): [Preface — Afterparty](../chapters/preface.md#afterparty-aim-importance-and-how-to-use).

Quick loop (read and run examples first — see [How to work through a chapter](../chapters/preface.md#how-to-work-through-a-chapter)):

1. Open a **new chat** for this chapter; paste the [starter context](../chapters/preface.md#how-to-use) from the preface.
2. Paste **one** Afterparty prompt; read the answer and follow up until you understand it — verify code in the playground or with `cargo check` before the next prompt.
3. Push back if the answer is vague or wrong.

Find a prompt by ID (`P046` → table below), open the linked chapter for context, paste the prompt into the **same chapter chat**, and keep correcting until the model matches the compiler.

---

## Preface

| ID | Prompt |
|----|--------|
| P001 | **Learning plan** — I already program in at least one language. Based on this preface, build me a 2-week study plan using only Rust Core's chapter list; 45 minutes per day. |
| P002 | **Gap check** — Ask me five quick questions to see if I should skip straight to Chapter 5 (lifetimes) or read Chapters 1–4 first. |
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
| P106 | **Stack vs heap quiz** — For 8 Rust variable declarations, I say stack-only, heap involved, or both; you correct and draw the pointer picture for `String` and `Vec`. |
| P107 | **Scope and drop** — Given a nested-block snippet with `String`, trace exactly when heap memory is freed vs when stack slots disappear. |
| P108 | **Stack frame drill** — Explain stack frames and pointer-bump allocation: I give you a 3-function call chain with locals; you trace frame push/pop and when each stack slot dies. |
| P109 | **Compile-time size** — Quiz me: for 10 Rust types (`i32`, `[u8; 4]`, `String`, `Vec<i32>`, `&str`, tuple, struct with String field), is the value's size known at compile time? Correct wrong answers and say what still lives on the heap. |
| P110 | **Pointer bump story** — In ≤150 words, explain stack allocation as a pointer bump inside a function frame; contrast with heap allocation through an allocator. Use one numeric example (e.g. three `i32` locals). |
| P111 | **Heap owner trace** — I paste a snippet with `String` and `Vec`; step through who owns the heap buffer after each line, when `drop` runs, and whether any heap copy happened. |
| P112 | **Latency compare** — Compare stack pop vs heap allocate+free for a 1 kHz loop that creates a small `String` every tick vs reusing a stack `i32` counter — worst-case latency in plain language. |
| P113 | **Three-language map** — Same program: one `int`, one growable string, one function return. For Java, Python, and Rust, tabulate: stack vs heap for each, and who frees heap memory. |
| P114 | **Move at two levels** — When `let s2 = s1` for `String`, explain what moves on the stack vs what stays on the heap; why the heap buffer is not copied unless I call `.clone()`. |
| P141 | **Checklist drill** — Match 8 Chapter 1 compiler errors to broken snippets; I name the smallest fix. |
| P142 | **r then push** — Show `let r = &s; s.push(...)` failing; I fix with nested block; explain borrow lifetime. |
| P143 | **Double &mut** — Two `&mut` to same `Vec`; explain error and rewrite safely. |
| P144 | **Move while borrowed** — `let r = &s; let t = s;` — predict error; contrast with Java. |
| P145 | **Call twice by value** — Same `String` to two `fn take(String)`; show failure and idiomatic fixes. |
| P146 | **Return & to local** — Explain `fn bad() -> &str` with local `String`; three fixes without heavy lifetime syntax. |
| P147 | **Frame buffer loan** — `&mut frame[0]` then `frame.push` — overlapping borrow; one safe automation pattern. |
| P148 | **Error translator** — Paste `E0382` or borrow error; I rewrite as one-sentence rule; you tighten. |
| P022 | **Borrow checker tutor** — I paste compiler errors; you explain the borrow conflict and show the smallest fix. |
| P023 | **Five snippets** — Move vs borrow quiz: 5 code fragments, I label ok/error and why. |
| P024 | **Python port** — This Python function mutates a list passed in; rewrite with `&mut Vec` and explain aliasing rules. |
| P025 | **Java port** — This Java method stores the passed List in a field; show the Rust ownership split (return owned vs `Arc`). |
| P026 | **Slice drill** — Given `&[i32]`, write `first` and `rest` without panicking on empty — use `Option`. |
| P027 | **clone audit** — Review my 30-line Rust snippet; mark unnecessary `.clone()` calls. |

## Chapter 2 — Types and expressions

| ID | Prompt |
|----|--------|
| P012 | **Integer pick** — Quiz me: for 8 scenarios (Modbus port, byte buffer, collection index, money cents, timestamp millis, hash output, loop counter, enum discriminant), I pick u8/u16/u32/u64/i32/usize; you correct and explain overflow risk. |
| P013 | **Suffix or annotate** — Give 5 snippets where integer inference is ambiguous or wrong; I add a suffix or type annotation; you verify. |
| P014 | **Overflow policy** — Three arithmetic scenarios in a control loop. For each, recommend plain +, checked_add, saturating_add, or wrapping_add; explain panic vs wrap in debug/release. |
| P015 | **char vs byte** — Explain why 'A' is not the same type as 65u8. Show one valid char literal and one invalid escape; mention UTF-8 only when discussing str/String. |
| P016 | **Tuple vs array** — When do I use (u16, u16) vs [u16; 2] for a coordinate pair? Give one idiomatic example of each. |
| P017 | **&str vs String API** — Five function signatures for logging labels. I choose &str or String for each; you explain ownership and allocation cost. |
| P018 | **Range quiz** — For half-open vs inclusive ranges, I predict output of four for loops using .. and ..=; you correct with printed values. |
| P019 | **match warm-up** — Extend a match on HTTP status codes with 3xx, 4xx, 5xx groupings using range patterns; I write arms; you check exhaustiveness. |
| P020 | **Type read-through** — Paste a 20-line main with mixed types. I annotate every binding with its inferred or explicit type without running the compiler; you correct. |
| P021 | **Protocol struct sketch** — Design a [u8; N] frame parser header: field names, types, and which values must be explicit width. No full impl — types only. |

## Chapter 3 — Functions and methods

| ID | Prompt |
|----|--------|
| P239 | **Parameter audit** — Five function signatures for logging: `&str`, `String`, `&String`, `Cow<str>`, `impl AsRef<str>`. I pick one per use case; you explain ownership cost. |
| P240 | **Move vs borrow** — Snippet calls `process(s)` then uses `s` again. I explain the error and show two fixes (`&s`, clone). |
| P241 | **Java map** — Map Java method `void consume(List<String> xs)` to idiomatic Rust — owned vs borrowed slice of strings. |
| P242 | **impl block** — Struct `Timer` with `start`, `elapsed`, `reset`. I write `impl`; you check `&self` vs `&mut self`. |
| P243 | **Associated fn** — When is `Type::new()` idiomatic vs `Default::default()`? One example each. |
| P244 | **Consume self** — Method `fn into_inner(self) -> Vec<u8>` — why must it take `self` by value? |
| P245 | **Semicolon trap** — Three tiny functions: one returns `i32` correctly, two fail due to `;`. I fix them. |
| P246 | **Early return** — Rewrite nested `if` in a parser as early `return None` / `?` style. |
| P247 | **Unit vs value** — Which functions should return `()` vs `bool` vs `Option<T>`? Three CLI helper names, I choose. |
| P248 | **Generic bounds** — Fix `fn max(a: T, b: T)` without bounds; add `T: Ord` or `PartialOrd`. |
| P249 | **Result signature** — Design `fn read_config(path: &str) -> Result<Config, ...>`; list error variants, no body. |
| P250 | **Capstone** — Split a 40-line `main` into 4 functions with clear signatures; list names and params only, I implement. |

| P381 | **impl Iterator return** — `top_n` returns `impl Iterator` — why two iterator types in `if` arms fail. |
| P382 | **mem take drain** — Buffer drains via `mem::take` — show before/after inner field. |
| P383 | **where clause** — Rewrite cluttered generic bounds with `where` block. |
| P384 | **Self return** — Method `into_inner(self) -> Vec<u8>` — why `Self` not concrete type? |
| P385 | **const fn** — `const fn is_valid_port(p: u16) -> bool` — compile-time limits. |
| P386 | **pick impl Trait** — Factory returning different iterator types — fix with `Box<dyn Iterator>`. |
| P387 | **Drain method** — Design `Buffer::drain_into(&mut Vec<u8>)` signature and ownership. |
| P388 | **Capstone signatures** — CLI tool four functions — signatures with ownership only. |
## Chapter 4 — Iterators

| ID | Prompt |
|----|--------|
| P115 | **Loop port** — Rewrite this C-style indexed loop as an iterator chain; preserve behavior and types. |
| P116 | **iter vs into_iter** — Give 4 snippets using `Vec`; I predict whether `v` is usable after the loop; you explain move vs borrow. |
| P117 | **collect turbofish** — Three `collect()` calls that fail without hints — I add type annotation or turbofish; you verify. |
| P118 | **Adapter chain** — Task: parse lines, trim, keep non-empty, parse as `u16`. I sketch `.lines().map(...).filter(...).collect()`; you refine. |
| P119 | **enumerate vs index** — Same sum-over-evens task twice: index `for` vs `.enumerate()`. Compare readability and bounds-check risk. |
| P120 | **Lazy vs eager** — Explain when `filter().map()` allocates vs when `.collect()` forces work. One example with println in `map` showing evaluation order. |
| P121 | **Stack pattern map** — I name a task in Java (`Stream`), Python (comprehension), and SQL. You show the idiomatic Rust iterator chain for each. |
| P122 | **Python comp port** — Translate `[x * 2 for x in nums if x > 0]` into a Rust `Vec` pipeline; explain lazy vs eager. |
| P123 | **Unix pipe analogy** — In ≤120 words, map `cmd1 \| cmd2` to Rust iterator adapters; where does `.collect()` fit? |
| P124 | **for desugar quiz** — Four `for` loops (`0..n`, `&vec`, `vec`, `&mut vec`). I say iter vs into_iter vs range; you correct. |
| P125 | **iter_mut drill** — Double every element in `Vec<f64>` in place; I write the loop; you review `*n` and borrows. |
| P126 | **Range vs collect** — When is `for i in 0..n` better than collecting `(0..n)`? Two automation examples. |
| P127 | **zip pairs** — Zip two `Vec<u16>` into pairs; I write it; you handle length mismatch. |
| P128 | **take and skip** — Paginate a log with `.skip().take()` on `.lines()`; show one pitfall. |
| P129 | **chain iterators** — Concatenate two `&[u8]` slices with `.chain()` without copying into one array first. |
| P130 | **find and Option** — Find first log line containing `ERROR` with `.find()`; contrast `.filter().next()`. |
| P131 | **fold vs sum** — Max and count in one `.fold()` pass vs separate calls — when is fold worth it? |
| P132 | **any and all** — Validate ports and comment lines with `.all()` / `.any()`; fix a double-reference mistake. |
| P133 | **Moved Vec mistake** — Code uses `for x in v` then `v` again; I explain and fix; you rank fixes. |
| P134 | **Borrow in chain** — `Vec<&str>` from `String` lines then drop strings; I explain failure and fix. |
| P135 | **Modbus-style scan** — Filter even `u16` registers, scale to `f64`, sum — write and check types/overflow. |
| P136 | **Zero-cost check** — Does `iter().filter().map().sum()` allocate intermediate `Vec`s in `--release`? |
| P137 | **Trap sheet drill** — Six snippets mixing `for x in v`, `&v`, `into_iter`, and `collect` errors; I predict ok/fail; you fix. |
| P138 | **&&i32 decoder** — Label closure param types in one `.iter().filter().map()` chain; show three compiling closure styles. |
| P139 | **Empty iterator policy** — Sum, find, all on empty `Vec`; I state results and domain bugs; you correct. |
| P140 | **zip truncation** — Zip unequal `Vec` lengths; explain silent loss and sketch a safe pairing strategy. |
| P348 | **PortScan impl** — Implement `Iterator` for ports 502..=505; collect and sum — show `type Item` and `fn next`. |
| P349 | **Skip blanks** — `NonEmptyLines` iterator trims and skips empty lines; collect keys before `=`. |
| P350 | **Infinite take** — Counter from 0 — why `.take(n)` before `.collect()`? Show hang vs bounded. |
| P351 | **IntoIterator pair** — Same struct implements `Iterator` and `IntoIterator` for `for` loop. |
| P352 | **Stateful parser** — Byte buffer iterator yielding 4-byte frames — sketch `next()` state machine. |
| P353 | **Capstone iterator** — CSV fields parse col 2 as `u16`, filter > 0 — custom struct + consumer chain. |

## Chapter 5 — Lifetimes

| ID | Prompt |
|----|--------|
| P028 | **Error archaeology** — I paste a "lifetime may not live long enough" error; walk me through owner vs reference diagram. |
| P029 | **Return type choice** — For API `fn title(book: &Book) -> ???` compare `&str` vs `String` trade-offs for a library. |
| P030 | **Struct lifetime** — Design `ConfigParser` holding `&str` slices into input buffer — when is it sound vs use owned `String`? |
| P031 | **Elision quiz** — Add explicit lifetimes to 4 function signatures where elision fails. |
| P032 | **Java analogy** — Compare Rust lifetimes to Java stack locals vs heap references — 120 words, accurate only. |
| P033 | **Fix mine** — I return `&String` built inside function; show three idiomatic fixes ranked by simplicity. |

| P389 | **static trap** — Return `&str` from `format!` — error and owned fix. |
| P390 | **two lifetimes** — `first<'a,'b>(x: &'a str, y: &'b str) -> &'a str` — drop `y` while result lives. |
| P391 | **Config struct** — Parse `host:port` into `Config<'a>` — when must caller keep line alive? |
| P392 | **Owned refactor** — Owned `Config { host: String }` vs borrowed — three tradeoffs. |
| P393 | **T: 'a bound** — Explain `Holder<'a, T: 'a>` — failure when `T` shorter-lived. |
| P394 | **Elision fail** — Four signatures: elision works vs fails — I label each. |
| P395 | **Iterator borrow** — `Vec<&str>` from `String` lines — drop order trap. |
| P396 | **Capstone API** — Public config loader: borrowed view vs owned — pick and defend. |
## Chapter 6 — Enums and pattern matching

| ID | Prompt |
|----|--------|
| P034 | **Null replacement** — Translate 5 Java methods returning null into `Option` Rust; explain callsite changes. |
| P035 | **Exhaustive match** — I have enum with 4 variants; generate match that compiles; then add variant and show compiler error. |
| P036 | **Result railway** — Chain parse → validate → compute with `?`; I fill blanks, you verify. |
| P037 | **if let vs match** — When is `if let` clearer than `match`? Give 3 contrasting snippets. |
| P038 | **State machine** — Model TCP connection states as enum; methods connect, send, close with illegal transition errors. |
| P039 | **Python Union** — This Python function accepts int \| str; design Rust enum + match without dynamic typing. |
| P149 | **unwrap audit** — Paste 20 lines with 4 `unwrap()` calls; I mark panic risk; you rewrite with `match` or `?`. |
| P150 | **Combinator chain** — Parse port `Option<u16>`, double if Some, default 502 — I write `map`/`unwrap_or`; you add `and_then`. |
| P151 | **let-else port** — Rewrite nested `match` on `parse()` into `let Ok(x) = ... else { ... }`; preserve behavior. |
| P152 | **Err arm missing** — Show `match` on `Result` without `Err` arm; I quote error and fix; add boundary `eprintln!` pattern. |
| P153 | **unwrap vs ?** — Same parser: `unwrap` vs `fn -> Result` with `?`; compare panic risk and signature honesty. |
| P154 | **Wildcard footgun** — Explain why `_` on your own enum hides new variants; show explicit arms vs `_` refactor story. |
| P155 | **Partial move** — `enum` with `String` field: `match` moves field; I fix with `ref` or `match &e`; show error text. |
| P156 | **Match on ref** — Owned `Status`: `match s` vs `match &s`; predict move errors; diagram ownership. |
| P157 | **Guard drill** — Classify `i32` with guards; I write arms; you check exhaustiveness. |
| P158 | **Opcode table** — Design `enum` for 3 frame types + `match` opcodes; add fourth type as compile-break exercise. |
| P159 | **ReadOutcome extend** — Add `Disconnected` to `ReadOutcome`; list `match` sites the compiler forces to update. |
| P160 | **Config sentinel** — Rewrite `port() -> i32` returning `-1` to `Option<u16>` + `match` in `main`. |
| P161 | **Checklist drill** — Match 6 Chapter 6 compiler errors to snippets; I name fix (`match` arm, `ref`, `?`). |
| P162 | **Java enum map** — Java `enum State` with method → Rust `enum` + `match` + `impl`; contrast nullability. |

| P359 | **Slice split** — Parse HTTP request line with slice patterns — handle single-token input. |
| P360 | **matches refactor** — Replace 6-arm bool `match` with `matches!` on `Mode` enum. |
| P361 | **if let chain** — Parse `host:port` — chain rejects `host:port:extra`. |
| P362 | **@ binding quiz** — Port range arms with `@` — label which values hit which arm. |
| P363 | **Exhaustive slice** — `[a, b]` on len 1 or 3 — predict `_` arm vs bug; suggest `..` rest. |
| P364 | **matches vs match** — When keep full `match` instead of `matches!`? Example returning `String`. |
| P365 | **Guard + @** — Match port `n @ 1024..=65535` with guard `n % 2 == 0`. |
| P366 | **Capstone parse** — Frame header slice pattern from byte slice — sketch `match` arms only. |

## Chapter 7 — Structs, traits, generics

| ID | Prompt |
|----|--------|
| P040 | **Interface port** — Convert Java interface `Measurable` + two classes to trait + two structs + `impl`. |
| P041 | **Duck typing** — Python function accepts anything with `.read()`; express as trait bound in Rust generic. |
| P042 | **dyn vs impl** — Quiz: 4 scenarios — pick `dyn Trait` or `impl Trait` and justify. |
| P043 | **Default trait methods** — Add default `summary()` on trait; override in one type only. |
| P044 | **Generic bounds** — Fix compiler error: `T` needs `Display + Clone`; minimal bound set. |
| P045 | **OOP myth** — Explain in 100 words why Rust has no inheritance and what you do instead. |

| P354 | **Item type quiz** — Change `PortScan`'s `type Item` from `u16` to `(u16, bool)` — list broken call sites. |
| P355 | **Summarizable design** — `Summarizable` with `type Output = String` for three sensor structs; one `report` fn. |
| P356 | **Associated vs generic** — `trait Get<T>` vs `trait Get { type Value; }` — when is each painful? |
| P357 | **Supertrait bounds** — `Exportable: Display + Debug` with default `export` — impl for `Port(u16)`. |
| P358 | **UFCS fix** — Type implements `A` and `B`, both define `name()` — ambiguous call and UFCS fix. |

## Chapter 8 — Errors and testing

| ID | Prompt |
|----|--------|
| P046 | **? chain** — Refactor nested match on Results to `?` style; explain each change. |
| P047 | **Error enum design** — Design `AppError` for CLI that reads config + talks serial; variants + `From` impls sketch. |
| P048 | **panic audit** — Mark which of 10 `unwrap()` calls should stay vs become `Result`. |
| P049 | **Test generation** — Write table-driven tests for `parse_port` including edge ports. |
| P050 | **Java exceptions** — Map checked Exception flow to Rust `Result` for file-not-found scenario. |
| P051 | **anyhow vs thiserror** — When would I pick each for a binary vs library crate? |

## Chapter 9 — Modules, paths, and crates

| ID | Prompt |
|----|--------|
| P251 | **File tree** — Design module tree for a CLI that reads config and runs commands. Directories + `mod` lines only, no bodies. |
| P252 | **Path quiz** — From `crate::service::worker::run`, how do I reach `crate::config::load`? Show `use` and fully qualified call. |
| P253 | **lib vs bin** — What belongs in `main.rs` vs `lib.rs` for a tool with 500 lines of logic? |
| P254 | **pub audit** — List items that should be `pub` vs private in a library crate exposing `Client::connect`. |
| P255 | **pub(crate)** — When is `pub(crate)` better than `pub` for test helpers? |
| P256 | **Re-export** — Sketch `pub use` so users see `my_crate::Error` but you wrap `thiserror` internally. |
| P257 | **Workspace split** — Two crates: `core` library + `cli` binary. Write `Cargo.toml` dependency path only. |
| P258 | **Integration test** — Where does `tests/smoke.rs` live and how does it `use` the library? |
| P259 | **Orphan fix** — I want `Display` on `Vec<u8>` — show newtype wrapper module layout. |
| P260 | **Split monolith** — Given one `main.rs` with config + parser + runner, name three modules and what each owns. |
| P261 | **cfg test** — Explain why `mod tests` uses `#[cfg(test)]` and `use super::*`. |
| P262 | **Capstone** — Generate `src/` tree for `sensor_core` library + `sensor_cli` binary in one workspace; I implement. |

| P373 | **Feature flag** — Add `serial` feature gating `mod serial_io` — `Cargo.toml` and one `#[cfg]`. |
| P374 | **cfg vs cfg!** — `#[cfg(debug_assertions)]` vs `if cfg!(debug_assertions)` — release binary diff. |
| P375 | **Optional dep** — Optional `tokio` behind feature `async` — show `dep:tokio` line. |
| P376 | **Platform gate** — `#[cfg(target_os = "linux")]` module for device path — sketch. |
| P377 | **Integration layout** — Tree for `tests/load_config.rs` — what API is invisible to test? |
| P378 | **pub use prelude** — Re-export so users call `my_crate::connect` not `internal::connect`. |
| P379 | **doc hidden** — When mark helper `#[doc(hidden)]` on public re-export surface? |
| P380 | **Capstone crate** — `gateway` crate: `serial` feature, integration test, `///` on parse fn — tree only. |
## Chapter 10 — Smart pointers and interior mutability

| ID | Prompt |
|----|--------|
| P058 | **Box why** — When is `Box<[T]>` better than `Vec<T>` on the stack? Two cases. |
| P059 | **Rc cycle** — Explain why `Rc` cycles leak; contrast with Python reference cycles and `Weak` fix. |
| P062 | **Arc Mutex sketch** — Diagram thread-safe cache with Arc<Mutex<HashMap>> — no full code. |
| P063 | **Java heap map** — Map Java "everything is reference" to Rust ownership — when `Arc`, when plain `&`, when neither. |
| P283 | **RefCell trap** — Show double `borrow_mut` panic; fix with scoped borrows. |
| P284 | **Arc vs Rc** — Thread spawn with `Rc` — show error; fix with `Arc`; explain atomic count overhead. |
| P285 | **Deref coercion** — Why does `fn takes_str(s: &str)` accept `&String`, `&Box<String>`, and `&Rc<String>`? Trace steps. |
| P286 | **Pick pointer** — Five scenarios (AST node, thread cache, GUI graph, config string, plugin list) — I pick Box/Rc/Arc/RefCell/Weak; you grade. |
| P289 | **Recursive list** — Draw memory for `Cons(1, Box::new(Cons(2, Nil)))` — stack vs heap boxes. |
| P290 | **Move out of Box** — What happens after `let s = *box_string`? When idiomatic vs keeping the `Box`? |
| P291 | **Trait object box** — Three plugin types implement `Plugin` — sketch `Vec<Box<dyn Plugin>>`; why not `Vec<Plugin>`? |
| P292 | **Handle vs deep clone** — Audit snippet with `Rc<String>` and both `Rc::clone` and `(*rc).clone()` — label cost of each. |
| P293 | **Weak upgrade** — Parent/child with `Rc` parent and `Weak` child back-ref — I sketch types; you explain cycle break. |
| P294 | **strong_count debug** — `strong_count` stays at 2 after I thought I dropped all refs — list 5 places handles hide. |
| P295 | **Immutable then mut** — Hold `let r = cell.borrow()` and call `borrow_mut` — explain panic; fix with nested block. |
| P296 | **Cell vs RefCell** — Counter `u32` vs `Vec` cache — I pick `Cell` or `RefCell` each; you correct. |
| P297 | **Rc RefCell graph** — Two nodes share `Rc<RefCell<Node>>` — one updates, one reads — sketch borrow rules on one thread. |
| P298 | **Compile vs runtime** — Same overlapping-mut pattern: compile error with `&mut` and runtime panic with `RefCell` side by side. |
| P299 | **Drop order** — Three `Drop` structs in one function — I predict print order; you confirm reverse declaration rule. |
| P300 | **Rc last handle** — Two `Rc` clones dropped at different times — when does inner `Drop` run? Step through with println in `Drop`. |
| P301 | **Drop panic** — Explain double-panic abort if `Drop` panics during unwind — link to Ch8. |
| P302 | **Refactor to smart ptr** — I paste struct with `Box`, `Rc`, or raw tree — you suggest minimal smart pointer fix and justify. |
| P303 | **Leak hunt** — Sketch `Rc` cycle in observer pattern; refactor one edge to `Weak` and explain count after each drop. |

## Chapter 11 — Collections

| ID | Prompt |
|----|--------|
| P052 | **Loop port** — Rewrite C-style indexed loop as iterator chain; preserve behavior. |
| P053 | **HashMap merge** — Two maps of scores — merge by max per key; iterator + entry style. |
| P054 | **entry drill** — Word frequency from `Vec<&str>` using only `.entry` — no double lookup. |
| P055 | **collect types** — Three `collect()` calls that need type hints — fix with turbofish. |
| P056 | **Windows** — Detect rising edges in `Vec<f64>` with `.windows(2)`; extend to `.windows(3)`. |
| P057 | **Performance myth** — Do Rust iterators optimize to loops? When might they not? |
| P287 | **Pick collection** — Five tasks (dedup, range scan, FIFO, index by id, min-key) — I pick collection each. |
| P288 | **Set ops** — Tags on two records — union, intersection, difference with `HashSet`. |
| P304 | **Hash vs BTree** — Same 10k insert + range scan — when HashMap wins vs BTreeMap. |
| P305 | **Queue anti-pattern** — Review `v.remove(0)` queue loop — cost and `VecDeque` fix. |
| P306 | **get vs index** — Four access patterns — I pick `[i]` vs `.get(i)` vs `.get_mut` vs `if let Some`. |
| P307 | **sort dedup** — Dedup `[3,1,4,1,5]` wrong vs right — sort + dedup pipeline. |
| P308 | **retain vs filter** — Remove evens in-place vs new `Vec` — compare `retain` and `filter().collect()`. |
| P309 | **Borrow push trap** — Explain `let r = &v[0]; v.push(1)` error; fix with scope. |
| P310 | **or_insert_with** — Lazy cache: expensive `Vec` built once per key — sketch with `or_insert_with`. |
| P311 | **insert overwrite** — Track old value on port remap using `insert` return. |
| P312 | **Stable dedup** — Unique `String` lines preserving first-seen order — no HashSet-only collect. |
| P313 | **chunks vs windows** — Parse byte stream into 4-byte frames — `chunks(4)` vs `windows(4)` when? |
| P314 | **Duplicate keys collect** — `collect` to HashMap from duplicate-key pairs — predict final map. |
| P315 | **Capacity hint** — Read 1M lines into `Vec` — when `with_capacity` matters; sizing rule. |
| P316 | **Capstone store** — Register id → reading + range scan — pick map type, list three API methods. |

## Chapter 12 — Closures and the Fn traits

| ID | Prompt |
|----|--------|
| P263 | **Fn quiz** — Four closures: I label each Fn / FnMut / FnOnce; you correct and explain capture. |
| P264 | **move drill** — Thread spawn snippet missing `move` — show compile error and fix. |
| P265 | **Iterator chain** — `.filter` closure that uses `&config` — why `Fn` not `FnMut`? |
| P266 | **fn vs closure** — When can you pass `fn()` vs `impl Fn()` to the same helper? |
| P267 | **Return closure** — Write `make_multiplier(f: f64) -> impl Fn(f64) -> f64` and explain `move`. |
| P268 | **Double reference** — Fix `.iter().filter(|x| ...)` type error on `Vec<String>`. |
| P269 | **sort_by** — Sort `Vec<(String, u32)>` by count descending with `sort_by` closure. |
| P270 | **for_each vs for** — Same side-effect loop twice: `for` vs `.for_each` — style tradeoffs. |
| P271 | **Box dyn Fn** — Store heterogeneous callbacks in a Vec — sketch trait object version. |
| P272 | **Capstone** — Pipeline: lines, filter, parse `u16`, sum — all with closures; I write; you review Fn bounds. |
| P330 | **RefCell bump** — Closure mutates `RefCell<u32>` counter — which Fn trait and why? |
| P331 | **Callback registry** — Three log filters in `Vec<Box<dyn Fn(&str) -> bool>>` — I add wrong signature; you fix. |
| P332 | **Loop move trap** — Building `Vec<Box<dyn Fn()>>` in `for` over `String` — show move error and clone fix. |
| P333 | **sort_by_key** — Same sort with `sort_by_key` — when is key extraction cleaner? |
| P334 | **retain valid** — Drop invalid `SensorReading` rows with `.retain` — FnMut bound. |
| P335 | **Fn bound strict** — Helper takes `impl Fn()` but closure mutates — fix signature. |
| P336 | **Thread move** — `spawn` closure borrows `String` — show error without `move` and fix. |
| P337 | **Send Box Fn** — When does `Box<dyn Fn() + Send>` matter for thread callbacks? |
| P338 | **Callback capstone** — Registry + sort pipeline + thread handoff — review Fn bounds on my code. |
| P339 | **apply_twice generic** — `impl Fn` vs `Box<dyn Fn>` for helper called twice — cost comparison. |
| P340 | **Async move note** — One `async move` block capturing `String` — same rules as sync closure. |
| P341 | **Dedup_by closure** — Dedup adjacent `SensorReading` by id with `dedup_by` — FnMut. |
| P342 | **Filter config** — `.filter` reading `&settings` — prove closure is `Fn` not `FnMut`. |
| P343 | **FnOnce consume** — Closure that moves `String` out on first call — storage implications. |
| P344 | **Heterogeneous vec** — Why `Vec<Box<dyn Fn(i32)>>` cannot mix different capture sizes — one paragraph. |
| P345 | **Iterator sort chain** — `.filter().map().collect()` then `sort_by` — label each closure's Fn trait. |
| P346 | **Scope vs move** — Compare `thread::scope` borrow closure vs `spawn(move)` — when each. |
| P347 | **Capstone registry** — Full callback registry filtering logs by severity — I write; you audit traits. |

## Chapter 13 — Standard traits and conversions

| ID | Prompt |
|----|--------|
| P273 | **Debug vs Display** — When derive `Debug` only vs implement `Display` for a CLI status line? |
| P274 | **Redacted Debug** — Struct with `api_key: String` — sketch manual `Debug` with `[REDACTED]`. |
| P275 | **From chain** — `String` → `MyLabel` via `From`; add `From<&str>` without duplicating logic. |
| P276 | **TryFrom port** — Port validation with custom enum error `OutOfRange`. |
| P277 | **parse vs TryFrom** — When `s.parse::<u16>()` vs `u16::try_from(x)` vs custom `FromStr`? |
| P278 | **AsRef drill** — Rewrite three functions taking `&String` to `impl AsRef<str>`. |
| P279 | **Cow API** — Normalize slug: accept `Cow<str>`, return borrowed if valid else owned. |
| P280 | **Derive set quiz** — Map key, log line, sortable row, error enum — I list derives each needs. |
| P281 | **Display impl** — Implement `Display` for `Port(u16)` showing `Port(8080)`. |
| P282 | **Mini crate API** — Public `HostPort` with `Display`, `TryFrom<&str>` for `host:port` — list impl blocks only. |
| P317 | **Pretty debug** — Same struct with `{:#?}` vs `{:?}` — when does pretty-print help in tests? |
| P318 | **Default enum** — Three-variant mode enum — derive `Default` with `#[default]` on `Auto`. |
| P319 | **Eq on floats** — Struct with `f64` field — show `Eq` derive failure; three fixes. |
| P320 | **HashMap key** — Why does `UserId(String)` need `Eq + Hash` for `HashMap` keys? |
| P321 | **FromStr type** — Parse `host:port` into struct — sketch `FromStr` with split. |
| P322 | **Silent cast trap** — Show `70000i32 as u16` vs `TryFrom` — predict values. |
| P323 | **From in errors** — Wire `ParseIntError` into `AppError` with `From` — list impl only. |
| P324 | **AsRef bytes** — Log wire payload — `impl AsRef<[u8]>` accepts `Vec`, slice, array. |
| P325 | **Borrow lookup** — Explain `HashMap<String, V>.get(&str)` — role of `Borrow<str>`. |
| P326 | **into_owned** — When caller needs `String` after `Cow` helper — where `into_owned()`? |
| P327 | **Newtype Display** — Wrap `Vec<u8>` as `HexBytes` — implement `Display` without orphan violation. |
| P328 | **Derive audit** — Config with secrets + TOML — list safe vs unsafe derives. |
| P329 | **Capstone traits** — Design `RateLimit` public API: parsing, display, equality — traits only. |

## Chapter 14 — Multithreading

| ID | Prompt |
|----|--------|
| P064 | **Race quiz** — Which snippets are data races in C++ but rejected by Rust compiler? |
| P065 | **Channel design** — Worker pool with mpsc: I describe throughput; you sketch thread count + channel shape. |
| P066 | **Mutex vs RwLock** — Read-heavy sensor cache — pick primitive and why. |
| P067 | **Send fix** — I try to move `Rc` into thread; show fix with Arc. |
| P068 | **Join panic** — What happens if spawned thread panics? Handle in main. |
| P069 | **Python GIL** — Compare this Python threading example to Rust for same task. |

| P367 | **RwLock cache** — Sketch `Arc<RwLock<HashMap>>` read-heavy cache; when write starves readers? |
| P368 | **Mutex vs RwLock** — Same cache with 50% writes — pick primitive and justify. |
| P369 | **OnceLock init** — `get_or_init` behaviour when called twice — same value guarantee. |
| P370 | **Scope borrow** — Parallel sum over chunks with `thread::scope` — why plain `spawn` fails. |
| P371 | **Poison recovery** — Writer panics holding `RwLock` — poisoned `read()` and recovery. |
| P372 | **Capstone sync** — Lazy config (`OnceLock`), shared cache (`RwLock`), scoped workers — types only. |
## Chapter 15 — Atomics

| ID | Prompt |
|----|--------|
| P070 | **Ordering quiz** — For shutdown flag + published config pointer, which orderings? Justify briefly. |
| P071 | **Counter port** — Replace Mutex counter with AtomicUsize; discuss lost updates with Relaxed. |
| P072 | **ABA problem** — Explain ABA in 80 words for compare_exchange — no full queue impl. |
| P073 | **Java AtomicInteger** — Map Java atomic increment to Rust fetch_add snippet. |
| P074 | **When not** — Three cases atomics are the wrong tool; prefer channels or Mutex. |
| P075 | **Fence intuition** — Draw happens-before arrow diagram for Release store + Acquire load. |

## Chapter 16 — Async and Tokio

| ID | Prompt |
|----|--------|
| P076 | **Future diagram** — Draw state machine for async fn with two await points. |
| P077 | **Tokio scaffold** — Generate minimal Tcp echo server skeleton; I fill body. |
| P078 | **select! scenario** — Cancel slow request when fast path returns — outline `select!`. |
| P079 | **async vs thread** — 1000 Modbus polls — argue async vs thread pool for latency. |
| P080 | **blocking fix** — Identify blocking calls in async snippet; suggest `spawn_blocking`. |
| P081 | **Python asyncio** — Map asyncio gather to Tokio join — API comparison table. |

## Chapter 17 — Metaprogramming

| ID | Prompt |
|----|--------|
| P082 | **Macro vs fn** — Rewrite macro as generic fn if possible; when impossible, say why. |
| P083 | **derive need** — List derives I want for config struct loaded from TOML — justify each. |
| P084 | **Hygiene** — Explain macro hygiene in 60 words with `$crate` mention. |
| P085 | **Debug expand** — Walk me through `cargo expand` on derive Debug output (conceptual). |
| P086 | **DSL sketch** — Design tiny `command!` macro for CLI subcommands — tokens only. |
| P087 | **Java annotation** — Map Lombok `@Data` to Rust derive set — what's missing? |
| P163 | **Expansion order** — List compiler phases from tokens to LLVM; where do macros run? |
| P164 | **Tokens vs types** — Why can a macro compile but expanded code fail? One example. |
| P165 | **Follow-set why** — Explain in 80 words why `$a:expr = $b:expr` is forbidden in matchers. |
| P166 | **Scope honesty** — List 5 metaprogramming topics Ch17 skips and where to learn each. |
| P167 | **Trace checklist** — Give 6 reasons macro code is hard to trace and one mitigation each. |
| P168 | **Fragment picker** — I describe a DSL shape; you pick `expr`/`ident`/`tt` for each slot. |
| P169 | **Expr equals fix** — Fix `set_reg!($addr = $val)` matcher; show two valid surface syntaxes. |
| P170 | **Trailing comma** — Explain `$(x),*` vs `$(x),+` on empty input; show failing and fixed macro. |
| P171 | **Register DSL** — Extend `register_map!` with a third register; explain `stringify!` arm. |
| P172 | **Clone expand** — Show conceptual expanded `impl Clone` for struct with two `i32` fields. |
| P173 | **Enum vs struct** — How does derived `PartialEq` differ for enum vs struct? Sketch match arms. |
| P174 | **Field bound failure** — Struct with `Mutex<i32>` field + `#[derive(Clone)]` — quote error and fix. |
| P175 | **Redacted Debug** — When hand-write `Debug` instead of derive on command enum with secrets. |
| P176 | **Copy vs Clone quiz** — Classify 8 types: Copy, Clone only, or neither; justify. |
| P177 | **Hot-loop clone audit** — Audit poll loop with `.clone()` each tick; suggest move/`Arc`/borrow. |
| P178 | **Arc vs derive Clone** — Explain cheap `Arc` clone vs deep `String` clone with one snippet. |
| P179 | **Java clone compare** — Compare Java `.clone()` / Python `copy` to Rust `Clone` derive. |
| P180 | **Serde rename** — Field `poll_ms` in JSON as `pollIntervalMs` — show attr; trap on refactor. |
| P181 | **thiserror vs manual** — Same error enum: count lines derive vs hand-written (Ch8 style). |
| P182 | **clap subcommands** — Sketch `#[derive(Subcommand)]` enum for gateway start/stop/status. |
| P183 | **Float Eq trap** — Show `#[derive(Eq)]` on `f64` field failure; two fixes from Ch7. |
| P184 | **cargo expand walkthrough** — Step-by-step: install, run, read output for one derive. |
| P185 | **In expansion of** — Decode a 3-note compiler error chain from nested macro + derive. |
| P186 | **tt vs expr escape** — When switch matcher from `expr` to `tt`; tradeoffs in 80 words. |
| P187 | **Three-layer trace** — Derive inside attribute inside macro_rules — debug layer by layer. |
| P188 | **Port to const fn** — Replace tiny numeric macro with `const fn`; when macro still needed? |
| P189 | **env vs var** — Compare `env!`, `option_env!`, `std::env::var` — table with one use case each. |
| P190 | **include_str config** — Embed default TOML with `include_str!`; deserialize at startup sketch. |
| P191 | **Macro vs fn audit** — Mark 6 snippets: should be macro, derive, or plain fn — justify. |
| P192 | **When not proc macro** — Three scenarios where proc macro is overkill; alternative each. |
| P193 | **Minimal derive set** — Gateway config + error + CLI: smallest derive list that still ships. |
| P194 | **Trap quiz** — Mark 8 snippets: empty `+`, double brace, Default enum, Arc Clone, env!, duplicate impl, cfg macro, serde rename. |
| P195 | **Duplicate register DSL** — Design compile-time error for duplicate keys in register_map! |
| P196 | **Serde refactor test** — Integration test plan after renaming TOML field with serde attrs. |
| P197 | **Modbus table macro** — Spec register table macro generating lookup + const max address. |
| P198 | **Derive soup review** — I paste 40-line struct with 12 derives; trim to minimal set with reasons. |
| P199 | **For after expr** — Why is `$e:expr for $i:ident in $r:expr` illegal? Show legal `$p:pat in $r:expr` foreach macro. |
| P200 | **Double-brace fix** — Fix `poll_twice!` macro that mixes `let` and `for` for use in `let x = poll_twice!()`. |

## Chapter 18 — Unsafe

| ID | Prompt |
|----|--------|
| P088 | **Invariant list** — For raw pointer to buffer + length, list 5 invariants safe wrapper must enforce. |
| P089 | **Soundness** — Explain "safe Rust can't cause UB" vs unsafe — one paragraph; include unsound safe wrapper example. |
| P090 | **FFI checklist** — Checklist for calling C library from Rust binary. |
| P091 | **Miri** — What is Miri and when run it relative to unsafe changes? |
| P092 | **Avoid** — Review use case: speed up JSON — unsafe vs simd crate vs algorithm. |
| P093 | **Java JNI** — Compare JNI pitfalls to Rust FFI ownership rules. |
| P201 | **Scope honesty** — List 6 topics Ch18 skips and where to learn each (nomicon, Miri, Pin, …). |
| P202 | **Aim table** — Fill: why Vec needs `unsafe` internally while `push` stays safe for callers. |
| P203 | **Promise diagram** — Draw safe API → unsafe block → invariants → caller cannot UB; label soundness. |
| P204 | **`*const` vs `&T` quiz** — Give 5 snippets: legal ref, needs `unsafe` block, compile error; I classify each. |
| P205 | **from_raw_parts design** — Design `fn view_frame(ptr, len) -> Result<&[u8], Error>` without `&'static`; list invariants. |
| P206 | **Dangling audit** — Show stack pointer used after drop; I explain UB; you show Miri-style symptom. |
| P207 | **Modbus buffer** — Register table as `&[u8]` vs `from_raw_parts` — when is each idiomatic in a gateway? |
| P208 | **Hex preview port** — Port Level 2 `as_hex_preview` to return `Result` on empty buffer; no `unwrap`. |
| P209 | **set_len contract** — Document pre/post conditions for `set_len_unchecked`; what breaks `as_slice` if violated? |
| P210 | **Send proof** — I claim `Rc<*mut u8>` is Send; you disprove with compiler error quote. |
| P211 | **SerialHandle Sync** — When would `SerialHandle` need `unsafe impl Sync` vs `Arc<Mutex<...>>`? Two sentences each. |
| P212 | **Ch14 port** — Rewrite Level 4 spawn example using only safe types — when is it impossible? |
| P213 | **Proc-macro boundary** — Why do serde/tokio crates use `unsafe impl` you don't write? Link Ch17. |
| P214 | **CString trap** — Show `into_raw` forgotten `from_raw` leak; fix with RAII pattern sketch. |
| P215 | **Vendor SDK** — Diagram ownership: Rust owns config, C owns connection, callback pointer — boxes and arrows only. |
| P216 | **serialport hide** — Where does `unsafe` live in a typical serial crate vs my application code? |
| P217 | **CRC decision** — C `crc16` vs Rust `crc` crate vs hand-rolled — decision tree for production gateway. |
| P218 | **Trap quiz** — Mark 6 snippets: safe, UB, ordering bug, needs Miri, needs Mutex, unsound safe API. |
| P219 | **Review rubric** — 10-point code-review checklist for an `unsafe` PR in an automation repo. |
| P220 | **Test plan** — Unit + Miri + integration tests for new `extern 'C'` wrapper — bullet list only. |
| P221 | **Borrow checker fight** — I paste fight-the-borrow-checker code; you refactor to safe Rust without `unsafe`. |
| P222 | **static mut** — Compare `static mut` counter vs `AtomicUsize` from Ch15 — UB vs defined behavior. |
| P223 | **PlcDriver API** — Design safe `PlcDriver` Rust API over fictional `extern 'C'` — types, `Result`, no raw pointers in public API. |
| P224 | **Level ladder recap** — Explain Levels 1–5 in one paragraph each for a Java teammate who knows JNI. |

## Chapter 19 — I/O and processes

| ID | Prompt |
|----|--------|
| P094 | **Trait refactor** — Refactor file copy loop to generic `copy<R: Read, W: Write>`; discuss error propagation. |
| P095 | **CSV tool** — Spec for CLI: read two-column CSV, emit `name=value`; I implement with BufRead. |
| P096 | **Packet layout** — Add CRC byte to 4-byte packet; update encode/decode with XOR — show tests. |
| P097 | **Command safety** — Review shell=True style command; rewrite without shell when possible. |
| P098 | **Endian trap** — Quiz: 3 scenarios pick LE vs BE for Modbus-style register. |
| P099 | **Pipeline** — Design `program A \| program B` using only Rust std (two processes, pipe). |
| P100 | **Capstone scaffold** — Generate module tree and function signatures for sensor_gateway; no bodies. |
| P101 | **Serial debug** — I get timeout on read; give systematic checklist (baud, cable, permissions). |
| P102 | **Retry policy** — Design exponential backoff for Modbus-style poll errors; Rust pseudocode. |
| P103 | **Log schema** — Propose JSON log lines for sensor events with timestamp and error codes. |
| P104 | **GPIO next step** — After serial works on Pi, outline migration to gpio-cdev for one LED. |
| P105 | **Code review** — I paste capstone main loop; review for panic risks and missing flush. |
| P225 | **read vs read_exact** — Give 3 protocol shapes; I pick `read` loop vs `read_exact` each time; you verify. |
| P226 | **Cursor test** — Write unit test for `parse_kv_lines` using `Cursor` — no filesystem. |
| P227 | **BufReader why** — Explain in 60 words why `BufReader` matters for 10k-line log files. |
| P228 | **Boundary errors** — Sketch `main` mapping `io::Error` to exit code 1 with context path — no `unwrap`. |
| P229 | **read_to_string trap** — When is `read_to_string` wrong for automation configs? Give size threshold rule. |
| P230 | **Bit field port** — Java status int with flags — port to Rust `encode` with `\|=` and `&` masks. |
| P231 | **CRC upgrade** — Replace XOR toy CRC with CRC-16-Modbus — outline steps, no full crate required. |
| P232 | **Exit status** — Child exited 2 — how should gateway log and retry? Table: fatal vs transient. |
| P233 | **Env and cwd** — Show `Command` with `.env("PORT","502")` and `.current_dir` — when needed? |
| P234 | **Sync vs async pick** — Three gateway designs: I pick sync thread vs Tokio per scenario; you justify. |
| P235 | **serialport traits** — Explain how `serialport` maps to `Read`/`Write` — diagram only. |
| P236 | **Blocking in async** — Show wrong `std::fs::read` inside `async fn`; fix with `tokio::fs` or `spawn_blocking`. |
| P237 | **Gateway capstone** — End-to-end: config file → serial poll → JSON log line — module list and error types only. |
| P238 | **Ch16 bridge** — Same echo server: sketch sync thread version vs Tokio version — tradeoffs table. |

| P397 | **Path join** — Config path from `HOME` + `.config/app.toml` — `Path::join` vs string concat. |
| P398 | **Env default** — `TIMEOUT` env var default 30 — `unwrap_or_else` pattern. |
| P399 | **Metadata guard** — Reject config over 1MB before `read_to_string`. |
| P400 | **Stdin fallback** — Port from argv or interactive prompt — one `main` both paths. |
| P401 | **Line parser** — BufRead lines, skip `#`, parse `key=value` — trailing newline. |
| P402 | **Capstone CLI** — env path → metadata check → line parse → print port — function list only. |

---

**Total: 402 prompts** (P001–P402).

## By theme

| Theme | IDs |
|-------|-----|
| Ownership / borrow | P006–P011, P106–P114, P141–P148, P022–P027, P028–P033, P389–P396 |
| Stack / heap | P106–P114 |
| Functions / methods | P239–P250, P381–P388 |
| Iterators | P115–P140, P348–P353 |
| Enums / match | P034–P039, P149–P162, P359–P366 |
| Types / traits | P040–P045, P273–P282, P317–P329, P354–P358 |
| Modules / crates | P251–P262, P373–P380 |
| Smart pointers | P058–P059, P062–P063, P283–P286, P289–P303 |
| Collections | P052–P057, P287–P288, P304–P316 |
| Closures | P263–P272, P330–P347 |
| Errors / tests | P046–P051 |
| Concurrency | P064–P081, P367–P372 |
| Systems / I/O | P094–P105, P225–P238, P397–P402 |
| Meta / tooling | P001–P021, P082–P087, P088–P093, P163–P224 |

## See also

- [PLAYGROUND_GUIDE.md](PLAYGROUND_GUIDE.md)
- [JAVA_PYTHON_RUST_MAP.md](JAVA_PYTHON_RUST_MAP.md)
- [CONTENTS.md](../CONTENTS.md)