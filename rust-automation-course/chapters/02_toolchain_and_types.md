# Chapter 2: Toolchain and Types

## Hook

Python runs `python script.py`. Java builds JARs with Maven/Gradle. Rust’s daily driver is **Cargo**: one manifest (`Cargo.toml`), one entry (`src/main.rs`), reproducible deps. Types are not bureaucracy — they are how the compiler enforces ownership before you run anything.

## rustup and Cargo

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo new my_app --bin
cd my_app && cargo run
```

| Concept | Python | Java | Rust / Cargo |
|---------|--------|------|--------------|
| Package manager | pip | Maven/Gradle | Cargo |
| Lockfile | pip-tools / poetry | pom lock | `Cargo.lock` |
| Entry | `main.py` | `public static void main` | `fn main()` in `src/main.rs` |
| Registry | PyPI | Maven Central | [crates.io](https://crates.io) |

`env!("CARGO_PKG_NAME")` is a **compile-time** string from `Cargo.toml`, not an OS environment variable.

## Scalar types

| Rust | Approx. Java | Approx. Python |
|------|--------------|----------------|
| `i32`, `u64`, `f64` | `int`, `long`, `double` | `int`, `float` (unbounded int is different) |
| `bool` | `boolean` | `bool` |
| `char` | — (use UTF-16 code units) | one Unicode scalar |
| `()` | `void` (in expressions) | `None` as return only |

Integers default-infer; always suffix or annotate when it matters: `42u8`, `let x: i64 = 0`.

## Variables and mutability

```rust
// Playground
fn main() {
    let x = 10;
    let mut y = 20;
    y += x;
    println!("{}", y);
}
```

`let` is immutable unless `mut`. Java’s `final` is closer to default `let` than to Python’s casual rebinding.

## Control flow

```rust
// Playground
fn main() {
    let n = 7;
    let label = if n % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", n, label);

    for i in 0..3 {
        println!("i = {}", i);
    }
}
```

`if` is an **expression** (returns a value). Ranges `0..3` are half-open, like Python’s `range(3)`.

## `match` preview

Full power in [Chapter 5](05_types_enums_pattern_matching.md); here, integers:

```rust
// Playground
fn main() {
    let code = 404;
    let msg = match code {
        200 => "ok",
        404 => "not found",
        _ => "other",
    };
    println!("{}", msg);
}
```

## Idiom spotlight

> **One binary, one manifest.** Prefer `cargo run` and `cargo test` over invoking `rustc` directly. Commit `Cargo.lock` for applications (automation tools, CLIs), not always for libraries.

## Cargo lab: first project

**Cargo only** — create locally:

```toml
# Cargo.toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"
```

```rust
// src/main.rs
fn main() {
    println!("Hello from {}", env!("CARGO_PKG_NAME"));
}
```

## Go deeper

- [crates.io](https://crates.io) — discover dependencies
- Archive: [CHAPTER_01 §1](../archive/CHAPTER_01_RUST_BASICS.md)

## See also

- [Chapter 3: Ownership](03_ownership_borrowing.md)
- [Chapter 9: Modules](09_smart_pointers_modules.md)

### Afterparty: AI Lego blocks

1. **Cargo vs pip** — “Compare Cargo.toml + Cargo.lock to requirements.txt + venv in 8 bullet points; include reproducibility.”
2. **Type annotate** — “Give 5 Rust expressions where inference fails; I add types; you check.”
3. **match warm-up** — “Extend my match on HTTP codes to include redirects and server errors; exhaustiveness hints only.”
4. **Java bridge** — “Map Java `int`/`long`/`BigInteger` habits to Rust integer types for a finance-lite exercise.”
5. **Playground drill** — “Rewrite this Python loop over range(10) as idiomatic Rust `for`; explain `..` vs `..=`.”
6. **env! vs std::env** — “When do I use `env!` vs `std::env::var`? Three real scenarios.”
