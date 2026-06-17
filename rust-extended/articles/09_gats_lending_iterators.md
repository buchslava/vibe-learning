# GATs and Lending Iterators

Standard `Iterator::Item` is owned — `.next()` cannot return `&self.buffer[0..n]` borrowing from the iterator. **Generic Associated Types (GATs)** let associated types take lifetime parameters, enabling **lending** iterators.

Deferred from [Rust Core Chapter 5](../rust-core/chapters/05_lifetimes.md).

---

## 1. Associated types vs GATs

Classic associated type — one `Item` for all implementations:

```rust
// Playground
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

GAT — associated type with its own generics:

```rust
// Playground — GAT syntax (Rust 1.65+)
trait LendingIter {
    type Item<'a> where Self: 'a;
    fn next(&mut self) -> Option<Self::Item<'_>>;
}
```

**Takeaway:** GATs parameterize associated types by lifetime (or type/const).

---

## 2. `Item<'a>` borrowing from `&'a self`

Each `next()` call may return a slice tied to the iterator's internal buffer:

```rust
// See demos/type-system/demo_gats/ for full Words iterator
```

The returned reference cannot outlive the iterator — the compiler enforces that through GAT lifetimes.

Demo: [`demos/type-system/demo_gats/`](../demos/type-system/demo_gats/).

**Takeaway:** Lending iterators return borrows from `self`, not owned values.

---

## 3. Streaming / sliding-window pattern

Use cases:

- Parse chunks from a buffer without copying each token.
- Walk windows over a `&str` or byte slice.
- Async stream adapters (with care — often prefer owned buffers in async).

When copying is cheap (`Copy` items), stick with normal `Iterator`.

**Takeaway:** GATs pay off when copies are expensive or semantics require borrows.

---

## 4. Object-safety limitation

Traits with GATs are **not object-safe** — no `dyn LendingIter` today. Use generics or type erasure at a higher level.

Same limitation noted in the [Rust blog GATs stabilization post](https://blog.rust-lang.org/2021/08/03/GATs-stabilization-push/).

**Takeaway:** GAT traits work with monomorphization, not trait objects.

---

## 5. When to use owned iteration instead

| Prefer `Iterator` | Prefer GAT lending |
|-------------------|-------------------|
| `map` / `collect` pipelines | Zero-copy tokenization |
| Async handlers (often clone) | Sync parsers on one buffer |
| Public API simplicity | Internal hot loops |

**Takeaway:** Default to std iterators; reach for GATs when profiling shows copy cost or API needs borrows.

---

## See also

- [Rust Core → Chapter 4: Iterators](../rust-core/chapters/04_iterators.md)
- [Rust Core → Chapter 5: Lifetimes](../rust-core/chapters/05_lifetimes.md)
- [Rust Extended → HRTB and Fn Traits](07_hrtb_and_fn_traits.md)

## Go deeper

- [Rust blog — GATs stabilization push](https://blog.rust-lang.org/2021/08/03/GATs-stabilization-push/)
- [RFC 1598 — generic associated types](https://rust-lang.github.io/rfcs/1598-generic_associated_types.html)
- [Rust Patterns — lifetime patterns (GAT section)](https://www.rust-patterns.com/book/06-lifetime-patterns.html)
