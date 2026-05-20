# Chapter 1: Paradigm Shift

## Hook

Java and Python let you **share** data freely: references, object graphs, mutable globals. Rust says: **one owner at a time**, checked at compile time. That single rule replaces a garbage collector *and* most data races — but only if you learn to read the compiler as a collaborator, not an adversary.

## Compiled vs interpreted

| | Java | Python | Rust |
|---|------|--------|------|
| Execution | Bytecode on JVM | Interpreter / bytecode | Native machine code |
| Type checks | Mostly compile-time | Runtime | Compile-time |
| Memory | GC | GC (refcount + cycle GC) | Ownership + deterministic drop |

Rust has **no GC**. When a value’s owner goes out of scope, `drop` runs immediately. No stop-the-world pauses — valuable for automation loops and low-latency tools.

## Ownership vs garbage collection

**Python:** `b = a` makes two names point at one list; mutating through `b` affects `a`.

**Java:** references to the same object; GC reclaims when unreachable.

**Rust:** `let s2 = s1` for a `String` **moves** ownership. `s1` is invalid afterward unless you borrow or clone.

```rust
// Playground
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // move
    println!("{}", s2);
    // println!("{}", s1); // would not compile
}
```

Think of a move as **renaming** the value, not copying.

## Zero-cost abstractions

Rust’s iterators, traits, and generics are designed to compile down to code as tight as hand-written C **when you use release builds** (`cargo build --release`). You do not pay for “elegant” APIs at runtime the way heavy OOP patterns can cost in Java.

## Fearless concurrency (preview)

The same borrow rules that prevent use-after-free also prevent **data races** in safe Rust: you cannot mutate shared state from two threads without synchronization (`Mutex`, channels, atomics — Part II). The compiler enforces this; you do not rely on discipline alone.

## Idiom spotlight

> **Let the type system carry intent.** Prefer `Result` and `Option` over sentinel values (`null`, `-1`, magic strings). You will formalize this in [Chapter 5](05_types_enums_pattern_matching.md) and [Chapter 7](07_errors_and_testing.md).

## Playground: stack vs heap (conceptual)

```rust
// Playground
fn main() {
    let x: i32 = 42;           // stack
    let s = String::from("hi"); // heap, owned
    let r = &s;                // borrow, no copy
    println!("{} {}", x, r);
}
```

## Go deeper

- [Functional Rust — Option basics](https://hightechmind.io/rust/)
- Archive: [CHAPTER_01 §2.1](../archive/CHAPTER_01_RUST_BASICS.md) (extended Python comparison)

## See also

- [Chapter 3: Ownership](03_ownership_borrowing.md)
- [Chapter 10: Multithreading](10_multithreading.md)

### Afterparty: AI Lego blocks

1. **GC quiz** — “Quiz me: for 8 snippets, say whether Java GC, Python refcount, or Rust drop applies; then explain Rust’s case.”
2. **Move drill** — “Give 5 tiny Rust snippets; I predict compile error or ok; you reveal answers with one-line fixes.”
3. **Latency story** — “Explain stop-the-world GC pauses vs Rust drop for a 1 kHz control loop — 200 words, no jargon pile-up.”
4. **Java habit** — “I wrote `clone()` everywhere in Java; how should I think about `clone()` in Rust? When is `.clone()` idiomatic vs a smell?”
5. **Paradigm essay** — “In 150 words: what does ‘fearless concurrency’ mean if I only know Python’s GIL?”
6. **Refactor fantasy** — “Take this Python class with shared mutable list; sketch the Rust struct + ownership split without full code.”
