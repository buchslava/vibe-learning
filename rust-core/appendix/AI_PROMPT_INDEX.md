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
| P006 | **Move drill** — Give 6 tiny Rust snippets mixing `String`, `Vec`, and `i32`. I predict `ok` or compile error; you reveal answers, quote the error message, and give the smallest fix. |
| P007 | **Use-after-move decoder** — Show one `println!` after a move that fails to compile. I explain the error in plain English; you refine my explanation and show the three fixes: borrow, `.clone()`, or restructure scope. |
| P008 | **Return transfers ownership** — Write a function `fn take(s: String) -> String` and a `main` that calls it. I trace who owns the heap buffer at each line, including after the return. |
| P009 | **Move into a call** — Snippet: `process(build_label())` where `build_label() -> String`. Explain when the heap allocation happens, when ownership moves into `process`, and when `drop` runs if `process` takes `String` by value. |
| P010 | **Scope and drop** — Give a nested-block snippet with two `String`s and one `Vec`. I mark the exact line where each heap buffer is freed vs where stack slots disappear; you correct and explain drop order. |
| P011 | **Single-owner principle** — State Rust’s rule ‘every value has exactly one owner’ in one sentence, then give one `String` example where breaking that rule would double-free. Use the key-handoff metaphor. |
| P012 | **Stack vs heap quiz** — For 10 declarations (`i32`, `bool`, `[u8; 4]`, `String`, `Vec<i32>`, `&str`, `&String`, `(i32, f64)`, `Box<i32>`, `()`), I say stack-only, heap involved, or both; you draw the pointer picture for any I miss. |
| P013 | **String layout sketch** — For `let s = String::from("hi")`, I describe stack fields (ptr, len, cap) and heap bytes; you correct and extend to `let v = vec![1, 2, 3]`. |
| P014 | **Stack frame drill** — I give a 3-function call chain with `i32` locals and one `String` passed by value. Trace stack frame push/pop, when each slot dies, and when the `String` heap buffer is dropped. |
| P015 | **Nested block live ranges** — Snippet with `{ let inner = ... }` inside `main`. I list which bindings are alive on each line; you explain why shorter live ranges matter for borrowing later. |
| P016 | **Borrow without heap copy** — Show `let s = String::from("x"); let r = &s;` — I explain what is on stack vs heap and why `r` does not duplicate the buffer; you add one line that would fail because of move/borrow conflict. |
| P017 | **`&` vs `&mut` pick** — Five tasks (log a label, increment a tick counter, parse into a temp struct, share read-only config, swap two buffers in place). I choose pass-by-value, `&T`, or `&mut T`; you explain owner count and alias rules. |
| P018 | **Create the borrow** — Given `let mut n = 10;` and `let s = String::from(\"plc\");`, I write the types and expressions for one `&` and one `&mut` borrow; you verify the owner is still usable afterward. |
| P019 | **When is `*` required?** — Four expressions mixing `&i32`, `&mut i32`, and `println!`. I mark where explicit `*r` is needed vs where auto-deref handles it; you correct with compiled examples. |
| P020 | **Mutate through `*`** — Fill in `fn reset(n: &mut u32) { ... }` and a `main` that calls it. I must use `*` to zero the caller’s value; you show a broken version that tries to reassign `n` instead. |
| P021 | **Reference copy vs move** — After `let r1 = &s; let r2 = r1;` vs `let s2 = s1;`: I compare heap owner count, which bindings stay valid, and whether heap data was copied. |
| P022 | **Type of `*r`** — Bindings `r: &i32`, `m: &mut i32`, `b: Box<i32>`. I name the type of `*r`, `*m`, and whether `*m = 5` mutates the owner; you extend with one `&T` where `*r = 5` fails. |
| P023 | **Borrow blocks mutation** — Snippet: immutable `let r = &s;` then `s.push('!')`. I explain the compile error; you fix by shrinking `r`’s scope with a nested block. |
| P024 | **Automation counter** — Sketch a 1 kHz loop with `ticks: u64` and a helper `fn maybe_rollover(t: &mut u64)`. I write the call site and one `*t` mutation inside the helper; you review without full thread code. |
| P025 | **Move or `&mut` quiz** — Six tasks (append to a reusable log line, send a message on a channel, fill a frame `Vec<u8>` in a loop, store a device name in a struct, transform-and-return a label, increment a counter). I pick `fn take(T)` move vs `fn tweak(&mut T)`; you explain who owns the heap data after the call. |
| P026 | **After the call** — Two functions: `consume(String)` and `append_bang(&mut String)`. I trace which bindings in `main` are valid **after** each call and whether the heap buffer was dropped, reused, or returned. |
| P027 | **Fix the signature** — Show code that moves a `String` into a helper but then tries to `println!` it in `main`. I explain the error and choose the smallest fix: change to `&mut String`, restructure with a return value, or `.clone()` — you rank the idiomatic options. |
| P028 | **Loop reuse** — A serial parser reuses `let mut buf = Vec::with_capacity(256)` every tick. I explain why `parse_frame(buf)` (move) is wrong and write `parse_frame(&mut buf)` instead; you add one line showing the caller reading updated length after parse. |
| P029 | **Transform-and-return** — Task: uppercase a `String` and give it back. I write `fn upper(s: String) -> String` and call site; you contrast with an anti-pattern that takes `&mut String` when ownership should transfer. |
| P030 | **`i32` vs `String` move** — Same pattern with `fn add_one(n: i32)` and `fn add_bang(s: String)`: I explain why the caller can still use `n` after the call but not `s`; you map each to Copy vs heap move. |
| P031 | **Call it twice** — Snippet that needs to pass the same `String` to two helpers in one scope. I show why two by-value calls fail, then fix with `&str`/`&mut` borrows or one move plus `.clone()`; you flag the smell. |
| P032 | **Copy eligibility quiz** — Quiz me on 12 types (`i32`, `String`, `&str`, `Vec<i32>`, `[u8; 8]`, `(i32, String)`, `Box<i32>`, `fn()`, `bool`, `char`, `Rc<i32>`, struct with only `i32` fields). Copy, move, or ‘Copy only if derived’? Cite the rule each time. |
| P033 | **Copy vs Drop paradox** — Why can’t a type be both `Copy` and `Drop`? Use a `File` or socket handle: show what goes wrong if assignment bitwise-copies the handle. |
| P034 | **Semantic copy test** — For `i32`, `&T`, and `String`: if I duplicate the stack bits, is the result always semantically identical? When does it fail, and what does Rust do instead? |
| P035 | **Double-free thought experiment** — Hypothetical: `String` were `Copy`. Walk line-by-line through `let a = ...; let b = a; }` end of block — show both drops freeing the same address. Contrast with real move semantics. |
| P036 | **When to derive Copy** — I put `#[derive(Copy, Clone)]` on a struct with a `String` field — explain the error. Then give three struct shapes: safe to derive `Copy`, must stay move-only, and where `.clone()` belongs in the public API. |
| P037 | **Reference is Copy** — Four snippets: `let r2 = r1` for `&String` vs `let s2 = s1` for `String`, plus one with `&mut`. I predict ok/error; you count owners of the heap buffer after each line. |
| P038 | **`.clone()` judgment** — Five scenarios (store config `String`, pass into fn twice, cache key in a map, loop append, return from fn). For each, say move, borrow, or `.clone()` — and flag when `.clone()` is a design smell. |

## Chapter 2 — Types and expressions

| ID | Prompt |
|----|--------|
| P039 | **Integer pick** — Quiz me: for 8 scenarios (Modbus port, byte buffer, collection index, money cents, timestamp millis, hash output, loop counter, enum discriminant), I pick `u8`/`u16`/`u32`/`u64`/`i32`/`usize`; you correct and explain overflow risk. |
| P040 | **`char` vs byte** — Explain why `'A'` is not the same type as `65u8`. Show one valid `char` literal and one invalid escape; mention UTF-8 only when discussing `str`/`String`. |
| P041 | **Tuple vs array** — When do I use `(u16, u16)` vs `[u16; 2]` for a coordinate pair? Give one idiomatic example of each. |
| P042 | **Array bounds** — Snippet with `[u8; 4]` and an index from user input. I explain compile-time vs runtime safety; you show bounds checking with `.get()` vs `[i]`. |
| P043 | **`&str` vs `String` API** — Five function signatures for logging labels. I choose `&str` or `String` for each; you explain ownership and allocation cost. |
| P044 | **Parse drill** — Give `let s = \"502\";` — I write two ways to get `u16` (unwrap version and proper `Result` version); you grade idiomatic error handling. |
| P045 | **Slice from array** — Given `[u8; 8]` with a 2-byte header and 6-byte payload, I write range expressions for `header`, `payload`, and `full` as `&[u8]`; you check half-open bounds. |
| P046 | **`.get` vs `[i]`** — Three indexing scenarios (fixed compile-time index, user-supplied index, loop variable). I pick `[i]` or `.get(i)` for each; you explain panic risk. |
| P047 | **UTF-8 length trap** — For `\"naïve\"` and `\"hi 🦀\"`, I predict `.len()` vs `.chars().count()`; you show one bad string slice that panics and the safe fix (`.chars()` or careful byte ranges). |
| P048 | **Coercion quiz** — Five call sites passing `&str`, `String`, `&String`, and a literal into `fn log(msg: &str)`. I say ok or what coercion happens; you quote the effective type. |
| P049 | **Empty slice** — Explain whether `&frame[0..0]` is valid, what `.len()` returns, and how `if slice.is_empty()` reads in a parser guard. |
| P050 | **Lines and config** — Multiline config string with comments and blank lines. I sketch a loop using `.lines()` and `.trim().starts_with('#')`; you extend with one `strip_prefix` for `KEY=` rows. |
| P051 | **Range quiz** — For half-open vs inclusive ranges, I predict output of four `for` loops using `..` and `..=`; you correct with printed values. |
| P052 | **`if` expression types** — Show an `if` where branches return different types and fail to compile. I explain the error; you fix with a unified type (same type in both arms or wrap in enum preview). |
| P053 | **match warm-up** — Extend a `match` on HTTP status codes with 3xx, 4xx, 5xx groupings using range patterns; I write arms; you check exhaustiveness. |
| P054 | **Loop choice** — Four iteration tasks (infinite retry with break, consume iterator, index 0..len, early exit on condition). I pick `loop`/`while`/`for`; you confirm idiomatic choice. |

