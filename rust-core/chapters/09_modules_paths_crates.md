# Chapter 9: Modules, Paths, and Crates

## Hook

Packages and modules group code by namespace in **Java**, **Python**, and many other stacks. Rust uses **modules** inside a **crate** — one compiled unit. The module tree splits projects across files, controls visibility, and exposes a public API.

Most of this chapter is **Cargo only** — you need a filesystem and `cargo build`. Playground snippets show the syntax in a single file; multi-file layout is the production pattern.

## Scope — a brief tour

Module tree, `pub`, and Cargo layout — not publishing or semver policy.

| This chapter covers | Deferred |
|---------------------|----------|
| Module tree, `pub`, `use`, workspaces | `cargo publish`, semver policy |
| `#[cfg]` and Cargo `[features]` | Full cross-compilation matrix |
| Unit and integration tests | Fuzzing, property tests |
| `///` docs and `cargo doc` | Custom rustdoc themes |

## Crate, package, and workspace

| Term | Meaning |
|------|---------|
| **Crate** | One library (`rlib`) or binary (`bin`) the compiler builds from a root file |
| **Package** | One `Cargo.toml` + source that produces one or more crates |
| **Workspace** | Several packages in one repo sharing one `Cargo.lock` |

A default binary package looks like:

```
my_app/
  Cargo.toml
  src/
    main.rs      # crate root for the binary
```

A library + binary in one package:

```
my_lib/
  Cargo.toml
  src/
    lib.rs       # library crate root — `pub` API lives here
    main.rs      # binary uses the library via `use my_lib::...`
    config.rs    # submodule file
```

Run `cargo build` from the package directory. The **crate name** in `Cargo.toml` is the root for `use crate_name::...` in dependent packages.

## Declaring modules

Two ways to add a module:

**Inline** — code in the same file:

```rust
// Playground
mod math {
    pub fn double(x: i32) -> i32 {
        x * 2
    }
}

fn main() {
    println!("{}", math::double(21));
}
```

**File-backed** — in `lib.rs` or `main.rs`:

```rust
// src/lib.rs or main.rs
mod config;  // loads src/config.rs or src/config/mod.rs
```

```rust
// src/config.rs
pub fn app_name() -> &'static str {
    "gateway"
}
```

| Form | Use when |
|------|----------|
| `mod foo { ... }` | tiny helpers, tests, one-off nesting |
| `mod foo;` + `foo.rs` | normal production split |

## Paths: `self`, `super`, `crate`

Paths mirror the module tree:

| Prefix | Points to |
|--------|-----------|
| `crate::` | root of **this** crate |
| `super::` | parent module |
| `self::` | current module (often omitted) |
| `foo::bar` | child module `foo`, item `bar` |

```rust
// Playground
mod outer {
    pub fn tag() -> &'static str {
        "outer"
    }

    pub mod inner {
        pub fn full() -> String {
            format!("{}::inner", super::tag())
        }
    }
}

fn main() {
    println!("{}", outer::inner::full());
}
```

**Java:** `com.example.app.Config`. **Python:** `package.submodule`. **Rust:** explicit tree from crate root; no classpath scanning.

## Visibility: the `pub` ladder

By default, items are **private** to their module. Mark what callers may use:

| Visibility | Visible from |
|------------|--------------|
| (none) | same module + child modules |
| `pub` | anywhere that can reach the path |
| `pub(crate)` | anywhere in this crate |
| `pub(super)` | parent module |
| `pub(in path::to::module)` | specific ancestor branch |

```rust
// Playground
mod api {
    pub fn public_entry() -> i32 {
        helper()
    }

    fn helper() -> i32 {
        42
    }
}

fn main() {
    println!("{}", api::public_entry());
    // api::helper(); // ERROR: private
}
```

**Library design:** keep fields private; expose `pub fn` constructors and accessors ([Chapter 3](03_functions.md)). Hide internals so you can refactor without breaking callers.

## `use` and re-exports

`use` brings names into scope so you do not repeat long paths:

```rust
// Playground
mod shapes {
    pub mod circle {
        pub fn area(r: f64) -> f64 {
            std::f64::consts::PI * r * r
        }
    }
}

use shapes::circle;

fn main() {
    println!("{}", circle::area(2.0));
}
```

Re-export a dependency’s type under your crate’s API:

```rust
// lib.rs pattern (Cargo only — conceptual)
// pub use serde::Deserialize;
```

