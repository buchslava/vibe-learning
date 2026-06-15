# Chapter 20: Production Rust Standards

## Hook

You can write Rust that compiles and still miss what experienced reviewers look for: typed boundaries, errors as data, minimal cloning, and panic-free production paths. This chapter is a **review checklist** — the habits teams enforce in code review and the patterns worth asking an AI to verify after every Rust change.

## Scope — a brief tour

| This chapter covers | Assumes you have read |
|---------------------|------------------------|
| Review checklist with examples + motivation | [Ch 6–8](06_types_enums_pattern_matching.md) types and errors |
| Ownership, allocation, and pointer idioms | [Ch 1](01_paradigm_shift.md), [Ch 10](10_smart_pointers_interior_mutability.md) |
| Workspace and test conventions | [Ch 9](09_modules_paths_crates.md), [Ch 8](08_errors_and_testing.md) |

Run these checks **only when Rust code changed**. No Rust diff → skip this chapter.

## How to use the checklist

Each section follows the same shape:

1. **Anti-pattern** (or weak pattern) — what reviewers reject.
2. **Idiomatic fix** — runnable example.
3. **Why** — short motivation: what breaks in production if you ignore it.

At the end, a **one-page table** collects every rule for quick scans.

---

## Types — newtypes and enums

Production code often smuggles meaning through **raw numbers**: two `u8` parameters that look interchangeable, and status codes checked with `== 0`. Newtypes and enums make that meaning visible to the compiler.

### Weak — primitives hide two different mistakes

```rust
// Playground — weak
fn write_register(unit_id: u8, register: u8, value: u16) {
    let _ = (unit_id, register, value);
}

fn is_healthy(status_code: u8) -> bool {
    status_code == 0 // 0 = ok, 1 = fault — magic number at the call site
}

fn main() {
    write_register(3, 1, 100);   // unit 3, register 1
    write_register(1, 3, 100);   // compiles — same types, wrong order

    let status_code = 1;
    if !is_healthy(status_code) {
        println!("device fault");
    }
}
```

Two separate problems:

| Problem | What goes wrong |
|---------|-----------------|
| `unit_id: u8` and `register: u8` | Swapping arguments **compiles** — both are `u8`. |
| `status_code: u8` with `== 0` | Callers pass undocumented magic numbers; easy to typo a valid `u8`. |

Different primitive types (`u16` vs `String`) already prevent some swaps — the real footgun is **two parameters of the same type**, plus **open-coded status values**.

### Misleading — type aliases rename, they do not create types

A **type alias** (`type UnitId = u8`) is only a synonym. The compiler still treats `UnitId`, `Register`, and `u8` as the **same** type — nicer names in signatures, but none of the swap protection below.

```rust
// Playground — misleading
type UnitId = u8;
type Register = u8;

fn write_register(unit: UnitId, reg: Register, value: u16) {
    let _ = (unit, reg, value);
}

fn main() {
    write_register(3, 1, 100);
    write_register(1, 3, 100); // still compiles — both args are u8
}
```

Use aliases when you want **shorter spelling of one identity** — e.g. `pub type Result<T> = std::result::Result<T, GatewayError>` ([Chapter 8](08_errors_and_testing.md)). Use **struct newtypes** when two domain concepts share a representation but must stay distinct.

### Strong — newtypes for identity, enums for state

```rust
// Playground
struct UnitId {
    id: u8,
}

struct Register {
    address: u8,
}

enum DeviceStatus {
    Healthy,
    Fault { code: u8 },
}

fn write_register(unit: UnitId, reg: Register, value: u16) {
    println!(
        "unit {} reg {} := {}",
        unit.id, reg.address, value
    );
}

fn describe(status: DeviceStatus) -> &'static str {
    match status {
        DeviceStatus::Healthy => "healthy",
        DeviceStatus::Fault { .. } => "fault",
    }
}

fn main() {
    write_register(
        UnitId { id: 3 },
        Register { address: 1 },
        100,
    );
    // write_register(
    //     Register { address: 1 },
    //     UnitId { id: 3 },
    //     100,
    // ); // ERROR: expected UnitId, found Register

    let status = DeviceStatus::Fault { code: 0x07 };
    println!("{} ({})", describe(status), match status {
        DeviceStatus::Fault { code } => format!("code 0x{:02X}", code),
        DeviceStatus::Healthy => "no code".into(),
    });
}
```

