# AI Prompt Index

All **Afterparty** prompts from the book, numbered for reuse. Paste into any AI assistant after reading the linked chapter.

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
| P012 | **Stack vs heap quiz** — For 8 Rust variable declarations, I say stack-only, heap involved, or both; you correct and draw the pointer picture for `String` and `Vec`. |
| P013 | **Scope and drop** — Given a nested-block snippet with `String`, trace exactly when heap memory is freed vs when stack slots disappear. |
| P014 | **Stack frame drill** — Explain stack frames and pointer-bump allocation: I give you a 3-function call chain with locals; you trace frame push/pop and when each stack slot dies. |
| P015 | **Heap owner trace** — I paste a snippet with `String` and `Vec`; step through who owns the heap buffer after each line, when `drop` runs, and whether any heap copy happened. |
| P016 | **Latency compare** — Compare stack pop vs heap allocate+free for a 1 kHz loop that creates a small `String` every tick vs reusing a stack `i32` counter — worst-case latency in plain language. |
| P017 | **Three-language map** — Same program: one `int`, one growable string, one function return. For Java, Python, and Rust, tabulate: stack vs heap for each, and who frees heap memory. |
| P018 | **Move at two levels** — When `let s2 = s1` for `String`, explain what moves on the stack vs what stays on the heap; why the heap buffer is not copied unless I call `.clone()`. |
| P019 | **Borrow checker tutor** — I paste compiler errors; you explain the borrow conflict and show the smallest fix. |
| P020 | **Five snippets** — Move vs borrow quiz: 5 code fragments, I label ok/error and why. |
| P021 | **Python port** — This Python function mutates a list passed in; rewrite with `&mut Vec` and explain aliasing rules. |
| P022 | **Java port** — This Java method stores the passed List in a field; show the Rust ownership split (return owned vs `Arc`). |
| P023 | **Slice drill** — Given `&[i32]`, write `first` and `rest` without panicking on empty — use `Option`. |
| P024 | **clone audit** — Review my 30-line Rust snippet; mark unnecessary `.clone()` calls. |

## Chapter 2 — Types and expressions

| ID | Prompt |
|----|--------|
| P025 | **Integer pick** — Quiz me: for 8 scenarios (Modbus port, byte buffer, collection index, money cents, timestamp millis, hash output, loop counter, enum discriminant), I pick u8/u16/u32/u64/i32/usize; you correct and explain overflow risk. |
| P026 | **char vs byte** — Explain why 'A' is not the same type as 65u8. Show one valid char literal and one invalid escape; mention UTF-8 only when discussing str/String. |
| P027 | **Tuple vs array** — When do I use (u16, u16) vs [u16; 2] for a coordinate pair? Give one idiomatic example of each. |
| P028 | **&str vs String API** — Five function signatures for logging labels. I choose &str or String for each; you explain ownership and allocation cost. |
| P029 | **Range quiz** — For half-open vs inclusive ranges, I predict output of four for loops using .. and ..=; you correct with printed values. |
| P030 | **match warm-up** — Extend a match on HTTP status codes with 3xx, 4xx, 5xx groupings using range patterns; I write arms; you check exhaustiveness. |

## Chapter 3 — Functions and methods

| ID | Prompt |
|----|--------|
| P031 | **Parameter audit** — Five function signatures for logging: `&str`, `String`, `&String`, `Cow<str>`, `impl AsRef<str>`. I pick one per use case; you explain ownership cost. |
| P032 | **Move vs borrow** — Snippet calls `process(s)` then uses `s` again. I explain the error and show two fixes (`&s`, clone). |
| P033 | **impl block** — Struct `Timer` with `start`, `elapsed`, `reset`. I write `impl`; you check `&self` vs `&mut self`. |
| P034 | **Associated fn** — When is `Type::new()` idiomatic vs `Default::default()`? One example each. |
| P035 | **Consume self** — Method `fn into_inner(self) -> Vec<u8>` — why must it take `self` by value? |
| P036 | **Semicolon trap** — Three tiny functions: one returns `i32` correctly, two fail due to `;`. I fix them. |
| P037 | **Early return** — Rewrite nested `if` in a parser as early `return None` / `?` style. |
| P038 | **Unit vs value** — Which functions should return `()` vs `bool` vs `Option<T>`? Three CLI helper names, I choose. |
| P039 | **Generic bounds** — Fix `fn max(a: T, b: T)` without bounds; add `T: Ord` or `PartialOrd`. |
| P040 | **Result signature** — Design `fn read_config(path: &str) -> Result<Config, ...>`; list error variants, no body. |

| P041 | **impl Iterator return** — `top_n` returns `impl Iterator` — why two iterator types in `if` arms fail. |
| P042 | **mem take drain** — Buffer drains via `mem::take` — show before/after inner field. |
| P043 | **where clause** — Rewrite cluttered generic bounds with `where` block. |
| P044 | **Self return** — Method `into_inner(self) -> Vec<u8>` — why `Self` not concrete type? |
| P045 | **const fn** — `const fn is_valid_port(p: u16) -> bool` — compile-time limits. |
| P046 | **pick impl Trait** — Factory returning different iterator types — fix with `Box<dyn Iterator>`. |
| P047 | **Drain method** — Design `Buffer::drain_into(&mut Vec<u8>)` signature and ownership. |
## Chapter 4 — Iterators

