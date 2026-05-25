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

Return an iterator without naming the concrete adapter type:

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

Every return path must produce the **same** concrete type. Different iterator types in each arm fail to compile. Use `Box<dyn Iterator<Item = f64>>` or an enum instead.

## Draining with `mem::take`

Move inner data out of a struct while leaving a valid empty value behind ([Chapter 10](10_smart_pointers_interior_mutability.md)):

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
    buf.drain_into(&mut batch);
    println!("batch={:?} buf={:?}", batch, buf.inner);
}
```

`take` replaces `inner` with `Vec::default()` — safe because an empty `Vec` is valid.

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

### Function edge cases

**Wrong — mismatched `impl Trait` return arms:**

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

## Idiom spotlight

> **Borrow in parameters, own at boundaries.** Library helpers take `&str` and `&[u8]`; store `String` / `Vec` only when the data outlives the call or crosses a thread/channel boundary.

## Go deeper

- [The Rust Book — Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Methods](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

## See also

- [Chapter 2: Types and expressions](02_types.md) — types in signatures
- [Chapter 1: Ownership](01_paradigm_shift.md#ownership-vs-garbage-collection) — move vs borrow in parameters
- [Chapter 4: Iterators](04_iterators.md) — functions as pipeline steps
- [Chapter 7: Structs, traits, and generics](07_structs_traits_generics.md) — `impl`, traits, generics
- [Chapter 8: Errors and testing](08_errors_and_testing.md) — `Result` and `?`

### Afterparty

#### Signatures and ownership

1. **Parameter audit** — “Five function signatures for logging: `&str`, `String`, `&String`, `Cow<str>`, `impl AsRef<str>`. I pick one per use case; you explain ownership cost.”
2. **Move vs borrow** — “Snippet calls `process(s)` then uses `s` again. I explain the error and show two fixes (`&s`, clone).”
3. **Java map** — “Map Java method `void consume(List<String> xs)` to idiomatic Rust — owned vs borrowed slice of strings.”

#### Methods and impl

4. **impl block** — “Struct `Timer` with `start`, `elapsed`, `reset`. I write `impl`; you check `&self` vs `&mut self`.”
5. **Associated fn** — “When is `Type::new()` idiomatic vs `Default::default()`? One example each.”
6. **Consume self** — “Method `fn into_inner(self) -> Vec<u8>` — why must it take `self` by value?”

#### Returns and control flow

7. **Semicolon trap** — “Three tiny functions: one returns `i32` correctly, two fail due to `;`. I fix them.”
8. **Early return** — “Rewrite nested `if` in a parser as early `return None` / `?` style.”
9. **Unit vs value** — “Which functions should return `()` vs `bool` vs `Option<T>`? Three CLI helper names, I choose.”

#### Generics and errors preview

10. **Generic bounds** — “Fix `fn max(a: T, b: T)` without bounds; add `T: Ord` or `PartialOrd`.”
11. **Result signature** — “Design `fn read_config(path: &str) -> Result<Config, ...>`; list error variants, no body.”
12. **Capstone** — “Split a 40-line `main` into 4 functions with clear signatures; list names and params only, I implement.”

#### impl Trait and drain

13. **impl Iterator return** — "Write `top_n(vals: &[f64], n: usize) -> impl Iterator<Item = f64>` — explain why two different iterator types in `if` arms fail."
14. **mem take drain** — "Buffer struct drains `Vec<u8>` to caller via `mem::take` — show before/after inner field."
15. **where clause** — "Rewrite cluttered `fn f<T: A + B + C>(x: T)` with `where` block — same behaviour."
16. **Self return** — "Method `fn into_inner(self) -> Vec<u8>` on wrapper — why `Self` not concrete type name?"
17. **const fn** — "One `const fn` port validator `fn is_valid(p: u16) -> bool` — what can and cannot run at compile time?"
18. **Capstone signatures** — "CLI tool: four functions `load_config`, `parse_port`, `run`, `main` — list signatures with ownership only."