| Fix | Effect |
|-----|--------|
| `UnitId` / `Register` | Swapping arguments is a **type error**, not a silent wrong write. |
| `DeviceStatus` enum | States are **named variants** — add `Degraded` and the compiler lists every `match` to update. |

Struct newtypes vs type aliases for the same underlying `u8`:

| Capability | `type X = u8` | `struct X { ... }` |
|------------|---------------|---------------------|
| Prevent same-primitive argument swaps | No — still `u8` | Yes — distinct nominal types |
| Separate `impl` blocks and traits | No — one `impl` for `u8` | Yes — `impl UnitId`, `impl Display for Register`, … |
| Implement foreign traits (orphan rule) | No | Yes — wrap and `impl` on your struct ([Chapter 7](07_structs_traits_generics.md)) |
| Validation at construction | Any `u8` passes through | `UnitId::new(n)?`, `TryFrom<u8>`, private fields |

**Why:** newtypes attach **identity** to values that share a representation (`u8`, `u16`, `String`). Enums attach **behaviour** to state instead of scattered integer checks. Together they turn whole classes of field and wiring bugs into compile-time failures ([Chapter 6](06_types_enums_pattern_matching.md), [Chapter 7](07_structs_traits_generics.md)).

---

## Lifetimes — correct and minimal

**Weak:** `'static` everywhere because it silences the borrow checker.

```rust
// Playground — smell: unnecessary 'static
fn label(_s: &'static str) -> &'static str {
    "fixed"
}
```

**Strong:** the **shortest** lifetime that describes the data — often elided on simple helpers.

```rust
// Playground
fn first_token(s: &str) -> Option<&str> {
    s.split_whitespace().next()
}

fn main() {
    let line = String::from("temp 22.5");
    println!("{:?}", first_token(&line));
}
```

**Why:** over-long lifetimes hide bugs (returning references to temporaries) and force callers to leak or allocate. Minimal signatures stay reusable and honest ([Chapter 5](05_lifetimes.md)). Add explicit lifetimes only when the compiler asks or when two inputs must outlive the same scope.

---

## `Option` and `Result` — not null-like sentinels

**Weak:** magic values (`-1`, empty string) or nullable references for “missing”.

```rust
// Playground — weak
fn find_port(config: &str) -> i32 {
    if config.is_empty() {
        -1
    } else {
        502
    }
}
```

**Strong:** `Option` for absence, `Result` for failure with a reason.

```rust
// Playground
fn find_port(config: &str) -> Option<u16> {
    if config.is_empty() {
        None
    } else {
        Some(502)
    }
}

fn parse_port(config: &str) -> Result<u16, &'static str> {
    let p = find_port(config).ok_or("missing port")?;
    if p < 1024 {
        Err("privileged port")
    } else {
        Ok(p)
    }
}

fn main() {
    println!("{:?} {:?}", find_port("modbus"), parse_port("8080"));
}
```

**Why:** sentinels collide with valid data and skip the type system. `Option`/`Result` force callers to handle absence and failure — the core of reliable automation ([Chapter 6](06_types_enums_pattern_matching.md), [Chapter 8](08_errors_and_testing.md)).

---

## Avoid `unwrap` and `expect` in production

**Weak:** I/O and parsing that panic on bad input.

```rust
// Playground — production anti-pattern
fn load_port(s: &str) -> u16 {
    s.parse().unwrap()
}
```

**Strong:** propagate or handle at a boundary.

```rust
// Playground
fn load_port(s: &str) -> Result<u16, String> {
    s.parse().map_err(|e| e.to_string())
}

fn main() {
    match load_port("502") {
        Ok(p) => println!("port {}", p),
        Err(e) => eprintln!("skip tick: {}", e),
    }
}
```

**Why:** `unwrap`/`expect` **panic** — they stop the thread (often the whole gateway) on expected failures like bad config or a dropped device. Reserve them for **tests**, **prototypes**, or truly impossible states after invariants you control ([Chapter 8](08_errors_and_testing.md)).

---

## Panic audit — indexing, `panic!`, and production paths

**Weak:** unchecked indexing and explicit panics on user data.

```rust
// Playground — can panic in production
fn first_byte(frame: &[u8]) -> u8 {
    frame[0] // panics if empty
}

fn validate_tag(tag: &str) {
    if tag.is_empty() {
        panic!("empty tag");
    }
}
```