| ID | Prompt |
|----|--------|
| P048 | **iter vs into_iter** — Give 4 snippets using `Vec`; I predict whether `v` is usable after the loop; you explain move vs borrow. |
| P049 | **collect turbofish** — Three `collect()` calls that fail without hints — I add type annotation or turbofish; you verify. |
| P050 | **Adapter chain** — Task: parse lines, trim, keep non-empty, parse as `u16`. I sketch `.lines().map(...).filter(...).collect()`; you refine. |
| P051 | **enumerate vs index** — Same sum-over-evens task twice: index `for` vs `.enumerate()`. Compare readability and bounds-check risk. |
| P052 | **Lazy vs eager** — Explain when `filter().map()` allocates vs when `.collect()` forces work. One example with println in `map` showing evaluation order. |
| P053 | **for desugar quiz** — Four `for` loops (`0..n`, `&vec`, `vec`, `&mut vec`). I say iter vs into_iter vs range; you correct. |
| P054 | **iter_mut drill** — Double every element in `Vec<f64>` in place; I write the loop; you review `*n` and borrows. |
| P055 | **Range vs collect** — When is `for i in 0..n` better than collecting `(0..n)`? Two automation examples. |
| P056 | **zip pairs** — Zip two `Vec<u16>` into pairs; I write it; you handle length mismatch. |
| P057 | **take and skip** — Paginate a log with `.skip().take()` on `.lines()`; show one pitfall. |
| P058 | **chain iterators** — Concatenate two `&[u8]` slices with `.chain()` without copying into one array first. |
| P059 | **find and Option** — Find first log line containing `ERROR` with `.find()`; contrast `.filter().next()`. |
| P060 | **fold vs sum** — Max and count in one `.fold()` pass vs separate calls — when is fold worth it? |
| P061 | **any and all** — Validate ports and comment lines with `.all()` / `.any()`; fix a double-reference mistake. |
| P062 | **Moved Vec mistake** — Code uses `for x in v` then `v` again; I explain and fix; you rank fixes. |
| P063 | **Borrow in chain** — `Vec<&str>` from `String` lines then drop strings; I explain failure and fix. |
| P064 | **Modbus-style scan** — Filter even `u16` registers, scale to `f64`, sum — write and check types/overflow. |
| P065 | **Zero-cost check** — Does `iter().filter().map().sum()` allocate intermediate `Vec`s in `--release`? |
| P066 | **Trap sheet drill** — Six snippets mixing `for x in v`, `&v`, `into_iter`, and `collect` errors; I predict ok/fail; you fix. |
| P067 | **&&i32 decoder** — Label closure param types in one `.iter().filter().map()` chain; show three compiling closure styles. |
| P068 | **Empty iterator policy** — Sum, find, all on empty `Vec`; I state results and domain bugs; you correct. |
| P069 | **zip truncation** — Zip unequal `Vec` lengths; explain silent loss and sketch a safe pairing strategy. |
| P070 | **PortScan impl** — Implement `Iterator` for ports 502..=505; collect and sum — show `type Item` and `fn next`. |
| P071 | **Skip blanks** — `NonEmptyLines` iterator trims and skips empty lines; collect keys before `=`. |
| P072 | **Infinite take** — Counter from 0 — why `.take(n)` before `.collect()`? Show hang vs bounded. |
| P073 | **IntoIterator pair** — Same struct implements `Iterator` and `IntoIterator` for `for` loop. |
| P074 | **Stateful parser** — Byte buffer iterator yielding 4-byte frames — sketch `next()` state machine. |
| P075 | **Capstone iterator** — CSV fields parse col 2 as `u16`, filter > 0 — custom struct + consumer chain. |

## Chapter 5 — Lifetimes

| ID | Prompt |
|----|--------|
| P076 | **Error archaeology** — I paste a "lifetime may not live long enough" error; walk me through owner vs reference diagram. |
| P077 | **Return type choice** — For API `fn title(book: &Book) -> ???` compare `&str` vs `String` trade-offs for a library. |
| P078 | **Struct lifetime** — Design `ConfigParser` holding `&str` slices into input buffer — when is it sound vs use owned `String`? |
| P079 | **Elision quiz** — Add explicit lifetimes to 4 function signatures where elision fails. |
| P080 | **Fix mine** — I return `&String` built inside function; show three idiomatic fixes ranked by simplicity. |

| P081 | **static trap** — Return `&str` from `format!` — error and owned fix. |
| P082 | **two lifetimes** — `first<'a,'b>(x: &'a str, y: &'b str) -> &'a str` — drop `y` while result lives. |
| P083 | **Config struct** — Parse `host:port` into `Config<'a>` — when must caller keep line alive? |
| P084 | **Owned refactor** — Owned `Config { host: String }` vs borrowed — three tradeoffs. |
| P085 | **T: 'a bound** — Explain `Holder<'a, T: 'a>` — failure when `T` shorter-lived. |
| P086 | **Elision fail** — Four signatures: elision works vs fails — I label each. |
| P087 | **Iterator borrow** — `Vec<&str>` from `String` lines — drop order trap. |
## Chapter 6 — Enums and pattern matching