## Chapter 3 — Functions and methods

| ID | Prompt |
|----|--------|
| P055 | **Parameter audit** — Five function signatures for logging: `&str`, `String`, `&String`, `Cow<str>`, `impl AsRef<str>`. I pick one per use case; you explain ownership cost. |
| P056 | **Move vs borrow** — Snippet calls `process(s)` then uses `s` again. I explain the error and show two fixes (`&s`, clone). |
| P057 | **impl block** — Struct `Timer` with `start`, `elapsed`, `reset`. I write `impl`; you check `&self` vs `&mut self`. |
| P058 | **Associated fn** — When is `Type::new()` idiomatic vs `Default::default()`? One example each. |
| P059 | **Consume self** — Method `fn into_inner(self) -> Vec<u8>` — why must it take `self` by value? |
| P060 | **Semicolon trap** — Three tiny functions: one returns `i32` correctly, two fail due to `;`. I fix them. |
| P061 | **Early return** — Rewrite nested `if` in a parser as early `return None` / `?` style. |
| P062 | **Unit vs value** — Which functions should return `()` vs `bool` vs `Option<T>`? Three CLI helper names, I choose. |
| P063 | **Generic bounds** — Fix `fn max(a: T, b: T)` without bounds; add `T: Ord` or `PartialOrd`. |
| P064 | **Result signature** — Design `fn read_config(path: &str) -> Result<Config, ...>`; list error variants, no body. |
| P065 | **impl Iterator return** — Write `top_n(vals: &[f64], n: usize) -> impl Iterator<Item = f64>` — explain why two different iterator types in `if` arms fail. |
| P066 | **mem take drain** — Buffer struct drains `Vec<u8>` to caller via `mem::take` — show before/after inner field. |
| P067 | **where clause** — Rewrite cluttered `fn f<T: A + B + C>(x: T)` with `where` block — same behaviour. |
| P068 | **Self return** — Method `fn into_inner(self) -> Vec<u8>` on wrapper — why `Self` not concrete type name? |
| P069 | **const fn** — One `const fn` port validator `fn is_valid(p: u16) -> bool` — what can and cannot run at compile time? |
| P070 | **Error constructors** — Add `missing_field(name, record)` on a parse error enum with `impl Into<String>`; compare to spelling the variant at each site. |
| P071 | **Config holder** — HTTP poll client: struct holds timeout/retries/endpoint; one `build_request(device_id)` — no builder crate. |
| P072 | **Extension trait** — Add `TrimOrEmpty for &str`; use in three parser call sites vs free functions. |

## Chapter 4 — Iterators

| ID | Prompt |
|----|--------|
| P073 | **for desugar quiz** — Four `for` loops (`0..n`, `&vec`, `vec`, `&mut vec`). I say which calls `iter`, `into_iter`, or range; you correct and show owner state after the loop. |
| P074 | **iter vs into_iter** — Give 4 snippets using `Vec`; I predict whether `v` is usable after the loop; you explain move vs borrow. |
| P075 | **iter_mut drill** — Task: double every element in `Vec<f64>` in place without allocating a new `Vec`. I write the loop; you review `*n` and borrow rules. |
| P076 | **Range vs collect** — When is `for i in 0..n` better than `(0..n).collect::<Vec<_>>()`? Give two automation examples (retry count vs materializing indices). |
| P077 | **Adapter chain** — Task: parse lines, trim, keep non-empty, parse as `u16`. I sketch `.lines().map(...).filter(...).collect()`; you refine. |
| P078 | **zip pairs** — Two `Vec<u16>`: register IDs and values. Build `Vec<(u16, u16)>` with `.zip()`; I write it; you handle length mismatch policy. |
| P079 | **take and skip** — Paginate a log: skip first 100 lines, take next 20. I use `.skip().take()` on `.lines()`; you show one pitfall if the source is not recomputed. |
| P080 | **chain iterators** — Concatenate header rows and body rows (two `&[u8]` slices) without copying bytes into one array first — sketch with `.iter().chain()`. |
| P081 | **enumerate vs index** — Same sum-over-evens task twice: index `for` vs `.enumerate()`. Compare readability and bounds-check risk. |
| P082 | **Lazy vs eager** — Explain when `filter().map()` allocates vs when `.collect()` forces work. One example with `println!` in `map` showing evaluation order. |
| P083 | **collect turbofish** — Three `collect()` calls that fail without hints — I add type annotation or turbofish; you verify. |
| P084 | **find and Option** — Wire scan: `Vec<&str>` of lines, find first containing `ERROR`. I return `Option<&str>` with `.find()`; you contrast with `.filter().next()`. |
| P085 | **fold vs sum** — Compute max and count in one pass with `.fold()` vs calling `.max()` and `.len()` separately — when is fold worth it? |
| P086 | **any and all** — Validate a batch: all ports in 1..=65535, any line starts with `#`. I write `.all()` / `.any()` predicates; you fix one double-reference mistake. |
| P087 | **Moved Vec mistake** — Show code that does `for x in v` then uses `v` again. I explain the error and fix with `.iter()` or clone; you rank fixes. |
| P088 | **Borrow in chain** — Snippet: build `Vec<&str>` from `String` lines then drop the `String`. I explain why it fails; you fix with owned `String` or different lifetime design. |
| P089 | **Modbus-style scan** — List of raw register values `Vec<u16>`: filter evens, map to `f64` scale 0.1, sum. I write the iterator chain; you check overflow and types. |
| P090 | **Zero-cost check** — Does `nums.iter().filter(...).map(...).sum()` allocate intermediate `Vec`s? Answer for `--release` and what to measure conceptually. |
| P091 | **Trap sheet drill** — Give 6 snippets mixing `for x in v`, `for x in &v`, `into_iter`, and `collect` type errors. I predict compile ok or fail and why; you show the fixed line. |
| P092 | **&&i32 decoder** — One `.iter().filter().map()` chain: I label the closure parameter type at each step; you correct and show `|x|`, `|&x|`, and `|&&x|` versions that compile. |
| P093 | **Empty iterator policy** — Three tasks (sum, find, all) on possibly empty `Vec`. I state the result and whether it is a domain bug; you correct (e.g. empty `all` is true). |
| P094 | **zip truncation** — IDs len 5, values len 100 — I write zip collect; you explain silent loss and sketch `zip` + length check or `enumerate` on the longer vec. |
| P095 | **RangeCounter impl** — Implement `Iterator` for integers 1..=5; collect to `Vec` and sum with `.sum()` — show `type Item` and `fn next` only. |
| P096 | **Skip blanks** — Config string with empty lines — write `NonEmptyLines` iterator that trims and skips `''`; collect keys before `=`. |
| P097 | **Infinite take** — Counter from 0 without end — why must you `.take(n)` before `.collect()`? Show hang vs bounded collect. |
| P098 | **IntoIterator pair** — Same struct: implement `Iterator` and `IntoIterator` so both `scan.next()` and `for p in scan` work — minimal impl blocks. |
| P099 | **Stateful parser** — Byte buffer iterator yielding complete 4-byte frames; partial frame stays in struct — sketch `next()` state machine. |
| P100 | **Capstone iterator** — CSV line iterator: split fields, parse col 2 as `u16`, filter > 0 — custom struct + one consumer chain. |

## Chapter 5 — Lifetimes