**Strong:** safe access and `Result` for invalid input.

```rust
// Playground
fn first_byte(frame: &[u8]) -> Option<u8> {
    frame.first().copied()
}

fn validate_tag(tag: &str) -> Result<(), &'static str> {
    if tag.is_empty() {
        Err("empty tag")
    } else {
        Ok(())
    }
}

fn main() {
    println!("{:?} {:?}", first_byte(&[]), validate_tag("temp"));
}
```

**Why:** every `vec[i]`, `unwrap`, `expect`, and `panic!` on a production path is an **unhandled incident** waiting for bad wire data. Tests may panic freely; field code should return `Err` and let the supervisor retry or alarm.

---

## Propagate with `?` consistently

**Weak:** nested `match` noise in fallible pipelines.

```rust
// Playground — verbose
fn load(s: &str) -> Result<u16, String> {
    let t = match s.parse::<u16>() {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    Ok(t)
}
```

**Strong:** `?` on the railway ([Chapter 8](08_errors_and_testing.md)).

```rust
// Playground
fn load(s: &str) -> Result<u16, String> {
    let t = s.parse::<u16>().map_err(|e| e.to_string())?;
    Ok(t)
}

fn main() {
    println!("{:?}", load("502"));
}
```

**Why:** consistent `?` keeps helpers thin and makes the **happy path** readable. Reviewers spot missing error handling when `?` is the default and `match` is reserved for recovery.

---

## Custom errors — `thiserror` in libraries, `anyhow` in small binaries

**Weak:** library API returning `Result<T, String>` or using `anyhow` in public interfaces.

```rust
// Playground — weak public API
pub fn connect(_host: &str) -> Result<(), String> {
    Err("timeout".into())
}
```

**Strong:** focused `enum` + **`thiserror`** in library crates; **`anyhow`** only in binary/`main` helpers.

```rust
// Cargo project — library pattern (thiserror = "2")
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectError {
    #[error("timeout after {ms} ms")]
    Timeout { ms: u64 },
    #[error("invalid host")]
    InvalidHost,
}

pub fn connect(_host: &str) -> Result<(), ConnectError> {
    Err(ConnectError::Timeout { ms: 500 })
}
```

```rust
// Cargo project — binary only (anyhow = "1")
use anyhow::Context;

fn run() -> anyhow::Result<()> {
    let _port = std::env::var("PORT").context("PORT must be set")?;
    Ok(())
}
```

**Why:** typed errors are **`match`**able for recovery; strings are not. `thiserror` removes boilerplate while keeping a stable library contract. `anyhow` is fine for **one-off scripts and `main`**, not for code other crates depend on ([Chapter 8](08_errors_and_testing.md)).

---

## Errors store data, not messages

**Weak:** formatting errors into strings — context lost, no structured logging.

```rust
// Playground — weak
enum AppError {
    Io(String), // "failed to read /etc/gateway.toml: permission denied"
}
```

**Strong:** variants carry **fields**; display text comes from `thiserror` or `Display`.

```rust
// Playground
#[derive(Debug)]
enum AppError {
    ConfigRead {
        path: String,
        source: std::io::Error,
    },
}

fn main() {
    let e = AppError::ConfigRead {
        path: "gateway.toml".into(),
        source: std::io::Error::new(std::io::ErrorKind::NotFound, "missing"),
    };
    println!("{:?}", e); // logs match on kind + path, not string grep
}
```

**Why:** structured variants support **metrics, retries, and tests** (`match` on `ErrorKind::NotFound`). Stringly errors force fragile substring checks and duplicate messages across call sites.

---

## Focused error enums per layer

**Weak:** one mega `AppError` for validation, Mongo, config, and serial — returned from a string trim helper.

```rust
// Playground — weak: validation returns unrelated variants
enum AppError {
    InvalidPort,
    MongoTimeout,
    ConfigParse,
}

fn trim_port(s: &str) -> Result<u16, AppError> {
    s.trim().parse().map_err(|_| AppError::InvalidPort)
    // why would MongoTimeout appear here?
}
```

**Strong:** each **module or concern** owns a small enum; map at boundaries.