| ID | Prompt |
|----|--------|
| P088 | **Null replacement** — Translate 5 Java methods returning null into `Option` Rust; explain callsite changes. |
| P089 | **Exhaustive match** — I have enum with 4 variants; generate match that compiles; then add variant and show compiler error. |
| P090 | **Result railway** — Chain parse → validate → compute with `?`; I fill blanks, you verify. |
| P091 | **if let vs match** — When is `if let` clearer than `match`? Give 3 contrasting snippets. |
| P092 | **State machine** — Model TCP connection states as enum; methods connect, send, close with illegal transition errors. |
| P093 | **Python Union** — This Python function accepts int \| str; design Rust enum + match without dynamic typing. |
| P094 | **unwrap audit** — Paste 20 lines with 4 `unwrap()` calls; I mark panic risk; you rewrite with `match` or `?`. |
| P095 | **Combinator chain** — Parse port `Option<u16>`, double if Some, default 502 — I write `map`/`unwrap_or`; you add `and_then`. |
| P096 | **let-else port** — Rewrite nested `match` on `parse()` into `let Ok(x) = ... else { ... }`; preserve behavior. |
| P097 | **Err arm missing** — Show `match` on `Result` without `Err` arm; I quote error and fix; add boundary `eprintln!` pattern. |
| P098 | **unwrap vs ?** — Same parser: `unwrap` vs `fn -> Result` with `?`; compare panic risk and signature honesty. |
| P099 | **Wildcard footgun** — Explain why `_` on your own enum hides new variants; show explicit arms vs `_` refactor story. |
| P100 | **Partial move** — `enum` with `String` field: `match` moves field; I fix with `ref` or `match &e`; show error text. |
| P101 | **Match on ref** — Owned `Status`: `match s` vs `match &s`; predict move errors; diagram ownership. |
| P102 | **Guard drill** — Classify `i32` with guards; I write arms; you check exhaustiveness. |
| P103 | **Opcode table** — Design `enum` for 3 frame types + `match` opcodes; add fourth type as compile-break exercise. |
| P104 | **ReadOutcome extend** — Add `Disconnected` to `ReadOutcome`; list `match` sites the compiler forces to update. |
| P105 | **Config sentinel** — Rewrite `port() -> i32` returning `-1` to `Option<u16>` + `match` in `main`. |
| P106 | **Checklist drill** — Match 6 Chapter 6 compiler errors to snippets; I name fix (`match` arm, `ref`, `?`). |
| P107 | **Java enum map** — Java `enum State` with method → Rust `enum` + `match` + `impl`; contrast nullability. |

| P108 | **Slice split** — Parse HTTP request line with slice patterns — handle single-token input. |
| P109 | **matches refactor** — Replace 6-arm bool `match` with `matches!` on `Mode` enum. |
| P110 | **if let chain** — Parse `host:port` — chain rejects `host:port:extra`. |
| P111 | **@ binding quiz** — Port range arms with `@` — label which values hit which arm. |
| P112 | **Exhaustive slice** — `[a, b]` on len 1 or 3 — predict `_` arm vs bug; suggest `..` rest. |
| P113 | **matches vs match** — When keep full `match` instead of `matches!`? Example returning `String`. |
| P114 | **Guard + @** — Match port `n @ 1024..=65535` with guard `n % 2 == 0`. |
| P115 | **Capstone parse** — Frame header slice pattern from byte slice — sketch `match` arms only. |

## Chapter 7 — Structs, traits, generics

| ID | Prompt |
|----|--------|
| P116 | **dyn vs impl** — Quiz: 4 scenarios — pick `dyn Trait` or `impl Trait` and justify. |
| P117 | **Default trait methods** — Add default `summary()` on trait; override in one type only. |
| P118 | **Generic bounds** — Fix compiler error: `T` needs `Display + Clone`; minimal bound set. |

| P119 | **Item type quiz** — Change `PortScan`'s `type Item` from `u16` to `(u16, bool)` — list broken call sites. |
| P120 | **Summarizable design** — `Summarizable` with `type Output = String` for three sensor structs; one `report` fn. |
| P121 | **Associated vs generic** — `trait Get<T>` vs `trait Get { type Value; }` — when is each painful? |
| P122 | **Supertrait bounds** — `Exportable: Display + Debug` with default `export` — impl for `Port(u16)`. |
| P123 | **UFCS fix** — Type implements `A` and `B`, both define `name()` — ambiguous call and UFCS fix. |

## Chapter 8 — Errors and testing

| ID | Prompt |
|----|--------|
| P124 | **? chain** — Refactor nested match on Results to `?` style; explain each change. |
| P125 | **Error enum design** — Design `AppError` for CLI that reads config + talks serial; variants + `From` impls sketch. |
| P126 | **panic audit** — Mark which of 10 `unwrap()` calls should stay vs become `Result`. |
| P127 | **Test generation** — Write table-driven tests for `parse_port` including edge ports. |
| P128 | **anyhow vs thiserror** — When would I pick each for a binary vs library crate? |

## Chapter 9 — Modules, paths, and crates

| ID | Prompt |
|----|--------|
| P129 | **File tree** — Design module tree for a CLI that reads config and runs commands. Directories + `mod` lines only, no bodies. |
| P130 | **Path quiz** — From `crate::service::worker::run`, how do I reach `crate::config::load`? Show `use` and fully qualified call. |
| P131 | **lib vs bin** — What belongs in `main.rs` vs `lib.rs` for a tool with 500 lines of logic? |
| P132 | **pub audit** — List items that should be `pub` vs private in a library crate exposing `Client::connect`. |
| P133 | **pub(crate)** — When is `pub(crate)` better than `pub` for test helpers? |
| P134 | **Re-export** — Sketch `pub use` so users see `my_crate::Error` but you wrap `thiserror` internally. |
| P135 | **Workspace split** — Two crates: `core` library + `cli` binary. Write `Cargo.toml` dependency path only. |
| P136 | **Integration test** — Where does `tests/smoke.rs` live and how does it `use` the library? |
| P137 | **Orphan fix** — I want `Display` on `Vec<u8>` — show newtype wrapper module layout. |
| P138 | **Split monolith** — Given one `main.rs` with config + parser + runner, name three modules and what each owns. |
| P139 | **cfg test** — Explain why `mod tests` uses `#[cfg(test)]` and `use super::*`. |
| P140 | **Capstone** — Generate `src/` tree for `sensor_core` library + `sensor_cli` binary in one workspace; I implement. |