Callers write `use my_crate::Deserialize` instead of depending on path details you might change later.

## Binary vs library crate

| Crate | Root file | Typical role |
|-------|-----------|--------------|
| Binary | `src/main.rs` | CLI entry, `fn main`, thin wiring |
| Library | `src/lib.rs` | reusable logic, most `pub` API, unit tests |

**Cargo only** — binary calling library in the same package:

```rust
// src/lib.rs
pub fn greet(name: &str) -> String {
    format!("Hello, {name}")
}
```

```rust
// src/main.rs
fn main() {
    println!("{}", my_lib::greet("Rust"));
}
```

Replace `my_lib` with the `[package] name` from `Cargo.toml`. Keep `main` small: parse args, call library functions, map errors to exit codes ([Chapter 8](08_errors_and_testing.md)).

## Tests as modules

Unit tests live in a nested module guarded by `#[cfg(test)]`:

```rust
// Playground
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(add(2, 2), 4);
    }
}

fn main() {
    println!("{}", add(1, 1));
}
```

`cargo test` compiles and runs tests; normal `cargo build` skips them. Integration tests go in `tests/*.rs` at the package root (**Cargo only**).

## Conditional compilation and Cargo features

Gate optional serial support behind a feature flag.

**Cargo.toml:**

```toml
# Cargo only
[package]
name = "gateway"
version = "0.1.0"
edition = "2021"

[features]
default = []
serial = []

[dependencies]
serialport = { version = "4", optional = true }
```

**src/lib.rs:**

```rust
// Cargo only — conceptual
#[cfg(feature = "serial")]
pub mod serial_io {
    pub fn list_ports() -> Vec<String> {
        serialport::available_ports()
            .unwrap_or_default()
            .into_iter()
            .map(|p| p.port_name)
            .collect()
    }
}

#[cfg(not(feature = "serial"))]
pub mod serial_io {
    pub fn list_ports() -> Vec<String> {
        vec![]
    }
}
```

Build with `cargo build --features serial` to compile the real backend. Without the flag, the stub stays in the binary. The API path stays identical.

### Common `cfg` attributes

```rust
// Playground
#[cfg(test)]
fn only_in_tests() -> i32 {
    42
}

fn main() {
    #[cfg(debug_assertions)]
    println!("debug build");

    println!("running");
}
```

| Attribute | Effect |
|-----------|--------|
| `#[cfg(test)]` | compiled only for `cargo test` |
| `#[cfg(debug_assertions)]` | debug builds only |
| `#[cfg(feature = "serial")]` | enabled when feature is on |
| `cfg!(feature = "serial")` | **runtime** bool — code still compiled |

`#[cfg(...)]` removes code at compile time. `if cfg!(...) { ... }` keeps both branches in the binary.

### Feature edge case — optional dependency

```toml
# Cargo only
tokio = { version = "1", optional = true, features = ["rt", "macros"] }

[features]
async = ["dep:tokio"]
```

Enabling `async` pulls in `tokio` with the listed features — one switch for callers.

## Integration tests

Integration tests live in `tests/` at the package root. Each file is a separate crate that links your library:

```
gateway/
  Cargo.toml
  src/lib.rs
  tests/
    parse_config.rs
```

```rust
// Cargo only — tests/parse_config.rs
// use gateway::parse_port;

// #[test]
// fn reads_port_from_line() {
//     assert_eq!(parse_port("502").unwrap(), 502);
// }
```

Run with `cargo test`. Unlike `#[cfg(test)] mod tests`, integration tests only see the **public** API — they simulate external callers.

### Re-export edge case — prelude module

```rust
// Playground
mod api {
    pub mod inner {
        pub fn connect() -> i32 {
            502
        }
    }
    pub use inner::connect;
}

fn main() {
    println!("{}", api::connect());
}
```

`pub use` re-exports `connect` at the `api` level so callers skip `inner`.

## Workspaces (brief)

When one repo holds multiple packages:

```
workspace/
  Cargo.toml       # [workspace] members = ["core", "cli"]
  core/
    Cargo.toml
    src/lib.rs
  cli/
    Cargo.toml
    src/main.rs    # depends on core via path dependency
```

Each member has its own crate graph. Shared versions live in the workspace `Cargo.lock`. Start with one package. Split when a library clearly deserves reuse.

## Orphan rule (crate boundary)