| ID | Prompt |
|----|--------|
| P101 | **Error archaeology** — I paste a ‘lifetime may not live long enough’ error; walk me through owner vs reference diagram. |
| P102 | **Return type choice** — For API `fn title(book: &Book) -> ???` compare `&str` vs `String` trade-offs for a library. |
| P103 | **Struct lifetime** — Design `ConfigParser` holding `&str` slices into input buffer — when is it sound vs use owned `String`? |
| P104 | **Elision quiz** — Add explicit lifetimes to 4 function signatures where elision fails. |
| P105 | **Fix mine** — I return `&String` built inside function; show three idiomatic fixes ranked by simplicity. |
| P106 | **static trap** — Function returns `&str` from `format!` — show error and owned fix. |
| P107 | **two lifetimes** — Write `fn first<'a,'b>(x: &'a str, y: &'b str) -> &'a str` — drop `y` while result lives. |
| P108 | **Config struct** — Parse `host:port` into `Config<'a>` — when must caller keep `line` alive? |
| P109 | **Owned refactor** — Same parser returning owned `Config { host: String, port: u16 }` — tradeoffs in 3 bullets. |
| P110 | **T: 'a bound** — Explain `struct Holder<'a, T: 'a> { value: &'a T }` — what fails if `T` is shorter-lived? |
| P111 | **Elision fail** — Four signatures where elision works vs fails — I label each. |
| P112 | **Iterator borrow** — Collect `Vec<&str>` from `String` lines — why drop order matters (link Ch 4). |

## Chapter 6 — Enums and pattern matching

| ID | Prompt |
|----|--------|
| P113 | **Null replacement** — Translate 5 Java methods returning null into `Option` Rust; explain callsite changes. |
| P114 | **unwrap audit** — Paste 20 lines with 4 `unwrap()` calls; I mark panic risk; you rewrite with `match` or `?`. |
| P115 | **Combinator chain** — Parse port `Option<u16>`, double if Some, default 502 — I write `map`/`unwrap_or`; you add `and_then` version. |
| P116 | **let-else port** — Rewrite nested `match` on `parse()` into `let Ok(x) = ... else { ... }`; preserve behavior. |
| P117 | **Result railway** — Chain parse → validate → compute with `?`; I fill blanks, you verify. |
| P118 | **Err arm missing** — Show `match` on `Result` without `Err` arm; I quote error and fix; add boundary `eprintln!` pattern. |
| P119 | **unwrap vs ?** — Same parser twice: `unwrap` vs `fn -> Result` with `?`; compare panic risk and signature honesty. |
| P120 | **Exhaustive match** — Enum with 4 variants; I write `match`; you add variant `Safe` and show compile error until I fix. |
| P121 | **Wildcard footgun** — Explain why `_` on your own enum hides new variants; show explicit arms vs `_` refactor story. |
| P122 | **Partial move** — `enum` with `String` field: `match` moves field; I fix with `ref` or `match &e`; you show error text. |
| P123 | **State machine** — TCP/serial states as enum; `connect`/`send`/`close` return `Result<(), IllegalTransition>`. |
| P124 | **Python Union** — Python `int | str` parameter → Rust enum + `match`; no `dyn`. |
| P125 | **if let vs match** — Three tasks: one variant, two variants, five variants — I pick `if let` or `match` each time. |
| P126 | **Match on ref** — Given owned `Status`, I choose `match s` vs `match &s`; predict move errors; you diagram ownership. |
| P127 | **Guard drill** — Classify `i32` with guards (`<0`, `==0`, `>100`); I write arms; you check exhaustiveness. |
| P128 | **Opcode table** — Design `enum` for 3 frame types + `match` returning `u8` opcode; add fourth type as compile-break exercise. |
| P129 | **ReadOutcome extend** — Add `Disconnected` to automation `ReadOutcome`; show all `match` sites compiler lists. |
| P130 | **Config sentinel** — Rewrite `fn port() -> i32` returning `-1` on failure to `Option<u16>` + `match` in `main`. |
| P131 | **Checklist drill** — Match 6 Chapter 6 compiler errors to snippets; I name fix (`match` arm, `ref`, `?`, return type). |
| P132 | **Java enum map** — Java `enum State { IDLE, RUN }` with method — port to Rust `enum` + `match` + `impl`; contrast nullability. |
| P133 | **Slice split** — Parse `POST /api/v1/run HTTP/1.1` with `[method, path @ .., _ver]` — I write; you handle single-token input. |
| P134 | **matches refactor** — Replace 6-arm `match` that returns `bool` with `matches!` — show before/after on `Mode` enum. |
| P135 | **if let chain** — Parse `host:port:extra` — chain should reject extra segments; fix my broken parser. |
| P136 | **@ binding quiz** — Three arms with `@` ranges for port classes — I label which values hit which arm. |
| P137 | **Exhaustive slice** — `[a, b]` on `Vec` of len 1 or 3 — predict `_` arm vs bug; suggest `..` rest pattern. |
| P138 | **matches vs match** — When must you keep full `match` instead of `matches!`? Give one example returning `String`. |
| P139 | **Guard + @** — Match port `n @ 1024..=65535` with guard `n % 2 == 0` — sketch arm. |
| P140 | **Capstone parse** — Frame header `[sync, len @ 1..=255, payload @ ..]` from byte slice — sketch `match` arms only. |

## Chapter 7 — Structs, traits, generics

| ID | Prompt |
|----|--------|
| P141 | **new vs default** — When is `Sensor::new` idiomatic vs `Default` + field update? One automation example each. |
| P142 | **Method receiver** — Same logic three ways: `fn f(self)`, `fn f(&self)`, `fn f(&mut self)` on a struct; I predict what calls compile. |
| P143 | **Enum trait impl** — Add variant `Fault` to `Status`; show every compile error until trait `impl` and inherent methods match. |
| P144 | **Struct vs enum layout** — Modbus/OpcUa device registry: justify `struct Device { kind: Enum }` vs `enum Device { Modbus(...), OpcUa(...) }`; sketch types. |
| P145 | **SensorReading port** — Python `Union[TempReading, PressReading, Skipped]` → Rust enum + struct payloads + `Measurable` trait; show `match` in impl. |
| P146 | **Two impl blocks** — Same type: add inherent `is_valid()` and trait `Summary`; explain which call sites see which methods. |
| P147 | **Partial move fix** — Given `enum Packet { Raw(String), Empty }`, reproduce ‘partially moved value’ and rewrite with `&self` or one `match`. |
| P148 | **matches! drill** — Rewrite `!matches!(self, Skipped { .. })` as longhand `match`; then back to `matches!`; link to macro_rules conceptually. |
| P149 | **Default override** — Trait `HasCode` with default `label()`; override for one enum variant only; use `HasCode::label(self)` for the rest. |
| P150 | **Recursion trap** — Show infinite recursion when override calls `self.label()` instead of `HasCode::label(self)`; fix it. |
| P151 | **One trait per impl** — Show `impl HasCode, Display for T` compile error; split into two blocks; add `fn show<T: HasCode + Display>(x: T)`. |
| P152 | **Command + log row** — `enum Command` + `struct SetSpeedLog` + `to_log()`; list which derives each type needs and why. |
| P153 | **Eq on floats** — Add `Analog(f64)` variant; show `#[derive(Eq)]` failure; fix with integer fixed-point or `PartialEq` only. |
| P154 | **Generic bounds** — Fix compiler error: `T` needs `Display + Clone`; minimal bound set on `fn duplicate_and_print<T>(x: T)`. |
| P155 | **largest pitfalls** — Why does `largest` need non-empty slice? Add `Option` return or document panic; compare to Java generics erasure story. |
| P156 | **where clause** — Rewrite `fn f<T: A + B + C>(x: T)` with a `where` block; same signature, longer trait list. |
| P157 | **dyn vs impl quiz** — Four scenarios (plug-in Vec, single helper fn, closed protocol, factory return) — pick `impl`, `dyn`, or `enum` each time. |
| P158 | **AlarmSink registry** — Implement `&[&dyn AlarmSink]` for log + metrics; then refactor to `enum Sink` if set is closed — compare trade-offs. |
| P159 | **Factory return** — `greeter_for(lang) -> Box<dyn Greeter>` vs `-> impl Greeter` — show why `impl` fails when arms return `En` and `Fr`. |
| P160 | **Driver three ways** — Same `poll()` behaviour with `&impl Driver`, `&[&dyn Driver]`, and `enum Device`; benchmark story without running code. |
| P161 | **Box vs borrow** — When is `&dyn Trait` enough vs `Box<dyn Trait>` required? Vec of mixed types + dangling return examples. |
| P162 | **Object safety audit** — Mark each trait dyn-safe or not: no-`self` method, generic method, `-> Self`, `trait Foo: Sized`. |
| P163 | **Reader fix** — Trait with `fn read() -> f64` fails as `dyn`; redesign for `&dyn Reader` or use enum. |
| P164 | **Clone not dyn** — Why no `Box<dyn Clone>` pattern for heterogenous clone list; suggest enum or generic `T: Clone` instead. |
| P165 | **Send + Sync** — Alarm handler shared across threads: write type as `Arc<dyn AlarmSink + Send + Sync>`; what breaks if handler holds `Rc`? |
| P166 | **Unsized trap** — Show three snippets that fail: `let g: dyn Greeter = En`, `Vec<dyn Greeter>`, returning `&dyn` from locals; fix each. |
| P167 | **Orphan rule** — Why `impl Display for Vec<u8>` fails; fix with newtype `struct Frame(pub Vec<u8>)` + `impl Display for Frame`. |
| P168 | **External trait** — Wrap third-party struct; implement your trait on the wrapper; call from automation main. |
| P169 | **Checklist drill** — Match 8 Chapter 7 compiler errors to snippets (non-exhaustive match, partial move, orphan, not dyn compatible, unsized, moved self, Eq+f64, multi-trait impl). |
| P170 | **PLC message model** — Design full model: enum frames, struct payloads, two traits, one `dyn` registry for sinks, derive list — no code over 80 lines. |
| P171 | **Refactor story** — Start with `Vec<Box<dyn Driver>>`; protocol closes to two devices; refactor to `enum`; list what the compiler now catches. |
| P172 | **Item type quiz** — Change `PortScan`'s `type Item` from `u16` to `(u16, bool)` — list every call site in a `.map().collect()` chain that breaks and why. |
| P173 | **Summarizable design** — Add `Summarizable` with `type Output = String` for three sensor structs; one `fn report(r: &impl Summarizable)` — no duplicate return types in the trait. |
| P174 | **Associated vs generic** — Same cache API twice: `trait Get<T>` vs `trait Get { type Value; }` — when is each painful at call sites? |
| P175 | **Supertrait bounds** — Trait `Exportable: Display + Debug` with default `fn export` — show impl for `Port(u16)`; what fails if `Display` is missing? |
| P176 | **UFCS fix** — Type implements `A` and `B`, both define `name()` — show ambiguous call error and UFCS fix with `A::name(&x)`. |