| P141 | **Feature flag** — Add `serial` feature gating `mod serial_io` — `Cargo.toml` and one `#[cfg]`. |
| P142 | **cfg vs cfg!** — `#[cfg(debug_assertions)]` vs `if cfg!(debug_assertions)` — release binary diff. |
| P143 | **Optional dep** — Optional `tokio` behind feature `async` — show `dep:tokio` line. |
| P144 | **Platform gate** — `#[cfg(target_os = "linux")]` module for device path — sketch. |
| P145 | **Integration layout** — Tree for `tests/load_config.rs` — what API is invisible to test? |
| P146 | **pub use prelude** — Re-export so users call `my_crate::connect` not `internal::connect`. |
| P147 | **doc hidden** — When mark helper `#[doc(hidden)]` on public re-export surface? |
| P148 | **Capstone crate** — `gateway` crate: `serial` feature, integration test, `///` on parse fn — tree only. |
## Chapter 10 — Smart pointers and interior mutability

| ID | Prompt |
|----|--------|
| P149 | **Box why** — When is `Box<[T]>` better than `Vec<T>` on the stack? Two cases. |
| P150 | **Rc cycle** — Explain why `Rc` cycles leak; contrast with Python reference cycles and `Weak` fix. |
| P151 | **Arc Mutex sketch** — Diagram thread-safe cache with Arc<Mutex<HashMap>> — no full code. |
| P152 | **Java heap map** — Map Java "everything is reference" to Rust ownership — when `Arc`, when plain `&`, when neither. |
| P153 | **RefCell trap** — Show double `borrow_mut` panic; fix with scoped borrows. |
| P154 | **Arc vs Rc** — Thread spawn with `Rc` — show error; fix with `Arc`; explain atomic count overhead. |
| P155 | **Deref coercion** — Why does `fn takes_str(s: &str)` accept `&String`, `&Box<String>`, and `&Rc<String>`? Trace steps. |
| P156 | **Pick pointer** — Five scenarios (AST node, thread cache, GUI graph, config string, plugin list) — I pick Box/Rc/Arc/RefCell/Weak; you grade. |
| P157 | **Recursive list** — Draw memory for `Cons(1, Box::new(Cons(2, Nil)))` — stack vs heap boxes. |
| P158 | **Move out of Box** — What happens after `let s = *box_string`? When idiomatic vs keeping the `Box`? |
| P159 | **Trait object box** — Three plugin types implement `Plugin` — sketch `Vec<Box<dyn Plugin>>`; why not `Vec<Plugin>`? |
| P160 | **Handle vs deep clone** — Audit snippet with `Rc<String>` and both `Rc::clone` and `(*rc).clone()` — label cost of each. |
| P161 | **Weak upgrade** — Parent/child with `Rc` parent and `Weak` child back-ref — I sketch types; you explain cycle break. |
| P162 | **strong_count debug** — `strong_count` stays at 2 after I thought I dropped all refs — list 5 places handles hide. |
| P163 | **Immutable then mut** — Hold `let r = cell.borrow()` and call `borrow_mut` — explain panic; fix with nested block. |
| P164 | **Cell vs RefCell** — Counter `u32` vs `Vec` cache — I pick `Cell` or `RefCell` each; you correct. |
| P165 | **Rc RefCell graph** — Two nodes share `Rc<RefCell<Node>>` — one updates, one reads — sketch borrow rules on one thread. |
| P166 | **Compile vs runtime** — Same overlapping-mut pattern: compile error with `&mut` and runtime panic with `RefCell` side by side. |
| P167 | **Drop order** — Three `Drop` structs in one function — I predict print order; you confirm reverse declaration rule. |
| P168 | **Rc last handle** — Two `Rc` clones dropped at different times — when does inner `Drop` run? Step through with println in `Drop`. |
| P169 | **Drop panic** — Explain double-panic abort if `Drop` panics during unwind — link to Ch8. |
| P170 | **Refactor to smart ptr** — I paste struct with `Box`, `Rc`, or raw tree — you suggest minimal smart pointer fix and justify. |
| P171 | **Leak hunt** — Sketch `Rc` cycle in observer pattern; refactor one edge to `Weak` and explain count after each drop. |

## Chapter 11 — Collections

| ID | Prompt |
|----|--------|
| P172 | **Loop port** — Rewrite C-style indexed loop as iterator chain; preserve behavior. |
| P173 | **HashMap merge** — Two maps of scores — merge by max per key; iterator + entry style. |
| P174 | **entry drill** — Word frequency from `Vec<&str>` using only `.entry` — no double lookup. |
| P175 | **collect types** — Three `collect()` calls that need type hints — fix with turbofish. |
| P176 | **Windows** — Detect rising edges in `Vec<f64>` with `.windows(2)`; extend to `.windows(3)`. |
| P177 | **Performance myth** — Do Rust iterators optimize to loops? When might they not? |
| P178 | **Pick collection** — Five tasks (dedup, range scan, FIFO, index by id, min-key) — I pick collection each. |
| P179 | **Set ops** — Tags on two records — union, intersection, difference with `HashSet`. |
| P180 | **Hash vs BTree** — Same 10k insert + range scan — when HashMap wins vs BTreeMap. |
| P181 | **Queue anti-pattern** — Review `v.remove(0)` queue loop — cost and `VecDeque` fix. |
| P182 | **get vs index** — Four access patterns — I pick `[i]` vs `.get(i)` vs `.get_mut` vs `if let Some`. |
| P183 | **sort dedup** — Dedup `[3,1,4,1,5]` wrong vs right — sort + dedup pipeline. |
| P184 | **retain vs filter** — Remove evens in-place vs new `Vec` — compare `retain` and `filter().collect()`. |
| P185 | **Borrow push trap** — Explain `let r = &v[0]; v.push(1)` error; fix with scope. |
| P186 | **or_insert_with** — Lazy cache: expensive `Vec` built once per key — sketch with `or_insert_with`. |
| P187 | **insert overwrite** — Track old value on port remap using `insert` return. |
| P188 | **Stable dedup** — Unique `String` lines preserving first-seen order — no HashSet-only collect. |
| P189 | **chunks vs windows** — Parse byte stream into 4-byte frames — `chunks(4)` vs `windows(4)` when? |
| P190 | **Duplicate keys collect** — `collect` to HashMap from duplicate-key pairs — predict final map. |
| P191 | **Capacity hint** — Read 1M lines into `Vec` — when `with_capacity` matters; sizing rule. |
| P192 | **Capstone store** — Register id → reading + range scan — pick map type, list three API methods. |

