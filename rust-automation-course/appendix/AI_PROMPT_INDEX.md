# AI Prompt Index

All **Afterparty: AI Lego blocks** from the book, numbered for reuse. Paste into any AI assistant after reading the linked chapter.

## How to use Afterparty

After each chapter, open your favourite AI assistant and paste one prompt at a time. Treat the model as a **sparring partner**:

1. Read the chapter (20–40 minutes).
2. Run the **Playground** example yourself.
3. Do 2–3 Afterparty prompts — insist on compiler-accurate answers.
4. Optionally follow **Go deeper** links on [Functional Rust](https://hightechmind.io/rust/).

Find a prompt by ID (`P046` → table below), open the linked chapter for context, paste the prompt, and push back if the answer is vague or wrong.

---

## Chapter 0 — Preface

| ID | Prompt |
|----|--------|
| P001 | **Learning plan** — I know Java and Python. Based on this preface, build me a 2-week study plan using only this book's chapter list; 45 minutes per day. |
| P002 | **Gap check** — Ask me five quick questions to see if I should skip straight to Chapter 4 (lifetimes) or read Chapters 1–3 first. |
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

## Chapter 3 — Iterators

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

## Chapter 4 — Lifetimes

| ID | Prompt |
|----|--------|
| P028 | **Error archaeology** — I paste a "lifetime may not live long enough" error; walk me through owner vs reference diagram. |
| P029 | **Return type choice** — For API `fn title(book: &Book) -> ???` compare `&str` vs `String` trade-offs for a library. |
| P030 | **Struct lifetime** — Design `ConfigParser` holding `&str` slices into input buffer — when is it sound vs use owned `String`? |
| P031 | **Elision quiz** — Add explicit lifetimes to 4 function signatures where elision fails. |
| P032 | **Java analogy** — Compare Rust lifetimes to Java stack locals vs heap references — 120 words, accurate only. |
| P033 | **Fix mine** — I return `&String` built inside function; show three idiomatic fixes ranked by simplicity. |

## Chapter 5 — Enums and pattern matching

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
| P161 | **Checklist drill** — Match 6 Chapter 5 compiler errors to snippets; I name fix (`match` arm, `ref`, `?`). |
| P162 | **Java enum map** — Java `enum State` with method → Rust `enum` + `match` + `impl`; contrast nullability. |

## Chapter 6 — Structs, traits, generics

| ID | Prompt |
|----|--------|
| P040 | **Interface port** — Convert Java interface `Measurable` + two classes to trait + two structs + `impl`. |
| P041 | **Duck typing** — Python function accepts anything with `.read()`; express as trait bound in Rust generic. |
| P042 | **dyn vs impl** — Quiz: 4 scenarios — pick `dyn Trait` or `impl Trait` and justify. |
| P043 | **Default trait methods** — Add default `summary()` on trait; override in one type only. |
| P044 | **Generic bounds** — Fix compiler error: `T` needs `Display + Clone`; minimal bound set. |
| P045 | **OOP myth** — Explain in 100 words why Rust has no inheritance and what you do instead. |

## Chapter 7 — Errors and testing

| ID | Prompt |
|----|--------|
| P046 | **? chain** — Refactor nested match on Results to `?` style; explain each change. |
| P047 | **Error enum design** — Design `AppError` for CLI that reads config + talks serial; variants + `From` impls sketch. |
| P048 | **panic audit** — Mark which of 10 `unwrap()` calls should stay vs become `Result`. |
| P049 | **Test generation** — Write table-driven tests for `parse_port` including edge ports. |
| P050 | **Java exceptions** — Map checked Exception flow to Rust `Result` for file-not-found scenario. |
| P051 | **anyhow vs thiserror** — When would I pick each for automation binary vs library crate? |

## Chapter 8 — Collections and iterators

| ID | Prompt |
|----|--------|
| P052 | **Loop port** — Rewrite this C-style indexed loop as iterator chain; preserve behavior. |
| P053 | **HashMap merge** — Two maps of scores — merge by taking max per key; iterator style. |
| P054 | **Closure capture** — Explain `FnOnce` vs `FnMut` for closure storing `String`. |
| P055 | **collect types** — Why does `collect()` need type hint sometimes? Show turbofish example. |
| P056 | **Windows** — Detect rising edges in `Vec<f64>` with `.windows(2)` — write snippet. |
| P057 | **Performance myth** — Do Rust iterators optimize to loops? When might they not? |

## Chapter 9 — Smart pointers and modules

| ID | Prompt |
|----|--------|
| P058 | **Box why** — When is `Box<[T]>` better than `Vec<T>` on stack semantics? Two cases. |
| P059 | **Rc cycle** — Explain why `Rc` cycles leak memory; contrast Rust with Python cycles. |
| P060 | **Module split** — Split monolithic main.rs into lib + bin; list file tree only, I implement. |
| P061 | **pub audit** — What should be `pub` in a library crate vs kept private? |
| P062 | **Arc Mutex sketch** — Diagram thread-safe cache with Arc<Mutex<HashMap>> — no full code. |
| P063 | **Java heap** — Map Java "everything is reference" to Rust ownership + when Arc applies. |

## Chapter 10 — Multithreading

| ID | Prompt |
|----|--------|
| P064 | **Race quiz** — Which snippets are data races in C++ but rejected by Rust compiler? |
| P065 | **Channel design** — Worker pool with mpsc: I describe throughput; you sketch thread count + channel shape. |
| P066 | **Mutex vs RwLock** — Read-heavy sensor cache — pick primitive and why. |
| P067 | **Send fix** — I try to move `Rc` into thread; show fix with Arc. |
| P068 | **Join panic** — What happens if spawned thread panics? Handle in main. |
| P069 | **Python GIL** — Compare this Python threading example to Rust for same task. |

## Chapter 11 — Atomics

| ID | Prompt |
|----|--------|
| P070 | **Ordering quiz** — For shutdown flag + published config pointer, which orderings? Justify briefly. |
| P071 | **Counter port** — Replace Mutex counter with AtomicUsize; discuss lost updates with Relaxed. |
| P072 | **ABA problem** — Explain ABA in 80 words for compare_exchange — no full queue impl. |
| P073 | **Java AtomicInteger** — Map Java atomic increment to Rust fetch_add snippet. |
| P074 | **When not** — Three cases atomics are the wrong tool; prefer channels or Mutex. |
| P075 | **Fence intuition** — Draw happens-before arrow diagram for Release store + Acquire load. |

## Chapter 12 — Async and Tokio

| ID | Prompt |
|----|--------|
| P076 | **Future diagram** — Draw state machine for async fn with two await points. |
| P077 | **Tokio scaffold** — Generate minimal Tcp echo server skeleton; I fill body. |
| P078 | **select! scenario** — Cancel slow request when fast path returns — outline `select!`. |
| P079 | **async vs thread** — 1000 Modbus polls — argue async vs thread pool for latency. |
| P080 | **blocking fix** — Identify blocking calls in async snippet; suggest `spawn_blocking`. |
| P081 | **Python asyncio** — Map asyncio gather to Tokio join — API comparison table. |

## Chapter 13 — Metaprogramming

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
| P166 | **Scope honesty** — List 5 metaprogramming topics Ch13 skips and where to learn each. |
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
| P181 | **thiserror vs manual** — Same error enum: count lines derive vs hand-written (Ch7 style). |
| P182 | **clap subcommands** — Sketch `#[derive(Subcommand)]` enum for gateway start/stop/status. |
| P183 | **Float Eq trap** — Show `#[derive(Eq)]` on `f64` field failure; two fixes from Ch6. |
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

## Chapter 14 — Unsafe

| ID | Prompt |
|----|--------|
| P088 | **Invariant list** — For raw pointer to buffer + length, list 5 invariants safe wrapper must enforce. |
| P089 | **Soundness** — Explain "safe Rust can't cause UB" vs unsafe — one paragraph; include unsound safe wrapper example. |
| P090 | **FFI checklist** — Checklist for calling C library from Rust binary. |
| P091 | **Miri** — What is Miri and when run it relative to unsafe changes? |
| P092 | **Avoid** — Review use case: speed up JSON — unsafe vs simd crate vs algorithm. |
| P093 | **Java JNI** — Compare JNI pitfalls to Rust FFI ownership rules. |
| P201 | **Scope honesty** — List 6 topics Ch14 skips and where to learn each (nomicon, Miri, Pin, …). |
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
| P212 | **Ch10 port** — Rewrite Level 4 spawn example using only safe types — when is it impossible? |
| P213 | **Proc-macro boundary** — Why do serde/tokio crates use `unsafe impl` you don't write? Link Ch13. |
| P214 | **CString trap** — Show `into_raw` forgotten `from_raw` leak; fix with RAII pattern sketch. |
| P215 | **Vendor SDK** — Diagram ownership: Rust owns config, C owns connection, callback pointer — boxes and arrows only. |
| P216 | **serialport hide** — Where does `unsafe` live in a typical serial crate vs my application code? |
| P217 | **CRC decision** — C `crc16` vs Rust `crc` crate vs hand-rolled — decision tree for production gateway. |
| P218 | **Trap quiz** — Mark 6 snippets: safe, UB, ordering bug, needs Miri, needs Mutex, unsound safe API. |
| P219 | **Review rubric** — 10-point code-review checklist for an `unsafe` PR in an automation repo. |
| P220 | **Test plan** — Unit + Miri + integration tests for new `extern 'C'` wrapper — bullet list only. |
| P221 | **Borrow checker fight** — I paste fight-the-borrow-checker code; you refactor to safe Rust without `unsafe`. |
| P222 | **static mut** — Compare `static mut` counter vs `AtomicUsize` from Ch11 — UB vs defined behavior. |
| P223 | **PlcDriver API** — Design safe `PlcDriver` Rust API over fictional `extern 'C'` — types, `Result`, no raw pointers in public API. |
| P224 | **Level ladder recap** — Explain Levels 1–5 in one paragraph each for a Java teammate who knows JNI. |

## Chapter 15 — I/O and processes

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
| P238 | **Ch12 bridge** — Same echo server: sketch sync thread version vs Tokio version — tradeoffs table. |

---

**Total: 200 prompts** (target ≥80 met).

## By theme

| Theme | IDs |
|-------|-----|
| Ownership / borrow | P006–P011, P106–P114, P141–P148, P022–P027, P028–P033 |
| Stack / heap | P106–P114 |
| Iterators | P115–P140 |
| Enums / match | P034–P039, P149–P162 |
| Types / traits | P040–P045 |
| Errors / tests | P046–P051 |
| Concurrency | P064–P081 |
| Systems / automation | P094–P105, P225–P238 |
| Meta / tooling | P001–P021, P082–P087, P088–P093, P163–P200 |

## See also

- [PLAYGROUND_GUIDE.md](PLAYGROUND_GUIDE.md)
- [JAVA_PYTHON_RUST_MAP.md](JAVA_PYTHON_RUST_MAP.md)
- [CONTENTS.md](../CONTENTS.md)