## Chapter 8 — Errors and testing

| ID | Prompt |
|----|--------|
| P177 | **? chain** — Refactor nested `match` on `Result`s to `?` railway; explain each desugared `return Err`. |
| P178 | **map / and_then** — Same parser with `.map_err` + `.map` vs `.and_then(validate)`; when to use each. |
| P179 | **Wrong return type** — Show `?` compile error in `fn -> u32`; fix signature and boundary `match`. |
| P180 | **Poll loop** — Rewrite `unwrap` Modbus config parse to log-and-continue; process must survive bad line. |
| P181 | **Internal vs boundary** — Mark 8 functions in a gateway crate: bubble with `?` or handle in `main`? |
| P182 | **Unwind vs Result** — Diagram stack for `open()?` vs `unwrap()` on missing file; who runs `Drop`? |
| P183 | **panic audit** — Mark 10 `unwrap`/`expect` sites: keep (test/invariant) vs `Result` (I/O/config). |
| P184 | **Drop double panic** — Explain why panicking in `Drop` aborts; sketch safe cleanup pattern. |
| P185 | **catch_unwind scope** — When is `catch_unwind` appropriate vs abuse? One automation anti-example. |
| P186 | **abort strategy** — Embedded firmware: `panic = abort` — what cleanup is skipped vs unwind? |
| P187 | **Std limits** — List 4 pain points of `io::Error` + `String` errors in a Modbus gateway; no FUD. |
| P188 | **Manual vs thiserror** — Same `AppError` twice: hand-written `Display`/`Error`/`From` vs `#[derive(Error)]`; count lines saved. |
| P189 | **thiserror design** — Design `GatewayError` with ConfigRead, ParsePort, Timeout, Serial sub-enum; full derive attrs. |
| P190 | **anyhow vs thiserror** — Pick for automation binary vs library crate; show `main -> anyhow::Result` + `.context()`. |
| P191 | **Why not String** — Explain why `pub fn connect() -> Result<(), String>` is weak; propose enum API. |
| P192 | **Error enum design** — Design `AppError` for config + serial + timeout; variant shapes + recovery `match`. |
| P193 | **Recovery match** — Poll loop: Timeout → retry, ParsePort → alert, DeviceOffline → safe state — sketch `match`. |
| P194 | **Nested SerialError** — Add `AppError::Serial(SerialError)`; show propagation with `#[from]`. |
| P195 | **Exhaustive trap** — Add `DeviceOffline` variant; quote non-exhaustive `match` errors until fixed. |
| P196 | **Boundary pattern** — `run() -> Result` + `main` maps to exit code; no `unwrap` in between. |
| P197 | **map_err drill** — Convert `ParseIntError` → `AppError::Parse` with and without `From` / `#[from]`. |
| P198 | **Table-driven ports** — Tests for `parse_port`: valid, zero, non-numeric, too large for `u16`. |
| P199 | **Integration test** — Sketch `tests/config_load.rs` that expects `Err` on missing file without panicking. |

## Chapter 9 — Modules, paths, and crates

| ID | Prompt |
|----|--------|
| P200 | **File tree** — Design module tree for a CLI that reads config and runs commands. Directories + `mod` lines only, no bodies. |
| P201 | **Nested `mod.rs`** — Draw `ui/theme/themes/` and `ui/theme/palettes/` as a directory tree. When is `foo/mod.rs` better than `foo.rs`? |
| P202 | **Path quiz** — From `crate::service::worker::run`, how do I reach `crate::config::load`? Show `use` and fully qualified call. |
| P203 | **use crate::** — In `app/events.rs`, import `AppState` from `app/state.rs` and `PanelLocation` from `core/location.rs` — write both `use` lines. |
| P204 | **super:: siblings** — `dsp/flowgraph.rs` needs `silence::silenced` from a sibling file under `dsp/`. Show the `use` line (not a path from crate root). |
| P205 | **lib vs bin** — What belongs in `main.rs` vs `lib.rs` for a tool with 500 lines of logic? |
| P206 | **Binary-only** — No `lib.rs`: list top-level `mod` lines in `main.rs` for a TUI with `core`, `ui`, `app`, and `util` modules. Where does `pub(crate)` fit? |
| P207 | **pub audit** — List items that should be `pub` vs private in a library crate exposing `Client::connect`. |
| P208 | **pub(super)** — `dsp/mod.rs` must call `flowgraph::run`, but `sdr.rs` must not reach it. Show `mod flowgraph;` and `pub(super) fn run` — who can call `run`? |
| P209 | **Facade module** — `ui/mod.rs` has `mod renderer;` and `pub use renderer::Renderer;`. Why not `pub mod renderer`? |
| P210 | **Barrel re-export** — `config/mod.rs` exposes `Station` and `load_stations` without callers writing `config::stations::`. Sketch `pub mod` + `pub use` lines. |
| P211 | **pub(crate)** — Name three helpers that should be `pub(crate)` in a binary crate (shared across `app/`, `ui/`, `main`) but must not be `pub`. |
| P212 | **pub(crate) mod** — `theme/mod.rs` keeps `palettes` visible inside the crate only. Show `pub(crate) mod palettes;` vs `pub mod palettes` — what breaks for external callers? |
| P213 | **pub mod vs mod** — In `browser/mod.rs`, `viewer_image` is private but `viewer` is `pub mod`. What can code outside `browser/` import? |
| P214 | **Re-export dep** — Sketch `pub use` so users see `my_crate::Error` but you wrap `thiserror` internally. |
| P215 | **Workspace split** — Two crates: `core` library + `cli` binary. Write `Cargo.toml` dependency path only. |
| P216 | **Integration test** — Where does `tests/smoke.rs` live and how does it `use` the library? |
| P217 | **Orphan fix** — I want `Display` on `Vec<u8>` — show newtype wrapper module layout. |
| P218 | **Lib name split** — Tauri package `sdr_fm` uses `[lib] name = \"sdr_fm_lib\"` and `main` calls `sdr_fm_lib::run()`. Why not name the lib `sdr_fm`? |
| P219 | **Domain layers** — Split a file manager TUI into `core/` (no ratatui), `ui/` (drawing), `app/` (state + events). What must never live in `core/`? |
| P220 | **Split monolith** — Given one `main.rs` with config + parser + runner, name three modules and what each owns. |
| P221 | **cfg test** — Explain why `mod tests` uses `#[cfg(test)]` and `use super::`*. |
| P222 | **Leaf tests** — `dsp/mod.rs` and `core/settings.rs` each have `#[cfg(test)] mod tests`. Why co-locate tests in submodule files instead of only at `lib.rs`? |
| P223 | **Capstone** — Generate `src/` tree for `sensor_core` library + `sensor_cli` binary in one workspace; I implement. |
| P224 | **Feature flag** — Add `serial` feature gating `mod serial_io` — write `Cargo.toml` `[features]` and one `#[cfg]` line. |
| P225 | **cfg vs cfg!** — Same debug log twice: `#[cfg(debug_assertions)]` block vs `if cfg!(debug_assertions)` — what stays in release binary? |
| P226 | **Optional dep** — Wire optional `tokio` behind feature `async` — show `[features]` and `dep:tokio` line. |
| P227 | **Platform module** — Sketch `#[cfg(target_os = \"linux\")] pub mod linux_home_trash;` in `core/mod.rs` — what happens on macOS builds? |
| P228 | **Platform stub fn** — Same `fn process_is_root() -> bool` on Unix (real check) and Windows (always `false`). Show the `#[cfg(unix)]` / `#[cfg(not(unix))]` pair. |
| P229 | **Empty stub fn** — `disable_spellcheck()` runs real code on macOS and `pub fn disable_spellcheck() {}` elsewhere. Why compile the module on all targets instead of gating the whole file? |
| P230 | **Target deps** — Add `libc` only on Unix in `Cargo.toml`: show `[target.'cfg(unix)'.dependencies]` block. |
| P231 | **Integration layout** — Draw tree for `tests/load_config.rs` calling public `load()` — what is invisible to the test? |
| P232 | **pub use prelude** — Users should call `my_crate::connect` not `my_crate::internal::connect` — show re-export. |
| P233 | **Module docs** — Top of `core/mod.rs` describes what the module owns. Show a `//!` inner doc comment (two lines) vs `///` on a single `pub fn`. |
| P234 | **doc hidden** — When to mark helper `#[doc(hidden)]` on a public re-export surface? |
| P235 | **Capstone crate** — Design `gateway` crate: `serial` feature, integration test, `///` on public parse fn — tree + TOML only. |