## Chapter 12 — Closures and the Fn traits

| ID | Prompt |
|----|--------|
| P193 | **Fn quiz** — Four closures: I label each Fn / FnMut / FnOnce; you correct and explain capture. |
| P194 | **move drill** — Thread spawn snippet missing `move` — show compile error and fix. |
| P195 | **Iterator chain** — `.filter` closure that uses `&config` — why `Fn` not `FnMut`? |
| P196 | **fn vs closure** — When can you pass `fn()` vs `impl Fn()` to the same helper? |
| P197 | **Return closure** — Write `make_multiplier(f: f64) -> impl Fn(f64) -> f64` and explain `move`. |
| P198 | **Double reference** — Fix `.iter().filter(|x| ...)` type error on `Vec<String>`. |
| P199 | **sort_by** — Sort `Vec<(String, u32)>` by count descending with `sort_by` closure. |
| P200 | **for_each vs for** — Same side-effect loop twice: `for` vs `.for_each` — style tradeoffs. |
| P201 | **Box dyn Fn** — Store heterogeneous callbacks in a Vec — sketch trait object version. |
| P202 | **Capstone** — Pipeline: lines, filter, parse `u16`, sum — all with closures; I write; you review Fn bounds. |
| P203 | **RefCell bump** — Closure mutates `RefCell<u32>` counter — which Fn trait and why? |
| P204 | **Callback registry** — Three log filters in `Vec<Box<dyn Fn(&str) -> bool>>` — I add wrong signature; you fix. |
| P205 | **Loop move trap** — Building `Vec<Box<dyn Fn()>>` in `for` over `String` — show move error and clone fix. |
| P206 | **sort_by_key** — Same sort with `sort_by_key` — when is key extraction cleaner? |
| P207 | **retain valid** — Drop invalid `SensorReading` rows with `.retain` — FnMut bound. |
| P208 | **Fn bound strict** — Helper takes `impl Fn()` but closure mutates — fix signature. |
| P209 | **Thread move** — `spawn` closure borrows `String` — show error without `move` and fix. |
| P210 | **Send Box Fn** — When does `Box<dyn Fn() + Send>` matter for thread callbacks? |
| P211 | **Callback capstone** — Registry + sort pipeline + thread handoff — review Fn bounds on my code. |
| P212 | **apply_twice generic** — `impl Fn` vs `Box<dyn Fn>` for helper called twice — cost comparison. |
| P213 | **Async move note** — One `async move` block capturing `String` — same rules as sync closure. |
| P214 | **Dedup_by closure** — Dedup adjacent `SensorReading` by id with `dedup_by` — FnMut. |
| P215 | **Filter config** — `.filter` reading `&settings` — prove closure is `Fn` not `FnMut`. |
| P216 | **FnOnce consume** — Closure that moves `String` out on first call — storage implications. |
| P217 | **Heterogeneous vec** — Why `Vec<Box<dyn Fn(i32)>>` cannot mix different capture sizes — one paragraph. |
| P218 | **Iterator sort chain** — `.filter().map().collect()` then `sort_by` — label each closure's Fn trait. |
| P219 | **Scope vs move** — Compare `thread::scope` borrow closure vs `spawn(move)` — when each. |
| P220 | **Capstone registry** — Full callback registry filtering logs by severity — I write; you audit traits. |

## Chapter 13 — Standard traits and conversions