You may implement **your trait** for **your type**, or either side from **your crate**. You cannot add `impl Display for Vec<u8>` in your app — both trait and type are defined elsewhere. Fix with a **newtype** wrapper ([Chapter 7](07_structs_traits_generics.md#orphan-rule-and-cross-crate-patterns)).

## Documentation comments

Public API items use `///` doc comments. Run `cargo doc --open` to browse generated HTML locally:

```rust
// Playground
/// Parses a port number from decimal text.
///
/// # Errors
/// Returns `ParseIntError` when the string is not a valid `u16`.
pub fn parse_port(s: &str) -> Result<u16, std::num::ParseIntError> {
    s.parse()
}

fn main() {
    println!("{}", parse_port("502").unwrap());
}
```

Use `# Examples`, `# Errors`, and `# Panics` sections for items callers depend on. Hide internal helpers with `#[doc(hidden)]` when re-exporting.


## When the compiler says no

Common errors in this chapter:

| Error (typical) | Cause | Fix |
|-----------------|-------|-----|
| file not found for module `foo` | missing `foo.rs` or `foo/mod.rs` | create file or use inline `mod` |
| private item | forgot `pub` on fn/struct | add `pub` or use `pub(crate)` |
| unresolved import | wrong path or typo | `use crate::...` from crate root |
| found duplicate module | `mod foo;` twice | one declaration per module |

## Idiom spotlight

> **Thin `main`, fat library.** Put logic in `lib.rs` modules; test there with `mod tests`. The binary is just the process entry point.

## Go deeper

- [The Rust Book — Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)

## See also

- [Preface](preface.md) — rustup and Cargo setup
- [Chapter 3: Functions and methods](03_functions.md) — `pub` on methods
- [Chapter 8: Errors and testing](08_errors_and_testing.md) — `mod tests`, `cargo test`
- [Chapter 10: Smart pointers](10_smart_pointers_interior_mutability.md) — `Arc` across modules
- [Chapter 7: Traits](07_structs_traits_generics.md) — orphan rule

### Afterparty

#### Layout and paths

1. **File tree** — “Design module tree for a CLI that reads config and runs commands. Directories + `mod` lines only, no bodies.”
2. **Path quiz** — “From `crate::service::worker::run`, how do I reach `crate::config::load`? Show `use` and fully qualified call.”
3. **lib vs bin** — “What belongs in `main.rs` vs `lib.rs` for a tool with 500 lines of logic?”

#### Visibility

4. **pub audit** — “List items that should be `pub` vs private in a library crate exposing `Client::connect`.”
5. **pub(crate)** — “When is `pub(crate)` better than `pub` for test helpers?”
6. **Re-export** — “Sketch `pub use` so users see `my_crate::Error` but you wrap `thiserror` internally.”

#### Crates and workspaces

7. **Workspace split** — “Two crates: `core` library + `cli` binary. Write `Cargo.toml` dependency path only.”
8. **Integration test** — “Where does `tests/smoke.rs` live and how does it `use` the library?”
9. **Orphan fix** — “I want `Display` on `Vec<u8>` — show newtype wrapper module layout.”

#### Practice

10. **Split monolith** — “Given one `main.rs` with config + parser + runner, name three modules and what each owns.”
11. **cfg test** — “Explain why `mod tests` uses `#[cfg(test)]` and `use super::*`.”
12. **Capstone** — “Generate `src/` tree for `sensor_core` library + `sensor_cli` binary in one workspace; I implement.”

#### Features and cfg

13. **Feature flag** — "Add `serial` feature gating `mod serial_io` — write `Cargo.toml` `[features]` and one `#[cfg]` line."
14. **cfg vs cfg!** — "Same debug log twice: `#[cfg(debug_assertions)]` block vs `if cfg!(debug_assertions)` — what stays in release binary?"
15. **Optional dep** — "Wire optional `tokio` behind feature `async` — show `[features]` and `dep:tokio` line."
16. **Platform gate** — "Sketch `#[cfg(target_os = "linux")]` module for `/dev/ttyUSB0` path only on Linux."

#### Integration and docs

17. **Integration layout** — "Draw tree for `tests/load_config.rs` calling public `load()` — what is invisible to the test?"
18. **pub use prelude** — "Users should call `my_crate::connect` not `my_crate::internal::connect` — show re-export."
19. **doc hidden** — "When to mark helper `#[doc(hidden)]` on a public re-export surface?"
20. **Capstone crate** — "Design `gateway` crate: `serial` feature, integration test, `///` on public parse fn — tree + TOML only."

