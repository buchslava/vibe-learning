# Chapter 14: Unsafe and When to Stop

## Hook

Java’s `sun.misc.Unsafe` and Python’s C extensions bypass safety for performance or FFI. Rust keeps the same power in **`unsafe` blocks** — but the **borrow checker is off inside**, and you must uphold invariants the compiler cannot verify.

## What `unsafe` allows

- Dereference raw pointers
- Call other `unsafe fn`
- Implement `unsafe trait`
- Access mutable statics

Safe Rust around `unsafe` must still be sound.

```rust
// Playground
fn main() {
    let n = 5;
    let p = &n as *const i32;
    unsafe {
        println!("{}", *p);
    }
}
```

## `unsafe fn` and `unsafe trait`

Libraries like `Vec` use `unsafe` internally; public API stays safe. `Send`/`Sync` for custom types sometimes need careful `unsafe impl` — only with proof.

## FFI sketch

Calling C from Rust (conceptual):

```rust
// Cargo only — needs libc and link flags
// extern "C" { fn strlen(s: *const u8) -> usize; }
```

Use `bindgen` or `cxx` in real projects; mind ABI and ownership across boundary.

## When **not** to use unsafe

- “Compiler annoys me” — fix design
- “Faster without measuring” — profile first
- “Avoid learning lifetimes” — unsound shortcuts

Most automation and application code never needs `unsafe`.

## Idiom spotlight

> **Encapsulate unsafe in the smallest module; document invariants in comments; test aggressively.** Prefer safe crates maintained by experts for FFI.

## Go deeper

- [Procedural macro intro](https://hightechmind.io/rust/) — 423 (boundary with unsafe traits)

## See also

- [Chapter 11: Atomics](11_atomics_and_lockfree.md)
- [Chapter 16: Hardware](16_hardware_automation_lab.md)

### Afterparty: AI Lego blocks

1. **Invariant list** — “For raw pointer to buffer + length, list 5 invariants safe wrapper must enforce.”
2. **Soundness** — “Explain ‘safe Rust can’t cause UB’ vs unsafe — one paragraph.”
3. **FFI checklist** — “Checklist for calling C library from Rust binary.”
4. ** Miri** — “What is Miri and when run it relative to unsafe changes?”
5. **Avoid** — “Review use case: speed up JSON — unsafe vs simd crate vs algorithm.”
6. **Java JNI** — “Compare JNI pitfalls to Rust FFI ownership rules.”