| ID | Prompt |
|----|--------|
| P221 | **Debug vs Display** — When derive `Debug` only vs implement `Display` for a CLI status line? |
| P222 | **Redacted Debug** — Struct with `api_key: String` — sketch manual `Debug` with `[REDACTED]`. |
| P223 | **From chain** — `String` → `MyLabel` via `From`; add `From<&str>` without duplicating logic. |
| P224 | **TryFrom port** — Port validation with custom enum error `OutOfRange`. |
| P225 | **parse vs TryFrom** — When `s.parse::<u16>()` vs `u16::try_from(x)` vs custom `FromStr`? |
| P226 | **AsRef drill** — Rewrite three functions taking `&String` to `impl AsRef<str>`. |
| P227 | **Cow API** — Normalize slug: accept `Cow<str>`, return borrowed if valid else owned. |
| P228 | **Derive set quiz** — Map key, log line, sortable row, error enum — I list derives each needs. |
| P229 | **Display impl** — Implement `Display` for `Port(u16)` showing `Port(8080)`. |
| P230 | **Mini crate API** — Public `HostPort` with `Display`, `TryFrom<&str>` for `host:port` — list impl blocks only. |
| P231 | **Pretty debug** — Same struct with `{:#?}` vs `{:?}` — when does pretty-print help in tests? |
| P232 | **Default enum** — Three-variant mode enum — derive `Default` with `#[default]` on `Auto`. |
| P233 | **Eq on floats** — Struct with `f64` field — show `Eq` derive failure; three fixes. |
| P234 | **HashMap key** — Why does `UserId(String)` need `Eq + Hash` for `HashMap` keys? |
| P235 | **FromStr type** — Parse `host:port` into struct — sketch `FromStr` with split. |
| P236 | **Silent cast trap** — Show `70000i32 as u16` vs `TryFrom` — predict values. |
| P237 | **From in errors** — Wire `ParseIntError` into `AppError` with `From` — list impl only. |
| P238 | **AsRef bytes** — Log wire payload — `impl AsRef<[u8]>` accepts `Vec`, slice, array. |
| P239 | **Borrow lookup** — Explain `HashMap<String, V>.get(&str)` — role of `Borrow<str>`. |
| P240 | **into_owned** — When caller needs `String` after `Cow` helper — where `into_owned()`? |
| P241 | **Newtype Display** — Wrap `Vec<u8>` as `HexBytes` — implement `Display` without orphan violation. |
| P242 | **Derive audit** — Config with secrets + TOML — list safe vs unsafe derives. |
| P243 | **Capstone traits** — Design `RateLimit` public API: parsing, display, equality — traits only. |

## Chapter 14 — Multithreading

| ID | Prompt |
|----|--------|
| P244 | **Race quiz** — Which snippets are data races in C++ but rejected by Rust compiler? |
| P245 | **Channel design** — Worker pool with mpsc: I describe throughput; you sketch thread count + channel shape. |
| P246 | **Mutex vs RwLock** — Read-heavy sensor cache — pick primitive and why. |
| P247 | **Send fix** — I try to move `Rc` into thread; show fix with Arc. |
| P248 | **Join panic** — What happens if spawned thread panics? Handle in main. |

| P249 | **RwLock cache** — Sketch `Arc<RwLock<HashMap>>` read-heavy cache; when write starves readers? |
| P250 | **Mutex vs RwLock** — Same cache with 50% writes — pick primitive and justify. |
| P251 | **OnceLock init** — `get_or_init` behaviour when called twice — same value guarantee. |
| P252 | **Scope borrow** — Parallel sum over chunks with `thread::scope` — why plain `spawn` fails. |
| P253 | **Poison recovery** — Writer panics holding `RwLock` — poisoned `read()` and recovery. |
| P254 | **Capstone sync** — Lazy config (`OnceLock`), shared cache (`RwLock`), scoped workers — types only. |
## Chapter 15 — Atomics

| ID | Prompt |
|----|--------|
| P255 | **Ordering quiz** — For shutdown flag + published config pointer, which orderings? Justify briefly. |
| P256 | **Counter port** — Replace Mutex counter with AtomicUsize; discuss lost updates with Relaxed. |
| P257 | **ABA problem** — Explain ABA in 80 words for compare_exchange — no full queue impl. |
| P258 | **When not** — Three cases atomics are the wrong tool; prefer channels or Mutex. |
| P259 | **Fence intuition** — Draw happens-before arrow diagram for Release store + Acquire load. |

## Chapter 16 — Async and Tokio

| ID | Prompt |
|----|--------|
| P260 | **Future diagram** — Draw state machine for async fn with two await points. |
| P261 | **Tokio scaffold** — Generate minimal Tcp echo server skeleton; I fill body. |
| P262 | **select! scenario** — Cancel slow request when fast path returns — outline `select!`. |
| P263 | **async vs thread** — 1000 Modbus polls — argue async vs thread pool for latency. |
| P264 | **blocking fix** — Identify blocking calls in async snippet; suggest `spawn_blocking`. |
| P265 | **Python asyncio** — Map asyncio gather to Tokio join — API comparison table. |

## Chapter 17 — Metaprogramming

