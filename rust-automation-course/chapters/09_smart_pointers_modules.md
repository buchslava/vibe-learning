# Chapter 9: Smart Pointers and Modules

## Hook

Python references everything; Java objects live on the heap. Rust defaults to **stack + unique ownership**. When you need heap indirection or shared ownership, reach for smart pointers — deliberately.

## `Box<T>` — heap, single owner

```rust
// Playground
fn main() {
    let b = Box::new(42);
    println!("{}", b);
}
```

Use for trait objects (`Box<dyn Trait>`) or large recursive types (e.g. trees).

## `Rc<T>` / `Arc<T>` — shared ownership

- **`Rc`**: single-threaded reference counting
- **`Arc`**: atomic ref count, thread-safe

```rust
// Playground
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a);
    println!("{} refs", Rc::strong_count(&a));
    println!("{} {}", a, b);
}
```

`Arc` + `Mutex` is a common multi-thread pattern ([Chapter 10](10_multithreading.md)).

## Modules and `pub`

**Cargo only** layout:

```
src/
  lib.rs      // pub mod config;
  config.rs   // pub fn load() -> ...
  main.rs     // use crate::config;
```

```rust
// Playground — single-crate mental model
mod math {
    pub fn double(x: i32) -> i32 { x * 2 }
}

fn main() {
    println!("{}", math::double(21));
}
```

| Keyword | Meaning |
|---------|---------|
| `mod` | declare module |
| `pub` | visible outside |
| `use` | import path |
| `crate::` | this package root |

Workspaces: multiple packages in one repo — each with its own `Cargo.toml`.

## Idiom spotlight

> **Reach for `Box`/`Arc` when ownership is genuinely shared or recursive — not by default.** Most data should stay owned or borrowed.

## Go deeper

- [Arc threads](https://hightechmind.io/rust/) — 109

## See also

- [Chapter 10: Threads + Arc](10_multithreading.md)
- [Chapter 2: Cargo](02_toolchain_and_types.md)

### Afterparty: AI Lego blocks

1. **Box why** — “When is `Box<[T]>` better than `Vec<T>` on stack semantics? Two cases.”
2. **Rc cycle** — “Explain why `Rc` cycles leak memory; contrast Rust with Python cycles.”
3. **Module split** — “Split monolithic main.rs into lib + bin; list file tree only, I implement.”
4. **pub audit** — “What should be `pub` in a library crate vs kept private?”
5. **Arc Mutex sketch** — “Diagram thread-safe cache with Arc<Mutex<HashMap>> — no full code.”
6. **Java heap** — “Map Java ‘everything is reference’ to Rust ownership + when Arc applies.”
