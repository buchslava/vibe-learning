# Chapter 13: Standard Traits and Conversions

## Hook

Rust’s standard library is built on **traits** — shared behaviour you opt into with `impl` or `#[derive]`. This chapter covers formatting, comparison, conversion, and flexible-argument traits you see in almost every crate. Deep mechanics (`dyn`, object safety) stay in [Chapter 7](07_structs_traits_generics.md). Here the focus is **idioms**.

## Scope — a brief tour

Formatting, comparison, and conversion traits — not custom `Formatter` depth or Serde derives.

| This chapter covers | Deferred |
|---------------------|----------|
| `Debug`, `Display`, `Default` | Custom `Formatter` flags in depth |
| `PartialEq`, `Eq`, `Hash`, `Ord` | Float ordering edge cases (nominal) |
| `From`/`Into`, `TryFrom`/`TryInto`, `FromStr` | Full error-type design → [Chapter 8](08_errors_and_testing.md) |
| `AsRef`, `Borrow`, `Cow` | Serde derives → [Chapter 17](17_metaprogramming.md) |

## Debug and Display

**`Debug`** — developer-oriented formatting via `{:?}`. Derive for most structs:

```rust
// Playground
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);
    println!("{:#?}", p); // pretty-print with newlines
}
```

**`Display`** — user-facing formatting via `{}`. No derive — implement manually:

```rust
// Playground
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    println!("{}", Point { x: 3, y: 4 });
}
```

| Trait | Format | Typical use |
|-------|--------|-------------|
| `Debug` | `{:?}`, `{:#?}` | logs, tests, derive |
| `Display` | `{}` | CLI output, user messages |

**`ToString`:** any type with `Display` gets `.to_string()` via a blanket impl — you rarely implement `ToString` yourself.

### Debug vs Display — when which

| Situation | Use |
|-----------|-----|
| `dbg!`, tests, internal logs | `Debug` + derive |
| CLI status line, user error text | `Display` |
| Secret field in struct | manual `Debug` with redaction |
| Enum in production logs | derive `Debug` or compact custom |

**Redacted Debug** — do not derive if logs would leak tokens:

```rust
// Playground
use std::fmt;

struct ApiConfig {
    pub name: String,
    api_key: String,
}

impl fmt::Debug for ApiConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiConfig")
            .field("name", &self.name)
            .field("api_key", &"[REDACTED]")
            .finish()
    }
}

fn main() {
    let c = ApiConfig {
        name: String::from("prod"),
        api_key: String::from("secret-123"),
    };
    println!("{:?}", c);
}
```

### Formatting edge cases

**Wrong — `{}` on type with only `Debug`:**

```rust
// Playground — does not compile
#[derive(Debug)]
struct OnlyDebug(i32);

fn main() {
    // println!("{}", OnlyDebug(1)); // ERROR: `OnlyDebug` doesn't implement Display
    println!("{:?}", OnlyDebug(1)); // ok
}
```

**Newtype for orphan rule** — implement `Display` on your wrapper, not on `Vec<u8>`:

```rust
// Playground
use std::fmt;

struct HexBytes(pub Vec<u8>);

impl fmt::Display for HexBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.0 {
            write!(f, "{:02X}", b)?;
        }
        Ok(())
    }
}

fn main() {
    println!("{}", HexBytes(vec![0xDE, 0xAD]));
}
```

### Formatting flags — quick reference

Use these in `println!`, `format!`, and `write!` ([Chapter 17](17_metaprogramming.md) covers macro syntax):

```rust
// Playground
#[derive(Debug)]
struct Reading {
    id: u32,
    value: f64,
}

fn main() {
    let r = Reading { id: 1, value: 22.456 };
    println!("{:?}", r);       // Reading { id: 1, value: 22.456 }
    println!("{:#?}", r);      // pretty-printed multiline Debug
    println!("{:.2}", r.value); // 22.46 — precision on floats
    println!("{:04}", r.id);    // 0001 — zero-padded width
}
```

| Flag | Effect | Typical use |
|------|--------|-------------|
| `{:?}` | `Debug` | logs, tests |
| `{:#?}` | pretty `Debug` | nested struct inspection |
| `{:.2}` | 2 decimal places | sensor readings |
| `{:04}` | width 4, zero-pad | register ids |
| `{:X}` / `{:02X}` | hex | byte dumps (`HexBytes` above) |