## Chapter 10 — Smart pointers and interior mutability

| ID | Prompt |
|----|--------|
| P236 | **Box why** — When is `Box<[T]>` better than `Vec<T>` on the stack? Two cases. |
| P237 | **Recursive list** — Draw memory for `Cons(1, Box::new(Cons(2, Nil)))` — stack vs heap boxes. |
| P238 | **Move out of Box** — What happens after `let s = *box_string`? When is that idiomatic vs keeping the `Box`? |
| P239 | **Trait object box** — Three plugin types implement `Plugin` — sketch `Vec<Box<dyn Plugin>>` factory; why not `Vec<Plugin>`? |
| P240 | **Rc cycle** — Explain why `Rc` cycles leak; contrast with Python reference cycles and `Weak` fix. |
| P241 | **Handle vs deep clone** — Audit snippet with `Rc<String>` and both `Rc::clone` and `(*rc).clone()` — label cost of each. |
| P242 | **Arc vs Rc** — Thread spawn with `Rc` — show error; fix with `Arc`; explain atomic count overhead in one sentence. |
| P243 | **Weak upgrade** — Parent/child with `Rc` parent and `Weak` child back-ref — I sketch types; you explain cycle break. |
| P244 | **strong_count debug** — `strong_count` stays at 2 after I thought I dropped all refs — list 5 places handles hide. |
| P245 | **RefCell trap** — Show double `borrow_mut` panic; fix with scoped borrows. |
| P246 | **Immutable then mut** — Hold `let r = cell.borrow()` and call `borrow_mut` — explain panic; fix with nested block. |
| P247 | **Cell vs RefCell** — Counter `u32` vs `Vec` cache — I pick `Cell` or `RefCell` each; you correct. |
| P248 | **Rc RefCell graph** — Two nodes share `Rc<RefCell<Node>>` — one updates field, one reads — sketch borrow rules on one thread. |
| P249 | **Compile vs runtime** — Same overlapping-mut pattern: show compile error with `&mut` and runtime panic with `RefCell` side by side. |
| P250 | **Deref coercion** — Why does `fn takes_str(s: &str)` accept `&String`, `&Box<String>`, and `&Rc<String>`? Trace steps. |
| P251 | **Drop order** — Three `Drop` structs in one function — I predict print order; you confirm reverse declaration rule. |
| P252 | **Rc last handle** — Two `Rc` clones dropped at different times — when does inner `Drop` run? Step through with println in `Drop`. |
| P253 | **Drop panic** — Explain double-panic abort if `Drop` panics during unwind — link to Ch8. |
| P254 | **Arc Mutex sketch** — Diagram thread-safe cache with `Arc<Mutex<HashMap>>` — no full code. |
| P255 | **Pick pointer** — Five scenarios (AST node, thread cache, GUI callback graph, config string, plugin list) — I pick Box/Rc/Arc/RefCell/Weak; you grade. |
| P256 | **Java heap map** — Map Java ‘everything is reference’ to Rust ownership — when `Arc`, when plain `&`, when neither. |
| P257 | **Refactor to smart ptr** — I paste struct with `Box`, `Rc`, or raw `Vec` tree — you suggest minimal smart pointer fix and justify. |
| P258 | **Leak hunt** — Sketch `Rc` cycle in observer pattern; refactor one edge to `Weak` and explain count after each drop. |

## Chapter 11 — Collections

| ID | Prompt |
|----|--------|
| P259 | **Pick collection** — Five tasks (dedup, sorted range scan, FIFO queue, index by id, min-key lookup) — I pick Vec/HashMap/BTree/VecDeque/HashSet each. |
| P260 | **Hash vs BTree** — Same 10k insert + range scan workload — when HashMap wins vs BTreeMap; one sentence each. |
| P261 | **Queue anti-pattern** — Review `while !v.is_empty() { v.remove(0) }` — cost and fix with `VecDeque`. |
| P262 | **Loop port** — Rewrite C-style indexed loop as iterator chain; preserve behavior. |
| P263 | **get vs index** — Four access patterns — I pick `[i]` vs `.get(i)` vs `.get_mut` vs `if let Some`. |
| P264 | **sort dedup** — Dedup `[3,1,4,1,5]` wrong vs right — show sort + dedup pipeline. |
| P265 | **retain vs filter** — Remove evens in-place vs new `Vec` — compare `retain` and `filter().collect()`. |
| P266 | **Borrow push trap** — Explain `let r = &v[0]; v.push(1)` error; fix with scope. |
| P267 | **entry drill** — Word frequency from `Vec<&str>` using only `.entry` — no double lookup. |
| P268 | **or_insert_with** — Lazy cache: expensive `Vec` built once per key — sketch with `or_insert_with`. |
| P269 | **HashMap merge** — Two maps of scores — merge by max per key; iterator + entry style. |
| P270 | **insert overwrite** — Track old value on port remap `502 -> 503` using `insert` return. |
| P271 | **Set ops** — Tags on two records — union, intersection, difference with `HashSet`. |
| P272 | **Stable dedup** — Unique `String` lines preserving first-seen order — no `HashSet`-only collect. |
| P273 | **BTree range** — List keys in `BTreeMap<u32, _>` between 100 and 200 inclusive. |
| P274 | **Windows** — Detect rising edges in `Vec<f64>` with `.windows(2)`; extend to `.windows(3)` for slope. |
| P275 | **chunks vs windows** — Parse byte stream into 4-byte frames — `chunks(4)` vs `windows(4)` when? |
| P276 | **collect types** — Three `collect()` calls that need type hints — fix with turbofish. |
| P277 | **Duplicate keys** — `collect` to HashMap from duplicate-key pairs — predict final map; explain overwrite rule. |
| P278 | **Performance myth** — Do Rust iterators optimize to loops? When might they not? |
| P279 | **Capacity hint** — Read 1M lines into `Vec` — when `with_capacity` matters; rough sizing rule. |
| P280 | **Capstone** — Design in-memory store: register id `u16` → last reading `f64`, need range scan by id — pick map type, list three API methods, no impl. |

## Chapter 12 — Closures and the Fn traits

| ID | Prompt |
|----|--------|
| P281 | **Fn quiz** — Four closures: I label each Fn / FnMut / FnOnce; you correct and explain capture. |
| P282 | **move drill** — Thread spawn snippet missing `move` — show compile error and fix. |
| P283 | **Iterator chain** — `.filter` closure that uses `&config` — why `Fn` not `FnMut`? |
| P284 | **RefCell bump** — Closure mutates `RefCell<u32>` counter — which Fn trait and why? |
| P285 | **fn vs closure** — When can you pass `fn()` vs `impl Fn()` to the same helper? |
| P286 | **Box dyn Fn** — Store heterogeneous callbacks in a Vec — sketch trait object version. |
| P287 | **Return closure** — Write `make_multiplier(f: f64) -> impl Fn(f64) -> f64` and explain `move`. |
| P288 | **Callback registry** — Three log filters in `Vec<Box<dyn Fn(&str) -> bool>>` — all must match signature; I add a wrong one; you fix. |
| P289 | **Loop move trap** — Building `Vec<Box<dyn Fn()>>` in a `for` loop over `String` — show move error and clone fix. |
| P290 | **Double reference** — Fix `.iter().filter(|x| ...)` type error on `Vec<String>` — show `|s|` vs `|&s|` patterns. |
| P291 | **sort_by** — Sort `Vec<(String, u32)>` by count descending with `sort_by` closure. |
| P292 | **sort_by_key** — Same sort with `sort_by_key` — when is key extraction cleaner? |
| P293 | **retain valid** — Drop invalid `SensorReading` rows in-place with `.retain` — FnMut bound. |
| P294 | **for_each vs for** — Same side-effect loop twice: `for` vs `.for_each(|...|)` — style tradeoffs. |
| P295 | **Fn bound too strict** — Helper takes `impl Fn()` but caller passes closure that mutates — fix signature. |
| P296 | **Thread move** — `spawn` closure borrows `String` — show error without `move` and fix. |
| P297 | **Send on Box Fn** — When does `Box<dyn Fn() + Send>` matter for thread pool callbacks? |
| P298 | **Capstone** — Pipeline: read lines, filter non-empty, parse `u16`, sum — all with closures; I write; you review Fn bounds. |