| ID | Prompt |
|----|--------|
| P266 | **Macro vs fn** — Rewrite macro as generic fn if possible; when impossible, say why. |
| P267 | **derive need** — List derives I want for config struct loaded from TOML — justify each. |
| P268 | **Hygiene** — Explain macro hygiene in 60 words with `$crate` mention. |
| P269 | **Debug expand** — Walk me through `cargo expand` on derive Debug output (conceptual). |
| P270 | **DSL sketch** — Design tiny `command!` macro for CLI subcommands — tokens only. |
| P271 | **Expansion order** — List compiler phases from tokens to LLVM; where do macros run? |
| P272 | **Tokens vs types** — Why can a macro compile but expanded code fail? One example. |
| P273 | **Follow-set why** — Explain in 80 words why `$a:expr = $b:expr` is forbidden in matchers. |
| P274 | **Scope honesty** — List 5 metaprogramming topics Ch17 skips and where to learn each. |
| P275 | **Trace checklist** — Give 6 reasons macro code is hard to trace and one mitigation each. |
| P276 | **Fragment picker** — I describe a DSL shape; you pick `expr`/`ident`/`tt` for each slot. |
| P277 | **Expr equals fix** — Fix `set_reg!($addr = $val)` matcher; show two valid surface syntaxes. |
| P278 | **Trailing comma** — Explain `$(x),*` vs `$(x),+` on empty input; show failing and fixed macro. |
| P279 | **Register DSL** — Extend `register_map!` with a third register; explain `stringify!` arm. |
| P280 | **Clone expand** — Show conceptual expanded `impl Clone` for struct with two `i32` fields. |
| P281 | **Enum vs struct** — How does derived `PartialEq` differ for enum vs struct? Sketch match arms. |
| P282 | **Field bound failure** — Struct with `Mutex<i32>` field + `#[derive(Clone)]` — quote error and fix. |
| P283 | **Redacted Debug** — When hand-write `Debug` instead of derive on command enum with secrets. |
| P284 | **Copy vs Clone quiz** — Classify 8 types: Copy, Clone only, or neither; justify. |
| P285 | **Hot-loop clone audit** — Audit poll loop with `.clone()` each tick; suggest move/`Arc`/borrow. |
| P286 | **Arc vs derive Clone** — Explain cheap `Arc` clone vs deep `String` clone with one snippet. |
| P287 | **Serde rename** — Field `poll_ms` in JSON as `pollIntervalMs` — show attr; trap on refactor. |
| P288 | **thiserror vs manual** — Same error enum: count lines derive vs hand-written (Ch8 style). |
| P289 | **Float Eq trap** — Show `#[derive(Eq)]` on `f64` field failure; two fixes from Ch7. |
| P290 | **cargo expand walkthrough** — Step-by-step: install, run, read output for one derive. |
| P291 | **In expansion of** — Decode a 3-note compiler error chain from nested macro + derive. |
| P292 | **tt vs expr escape** — When switch matcher from `expr` to `tt`; tradeoffs in 80 words. |
| P293 | **Three-layer trace** — Derive inside attribute inside macro_rules — debug layer by layer. |
| P294 | **Port to const fn** — Replace tiny numeric macro with `const fn`; when macro still needed? |
| P295 | **env vs var** — Compare `env!`, `option_env!`, `std::env::var` — table with one use case each. |
| P296 | **include_str config** — Embed default TOML with `include_str!`; deserialize at startup sketch. |
| P297 | **Macro vs fn audit** — Mark 6 snippets: should be macro, derive, or plain fn — justify. |
| P298 | **When not proc macro** — Three scenarios where proc macro is overkill; alternative each. |
| P299 | **Minimal derive set** — Gateway config + error + CLI: smallest derive list that still ships. |
| P300 | **Trap quiz** — Mark 8 snippets: empty `+`, double brace, Default enum, Arc Clone, env!, duplicate impl, cfg macro, serde rename. |
| P301 | **Duplicate register DSL** — Design compile-time error for duplicate keys in register_map! |
| P302 | **Serde refactor test** — Integration test plan after renaming TOML field with serde attrs. |
| P303 | **Modbus table macro** — Spec register table macro generating lookup + const max address. |
| P304 | **Derive soup review** — I paste 40-line struct with 12 derives; trim to minimal set with reasons. |
| P305 | **For after expr** — Why is `$e:expr for $i:ident in $r:expr` illegal? Show legal `$p:pat in $r:expr` foreach macro. |
| P306 | **Double-brace fix** — Fix `poll_twice!` macro that mixes `let` and `for` for use in `let x = poll_twice!()`. |

## Chapter 18 — Unsafe

| ID | Prompt |
|----|--------|
| P307 | **Invariant list** — For raw pointer to buffer + length, list 5 invariants safe wrapper must enforce. |
| P308 | **Soundness** — Explain "safe Rust can't cause UB" vs unsafe — one paragraph; include unsound safe wrapper example. |
| P309 | **FFI checklist** — Checklist for calling C library from Rust binary. |
| P310 | **Miri** — What is Miri and when run it relative to unsafe changes? |
| P311 | **Avoid** — Review use case: speed up JSON — unsafe vs simd crate vs algorithm. |
| P312 | **Java JNI** — Compare JNI pitfalls to Rust FFI ownership rules. |
| P313 | **Scope honesty** — List 6 topics Ch18 skips and where to learn each (nomicon, Miri, Pin, …). |
| P314 | **Aim table** — Fill: why Vec needs `unsafe` internally while `push` stays safe for callers. |
| P315 | **Promise diagram** — Draw safe API → unsafe block → invariants → caller cannot UB; label soundness. |
| P316 | **`*const` vs `&T` quiz** — Give 5 snippets: legal ref, needs `unsafe` block, compile error; I classify each. |
| P317 | **from_raw_parts design** — Design `fn view_frame(ptr, len) -> Result<&[u8], Error>` without `&'static`; list invariants. |
| P318 | **Dangling audit** — Show stack pointer used after drop; I explain UB; you show Miri-style symptom. |
| P319 | **Modbus buffer** — Register table as `&[u8]` vs `from_raw_parts` — when is each idiomatic in a gateway? |
| P320 | **Hex preview port** — Port Level 2 `as_hex_preview` to return `Result` on empty buffer; no `unwrap`. |
| P321 | **set_len contract** — Document pre/post conditions for `set_len_unchecked`; what breaks `as_slice` if violated? |
| P322 | **Send proof** — I claim `Rc<*mut u8>` is Send; you disprove with compiler error quote. |
| P323 | **SerialHandle Sync** — When would `SerialHandle` need `unsafe impl Sync` vs `Arc<Mutex<...>>`? Two sentences each. |
| P324 | **Ch14 port** — Rewrite Level 4 spawn example using only safe types — when is it impossible? |
| P325 | **Proc-macro boundary** — Why do serde/tokio crates use `unsafe impl` you don't write? Link Ch17. |
| P326 | **CString trap** — Show `into_raw` forgotten `from_raw` leak; fix with RAII pattern sketch. |
| P327 | **Vendor SDK** — Diagram ownership: Rust owns config, C owns connection, callback pointer — boxes and arrows only. |
| P328 | **serialport hide** — Where does `unsafe` live in a typical serial crate vs my application code? |
| P329 | **CRC decision** — C `crc16` vs Rust `crc` crate vs hand-rolled — decision tree for production gateway. |
| P330 | **Trap quiz** — Mark 6 snippets: safe, UB, ordering bug, needs Miri, needs Mutex, unsound safe API. |
| P331 | **Review rubric** — 10-point code-review checklist for an `unsafe` PR in an automation repo. |
| P332 | **Test plan** — Unit + Miri + integration tests for new `extern 'C'` wrapper — bullet list only. |
| P333 | **Borrow checker fight** — I paste fight-the-borrow-checker code; you refactor to safe Rust without `unsafe`. |
| P334 | **static mut** — Compare `static mut` counter vs `AtomicUsize` from Ch15 — UB vs defined behavior. |
| P335 | **PlcDriver API** — Design safe `PlcDriver` Rust API over fictional `extern 'C'` — types, `Result`, no raw pointers in public API. |
| P336 | **Level ladder recap** — Explain Levels 1–5 in one paragraph each for a Java teammate who knows JNI. |