```rust
// Playground
#[derive(Debug)]
enum ValidateError {
    InvalidPort,
    OutOfRange(u16),
}

#[derive(Debug)]
enum StorageError {
    Timeout { ms: u64 },
}

fn trim_port(s: &str) -> Result<u16, ValidateError> {
    let p: u16 = s.trim().parse().map_err(|_| ValidateError::InvalidPort)?;
    if p < 1024 {
        Err(ValidateError::OutOfRange(p))
    } else {
        Ok(p)
    }
}

fn save(_port: u16) -> Result<(), StorageError> {
    Err(StorageError::Timeout { ms: 500 })
}

fn main() {
    println!("{:?} {:?}", trim_port("8080"), save(8080));
}
```

**Why:** callers of `trim_port` should not handle database timeouts. Focused enums document **who can recover from what** and keep `match` arms honest. Compose or wrap at the service boundary ([Chapter 8 — error aggregation](08_errors_and_testing.md#error-aggregation-for-batch-work)).

---

## Cloning — appropriate, not a band-aid

**Weak:** `.clone()` to silence borrow checker errors without understanding ownership.

```rust
// Playground — weak: clone hides that you could borrow
fn print_twice(s: &String) {
    println!("{} {}", s.clone(), s.clone());
}
```

**Strong:** clone when you **need** an owned copy; borrow otherwise.

```rust
// Playground
fn print_twice(s: &str) {
    println!("{} {}", s, s);
}

fn store_label(s: &str) -> String {
    s.to_string() // one intentional allocation for storage
}

fn main() {
    let label = String::from("sensor-1");
    print_twice(&label);
    let owned = store_label(&label);
    println!("{}", owned);
}
```

**Why:** unnecessary clones cost CPU and memory in hot paths (parsers, poll loops). Cloning to “make it compile” often means the API should take `&str`, return owned data once, or use `Cow` ([Chapter 13](13_standard_traits.md)).

---

## Avoid unnecessary `.clone()` and `.to_owned()`

**Weak:** cloning collections on every iteration.

```rust
// Playground — weak
fn sum_tags(tags: &Vec<String>) -> usize {
    tags.clone().iter().map(|t| t.len()).sum()
}
```

**Strong:** iterate by reference.

```rust
// Playground
fn sum_tags(tags: &[String]) -> usize {
    tags.iter().map(|t| t.len()).sum()
}

fn main() {
    let tags = vec!["a".into(), "bb".into()];
    println!("{}", sum_tags(&tags));
}
```

**Why:** each stray clone duplicates heap data. Accept `&[T]` / `&str` in helpers; clone only when crossing threads, storing long-term, or transforming into an owned value.

---

## `Rc::clone` / `Arc::clone` — not `.clone()` on the smart pointer

**Weak:** `arc.clone()` — ambiguous (clone the pointer or the inner value?).

```rust
// Playground — works but unclear in review
use std::sync::Arc;

fn share(data: Arc<String>) -> Arc<String> {
    data.clone()
}
```

**Strong:** explicit **`Arc::clone(&data)`** (same for **`Rc::clone`**).

```rust
// Playground
use std::sync::Arc;

fn share(data: Arc<String>) -> Arc<String> {
    Arc::clone(&data)
}

fn main() {
    let a = Arc::new("config".to_string());
    let b = share(Arc::clone(&a));
    println!("{} {}", a, b);
}
```

**Why:** `Arc::clone` increments the **reference count** only — cheap and obvious in code review. `.clone()` on `Arc` does the same thing but reads like a deep copy of the inner `String`. Team style guides often require the explicit form ([Chapter 10](10_smart_pointers_interior_mutability.md)).

---

## Borrow instead of move when you only read

**Weak:** taking `String` by value when the caller still needs it.

```rust
// Playground — weak
fn log_name(name: String) {
    println!("{}", name);
}

fn main() {
    let name = String::from("plc-1");
    log_name(name);
    // println!("{}", name); // ERROR: moved
}
```

**Strong:** `&str` / `&T` for read-only use ([Chapter 3](03_functions.md)).

```rust
// Playground
fn log_name(name: &str) {
    println!("{}", name);
}

fn main() {
    let name = String::from("plc-1");
    log_name(&name);
    println!("still have {}", name);
}
```

**Why:** moving values forces clones at call sites or loses data after one call. Borrowing keeps ownership with the caller and documents read-only intent.

---

## Heap allocations — only when needed

**Weak:** `Box`, `Vec`, and `String` for tiny stack-sized data.

```rust
// Playground — weak
fn port() -> Box<u16> {
    Box::new(502)
}
```

**Strong:** stack types and iterators; heap when size is dynamic or ownership must move.

```rust
// Playground
fn port() -> u16 {
    502
}

fn readings(count: usize) -> Vec<f64> {
    (0..count).map(|i| i as f64).collect()
}

fn main() {
    println!("{} {:?}", port(), readings(3));
}
```

**Why:** heap allocations add latency and fragmentation. Prefer stack values, `&str`, and `impl Iterator` return types until you need dynamic size or trait objects ([Chapter 3](03_functions.md#impl-trait-in-return-position), [Chapter 7](07_structs_traits_generics.md)).

---

## `Vec::with_capacity` when size is bounded

**Weak:** repeated `push` on an empty `Vec` — many reallocations.

```rust
// Playground — weak for known upper bound
fn build_tags(n: usize) -> Vec<String> {
    let mut v = Vec::new();
    for i in 0..n {
        v.push(format!("tag-{i}"));
    }
    v
}
```

**Strong:** **`with_capacity`** when max length is known.

```rust
// Playground
fn build_tags(n: usize) -> Vec<String> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push(format!("tag-{i}"));
    }
    v
}

fn main() {
    println!("len={}", build_tags(100).len());
}
```

**Why:** pre-allocation avoids O(n) realloc copies in parsers and batch builders. If capacity is unknown, start empty; if you know “at most N registers”, reserve N ([Chapter 11](11_collections.md)).

---

## Workspace dependencies — `{ workspace = true }`

**Weak:** duplicate version pins in every crate of a workspace.

```toml
# member Cargo.toml — weak
[dependencies]
tokio = { version = "1.40", features = ["full"] }
serde = "1.0.210"
```

**Strong:** centralize in the **root** `Cargo.toml`, inherit in members.

```toml
# root Cargo.toml
[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
serde = "1"
thiserror = "2"
```

```toml
# member Cargo.toml
[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
thiserror = { workspace = true }
```

**Why:** one bump updates every crate; CI and local builds stay on the same versions. Exception: a member-only dependency that is large and unrelated to the rest — pin it locally with a comment ([Chapter 9](09_modules_paths_crates.md)).

---

## Tests — equality, not field-by-field drift

**Weak:** asserting each field manually — tests break on every harmless derive change.

```rust
// Playground — brittle
#[derive(Debug, PartialEq)]
struct Reading { tag: String, value: f64 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading() {
        let r = Reading { tag: "t".into(), value: 1.0 };
        assert_eq!(r.tag, "t");
        assert_eq!(r.value, 1.0);
    }
}

fn main() {}
```

**Strong:** **`assert_eq!` on the whole value** when `PartialEq` is implemented; use **`pretty_assertions`** in Cargo projects for diffs.

```rust
// Playground
#[derive(Debug, PartialEq)]
struct Reading { tag: String, value: f64 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading() {
        let got = Reading { tag: "t".into(), value: 1.0 };
        let want = Reading { tag: "t".into(), value: 1.0 };
        assert_eq!(got, want);
    }
}

fn main() {}
```

```toml
# Cargo — dev-dependencies
[dev-dependencies]
pretty_assertions = "1"
```

```rust
// Cargo test module
use pretty_assertions::assert_eq;
```

**Why:** struct equality catches **any** field drift in one failure. `pretty_assertions` prints side-by-side diffs for nested data — essential for parser golden tests ([Chapter 8](08_errors_and_testing.md#comparable-test-dtos--normalize-before-equality)).

---

## Tests — avoid time and global state pitfalls

**Weak:** tests that sleep real time or mutate static globals — flaky in CI.

```rust
// Playground — flaky pattern (conceptual)
// static mut COUNTER: u32 = 0;
// #[test]
// fn depends_on_wall_clock() {
//     std::thread::sleep(Duration::from_secs(1));
//     assert!(true);
// }
```

**Strong:** inject clocks and stores; use temp dirs in integration tests.

```rust
// Playground
trait Clock {
    fn now_ms(&self) -> u64;
}

struct FixedClock(u64);

impl Clock for FixedClock {
    fn now_ms(&self) -> u64 {
        self.0
    }
}

fn is_stale(c: &impl Clock, last: u64, max_age: u64) -> bool {
    c.now_ms().saturating_sub(last) > max_age
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stale_after_threshold() {
        let c = FixedClock(1000);
        assert!(is_stale(&c, 0, 500));
        assert!(!is_stale(&c, 900, 500));
    }
}

fn main() {}
```

**Why:** wall-clock sleeps slow CI and fail under load. Global mutable state makes tests **order-dependent**. Fake clocks and per-test fixtures keep suites deterministic ([Chapter 8](08_errors_and_testing.md)).

---

## Quick reference — full checklist

| # | Rule | One-line test |
|---|------|----------------|
| 1 | Newtypes / enums for domain meaning | Same-type args swappable? Status as raw `u8`/`bool`? |
| 2 | Minimal lifetimes | Any `'static` that could be elided? |
| 3 | `Option` / `Result`, not sentinels | Any magic `-1` / empty meaning “error”? |
| 4 | No `unwrap` / `expect` in production paths | Only tests and impossible invariants? |
| 5 | Panic audit | Any `[]`, `panic!`, or unwrap on external input? |
| 6 | Consistent `?` | Fallible helpers return `Result`? |
| 7 | `thiserror` in libs; `anyhow` in bins/scripts | Public API free of `anyhow`? |
| 8 | Errors carry **data** | Variants have fields, not preformatted strings? |
| 9 | Focused error enums per layer | Does validation return DB errors? |
| 10 | Clone deliberately | Clone fixes borrow errors → fix API first? |
| 11 | No stray `.clone()` / `.to_owned()` | Could this be `&str` / `&[T]`? |
| 12 | `Rc::clone` / `Arc::clone` | Smart-pointer clones explicit? |
| 13 | Borrow for read-only | Parameters own `String` without storing? |
| 14 | Heap when needed | `Box<u16>`-style noise removed? |
| 15 | `with_capacity` when bounded | Known max len → reserve? |
| 16 | `{ workspace = true }` | Member deps aligned with root? |
| 17 | Tests use `assert_eq!` on values | Field-by-field asserts replaced? |
| 18 | `pretty_assertions` for rich diffs | Dev-dependency in workspace? |
| 19 | No flaky time / global state | Clocks and stores injected? |

---

## Idiom spotlight

> **Production Rust is typed, panic-averse, and layered.** Model meaning with newtypes and enums; propagate with `?` and **focused** `thiserror` enums; borrow by default; allocate with intent; test with **equality** and deterministic fixtures. Review Rust diffs against this table before merge.

## Go deeper

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [The Rust Book — Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [The Rust Book — Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Clippy lints](https://doc.rust-lang.org/clippy/) — many rules overlap this checklist

## See also

- [Chapter 8: Errors and testing](08_errors_and_testing.md) — `Result`, `thiserror`, mocks
- [Chapter 7: Structs and traits](07_structs_traits_generics.md) — newtypes, trait boundaries
- [Chapter 9: Modules and crates](09_modules_paths_crates.md) — workspaces
- [Chapter 10: Smart pointers](10_smart_pointers_interior_mutability.md) — `Arc` / `Rc`
- [Chapter 11: Collections](11_collections.md) — capacity and maps
- [Chapter 13: Standard traits](13_standard_traits.md) — `From`, `AsRef`, `PartialEq`

### Afterparty

#### Checklist drills

1. **Diff review** — “Paste a 30-line Rust PR; I mark each checklist row pass/fail with one sentence.”
2. **Mega-error refactor** — “Split one `AppError` into `ValidateError` + `StorageError`; show boundary mapping.”
3. **Panic hunt** — “Find five panic sources in a snippet; replace with `Option`/`Result`.”
4. **Clone audit** — “Remove three unnecessary clones by fixing signatures to `&str` / `&[T]`.”
5. **Arc style** — “Rewrite `arc.clone()` call sites to `Arc::clone(&arc)`; explain review benefit.”

#### Workspace and tests

6. **Workspace.toml** — “Sketch root + two members; all shared deps use `{ workspace = true }`.”
7. **Golden test** — “Parser output: one `assert_eq!(got, want)` + `pretty_assertions`; no per-field asserts.”
8. **Flaky test fix** — “Replace `thread::sleep` in test with injected `Clock` trait.”