## Chapter 13 — Standard traits and conversions

| ID | Prompt |
|----|--------|
| P299 | **Debug vs Display** — When derive `Debug` only vs implement `Display` for a CLI status line? |
| P300 | **Redacted Debug** — Struct with `api_key: String` — sketch manual `Debug` with `[REDACTED]`. |
| P301 | **Pretty debug** — Same struct with `{:#?}` vs `{:?}` — when does pretty-print help in tests? |
| P302 | **Display impl** — Implement `Display` for `Port(u16)` showing `Port(8080)` — I write `fmt`, you review. |
| P303 | **Default enum** — Three-variant mode enum — derive `Default` with `#[default]` on `Auto`; show update syntax. |
| P304 | **Derive set quiz** — Map key, log line, sortable row, error enum — I list derives each needs. |
| P305 | **Eq on floats** — Struct with `f64` field — show `Eq` derive failure; three fixes. |
| P306 | **HashMap key** — Why does `UserId(String)` need `Eq + Hash` for `HashMap` keys? |
| P307 | **Ord sort** — Sort `Vec<MyRecord>` by timestamp field — bounds needed on `MyRecord`? |
| P308 | **From chain** — `String` → `MyLabel` via `From`; add `From<&str>` without duplicating logic. |
| P309 | **TryFrom port** — Port validation with custom enum error `OutOfRange` instead of `&str`. |
| P310 | **parse vs TryFrom** — When `s.parse::<u16>()` vs `u16::try_from(x)` vs custom `FromStr`? |
| P311 | **FromStr type** — Parse `host:port` into struct — sketch `FromStr` with split and two parse steps. |
| P312 | **Silent cast trap** — Show `70000i32 as u16` vs `TryFrom` — predict values; argue for validation. |
| P313 | **From in errors** — Wire `ParseIntError` into `AppError` with `From` so `?` works — list impl only. |
| P314 | **AsRef drill** — Rewrite three functions taking `&String` to `impl AsRef<str>`. |
| P315 | **AsRef bytes** — Function logging wire payload — signature with `impl AsRef<[u8]>`; accept `Vec`, `&[u8]`, array. |
| P316 | **Borrow lookup** — Explain `HashMap<String, V>.get(&str)` — role of `Borrow<str>`. |
| P317 | **Cow API** — Normalize slug: accept `Cow<str>`, return borrowed if already valid else owned. |
| P318 | **into_owned** — When caller needs `String` after your `Cow` helper — where call `into_owned()`? |
| P319 | **to_owned vs clone** — Three snippets: `&str`→store, `&String`→store, already-`String` — pick `to_owned`, borrow, or move; explain each. |
| P320 | **Newtype Display** — Wrap `Vec<u8>` as `HexBytes` — implement `Display` without orphan violation. |
| P321 | **Derive audit** — Config with secrets + TOML — list safe vs unsafe derives. |
| P322 | **Mini crate API** — Public `HostPort { host, port }` with `Display`, `TryFrom<&str>` for `host:port` — list impl blocks only. |
| P323 | **Capstone** — Design public API for `RateLimit { max: u32, window_secs: u64 }`: parsing, display, equality — traits only, no bodies. |

## Chapter 14 — Multithreading

| ID | Prompt |
|----|--------|
| P324 | **Race quiz** — Which snippets are data races in C++ but rejected by Rust compiler? |
| P325 | **Move fix** — Show spawn without `move` that fails; fix with `move` or `clone`. |
| P326 | **Join panic** — Worker panics; rewrite main to `match join()` and keep supervisor alive. |
| P327 | **Detached threads** — Why is `mem::forget(handle)` after spawn dangerous? Better pattern? |
| P328 | **Channel design** — Worker pool with mpsc: I describe throughput; sketch thread count + channel shape. |
| P329 | **Drop tx footgun** — Main exits before worker sends — diagram who holds `tx`/`rx`. |
| P330 | **Multiple producers** — Clone `tx` to two workers; main receives merged stream — sketch code. |
| P331 | **Bounded vs unbounded** — When does unbounded `mpsc` blow memory in automation? Bounded alternative? |
| P332 | **Mutex vs RwLock** — Read-heavy sensor cache — pick primitive and why. |
| P333 | **Hold lock briefly** — Refactor bad code that calls network I/O while holding `Mutex` lock. |
| P334 | **Deadlock sketch** — Two mutexes, lock order A then B vs B then A — show hang scenario. |
| P335 | **Poison recovery** — Thread panics holding lock; show `PoisonError` and `into_inner()` recovery. |
| P336 | **Send fix** — I try to move `Rc` into thread; show fix with `Arc`. |
| P337 | **RefCell trap** — Why `Arc<RefCell<T>>` is not `Sync`; fix pattern for shared mutation. |
| P338 | **PLC gateway layout** — Sketch poll thread + command channel + main supervisor; no code over 40 lines. |
| P339 | **Lock latency** — Modbus cycle 20 ms — max time holding mutex for register cache update? |
| P340 | **Level ladder recap** — Explain Levels 1–6 in one paragraph each for a Java teammate. |
| P341 | **RwLock cache** — Read-heavy sensor map — sketch `Arc<RwLock<HashMap>>`; when does write starve readers? |
| P342 | **Mutex vs RwLock** — Same cache with 50% writes — pick Mutex or RwLock and justify in two sentences. |
| P343 | **OnceLock init** — `get_or_init` fails second init with different value — show `OnceLock` behaviour. |
| P344 | **Scope borrow** — Parallel sum over `Vec<[u8;512]>` with `thread::scope` — why plain `spawn` fails on `&chunk`. |
| P345 | **Poison recovery** — Writer thread panics holding `RwLock` — show poisoned `read()` error and recovery options. |
| P346 | **Capstone sync** — Design: lazy config (`OnceLock`), shared cache (`RwLock`), scoped batch workers — list types only. |

## Chapter 15 — Atomics

| ID | Prompt |
|----|--------|
| P347 | **Threads vs async atomics** — Same `Arc<AtomicBool>` in `thread::spawn` vs `tokio::spawn` — what differs, what stays the same? |
| P348 | **Data race vs logic race** — Define both; classify lost `static mut` increment vs `fetch_add`. |
| P349 | **Ch14 port** — Rewrite Mutex counter (Ch14 L4) as `AtomicUsize`; when is each better? |
| P350 | **Ordering quiz** — Shutdown flag + published config pointer — pick orderings; justify. |
| P351 | **When Relaxed lies** — Metrics OK, config handoff broken — show Release/Acquire fix. |
| P352 | **Fence intuition** — Draw happens-before for Release store + Acquire load (Level 4). |
| P353 | **Relaxed polls vs version** — Why is `Relaxed` OK for Level 2 `polls` but wrong for Level 4 `version`? One sentence each. |
| P354 | **Counter port** — Cap counter with CAS loop; explain spurious `compare_exchange_weak` failure. |
| P355 | **ABA problem** — Explain ABA in 80 words for pointer `compare_exchange` — no full queue. |
| P356 | **Retry loop** — Show single-shot CAS anti-pattern; fix with loop. |
| P357 | **Double-checked locking** — Show broken lazy init with atomics; fix with `OnceLock` or explain why not hand-roll. |
| P358 | **Race quiz** — Mark 6 snippets: safe atomic, UB `static mut`, ordering bug, needs Mutex. |
| P359 | **Visibility story** — Writer updates `port` then `Relaxed` flag — what can reader see? |
| P360 | **When not** — Three cases atomics are the wrong tool; prefer channels or Mutex. |
| P361 | **Mutex vs RwLock** — Read-heavy sensor cache — atomic counter vs RwLock vs Mutex. |
| P362 | **Vec push** — Why many threads cannot `push` to shared `Vec`; three fixes. |
| P363 | **Profile-first** — Gateway uses `Mutex` for counter; profiler shows lock contention — minimal atomic refactor. |
| P364 | **Channel vs atomic flag** — When is `crossbeam`/bounded channel + shutdown better than lone `AtomicBool`? |
| P365 | **Gateway metrics** — Design `Gateway { polls, shutdown }` for async; orderings per field. |
| P366 | **False sharing** — Two hot atomics on same cache line — problem and mitigation in 60 words. |
| P367 | **Lock-free queue** — Why this chapter says don’t hand-roll; name crates/patterns instead. |

## Chapter 16 — Async and Tokio