For CLI path helpers, accept `impl AsRef<Path>`. User-facing path formatting is in [Chapter 19](19_io_processes_bits.md#cli-utility--paths-env-and-time).

## Default

Types with a sensible “empty” or “zero” value implement **`Default`**:

```rust
// Playground
#[derive(Default, Debug)]
struct Config {
    timeout_ms: u64,
    retries: u32,
}

fn main() {
    let c = Config {
        retries: 3,
        ..Default::default()
    };
    println!("{:?}", c);
}
```

Enums need `#[default]` on one variant (Rust 1.62+) to derive `Default`:

```rust
// Playground
#[derive(Default, Debug)]
enum Mode {
    #[default]
    Auto,
    Manual,
}

fn main() {
    println!("{:?}", Mode::default());
}
```

Use `Default` with struct update syntax (`..Default::default()`) and `HashMap::entry(...).or_default()`.

## PartialEq, Eq, Hash, and Ord

Collection APIs from [Chapter 11](11_collections.md) require specific traits:

| Trait | Enables |
|-------|---------|
| `PartialEq` | `==`, `!=` |
| `Eq` | full equivalence (no `NaN`-style holes) |
| `Hash` | `HashMap` / `HashSet` keys |
| `Ord` | `BTreeMap` / `BTreeSet`, sorting |

```rust
// Playground
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UserId(u64);

fn main() {
    let a = UserId(1);
    let b = UserId(1);
    println!("{} {:?}", a == b, a);
}
```

**Wrong — `Eq` with `f64` field:**

```rust
// Playground — does not compile
// #[derive(Eq, PartialEq)]
// struct Sample { value: f64 }
// ERROR: binary operation `==` cannot be applied to type `f64` in Eq context
```

**Fix:** use integer fixed-point, ordered-float crates, or derive only `PartialEq` (no `Eq`, no `Hash` on floats).

| Goal | Minimum derives |
|------|-----------------|
| test equality | `PartialEq` (+ `Eq` if valid) |
| hash map key | `Eq + Hash` |
| btree key / sort | `Ord` (implies `Eq`) |

## Clone and Copy (brief)

| Trait | Meaning |
|-------|---------|
| `Copy` | bitwise duplicate on assign — no move ([Chapter 1](01_paradigm_shift.md)) |
| `Clone` | explicit `.clone()` — may allocate |

Derive `Clone` on small config types; skip on unique handles (file descriptors, connection pools). `#[derive(Copy, Clone)]` together when all fields are `Copy`.

## From and Into

**`From<T>`** defines conversion **into** your type. **`Into<T>`** is auto-implemented when you implement `From`:

```rust
// Playground
struct Port(u16);

impl From<u16> for Port {
    fn from(n: u16) -> Self {
        Port(n)
    }
}

fn main() {
    let p: Port = 8080.into();
    let q = Port::from(443);
    println!("{} {}", p.0, q.0);
}
```

**Rule:** implement **`From`** on your type — not `Into` on the source.

### Chaining `From` without duplication

```rust
// Playground
struct Label(String);

impl From<String> for Label {
    fn from(s: String) -> Self {
        Label(s)
    }
}

impl From<&str> for Label {
    fn from(s: &str) -> Self {
        Label(s.to_string())
    }
}

fn main() {
    let a = Label::from("hello");
    let b = Label::from(String::from("world"));
    println!("{} {}", a.0, b.0);
}
```

Std library is full of `From`: `String::from("text")`, `Vec::from([1, 2])`, etc.

**`?` and `From`:** `?` converts errors via `From` ([Chapter 8](08_errors_and_testing.md)). Implement `From<ParseIntError> for MyError` once. Then `parse()?` works in `fn -> Result<_, MyError>`.

## TryFrom, TryInto, and FromStr

**Fallible conversions** use **`TryFrom` / `TryInto`** when validation can fail:

```rust
// Playground
use std::convert::TryFrom;

struct Port(u16);

impl TryFrom<i32> for Port {
    type Error = &'static str;

    fn try_from(n: i32) -> Result<Self, Self::Error> {
        if (1..=65535).contains(&n) {
            Ok(Port(n as u16))
        } else {
            Err("port out of range")
        }
    }
}

fn main() {
    println!("{:?}", Port::try_from(8080));
    println!("{:?}", Port::try_from(0));
}
```

**`FromStr`** — parse from string slice (what `.parse()` calls):

```rust
// Playground
use std::str::FromStr;

#[derive(Debug)]
struct Port(u16);

impl FromStr for Port {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: u16 = s.parse()?;
        Ok(Port(n))
    }
}

fn main() {
    let p: Port = "8080".parse().unwrap();
    println!("{:?}", p);
}
```

| Mechanism | Input | Error type |
|-----------|-------|------------|
| `From` / `into()` | known-valid value | none |
| `TryFrom` / `try_into()` | value needing check | associated `Error` |
| `str::parse()` / `FromStr` | text | `FromStr::Err` |

**When to use which:** use `s.parse::<u16>()` for plain numbers. Use custom `FromStr` for **domain types** (`Port`, `HostPort`) with extra rules.

### TryFrom edge cases

**Wrong — `as` cast that silently truncates:**

```rust
// Playground
fn main() {
    let big: i32 = 70000;
    let p = big as u16; // wraps — no error
    println!("{}", p);  // 14464 — not what you want for ports
}
```

**Fix:** `u16::try_from(big)` or custom `TryFrom` with validation.

## AsRef and Borrow

Flexible APIs accept many input forms **without allocating**:

| Trait | Role |
|-------|------|
| `AsRef<T>` | cheap ref conversion (`String` → `&str`, `Vec<u8>` → `&[u8]`) |
| `Borrow<T>` | like `AsRef` but tied to **hashing/equality** for map keys |

```rust
// Playground
fn print_label<S: AsRef<str>>(s: S) {
    println!("{}", s.as_ref());
}

fn main() {
    print_label("literal");
    print_label(String::from("owned"));
}
```

**Why `HashMap<String, V>` accepts `get(&str)`:** keys implement `Borrow<str>`. You can look up with `&str` without allocating a `String`.

```rust
// Playground
use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    m.insert(String::from("alice"), 10);
    println!("{:?}", m.get("alice")); // &str borrows as key
}
```

### AsRef vs `&T` parameter

| Signature | Accepts | Notes |
|-----------|---------|-------|
| `fn f(s: &str)` | `&str` only | callers with `String` pass `&s` |
| `fn f(s: impl AsRef<str>)` | `&str`, `String`, `Cow<str>`, … | ergonomic public API |
| `fn f(s: &String)` | `&String` only | usually a smell |

Same pattern: `impl AsRef<Path>`, `impl AsRef<[u8]>`.

## Cow — clone on write

**`Cow<'a, T>`** (clone-on-write) is either **borrowed** or **owned**:

```rust
// Playground
use std::borrow::Cow;

fn normalize<'a>(s: Cow<'a, str>) -> Cow<'a, str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_"))
    } else {
        s
    }
}

fn main() {
    let borrowed = normalize(Cow::Borrowed("hello"));
    let owned = normalize(Cow::Borrowed("hello world"));
    println!("{} | {}", borrowed, owned);
}
```

| Variant | When |
|---------|------|
| `Cow::Borrowed(&T)` | no allocation; return as-is |
| `Cow::Owned(T)` | had to mutate or build owned data |

Call `.into_owned()` when the caller must keep an owned value after the function returns.

**API pattern:** accept `Cow<'a, str>` when you might normalize or pass through unchanged. Common in parsers and HTTP header helpers.

## Common derive sets (cheat sheet)

| Type role | Typical derives |
|-----------|-----------------|
| log / test DTO | `Debug, Clone, PartialEq` |
| config file | `Debug, Clone, Deserialize, Default` |
| map key | `Eq, Hash, Clone, Debug` |
| sortable record | `Eq, Ord, PartialOrd, Debug` |
| error enum | `Debug, Error` (thiserror) — [Chapter 8](08_errors_and_testing.md) |

Do not derive `Clone` on unique handles. Do not derive `Debug` on secrets without redaction.

## Java / Python contrast

| Idea | Java | Python | Rust |
|------|------|--------|------|
| toString | `toString()` | `__str__` | `Display` / `Debug` |
| equals / hashCode | `equals`, `hashCode` | `__eq__`, `__hash__` | `PartialEq`, `Eq`, `Hash` |
| parsing | constructors, `valueOf` | `int(s)` | `FromStr`, `TryFrom`, `.parse()` |
| flexible param | overloads | duck typing | `AsRef`, `impl Trait` |

## When the compiler says no

Common errors in this chapter:

| Error (typical) | Cause | Fix |
|-----------------|-------|-----|
| `X doesn't implement Display` | used `{}` | `{:?}` or impl `Display` |
| `the trait bound Eq is not satisfied` | `f64` or partial type | fixed-point or `PartialEq` only |
| `T: Hash` not satisfied | key not hashable | derive `Hash` + `Eq` |
| conflicting `From` impls | two `From` to same type | one canonical path |
| orphan rule | `impl Display for Vec<u8>` | newtype wrapper |
| `?` can't convert error | missing `From` impl | `From` or `map_err` |
| `Borrow` / key mismatch | wrong lookup type | use `Borrow` target (`&str` for `String` keys) |

## Idiom spotlight

> **Implement `From`, accept `impl AsRef<str>`** in public helpers. Callers pass literals or owned strings; you stay allocation-free when borrowing suffices.
>
> **`Display` for users, `Debug` for developers** — derive `Debug` freely; invest in `Display` where humans read the output.
>
> **Validate at the boundary** — `TryFrom` / `FromStr` at parse time; trust types inside the core.

## Go deeper

- [The Rust Book — Useful Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [From/Into](https://doc.rust-lang.org/std/convert/trait.From.html)
- [Borrow trait](https://doc.rust-lang.org/std/borrow/trait.Borrow.html)

## See also

- [Chapter 7: Structs, traits, and generics](07_structs_traits_generics.md) — orphan rule, `impl Trait`
- [Chapter 8: Errors and testing](08_errors_and_testing.md) — `From` in `?`, custom errors
- [Chapter 11: Collections](11_collections.md) — trait bounds on keys
- [Chapter 3: Functions](03_functions.md) — flexible parameters
- [Chapter 19: I/O](19_io_processes_bits.md) — `AsRef<Path>`, CLI path/env patterns

### Afterparty

#### Debug, Display, Default

1. **Debug vs Display** — “When derive `Debug` only vs implement `Display` for a CLI status line?”
2. **Redacted Debug** — “Struct with `api_key: String` — sketch manual `Debug` with `[REDACTED]`.”
3. **Pretty debug** — “Same struct with `{:#?}` vs `{:?}` — when does pretty-print help in tests?”
4. **Display impl** — “Implement `Display` for `Port(u16)` showing `Port(8080)` — I write `fmt`, you review.”
5. **Default enum** — “Three-variant mode enum — derive `Default` with `#[default]` on `Auto`; show update syntax.”

#### Eq, Hash, Ord

6. **Derive set quiz** — “Map key, log line, sortable row, error enum — I list derives each needs.”
7. **Eq on floats** — “Struct with `f64` field — show `Eq` derive failure; three fixes.”
8. **HashMap key** — “Why does `UserId(String)` need `Eq + Hash` for `HashMap` keys?”
9. **Ord sort** — “Sort `Vec<MyRecord>` by timestamp field — bounds needed on `MyRecord`?”

#### From, TryFrom, FromStr

10. **From chain** — “`String` → `MyLabel` via `From`; add `From<&str>` without duplicating logic.”
11. **TryFrom port** — “Port validation with custom enum error `OutOfRange` instead of `&str`.”
12. **parse vs TryFrom** — “When `s.parse::<u16>()` vs `u16::try_from(x)` vs custom `FromStr`?”
13. **FromStr type** — “Parse `host:port` into struct — sketch `FromStr` with split and two parse steps.”
14. **Silent cast trap** — “Show `70000i32 as u16` vs `TryFrom` — predict values; argue for validation.”
15. **From in errors** — “Wire `ParseIntError` into `AppError` with `From` so `?` works — list impl only.”

#### AsRef, Borrow, Cow

16. **AsRef drill** — “Rewrite three functions taking `&String` to `impl AsRef<str>`.”
17. **AsRef bytes** — “Function logging wire payload — signature with `impl AsRef<[u8]>`; accept `Vec`, `&[u8]`, array.”
18. **Borrow lookup** — “Explain `HashMap<String, V>.get(&str)` — role of `Borrow<str>`.”
19. **Cow API** — “Normalize slug: accept `Cow<str>`, return borrowed if already valid else owned.”
20. **into_owned** — “When caller needs `String` after your `Cow` helper — where call `into_owned()`?”

#### API design and capstone

21. **Newtype Display** — “Wrap `Vec<u8>` as `HexBytes` — implement `Display` without orphan violation.”
22. **Derive audit** — “Config with secrets + TOML — list safe vs unsafe derives.”
23. **Mini crate API** — “Public `HostPort { host, port }` with `Display`, `TryFrom<&str>` for `host:port` — list impl blocks only.”
24. **Capstone** — “Design public API for `RateLimit { max: u32, window_secs: u64 }`: parsing, display, equality — traits only, no bodies.”
