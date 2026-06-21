# Chapter 3: Functions and Methods

## Hook

You already know how to call functions. In Rust, every signature is a **contract** about ownership and types. A parameter is not “just a reference”; it tells you whether the callee **borrows**, **mutably borrows**, or **takes** the value.

This chapter covers standalone functions, methods, and return-type patterns you will reuse everywhere.

## Scope — a brief tour

Function signatures, receivers, and return patterns — not full generics or async.

| This chapter covers | Deferred |
|---------------------|----------|
| Signatures, ownership in params, `impl` methods | Full trait system → Ch 7 |
| `impl Trait` return preview, `mem::take` | Async fns → Ch 16 |
| `Result` signature preview | Error design → Ch 8 |

## Function signatures

A Rust function names its inputs and output explicitly:

```rust
// Playground
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", add(2, 3));
}
```

| Piece | Meaning |
|-------|---------|
| `fn name` | declare a function |
| `(a: i32, b: i32)` | parameters with types |
| `-> i32` | return type |
| `{ ... }` | body; the **last expression** (no semicolon) is the return value |

**Java / Python contrast**

| | Java | Python | Rust |
|---|------|--------|------|
| Return | `return x;` or last statement | `return x` optional | last **expression** returns; use `return` for early exit |
| Missing return | `void` | implicit `None` | `-> ()` unit type |
| Overloading | same name, different params | duck typing | **no** overloading — use different names or generics ([Chapter 7](07_structs_traits_generics.md)) |

## Parameters and ownership

