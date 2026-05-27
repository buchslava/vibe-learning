# Chapter 6: Enums and Pattern Matching

## Hook

**Java** uses `null` for “missing” and often throws exceptions for failures. **Python** uses `None` and exceptions, or informal unions (`int | str`) without the type checker enforcing branches. If you know other languages, map their “missing value” and error habits to the table below.

**Rust** encodes “one of several possibilities” in the **type system**:

| Idea | Java | Python | Rust |
|------|------|--------|------|
| Missing value | `null` | `None` | `Option<T>` |
| Failure | exception | exception | `Result<T, E>` |
| Several shapes | class hierarchy | duck typing / Union | `enum` + `match` |
| Must handle all cases | runtime surprise | runtime surprise | **compile error** if `match` is incomplete |

`match` is not a fancy `switch`. It **destructures** data, and the compiler insists you cover every variant. That is how Rust replaces null checks and many runtime type errors with reviewable compile-time rules ([Chapter 1](01_paradigm_shift.md#idiom-spotlight)).

[Chapter 2](02_types.md) previewed `match` on integers. This chapter applies the same machinery to `Option`, `Result`, and your own `enum`s.

## `Option<T>` — no null

`Option` is an enum with two variants: `Some(T)` or `None`.

```rust
// Playground
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn main() {
    match divide(10.0, 2.0) {
        Some(v) => println!("{}", v),
        None => println!("undefined"),
    }
}
```

**Callers must decide** what `None` means — log, default, propagate, or abort. Rust will not let you treat `Option` like a plain `T` without handling `None` (unless you explicitly force it — see edge cases below).

### `if let`, `while let`, and `let ... else`

When only **one** variant matters:

```rust
// Playground
fn main() {
    let maybe_port = Some(502u16);
    if let Some(p) = maybe_port {
        println!("port {}", p);
    }

    let raw = "8080";
    let Ok(n) = raw.parse::<u16>() else {
        println!("bad port");
        return;
    };
    println!("parsed {}", n);
}
```

`let ... else` ([Rust 1.65+](https://doc.rust-lang.org/edition-guide/rust-2021/let-else.html)) is idiomatic for “parse or bail early” without nested `match`.

### Option combinators (before heavy `match`)

| Method | Use when |
|--------|----------|
| `.map(f)` | transform inner value if `Some` |
| `.and_then(f)` | chain fallible steps (`f` returns `Option`) |
| `.unwrap_or(default)` | supply fallback |
| `.ok_or(err)` | turn `None` into `Err` for `Result` chains |

```rust
// Playground
fn main() {
    let s = "502";
    let port = s.trim().parse::<u16>().ok(); // Option<u16>
    let doubled = port.map(|p| p * 2);
    println!("{:?}", doubled);
}
```

Full error typing for `Result` chains: [Chapter 8](08_errors_and_testing.md).

### Option edge cases and compiler traps

**Wrong — treat `Option` like nullable Java without branching:**

```rust
// Playground — does not compile
fn main() {
    let maybe: Option<i32> = Some(5);
    // let x: i32 = maybe; // ERROR: expected i32, found Option<i32>
    let x = maybe.unwrap(); // compiles — panics on None at runtime
}
```

| Approach | `None` behavior | Idiom |
|----------|-----------------|-------|
| `match` / `if let` | you handle it | **preferred** |
| `unwrap()` / `expect("...")` | **panic** | prototypes, tests, “impossible” invariants |
| `unwrap_or` / `unwrap_or_else` | default | config with safe fallback |
| `?` in `fn -> Option` | propagate `None` | chaining parsers |

**Wrong — `unwrap` in production paths:**

```rust
// Playground — runs, then may panic in production
fn read_port(s: &str) -> u16 {
    s.parse().ok().unwrap() // panic if config line is wrong
}
```

Prefer `match`, `if let`, or `Result` with `?` so a bad config file returns an error instead of crashing a long-running service.

**Exhaustive `match` on `Option`:**

```rust
// Playground — does not compile if you omit an arm
fn main() {
    let x = Some(1);
    match x {
        Some(n) => println!("{}", n),
        // None => {}  // ERROR: non-exhaustive patterns: `None` not covered
    }
}
```

## `Result<T, E>` — errors as values

`Result` is also a two-variant enum: `Ok(T)` or `Err(E)`.

```rust
// Playground
fn parse_positive(s: &str) -> Result<i32, &'static str> {
    let n: i32 = s.parse().map_err(|_| "not a number")?;
    if n > 0 { Ok(n) } else { Err("not positive") }
}

fn main() {
    println!("{:?}", parse_positive("42"));
    println!("{:?}", parse_positive("-1"));
    println!("{:?}", parse_positive("oops"));
}
```

The `?` operator propagates `Err` early — like `throw`, but **typed** and visible in the signature. Custom error types, **`thiserror`**, and [errors as enums](08_errors_and_testing.md#errors-and-enums): [Chapter 8](08_errors_and_testing.md).

### Result edge cases

**Wrong — ignore the error arm:**

```rust
// Playground — does not compile
fn main() {
    let r: Result<i32, &str> = Ok(1);
    match r {
        Ok(n) => println!("{}", n),
        // Err(e) => ...  // ERROR: non-exhaustive patterns
    }
}
```

**Wrong — `unwrap` on `Result` in library code:**

```rust
// Playground
fn load_timeout(s: &str) -> u32 {
    s.parse().unwrap() // panic on "abc" — use ? or match instead
}
```

**Idiomatic boundary pattern:**

```rust
// Playground
fn parse_port(s: &str) -> Result<u16, String> {
    s.trim().parse::<u16>().map_err(|e| format!("port: {}", e))
}

fn main() {
    match parse_port("502") {
        Ok(p) => println!("ok {}", p),
        Err(msg) => eprintln!("config error: {}", msg),
    }
}
```

At **boundaries** (`main`, HTTP handler, CLI), match on `Result` and map to user-facing messages. **Inside** helpers, prefer `?` to bubble `Err` up.

## Custom enums — sum types

Your own `enum` lists allowed states or message kinds. Each variant can be a **unit**, **tuple**, or **struct** shape.

```rust
// Playground
enum Status {
    Idle,
    Running { rpm: u32 },
    Fault { code: u8 },
}

fn describe(s: &Status) -> String {
    match s {
        Status::Idle => "idle".into(),
        Status::Running { rpm } => format!("running at {}", rpm),
        Status::Fault { code } => format!("fault {}", code),
    }
}

fn main() {
    let s = Status::Running { rpm: 1500 };
    println!("{}", describe(&s));
}
```

### Enum shapes (pattern matching preview)

| Variant syntax | Example | Pattern in `match` |
|----------------|---------|----------------------|
| Unit | `Idle` | `Status::Idle` |
| Tuple | `Ping(u32)` | `Message::Ping(ms)` |
| Struct | `Fault { code }` | `Status::Fault { code }` |

```rust
// Playground
enum Frame {
    Heartbeat,
    Data { len: u16, payload: [u8; 4] },
}

fn opcode(f: &Frame) -> u8 {
    match f {
        Frame::Heartbeat => 0x00,
        Frame::Data { len, .. } => {
            if *len > 0 { 0x01 } else { 0xFF }
        }
    }
}

fn main() {
    let f = Frame::Data { len: 2, payload: [1, 2, 0, 0] };
    println!("op={:02X}", opcode(&f));
}
```

`..` in a pattern ignores fields you do not need ([Chapter 2](02_types.md) ranges use the same token for a different feature — context disambiguates).

## Pattern matching mechanics

### Exhaustive `match`

For an `enum`, **every variant** needs an arm (or a catch-all that is intentionally broad).

```rust
// Playground
enum Mode { Auto, Manual }

fn label(m: Mode) -> &'static str {
    match m {
        Mode::Auto => "auto",
        Mode::Manual => "manual",
    }
}

fn main() {
    println!("{}", label(Mode::Auto));
}
```

#### Why `-> &'static str`?

The return type is a **borrowed string slice** (`&str`), not an owned `String`. Each match arm returns a **string literal** — `"auto"`, `"manual"` — which is stored in the program binary and lives for the **entire run**. Rust types that as `&'static str`: a reference valid for the **`'static`** lifetime (see [Chapter 5 — Lifetimes](05_lifetimes.md#what-lifetimes-are-for)).

| Return expression | Lifetime meaning | Who must stay alive? |
|-----------------|------------------|----------------------|
| `"auto"` (literal) | `'static` | nobody — bytes are in the binary |
| `s.as_str()` from caller’s `String` | usually `'a` tied to `s` | the owner `String` |
| `format!(...)` | cannot return `&str` at all | need owned `String` |

So `label` does **not** borrow from `m` or from any local variable. It only returns pointers to immortal literals. That is why the signature can be `&'static str` without a generic `'a` parameter.

**Wrong — return `&str` pointing at a local `String`:**

```rust
// Playground — does not compile
fn bad_label(m: Mode) -> &str {
    let s = format!("{:?}", m); // owned String on stack
    s.as_str() // ERROR: `s` does not live long enough — returned ref would dangle
}
```

**Idiomatic alternatives** when text is built at run time:

```rust
// Playground
enum Mode { Auto, Manual }

fn label_owned(m: Mode) -> String {
    match m {
        Mode::Auto => "auto".to_string(),
        Mode::Manual => "manual".to_string(),
    }
}

fn main() {
    println!("{}", label_owned(Mode::Auto));
}
```

Use `&'static str` for fixed catalog strings; use `String` (or `&str` with an explicit lifetime tied to an input) when the text depends on borrowed or formatted data — [Chapter 5](05_lifetimes.md) covers the contract, [Chapter 2](02_types.md) covers `String` vs `&str`.

**Add a variant — compiler forces updates:**

```rust
// Playground — does not compile until you add `Mode::Safe => ...`
enum Mode { Auto, Manual, Safe }

fn label(m: Mode) -> &'static str {
    match m {
        Mode::Auto => "auto",
        Mode::Manual => "manual",
    } // ERROR: non-exhaustive patterns: `Safe` not covered
}
```

After you add `Mode::Safe => "safe"`, the match compiles again — that is the safety net when protocols or state machines grow.

That is a **feature**: refactors cannot silently forget a new PLC mode or protocol opcode.

### Wildcard `_` — power and footgun

On integers, `_` means “everything else.” On enums, `_` means “any variant I did not list”:

```rust
// Playground
fn severity(code: u16) -> &'static str {
    match code {
        0..=99 => "info",
        _ => "unknown", // catches 100+ — fine for integers
    }
}
```

**Footgun on enums:** if you write `_` instead of listing variants, **adding a new variant will not force you to update this `match`**. The compiler assumes you meant to lump new cases into `_`. Prefer explicit arms for domain enums you control. Reserve `_` for “truly don’t care” cases or integer ranges.

### `match` on references — borrow vs move

```rust
// Playground
enum Status {
    Idle,
    Running { rpm: u32 },
    Fault { code: u8 },
}

fn describe(s: &Status) -> String {
    match s {
        Status::Idle => "idle".into(),
        Status::Running { rpm } => format!("running at {}", rpm),
        Status::Fault { code } => format!("fault {}", code),
    }
}

fn main() {
    let s = Status::Running { rpm: 100 };
    match &s {
        Status::Running { rpm } => println!("rpm {}", rpm),
        _ => println!("other"),
    }
    println!("still own s: {}", describe(&s));
}
```

`match &s` borrows; `match s` **moves** `s` into the match (often wrong unless you intentionally consume).

**Wrong — move field out of non-`Copy` enum:**

```rust
// Playground — does not compile
enum Packet { Text(String) }

fn main() {
    let p = Packet::Text(String::from("hi"));
    match p {
        Packet::Text(s) => println!("{}", s), // moves s out of p
    }
    // use p again // ERROR: use of partially moved value: `p`
}
```

Use `ref` in the pattern to borrow inside fields: `Packet::Text(ref s)` or `match &p`.

### Match guards

Add a boolean condition on an arm:

```rust
// Playground
fn classify(v: i32) -> &'static str {
    match v {
        n if n < 0 => "negative",
        0 => "zero",
        n if n > 100 => "high",
        _ => "normal",
    }
}

fn main() {
    println!("{}", classify(150));
}
```

Guards do not replace exhaustiveness — you still need arms that cover all cases (often with `_`).

### `if let` vs `match`

| Use `if let` / `while let` | Use `match` |
|---------------------------|-------------|
| one variant matters | several variants or mixed handling |
| quick peel `Some` / `Ok` | equal weight per branch |
| `while let Some(line) = iter.next()` | full `Result` + `Err` mapping |

Long chains of `if let` on the same value are a smell — switch to `match` for clarity.

## Java / Python contrast (deeper)

| Pitfall | Java / Python habit | Rust idiom |
|---------|---------------------|------------|
| Forgot null check | NPE / `AttributeError` at runtime | `Option` + `match` / `?` |
| Swallowed exception | empty `catch` | `match` on `Err` or explicit `?` |
| Stringly-typed states | `"IDLE"`, `"RUN"` | `enum Mode { Idle, Run }` |
| Unchecked union | `isinstance` ladders | one `match` on `enum` |

**Python `match` (3.10+)** looks like Rust but matches object structure; Rust `match` ties to **algebraic types** and ownership.

## Service-shaped example

```rust
// Playground
enum ReadOutcome {
    Value(u16),
    Timeout,
    CrcFault,
}

fn handle(r: ReadOutcome) {
    match r {
        ReadOutcome::Value(v) => println!("register = {}", v),
        ReadOutcome::Timeout => eprintln!("retry"),
        ReadOutcome::CrcFault => eprintln!("bus fault"),
    }
}

fn main() {
    handle(ReadOutcome::Value(42));
    handle(ReadOutcome::CrcFault);
}
```

Adding `ReadOutcome::Disconnected` later without updating `handle` → **compile error**, not a silent log gap.

## Advanced patterns

Patterns for parsers and config loaders — same domain as the service example above.

### Slice patterns — split a command line

Parse `"GET /path HTTP/1.1"` into verb and path without index math:

```rust
// Playground
fn parse_request(line: &str) -> Option<(&str, &str, &str)> {
    match line.split_whitespace().collect::<Vec<_>>().as_slice() {
        [method, path, version] => Some((*method, *path, *version)),
        _ => None,
    }
}

fn main() {
    let line = "GET /api/status HTTP/1.1";
    if let Some((m, p, v)) = parse_request(line) {
        println!("{} {} {}", m, p, v);
    }
}
```

For paths with spaces, collect into an owned `String` or match `[method, rest @ .., version]` and join `rest` into a local `String` before returning.

### `matches!` — enum check without full `match`

**Before — verbose `match`:**

```rust
// Playground
#[derive(Debug)]
enum Mode { Auto, Manual, Off }

fn is_active(mode: &Mode) -> bool {
    match mode {
        Mode::Auto | Mode::Manual => true,
        Mode::Off => false,
    }
}
```

**After — `matches!` macro:**

```rust
// Playground
#[derive(Debug)]
enum Mode { Auto, Manual, Off }

fn is_active(mode: &Mode) -> bool {
    matches!(mode, Mode::Auto | Mode::Manual)
}

fn main() {
    println!("{}", is_active(&Mode::Auto));
}
```

Use `matches!` for boolean guards. Keep full `match` when arms return different values.

### `if let` chains — bind host and port together

Parse `host:port` from a config line:

```rust
// Playground
fn parse_endpoint(line: &str) -> Option<(&str, u16)> {
    let mut parts = line.split(':');
    let host = parts.next()?.trim();
    if host.is_empty() {
        return None;
    }
    let port_str = parts.next()?;
    if parts.next().is_some() {
        return None; // reject host:port:extra
    }
    let port: u16 = port_str.parse().ok()?;
    if port == 0 {
        return None;
    }
    Some((host, port))
}

fn main() {
    println!("{:?}", parse_endpoint("127.0.0.1:502"));
}
```

Each condition must pass for the binding to succeed — good for multi-step parsing without nested `match`.

### `@` bindings — match and bind in one arm

Accept only valid port numbers while keeping the value:

```rust
// Playground
fn classify_port(p: u16) -> &'static str {
    match p {
        n @ 1..=1023 => "system",
        n @ 1024..=49151 => "registered",
        _ => "other",
    }
}

fn main() {
    println!("{}", classify_port(502));
}
```

### Advanced pattern edge cases

**Wrong — fixed-length slice on variable input:**

```rust
// Playground — logic bug, not compile error
fn main() {
    let words: Vec<&str> = vec!["GET", "/only"];
    match words.as_slice() {
        [method, path] => println!("{} {}", method, path),
        _ => println!("bad request"),
    }
}
```

Three or one token hits `_` — use `[method, path @ ..]` or check length explicitly.

**Empty slice:**

```rust
// Playground
fn main() {
    let empty: &[&str] = &[];
    match empty {
        [] => println!("empty"),
        [first, ..] => println!("first={}", first),
    }
}
```

## When the compiler says no

Common errors in this chapter:

| Error (typical) | Cause | Fix |
|-----------------|-------|-----|
| non-exhaustive patterns | missing enum variant or `None`/`Err` | add arm or intentional `_` |
| expected `T`, found `Option<T>` | treated `Option` as value | `match` / `if let` / `?` |
| use of partially moved value | field moved out in `match` | `match &e`, `ref` in pattern, or clone |
| cannot move out of … behind a reference | `match` on `&` but pattern moves | `ref` / `ref mut` in pattern |
| unreachable pattern | arm duplicated or shadowed by `_` above | reorder arms; remove redundant `_` |
| `?` in `fn` without `Result` return | `?` only propagates in `Result`/`Option` fns | change return type or use `match` |

## Idiom spotlight

> **`match` on `Result` at boundaries, `?` inside.** At `main` or API edge, convert to user-facing messages; inside libraries, propagate with `?`.
>
> **Prefer explicit enum variants over strings** for device and protocol states. Let the compiler remind you when the protocol adds a code.
>
> **Avoid `unwrap` in long-running services** — use `Option`/`Result` so bad input is data, not a panic.

## Go deeper

- [The Rust Book — Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [The Rust Book — Pattern matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)
- [Option basics](https://hightechmind.io/rust/) — examples 041–044
- [Result basics](https://hightechmind.io/rust/) — examples 045–048

## See also

- [Chapter 2: Types](02_types.md) — integer `match`, `if` expressions, `String` vs `&str`
- [Chapter 5: Lifetimes](05_lifetimes.md) — `'static`, returning `&str`, why `bad_label` fails
- [Chapter 4: Iterators](04_iterators.md) — `find` returns `Option`
- [Chapter 8: Errors](08_errors_and_testing.md) — `?`, custom `Error`, testing `Result`
- [Chapter 7: Traits](07_structs_traits_generics.md) — traits on enums, `impl` blocks
- [Chapter 17: Metaprogramming](17_metaprogramming.md) — `matches!` macro

### Afterparty

#### `Option` and null habits

1. **Null replacement** — “Translate 5 Java methods returning null into `Option` Rust; explain callsite changes.”
2. **unwrap audit** — “Paste 20 lines with 4 `unwrap()` calls; I mark panic risk; you rewrite with `match` or `?`.”
3. **Combinator chain** — “Parse port `Option<u16>`, double if Some, default 502 — I write `map`/`unwrap_or`; you add `and_then` version.”
4. **let-else port** — “Rewrite nested `match` on `parse()` into `let Ok(x) = ... else { ... }`; preserve behavior.”

#### `Result` and propagation

5. **Result railway** — “Chain parse → validate → compute with `?`; I fill blanks, you verify.”
6. **Err arm missing** — “Show `match` on `Result` without `Err` arm; I quote error and fix; add boundary `eprintln!` pattern.”
7. **unwrap vs ?** — “Same parser twice: `unwrap` vs `fn -> Result` with `?`; compare panic risk and signature honesty.”

#### Enums and exhaustive `match`

8. **Exhaustive match** — “Enum with 4 variants; I write `match`; you add variant `Safe` and show compile error until I fix.”
9. **Wildcard footgun** — “Explain why `_` on your own enum hides new variants; show explicit arms vs `_` refactor story.”
10. **Partial move** — “`enum` with `String` field: `match` moves field; I fix with `ref` or `match &e`; you show error text.”
11. **State machine** — “TCP/serial states as enum; `connect`/`send`/`close` return `Result<(), IllegalTransition>`.”
12. **Python Union** — “Python `int | str` parameter → Rust enum + `match`; no `dyn`.”

#### Patterns and style

13. **if let vs match** — “Three tasks: one variant, two variants, five variants — I pick `if let` or `match` each time.”
14. **Match on ref** — “Given owned `Status`, I choose `match s` vs `match &s`; predict move errors; you diagram ownership.”
15. **Guard drill** — “Classify `i32` with guards (`<0`, `==0`, `>100`); I write arms; you check exhaustiveness.”
16. **Opcode table** — “Design `enum` for 3 frame types + `match` returning `u8` opcode; add fourth type as compile-break exercise.”

#### Errors and automation

17. **ReadOutcome extend** — “Add `Disconnected` to automation `ReadOutcome`; show all `match` sites compiler lists.”
18. **Config sentinel** — “Rewrite `fn port() -> i32` returning `-1` on failure to `Option<u16>` + `match` in `main`.”
19. **Checklist drill** — “Match 6 Chapter 6 compiler errors to snippets; I name fix (`match` arm, `ref`, `?`, return type).”
20. **Java enum map** — “Java `enum State { IDLE, RUN }` with method — port to Rust `enum` + `match` + `impl`; contrast nullability.”

#### Advanced patterns

17. **Slice split** — "Parse `POST /api/v1/run HTTP/1.1` with `[method, path @ .., _ver]` — I write; you handle single-token input."
18. **matches refactor** — "Replace 6-arm `match` that returns `bool` with `matches!` — show before/after on `Mode` enum."
19. **if let chain** — "Parse `host:port:extra` — chain should reject extra segments; fix my broken parser."
20. **@ binding quiz** — "Three arms with `@` ranges for port classes — I label which values hit which arm."
21. **Exhaustive slice** — "`[a, b]` on `Vec` of len 1 or 3 — predict `_` arm vs bug; suggest `..` rest pattern."
22. **matches vs match** — "When must you keep full `match` instead of `matches!`? Give one example returning `String`."
23. **Guard + @** — "Match port `n @ 1024..=65535` with guard `n % 2 == 0` — sketch arm."
24. **Capstone parse** — "Frame header `[sync, len @ 1..=255, payload @ ..]` from byte slice — sketch `match` arms only."