| ID | Prompt |
|----|--------|
| P368 | **Future diagram** — Draw state machine for `async fn` with two `.await` points. |
| P369 | **Poll vs await** — Who polls the Future — caller, executor, or Tokio runtime? |
| P370 | **Executor vs thread** — One paragraph: cooperative async vs preemptive OS threads. |
| P371 | **Forgotten await** — Show unused Future bug; fix with `.await` or `spawn`. |
| P372 | **Tokio scaffold** — Minimal `#[tokio::main]` + `spawn` + join handle — explain each line. |
| P373 | **Spawn vs await** — What if `main` drops join handle without `.await`? |
| P374 | **Join handle `??`** — Explain the `??` on `handle.await` in Level 6 — outer join `Result` vs inner `run_worker` `Result`. |
| P375 | **Bad join timing** — Walk through Level 5 bad path: why does a 10 ms task wait ~100 ms when paired with `thread::sleep`? |
| P376 | **Ch15 port** — Compare Level 6 to [Ch15 L6](15_atomics_and_lockfree.md) — what changes with `tokio::spawn`, what stays the same? |
| P377 | **Supervisor timeline** — Trace Level 6 step by step: when does the worker stop, and why must it use `tokio::time::sleep` not `thread::sleep`? |
| P378 | **join vs select** — Two slow tasks: when `join!` vs `select!`? One automation example each. |
| P379 | **select! scenario** — Cancel slow request when fast path returns — full `select!` sketch. |
| P380 | **Timeout drill** — Wrap Modbus read `async fn` with 100 ms timeout; handle `Elapsed`. |
| P381 | **Cancellation hygiene** — What runs when `select!` drops the losing branch? Modbus read vs cache hit example. |
| P382 | **Blocking fix** — Audit async snippet with `thread::sleep`, `std::fs::read`; fix each. |
| P383 | **spawn_blocking** — When `tokio::fs` vs `spawn_blocking` for config file read? |
| P384 | **Tcp echo** — Expand Async I/O outline to multi-connection echo with `spawn` per accept. |
| P385 | **async vs thread** — 1000 Modbus polls — argue async vs thread pool for latency. |
| P386 | **200 TCP connections** — One process — async vs thread-per-connection memory story. |
| P387 | **When thread enough** — Single serial port poll — justify thread loop over Tokio. |
| P388 | **Async sleep in worker** — Why must Level 6’s poll loop use `tokio::time::sleep().await` on every iteration, not `thread::sleep`? |
| P389 | **tokio Mutex** — Rewrite bad `std::sync::MutexGuard` across await with `tokio::sync::Mutex`. |

## Chapter 17 — Metaprogramming

| ID | Prompt |
|----|--------|
| P390 | **Expansion order** — List compiler phases from tokens to LLVM; where do macros run? |
| P391 | **Tokens vs types** — Why can a macro compile but expanded code fail? One example. |
| P392 | **Follow-set why** — Explain in 80 words why `$a:expr = $b:expr` is forbidden in matchers. |
| P393 | **Scope honesty** — List 5 metaprogramming topics Ch17 skips and where to learn each. |
| P394 | **Trace checklist** — Give 6 reasons macro code is hard to trace and one mitigation each. |
| P395 | **Macro vs fn** — Rewrite macro as generic fn if possible; when impossible, say why. |
| P396 | **Fragment picker** — I describe a DSL shape; you pick `expr`/`ident`/`tt` for each slot. |
| P397 | **Expr equals fix** — Fix `set_reg!($addr = $val)` matcher; show two valid surface syntaxes. |
| P398 | **For after expr** — Why is `$e:expr for $i:ident in $r:expr` illegal? Show legal `$p:pat in $r:expr` foreach macro. |
| P399 | **Double-brace fix** — Fix `poll_twice!` macro that mixes `let` and `for` for use in `let x = poll_twice!()`. |
| P400 | **Trailing comma** — Explain `$(x),*` vs `$(x),+` on empty input; show failing and fixed macro. |
| P401 | **Hygiene** — Explain macro hygiene in 60 words with `$crate` mention. |
| P402 | **Register DSL** — Extend `register_map!` with a third register; explain `stringify!` arm. |
| P403 | **Debug expand** — Walk me through `cargo expand` on derive Debug output (conceptual). |
| P404 | **Clone expand** — Show conceptual expanded `impl Clone` for struct with two `i32` fields. |
| P405 | **Enum vs struct** — How does derived `PartialEq` differ for enum vs struct? Sketch match arms. |
| P406 | **Field bound failure** — Struct with `Mutex<i32>` field + `#[derive(Clone)]` — quote error and fix. |
| P407 | **Redacted Debug** — When hand-write `Debug` instead of derive on command enum with secrets. |
| P408 | **Copy vs Clone quiz** — Classify 8 types: Copy, Clone only, or neither; justify. |
| P409 | **Hot-loop clone audit** — Audit poll loop with `.clone()` each tick; suggest move/`Arc`/borrow. |
| P410 | **Arc vs derive Clone** — Explain cheap `Arc` clone vs deep `String` clone with one snippet. |
| P411 | **derive need** — List derives I want for config struct loaded from TOML — justify each. |
| P412 | **Serde rename** — Field `poll_ms` in JSON as `pollIntervalMs` — show attr; trap on refactor. |
| P413 | **thiserror vs manual** — Same error enum: count lines derive vs hand-written (Ch8 style). |
| P414 | **Float Eq trap** — Show `#[derive(Eq)]` on `f64` field failure; two fixes from Ch7. |
| P415 | **cargo expand walkthrough** — Step-by-step: install, run, read output for one derive. |
| P416 | **In expansion of** — Decode a 3-note compiler error chain from nested macro + derive. |
| P417 | **tt vs expr escape** — When switch matcher from `expr` to `tt`; tradeoffs in 80 words. |
| P418 | **Three-layer trace** — Derive inside attribute inside macro_rules — how to debug layer by layer. |
| P419 | **Port to const fn** — Replace tiny numeric macro with `const fn`; when macro still needed? |
| P420 | **env vs var** — Compare `env!`, `option_env!`, `std::env::var` — table with one use case each. |
| P421 | **include_str config** — Embed default TOML with `include_str!`; deserialize at startup sketch. |
| P422 | **Macro vs fn audit** — Mark 6 snippets: should be macro, derive, or plain fn — justify. |
| P423 | **When not proc macro** — Three scenarios where proc macro is overkill; alternative each. |
| P424 | **Minimal derive set** — Gateway config + error + CLI: smallest derive list that still ships. |
| P425 | **Trap quiz** — Mark 8 snippets: empty `+`, double brace, Default enum, Arc Clone, env!, duplicate impl, cfg macro, serde rename. |
| P426 | **Duplicate register DSL** — Design compile-time error for duplicate keys in register_map! |
| P427 | **Serde refactor test** — Integration test plan after renaming TOML field with serde attrs. |
| P428 | **Derive vs decorator** — I come from Python/Java. Explain why `#[derive(Debug)]` is not a decorator or annotation that runs at call time — what actually happens at compile time? |
| P429 | **Three kinds quiz** — Classify 8 snippets: derive proc macro, attribute proc macro, function-like macro, compiler attribute (`#[inline]`, `#[cfg]`), field meta parsed inside a derive (`#[serde(rename)]`). |
| P430 | **`tokio::main` expand** — Sketch the conceptual expansion of `#[tokio::main] async fn main() { ... }`. Why is this an attribute proc macro, not `#[derive]`? |
| P431 | **Custom attribute inputs** — For `#[my_attr(some = \"config\")] fn poll() { ... }`, what two token streams does the proc macro receive? Give two things the macro might emit. |
| P432 | **Field attr vs item attr** — Contrast `#[serde(rename = \"pollIntervalMs\")]` on a struct field vs `#[tracing::instrument]` on `fn poll` — same proc-macro kind or not? |
| P433 | **Custom attribute when** — Three scenarios (poll-loop tracing, type-level JSON mapping, wrapping `main` with a runtime). Pick: custom attribute proc macro, derive, or plain helper fn — justify each. |
| P434 | **Runtime myth** — Does `#[tracing::instrument]` or `#[test]` run every time I call the function? Explain what runs at compile time vs run time. |
| P435 | **DSL sketch** — Design tiny `command!` macro for CLI subcommands — tokens only. |
| P436 | **Modbus table macro** — Spec register table macro generating lookup + const max address. |
| P437 | **Derive soup review** — I paste 40-line struct with 12 derives; trim to minimal set with reasons. |

## Chapter 18 — Unsafe