## Chapter 19 — I/O and processes

| ID | Prompt |
|----|--------|
| P337 | **Trait refactor** — Refactor file copy loop to generic `copy<R: Read, W: Write>`; discuss error propagation. |
| P338 | **CSV tool** — Spec for CLI: read two-column CSV, emit `name=value`; I implement with BufRead. |
| P339 | **Packet layout** — Add CRC byte to 4-byte packet; update encode/decode with XOR — show tests. |
| P340 | **Command safety** — Review shell=True style command; rewrite without shell when possible. |
| P341 | **Endian trap** — Quiz: 3 scenarios pick LE vs BE for Modbus-style register. |
| P342 | **Pipeline** — Design `program A \| program B` using only Rust std (two processes, pipe). |
| P343 | **Capstone scaffold** — Generate module tree and function signatures for sensor_gateway; no bodies. |
| P344 | **Serial debug** — I get timeout on read; give systematic checklist (baud, cable, permissions). |
| P345 | **Retry policy** — Design exponential backoff for Modbus-style poll errors; Rust pseudocode. |
| P346 | **Log schema** — Propose JSON log lines for sensor events with timestamp and error codes. |
| P347 | **GPIO next step** — After serial works on Pi, outline migration to gpio-cdev for one LED. |
| P348 | **Code review** — I paste capstone main loop; review for panic risks and missing flush. |
| P349 | **read vs read_exact** — Give 3 protocol shapes; I pick `read` loop vs `read_exact` each time; you verify. |
| P350 | **Cursor test** — Write unit test for `parse_kv_lines` using `Cursor` — no filesystem. |
| P351 | **BufReader why** — Explain in 60 words why `BufReader` matters for 10k-line log files. |
| P352 | **Boundary errors** — Sketch `main` mapping `io::Error` to exit code 1 with context path — no `unwrap`. |
| P353 | **read_to_string trap** — When is `read_to_string` wrong for automation configs? Give size threshold rule. |
| P354 | **Bit field port** — Java status int with flags — port to Rust `encode` with `\|=` and `&` masks. |
| P355 | **CRC upgrade** — Replace XOR toy CRC with CRC-16-Modbus — outline steps, no full crate required. |
| P356 | **Exit status** — Child exited 2 — how should gateway log and retry? Table: fatal vs transient. |
| P357 | **Env and cwd** — Show `Command` with `.env("PORT","502")` and `.current_dir` — when needed? |
| P358 | **Sync vs async pick** — Three gateway designs: I pick sync thread vs Tokio per scenario; you justify. |
| P359 | **serialport traits** — Explain how `serialport` maps to `Read`/`Write` — diagram only. |
| P360 | **Blocking in async** — Show wrong `std::fs::read` inside `async fn`; fix with `tokio::fs` or `spawn_blocking`. |
| P361 | **Gateway capstone** — End-to-end: config file → serial poll → JSON log line — module list and error types only. |
| P362 | **Ch16 bridge** — Same echo server: sketch sync thread version vs Tokio version — tradeoffs table. |

| P363 | **Path join** — Config path from `HOME` + `.config/app.toml` — `Path::join` vs string concat. |
| P364 | **Env default** — `TIMEOUT` env var default 30 — `unwrap_or_else` pattern. |
| P365 | **Metadata guard** — Reject config over 1MB before `read_to_string`. |
| P366 | **Stdin fallback** — Port from argv or interactive prompt — one `main` both paths. |
| P367 | **Line parser** — BufRead lines, skip `#`, parse `key=value` — trailing newline. |
| P368 | **Capstone CLI** — env path → metadata check → line parse → print port — function list only. |

---

**Total: 368 prompts** (P001–P368).

## By theme

| Theme | IDs |
|-------|-----|
| Ownership / borrow | P006–P024, P076–P087 |
| Stack / heap | P012–P017 |
| Functions / methods | P031–P047 |
| Iterators | P048–P075 |
| Enums / match | P088–P115 |
| Types / traits | P025–P030, P116–P123, P221–P243 |
| Modules / crates | P129–P148 |
| Smart pointers | P149–P171 |
| Collections | P172–P192 |
| Closures | P193–P220 |
| Errors / tests | P124–P128 |
| Concurrency | P244–P265 |
| Systems / I/O | P337–P368 |
| Meta / tooling | P001–P005, P266–P336 |

## See also

- [PLAYGROUND_GUIDE.md](PLAYGROUND_GUIDE.md)
- [JAVA_PYTHON_RUST_MAP.md](JAVA_PYTHON_RUST_MAP.md)
- [CONTENTS.md](../CONTENTS.md)