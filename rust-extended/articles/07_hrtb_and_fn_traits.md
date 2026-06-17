# HRTB and the Magic Fn Traits

Closures can accept `&str` with **any** lifetime — but how do you write that in a trait bound? **Higher-Ranked Trait Bounds (HRTB)** use `for<'a>` to mean “for all lifetimes `'a`.”

This article answers the deferral in [Rust Core Chapter 5](../rust-core/chapters/05_lifetimes.md) and [Chapter 12](../rust-core/chapters/12_closures.md).

---

## 1. Why `Fn(&str)` needs a lifetime

A closure stored in a struct may be called with a borrow of `self.data`. The lifetime on that borrow is **not known** when you write the struct — it depends on each `call()`:

```rust
// Playground — simplified problem
struct Holder<F> {
    data: String,
    func: F,
}
// What bound on F? Fn(&??? String) -> ???
```

You cannot pick one lifetime `'a` on the struct — `call` must work for **whatever** lifetime `&self` has at call time.

**Takeaway:** Stored closures need lifetime-polymorphic bounds, not a single `'a`.

---

## 2. `for<'a> Fn(&'a T)` desugaring

HRTB quantifies over lifetimes:

```rust
// Playground
where for<'a> F: Fn(&'a str) -> &'a str
```

Read `for<'a>` as: “for every choice of `'a`, `F` must implement this.”

Rust also elides this for many closure traits — `Fn(&str)` is shorthand for `for<'a> Fn(&'a str)`.

Demo: [`demos/type-system/demo_hrtb/`](../demos/type-system/demo_hrtb/).

**Takeaway:** HRTB is how Rust types “works with any lifetime.”

---

## 3. Returning `impl for<'a> Fn(&'a str)`

When a function **returns** a closure that must accept any borrow:

```rust
// Playground — pattern only
fn make_printer() -> impl for<'a> Fn(&'a str) + Clone {
    |s| println!("{}", s)
}
```

Without `for<'a>`, the returned closure might be tied to one lifetime too long — callers with stack-local strings fail.

**Takeaway:** Returned closures often need HRTB on the opaque return type.

---

## 4. Connection to capture modes

A `move` closure that **owns** its data does not need HRTB on input — it takes `String`, not `&str`. HRTB appears when the closure **borrows** its environment or when a helper accepts arbitrary references.

See [Rust Core Chapter 12](../rust-core/chapters/12_closures.md) capture modes.

**Takeaway:** HRTB and capture mode interact — owned closures sidestep many HRTB errors.

---

## 5. Where else HRTB appears

Mostly **`Fn` / `FnMut` / `FnOnce`** and some trait-object coercions. GATs and lending iterators ([article 09](09_gats_lending_iterators.md)) solve different problems.

Using `'static` instead of `for<'a>` fails when the closure must borrow stack locals — try in Playground:

```rust
// Playground — does not compile without HRTB
fn make_printer<'a>(s: &'a str) -> impl Fn() + 'a {
    || println!("{}", s)
}

fn main() {
    let printed = {
        let local = String::from("hi");
        make_printer(&local) // closure would outlive `local`
    };
    printed();
}
```

**Takeaway:** HRTB is rare outside closure traits — but essential there.

---

## See also

- [Rust Core → Chapter 5: Lifetimes](../rust-core/chapters/05_lifetimes.md)
- [Rust Core → Chapter 12: Closures](../rust-core/chapters/12_closures.md)
- [Rust Extended → Variance and Subtyping](06_variance_and_subtyping.md)

## Go deeper

- [Rustonomicon — HRTB](https://doc.rust-lang.org/nomicon/hrtb.html)
- [RFC 0387 — higher-ranked trait bounds](https://rust-lang.github.io/rfcs/0387-higher-ranked-trait-bounds.html)
- [Rust Reference — higher-ranked trait bounds](https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds)