| ID | Prompt |
|----|--------|
| P438 | **Invariant list** — For raw pointer to buffer + length, list 5 invariants a safe wrapper must enforce. |
| P439 | **Soundness** — Explain ‘safe Rust can’t cause UB’ vs `unsafe` — one paragraph; include unsound safe wrapper example. |
| P440 | **Scope honesty** — List 6 topics Ch18 skips and where to learn each (nomicon, Miri, Pin, …). |
| P441 | **Aim table** — Fill: why Vec needs `unsafe` internally while `push` stays safe for callers. |
| P442 | **Promise diagram** — Draw safe API → unsafe block → invariants → caller cannot UB; label soundness. |
| P443 | **`*const` vs `&T` quiz** — Give 5 snippets: legal ref, needs `unsafe` block, compile error; I classify each. |
| P444 | **from_raw_parts design** — Design `fn view_frame(ptr, len) -> Result<&[u8], Error>` without `&'static`; list invariants. |
| P445 | **Dangling audit** — Show stack pointer used after drop; I explain UB; you show Miri-style symptom. |
| P446 | **Modbus buffer** — Register table as `&[u8]` vs `from_raw_parts` — when is each idiomatic in a gateway? |
| P447 | **Hex preview port** — Port Level 2 `as_hex_preview` to return `Result` on empty buffer; no `unwrap`. |
| P448 | **set_len contract** — Document pre/post conditions for `set_len_unchecked`; what breaks `as_slice` if violated? |
| P449 | **Send proof** — I claim `Rc<*mut u8>` is Send; you disprove with compiler error quote. |
| P450 | **SerialHandle Sync** — When would `SerialHandle` need `unsafe impl Sync` vs `Arc<Mutex<...>>`? Two sentences each. |
| P451 | **Ch14 port** — Rewrite Level 4 spawn example using only safe types — when is it impossible? |
| P452 | **Proc-macro boundary** — Why do serde/tokio crates use `unsafe impl` you don’t write? Link Ch17. |
| P453 | **FFI checklist** — Checklist for calling a C Modbus library from a Rust binary; include panic and ownership rows. |
| P454 | **CString trap** — Show `into_raw` forgotten `from_raw` leak; fix with RAII pattern sketch. |
| P455 | **Vendor SDK** — Diagram ownership: Rust owns config, C owns connection, callback pointer — boxes and arrows only. |
| P456 | **serialport hide** — Where does `unsafe` live in a typical serial crate vs my application code? |
| P457 | **CRC decision** — C `crc16` vs Rust `crc` crate vs hand-rolled — decision tree for production gateway. |
| P458 | **Java JNI** — Compare JNI pitfalls (refs, exceptions, pinning) to Rust FFI ownership rules. |
| P459 | **Miri** — What is Miri and when should I run it relative to `unsafe` changes? Include one command. |
| P460 | **Trap quiz** — Mark 6 snippets: safe, UB, ordering bug, needs Miri, needs Mutex, unsound safe API. |
| P461 | **Review rubric** — 10-point code-review checklist for an `unsafe` PR in an automation repo. |
| P462 | **Test plan** — Unit + Miri + integration tests for new `extern 'C'` wrapper — bullet list only. |
| P463 | **Avoid** — Review use case: speed up JSON — `unsafe` vs `simd-json` vs algorithm; pick with justification. |
| P464 | **Borrow checker fight** — I paste fight-the-borrow-checker code; you refactor to safe Rust without `unsafe`. |
| P465 | **static mut** — Compare `static mut` counter vs `AtomicUsize` from Ch15 — UB vs defined behavior. |
| P466 | **PlcDriver API** — Design safe `PlcDriver` Rust API over fictional `extern 'C'` — types, `Result`, no raw pointers in public API. |
| P467 | **Level ladder recap** — Explain Levels 1–5 in one paragraph each for a Java teammate who knows JNI. |

## Chapter 19 — I/O and processes

| ID | Prompt |
|----|--------|
| P468 | **Trait refactor** — Refactor file copy loop to generic `copy<R: Read, W: Write>`; discuss error propagation. |
| P469 | **read vs read_exact** — Give 3 protocol shapes; I pick `read` loop vs `read_exact` each time; you verify. |
| P470 | **Cursor test** — Write unit test for `parse_kv_lines` using `Cursor` — no filesystem. |
| P471 | **BufReader why** — Explain in 60 words why `BufReader` matters for 10k-line log files. |
| P472 | **CSV tool** — Spec for CLI: read two-column CSV, emit `name=value`; I implement with BufRead. |
| P473 | **Boundary errors** — Sketch `main` mapping `io::Error` to exit code 1 with context path — no `unwrap`. |
| P474 | **read_to_string trap** — When is `read_to_string` wrong for automation configs? Give size threshold rule. |
| P475 | **Packet layout** — Add CRC byte to 4-byte packet; update encode/decode with XOR — show tests. |
| P476 | **Endian trap** — Quiz: 3 scenarios pick LE vs BE for Modbus-style register. |
| P477 | **Bit field port** — Java status int with flags — port to Rust `encode` with `|=` and `&` masks. |
| P478 | **CRC upgrade** — Replace XOR toy CRC with CRC-16-Modbus — outline steps, no full crate required. |
| P479 | **Command safety** — Review shell=True style command; rewrite without shell when possible. |
| P480 | **Pipeline** — Design `program A | program B` using only Rust std (two processes, pipe). |
| P481 | **Exit status** — Child exited 2 — how should gateway log and retry? Table: fatal vs transient. |
| P482 | **Env and cwd** — Show `Command` with `.env("PORT","502")` and `.current_dir` — when needed? |
| P483 | **Sync vs async pick** — Three gateway designs: I pick sync thread vs Tokio per scenario; you justify. |
| P484 | **Serial debug** — I get timeout on read; give systematic checklist (baud, cable, permissions). |
| P485 | **serialport traits** — Explain how `serialport` maps to `Read`/`Write` — diagram only. |
| P486 | **Blocking in async** — Show wrong `std::fs::read` inside `async fn`; fix with `tokio::fs` or `spawn_blocking`. |
| P487 | **Capstone scaffold** — Generate module tree and function signatures for sensor_gateway; no bodies. |
| P488 | **Retry policy** — Design exponential backoff for Modbus-style poll errors; Rust pseudocode. |
| P489 | **Log schema** — Propose JSON log lines for sensor events with timestamp and error codes. |
| P490 | **GPIO next step** — After serial works on Pi, outline migration to gpio-cdev for one LED. |
| P491 | **Code review** — I paste capstone main loop; review for panic risks and missing flush. |
| P492 | **Gateway capstone** — End-to-end: config file → serial poll → JSON log line — module list and error types only. |
| P493 | **Ch16 bridge** — Same echo server: sketch sync thread version vs Tokio version — tradeoffs table. |
| P494 | **Path join** — Build config path from `HOME` + `.config/app.toml` — show `Path::join` vs string concat trap. |
| P495 | **Env default** — `TIMEOUT` env var with default 30 — `var().unwrap_or_else` pattern. |
| P496 | **Metadata guard** — Reject config files over 1MB before `read_to_string` — sketch `metadata().len()` check. |
| P497 | **Stdin fallback** — Port from argv or interactive prompt — one `main` with both paths. |
| P498 | **Line parser** — BufRead lines, skip `#`, parse `key=value` — handle trailing newline. |
| P499 | **Capstone CLI** — End-to-end: env path → metadata check → line parse → print port — function list only. |

## Chapter 20 — Production standards

| ID | Prompt |
|----|--------|
| P500 | **Diff review** — Paste a 30-line Rust PR; I mark each checklist row pass/fail with one sentence. |
| P501 | **Mega-error refactor** — Split one `AppError` into `ValidateError` + `StorageError`; show boundary mapping. |
| P502 | **Panic hunt** — Find five panic sources in a snippet; replace with `Option`/`Result`. |
| P503 | **Clone audit** — Remove three unnecessary clones by fixing signatures to `&str` / `&[T]`. |
| P504 | **Arc style** — Rewrite `arc.clone()` call sites to `Arc::clone(&arc)`; explain review benefit. |
| P505 | **Workspace.toml** — Sketch root + two members; all shared deps use `{ workspace = true }`. |
| P506 | **Golden test** — Parser output: one `assert_eq!(got, want)` + `pretty_assertions`; no per-field asserts. |
| P507 | **Flaky test fix** — Replace `thread::sleep` in test with injected `Clock` trait. |
| P508 | **Pre-merge gate** — Checklist-only review of gateway `main.rs` + `lib.rs` — findings only. |
| P509 | **AI review prompt** — One paragraph prompt for an assistant to verify the Ch20 rules on a Rust diff. |
---

**Total: 509 prompts** (P001–P509).

## By theme

| Theme | IDs |
|-------|-----|
| Preface / study plan | P001–P005 |
| Ownership / borrow (Ch 1) | P006–P038 |
| Types / expressions (Ch 2) | P039–P054 |
| Functions / methods (Ch 3) | P055–P072 |
| Iterators (Ch 4) | P073–P100 |
| Lifetimes (Ch 5) | P101–P112 |
| Enums / match (Ch 6) | P113–P140 |
| Structs / traits (Ch 7) | P141–P176 |
| Errors / testing (Ch 8) | P177–P199 |
| Modules / crates (Ch 9) | P200–P235 |
| Smart pointers (Ch 10) | P236–P258 |
| Collections (Ch 11) | P259–P280 |
| Closures (Ch 12) | P281–P298 |
| Standard traits (Ch 13) | P299–P323 |
| Multithreading (Ch 14) | P324–P346 |
| Atomics (Ch 15) | P347–P367 |
| Async / Tokio (Ch 16) | P368–P389 |
| Metaprogramming (Ch 17) | P390–P437 |
| Unsafe (Ch 18) | P438–P467 |
| I/O and processes (Ch 19) | P468–P499 |
| Production standards (Ch 20) | P500–P509 |

## See also

- [PLAYGROUND_GUIDE.md](PLAYGROUND_GUIDE.md)
- [JAVA_PYTHON_RUST_MAP.md](JAVA_PYTHON_RUST_MAP.md)
- [CONTENTS.md](../CONTENTS.md)