Parameters follow the same ownership rules as `let` bindings ([Chapter 1](01_paradigm_shift.md#ownership-vs-garbage-collection)):

```rust
// Playground
fn consume(s: String) {
    println!("{}", s);
} // s dropped here

fn borrow(s: &str) {
    println!("{}", s);
}

fn main() {
    let label = String::from("sensor");
    borrow(&label);
    consume(label);
    // borrow(&label); // ERROR: label was moved into consume
}
```

| Parameter type | Caller after call |
|----------------|-------------------|
| `T` | value **moved** unless `T: Copy` |
| `&T` | still owns; shared borrow |
| `&mut T` | still owns; exclusive borrow |

**Rule of thumb:** take `&str` / `&[u8]` when you only read; take owned `String` / `Vec` when you store or send the data elsewhere.

## Expression bodies and early return

Omit braces for a single expression:

```rust
// Playground
fn double(x: i32) -> i32 {
    x * 2
}

fn abs_diff(a: i32, b: i32) -> i32 {
    if a > b { a - b } else { b - a }
}

fn main() {
    println!("{} {}", double(5), abs_diff(10, 3));
}
```

Use `return` when you exit before the end:

```rust
// Playground
fn first_positive(v: &[i32]) -> Option<i32> {
    for &n in v {
        if n > 0 {
            return Some(n);
        }
    }
    None
}

fn main() {
    println!("{:?}", first_positive(&[-1, 0, 4, 2]));
}
```

Adding a semicolon after the last expression **suppresses** the return value and gives `()`:

```rust
// Playground — does not compile if return type is i32
fn broken() -> i32 {
    42; // semicolon turns this into a statement, not the return value
    // ERROR: expected i32, found ()
}
```

## The unit type `()` and diverging functions

Functions that do side effects only often return **unit**:

```rust
// Playground
fn log_line(msg: &str) {
    println!("[log] {}", msg);
} // implicit -> ()

fn main() {
    log_line("ready");
}
```

Some functions **never return** — they diverge with `panic!`, infinite loops, or `std::process::exit`. Their return type is **`!`** (never type):

```rust
// Playground
fn fatal(msg: &str) -> ! {
    panic!("{}", msg);
}

fn main() {
    // fatal("stop"); // would never reach code below
    println!("ok");
}
```

You will see `!` rarely in application code. Knowing it exists explains why `match` arms can have different “no return” branches.

## Methods and associated functions

Rust splits **data** (`struct` / `enum`) from **behaviour** (`impl` blocks). Methods take `&self`, `&mut self`, or `self`; **associated functions** have no `self` (like static methods):

```rust
// Playground
struct Gauge {
    value: f64,
}

impl Gauge {
    // associated function — call as Gauge::new(0.0)
    fn new(value: f64) -> Self {
        Self { value }
    }

    // method — call as gauge.read()
    fn read(&self) -> f64 {
        self.value
    }

    fn set(&mut self, v: f64) {
        self.value = v;
    }

    fn reset(self) -> Self {
        Self { value: 0.0 }
    }
}

fn main() {
    let mut g = Gauge::new(12.5);
    println!("{}", g.read());
    g.set(99.0);
    let zero = g.reset();
    println!("{}", zero.read());
}
```

| Receiver | Meaning |
|----------|---------|
| `&self` | borrow for read |
| `&mut self` | exclusive borrow for mutation |
| `self` | **consume** `self` (move out) |

**Java:** instance methods inside classes. **Python:** functions in a class with `self`. **Rust:** explicit `impl` blocks — no inheritance chain.

## Multiple `impl` blocks and privacy

You can split `impl` for clarity; only items marked `pub` are visible outside the module ([Chapter 9](09_modules_paths_crates.md)):

```rust
// Playground
pub struct Counter {
    n: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self { n: 0 }
    }

    pub fn bump(&mut self) {
        self.n += 1;
    }

    pub fn get(&self) -> u32 {
        self.n
    }
}

fn main() {
    let mut c = Counter::new();
    c.bump();
    println!("{}", c.get());
}
```

Field `n` stays private; callers use methods — same encapsulation idea as Java `private` fields with public getters.

## Generic functions (preview)

One function body can work for many types when they share a capability (**trait bound**). Full treatment is in [Chapter 7](07_structs_traits_generics.md); the shape is:

```rust
// Playground
use std::fmt::Display;

fn show_twice<T: Display>(x: T) {
    println!("{} | {}", x, x);
}

fn main() {
    show_twice(42);
    show_twice("hello");
}
```

The compiler **monomorphizes** — generates `show_twice_i32`, `show_twice_str`, etc. — so you pay no runtime dispatch cost unless you ask for `dyn Trait`.

## Functions that return `Result` (preview)

Fallible operations return `Result` instead of throwing. [Chapter 8](08_errors_and_testing.md) covers this in depth; the signature pattern is:

```rust
// Playground
fn parse_port(s: &str) -> Result<u16, std::num::ParseIntError> {
    s.parse::<u16>()
}

fn main() {
    match parse_port("8080") {
        Ok(p) => println!("port {}", p),
        Err(e) => println!("bad port: {}", e),
    }
}
```

Inside a `fn -> Result<...>` body, **`?`** propagates errors upward. You will use it after [Chapter 6](06_types_enums_pattern_matching.md) and [Chapter 8](08_errors_and_testing.md).

## `impl Trait` in return position

Iterator pipelines chain adapters (`map`, `filter`, `take`, …). The **concrete** type of that chain is often long and brittle — e.g. `Take<IntoIter<f64>>` — and it changes every time you add or remove a step. Writing that type in the signature is noisy and couples callers to your implementation.

`impl Trait` in the return position names **what callers need** (here: “something iterable that yields `f64`”) while the compiler keeps track of the real adapter type. You get static dispatch with no heap allocation — unlike `Box<dyn Iterator<...>>`.

```rust
// Playground
fn top_readings(readings: &[f64], n: usize) -> impl Iterator<Item = f64> + '_ {
    let mut sorted: Vec<f64> = readings.to_vec();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
    sorted.into_iter().take(n)
}

fn main() {
    let vals = [1.2, 3.4, 2.1, 5.0];
    let top: Vec<_> = top_readings(&vals, 2).collect();
    println!("{:?}", top);
}
```

**Reading the signature:**

| Piece | Meaning |
|-------|---------|
| `impl Iterator<Item = f64>` | returns *some* type that implements `Iterator` and yields `f64`; callers can `.collect()`, `.sum()`, etc. without knowing the concrete adapter |
| `+ '_` | the returned iterator may borrow from `readings`; `'_` asks the compiler to infer that lifetime from the `&[f64]` parameter |

Inside the body, `sorted.into_iter().take(n)` is one specific iterator type. You never spell it out — Rust monomorphizes a version of `top_readings` for that exact chain at compile time.

**One concrete type per function:** `impl Trait` here does **not** mean “any iterator.” The compiler picks **one** concrete return type for the entire function, and every `return` path must produce exactly that type. Branches that build different adapters fail:

```rust
// Playground — does not compile
fn pick(flag: bool) -> impl Iterator<Item = i32> {
    if flag {
        vec![1, 2].into_iter()   // Vec<i32>::IntoIter
    } else {
        [3, 4].into_iter()       // std::array::IntoIter<i32, 2> — different type
    }
}
```

When you truly need different iterator shapes per path, erase the type: `Box<dyn Iterator<Item = f64>>` (heap + dynamic dispatch) or an enum wrapping each variant. See [Function edge cases](#function-edge-cases) below and [Chapter 4: Iterators](04_iterators.md).

## Draining with `mem::take`

**The problem:** A struct accumulates data in a field — here, bytes in a `Vec`. Eventually you want to **hand that data to the caller** and start fresh, but you only have `&mut self`. You cannot write `out.extend(self.inner)` — that would **move** `inner` out of `buf` while `buf` still exists, leaving a hole the compiler rejects.

**The aim:** Move the accumulated `Vec` to the caller **and** leave `buf` in a valid, reusable state (empty, ready for the next batch). `std::mem::take` is the standard idiom for that ([Chapter 10](10_smart_pointers_interior_mutability.md)).

```rust
// Playground
struct Buffer {
    inner: Vec<u8>,
}

impl Buffer {
    fn drain_into(&mut self, out: &mut Vec<u8>) {
        out.extend(std::mem::take(&mut self.inner));
    }
}

fn main() {
    let mut buf = Buffer { inner: vec![1, 2, 3] };
    let mut batch = Vec::new();
    println!("before: buf.inner={:?} batch={:?}", buf.inner, batch);
    buf.drain_into(&mut batch);
    println!("after:  buf.inner={:?} batch={:?}", buf.inner, batch);
    // before: buf.inner=[1, 2, 3]  batch=[]
    // after:  buf.inner=[]         batch=[1, 2, 3]
}
```

**What `take` does:** it replaces the field with `T::default()` and **returns the old value**.

| Step | `buf.inner` | value returned by `take` |
|------|-------------|--------------------------|
| before call | `[1, 2, 3]` | — |
| `take(&mut self.inner)` | `[]` (empty `Vec`, via `Default`) | `vec![1, 2, 3]` (moved out) |
| `out.extend(...)` | `[]` | appended into caller's `batch` |

`buf` is still a valid `Buffer` — just with an empty `inner`. You can keep using it; no need to drop and recreate the struct.

**Alternatives and why they differ:**

| Approach | Effect |
|----------|--------|
| `fn drain(self) -> Vec<u8>` | moves the whole `Buffer` — caller owns the vec, but **consumes** the struct |
| `self.inner.clear()` | empties in place — caller never gets ownership of the old allocation |
| `mem::take(&mut self.inner)` | caller gets the `Vec` (and its capacity); struct stays alive, field reset to empty |

**When you see this:** log/event batch flushes, connection write buffers handed to I/O, any “accumulate, then hand off the batch” API.

## `where` clauses — readable bounds

When generic bounds clutter the signature, move them to a `where` block:

```rust
// Playground
use std::fmt::Display;

fn log_pair<T, U>(a: T, b: U)
where
    T: Display,
    U: Display,
{
    println!("{} | {}", a, b);
}

fn main() {
    log_pair(502, "ready");
}
```

Same contract as `fn log_pair<T: Display, U: Display>(...)` — pick whichever reads cleaner.

## Error constructor factories

Parse failures are usually **enum variants** with named fields — e.g. `BadValue { field, raw }`. You could build them inline at every call site:

```rust
ParseError::BadValue { field: "port".into(), raw: raw.into() }
```

That gets repetitive, and it is easy to swap `field` and `raw`. Production enums add **named constructors** — small functions on `impl ParseError` — so call sites stay short and consistent:

```rust
ParseError::bad_value("port", raw)   // same variant, clearer intent
```

Those constructors take **`impl Into<String>`** instead of `String`. Callers pass `&str` literals (`"port"`), borrowed slices (`raw`), or owned `String`; the constructor calls `.into()` inside and stores owned strings in the enum. No `.to_string()` clutter at every error site.

The example wires both ideas together: `missing_field` and `bad_value` are the factories; `parse_port_field` calls them instead of spelling variants:

```rust
// Playground
#[derive(Debug)] // compile-time trait impl — not a Java annotation or Python decorator; see Ch17
enum ParseError {
    MissingField { field: String, record: String },
    BadValue { field: String, raw: String },
}

impl ParseError {
    #[inline]
    fn missing_field(field: impl Into<String>, record: impl Into<String>) -> Self {
        Self::MissingField {
            field: field.into(),
            record: record.into(),
        }
    }

    #[inline]
    fn bad_value(field: impl Into<String>, raw: impl Into<String>) -> Self {
        Self::BadValue {
            field: field.into(),
            raw: raw.into(),
        }
    }
}

fn parse_port_field(raw: Option<&str>) -> Result<u16, ParseError> {
    let raw = raw.ok_or_else(|| ParseError::missing_field("port", "config"))?;
    raw.parse()
        .map_err(|_| ParseError::bad_value("port", raw))
}

fn main() {
    for raw in [Some("502"), Some("oops"), None] {
        match parse_port_field(raw) {
            Ok(p) => println!("ok: {p}"),
            Err(ParseError::MissingField { field, record }) => {
                println!("missing field {field} in {record}")
            }
            Err(ParseError::BadValue { field, raw }) => {
                println!("bad value for {field}: {raw}")
            }
        }
    }
}
```

| Piece in the example | What it demonstrates |
|----------------------|----------------------|
| `fn missing_field(...)` / `fn bad_value(...)` on `impl ParseError` | **named constructors** — hide variant struct syntax |
| `impl Into<String>` on parameters | callers pass `"port"` or `raw` (`&str`); `.into()` runs inside the constructor |
| `ParseError::missing_field("port", "config")` | factory call when the field is absent (`None`) |
| `ParseError::bad_value("port", raw)` | factory call when the value fails to parse |
| `match` in `main` | consumers still see the enum variants; factories are only for **building** errors |

Use this in library crates with **`thiserror`** enums ([Chapter 8](08_errors_and_testing.md)) — the constructors stay even when `Display` is derived.

**`#[derive(...)]` preview:** the attribute auto-generates trait implementations at **compile time** (`impl Debug for ParseError { ... }`). It does not run at call time and is not runtime metadata. Std derives (`Debug`, `Clone`, …), ecosystem derives (`Serialize`, `Error`, …), and when custom derives are worth it — [Chapter 17: Derive attributes](17_metaprogramming.md#derive-attributes).

## Config-holder structs (lightweight builders)

You do not always need a builder crate. A small struct that **holds configuration** and exposes one method per use case is enough:

```rust
// Playground
struct RequestBuilder {
    timeout_ms: u64,
    retries: u32,
    endpoint: String,
}

impl RequestBuilder {
    fn new(endpoint: impl Into<String>) -> Self {
        Self {
            timeout_ms: 5000,
            retries: 3,
            endpoint: endpoint.into(),
        }
    }

    fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }

    fn build_poll_body(&self, device_id: u32) -> String {
        format!(
            "GET /devices/{device_id}?timeout={}&retries={}",
            self.timeout_ms, self.retries
        )
    }
}

fn main() {
    let req = RequestBuilder::new("gateway.local")
        .with_timeout(1000)
        .build_poll_body(7);
    println!("{}", req);
}
```

Reach for **`new` + chainable setters + one `build_*` method** when parameters are fixed at construction time but the output shape varies. Full builder crates pay off when you have many optional fields and validation at build time.

## Extension traits — add behaviour without newtypes

When you need a helper on a type you do not own, define a **trait in your crate** and implement it for the foreign type you control the orphan rule for — or implement for your wrapper. Common pattern: extend **`Option`** with domain-specific `?` helpers ([Chapter 8](08_errors_and_testing.md#extension-traits-on-option)) and extend **`&str`** with string normalizers ([Chapter 13](13_standard_traits.md#extension-traits-on-str)).

```rust
// Playground
trait TrimOrEmpty {
    fn trim_or_empty(&self) -> &str;
}

impl TrimOrEmpty for str {
    fn trim_or_empty(&self) -> &str {
        self.trim()
    }
}

fn label(raw: &str) -> &str {
    raw.trim_or_empty()
}

fn main() {
    println!("'{}'", label("  sensor-1  "));
}
```

Implement for **`str`** with **`&self`**, not `for &str` with `self` by value. When the method returns `&str`, the compiler must tie that borrow to the receiver — `&self` makes that relationship explicit. Call sites still write `raw.trim_or_empty()` on a `&str`; method resolution handles the rest.

Keep extension traits **small and focused** — one concern per trait, not a grab-bag of unrelated methods.

### Function edge cases

**Wrong — mismatched `impl Trait` return arms:** see [impl Trait in return position](#impl-trait-in-return-position) for the full explanation. Minimal repro:

```rust
// Playground — does not compile
fn pick(_use_a: bool) -> impl Iterator<Item = i32> {
    if _use_a {
        vec![1, 2].into_iter()
    } else {
        [3, 4].into_iter() // different concrete iterator type
    }
}
```

**`const fn` preview:** Rust can evaluate some functions at compile time (`const fn add(a: i32, b: i32) -> i32 { a + b }`). Full const evaluation rules are in [Chapter 17](17_metaprogramming.md).

## When the compiler says no

Common errors in this chapter:

| Error (typical) | Cause | Fix |
|-----------------|-------|-----|
| expected `T`, found `()` | semicolon on last expression | remove `;` or add `return` |
| use of moved value | took `String` by value, caller uses again | borrow with `&str` or clone |
| cannot borrow as mutable | `&mut` while `&` still alive | shrink borrow scope |
| type annotations needed | generic `T` unknown | turbofish or annotate call site |

## Go deeper

- [The Rust Book — Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Methods](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

## See also

- [Chapter 2: Types and expressions](02_types.md) — types in signatures
- [Chapter 1: Ownership](01_paradigm_shift.md#ownership-vs-garbage-collection) — move vs borrow in parameters
- [Chapter 4: Iterators](04_iterators.md) — functions as pipeline steps
- [Chapter 7: Structs, traits, and generics](07_structs_traits_generics.md) — `impl`, traits, generics
- [Chapter 8: Errors and testing](08_errors_and_testing.md) — `Result` and `?`
- [Chapter 17: Metaprogramming](17_metaprogramming.md#derive-attributes) — `#[derive]` syntax, not annotations/decorators

### Afterparty

#### Signatures and ownership

1. **Parameter audit** — “Five function signatures for logging: `&str`, `String`, `&String`, `Cow<str>`, `impl AsRef<str>`. I pick one per use case; you explain ownership cost.”
2. **Move vs borrow** — “Snippet calls `process(s)` then uses `s` again. I explain the error and show two fixes (`&s`, clone).”

#### Methods and impl

3. **impl block** — “Struct `Timer` with `start`, `elapsed`, `reset`. I write `impl`; you check `&self` vs `&mut self`.”
4. **Associated fn** — “When is `Type::new()` idiomatic vs `Default::default()`? One example each.”
5. **Consume self** — “Method `fn into_inner(self) -> Vec<u8>` — why must it take `self` by value?”

#### Returns and control flow

6. **Semicolon trap** — “Three tiny functions: one returns `i32` correctly, two fail due to `;`. I fix them.”
7. **Early return** — “Rewrite nested `if` in a parser as early `return None` / `?` style.”
8. **Unit vs value** — “Which functions should return `()` vs `bool` vs `Option<T>`? Three CLI helper names, I choose.”

#### Generics and errors preview

9. **Generic bounds** — “Fix `fn max(a: T, b: T)` without bounds; add `T: Ord` or `PartialOrd`.”
10. **Result signature** — “Design `fn read_config(path: &str) -> Result<Config, ...>`; list error variants, no body.”

#### impl Trait and drain

11. **impl Iterator return** — "Write `top_n(vals: &[f64], n: usize) -> impl Iterator<Item = f64>` — explain why two different iterator types in `if` arms fail."
12. **mem take drain** — "Buffer struct drains `Vec<u8>` to caller via `mem::take` — show before/after inner field."
13. **where clause** — "Rewrite cluttered `fn f<T: A + B + C>(x: T)` with `where` block — same behaviour."
14. **Self return** — "Method `fn into_inner(self) -> Vec<u8>` on wrapper — why `Self` not concrete type name?"
15. **const fn** — "One `const fn` port validator `fn is_valid(p: u16) -> bool` — what can and cannot run at compile time?"
16. **Error constructors** — "Add `missing_field(name, record)` on a parse error enum with `impl Into<String>`; compare to spelling the variant at each site."
17. **Config holder** — "HTTP poll client: struct holds timeout/retries/endpoint; one `build_request(device_id)` — no builder crate."
18. **Extension trait** — "Add `TrimOrEmpty for &str`; use in three parser call sites vs free functions."
