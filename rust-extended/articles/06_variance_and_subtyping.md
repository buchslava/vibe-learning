# Variance and Subtyping

Lifetimes are not just labels for the borrow checker — they form a **subtyping** lattice. **Variance** rules say when you may use a longer-lived reference where a shorter one is expected, and when you may not.

This article expands [Five Deep Rust Facts §5](01_five_deep_rust_facts.md). Read [Rust Core Chapter 5](../rust-core/chapters/05_lifetimes.md) first.

---

## 1. `'static` <: `'a` intuition

If `'a` is any lifetime, `'static` outlives it. So you may pass `&'static str` where `&'a str` is expected:

```rust
// Playground
fn print_str(s: &str) {
    println!("{}", s);
}

fn main() {
    let long: &'static str = "hello";
    print_str(long); // OK: subtyping
}
```

Subtyping is **compile-time only** — zero runtime cost.

Demo: [`demos/type-system/demo_variance/`](../demos/type-system/demo_variance/).

**Takeaway:** Longer lifetimes are subtypes of shorter ones for references.

---

## 2. The variance table

Given types `Sub <: Super`, how does `F<Sub>` relate to `F<Super>`?

| Type constructor | Variance in `'a` | Variance in `T` |
|------------------|------------------|-----------------|
| `&'a T` | covariant | covariant |
| `&'a mut T` | covariant | **invariant** |
| `Box<T>`, `Vec<T>` | — | covariant |
| `Cell<T>`, `UnsafeCell<T>` | — | invariant |
| `fn(T) -> U` | — | contravariant in `T`, covariant in `U` |

- **Covariant:** subtype passes through (`F<Sub>` usable as `F<Super>`).
- **Invariant:** must match exactly.
- **Contravariant:** reversed (only fn args in Rust).

**Takeaway:** `&mut T` is invariant in `T` — that blocks the Animal/Cat soundness hole.

---

## 3. Structs inherit field variance

A struct's variance over type parameter `A` is the **most restrictive** variance of all fields using `A`:

```rust
// Playground — types for intuition
use std::marker::PhantomData;

struct Covariant<'a, T: 'a>(PhantomData<&'a T>);
struct Invariant<T>(PhantomData<fn(T)>);
```

If one field uses `A` invariantly, the whole struct is invariant in `A`.

**Takeaway:** Field types decide your generic variance — choose markers deliberately.

---

## 4. Contravariance in function arguments

```rust
// Playground — conceptual
fn takes_fn(f: fn(&'static str)) {}
// You may pass a function accepting ANY lifetime — it accepts 'static too.
```

Contravariance rarely bites in application code. It matters for HRTB ([article 07](07_hrtb_and_fn_traits.md)).

**Takeaway:** Fn trait bounds use contravariance on arguments; HRTB makes that explicit.

---

## 5. Why `&mut T` invariant over `T`

If `&mut Cat` were a subtype of `&mut dyn Animal`, you could store a `Dog` through a `Cat` slot — unsound. Invariance forbids that conversion.

This fails to compile — paste into Playground:

```rust
// Playground — does not compile
trait Animal {}
struct Cat;
struct Dog;
impl Animal for Cat {}
impl Animal for Dog {}

fn as_animal(cat: &mut Cat) {
    let _animal: &mut dyn Animal = cat; // E: `&mut Cat` is not coercible to `&mut dyn Animal`
}

fn main() {}
```

**Takeaway:** Invariance on `&mut` is what makes exclusive mutation sound with subtyping elsewhere.

---

## See also

- [Rust Core → Chapter 5: Lifetimes](../rust-core/chapters/05_lifetimes.md)
- [Rust Extended → Five Deep Rust Facts](01_five_deep_rust_facts.md)
- [Rust Extended → Drop Check and PhantomData](11_drop_check_phantom_data.md)

## Go deeper

- [Rustonomicon — Subtyping and Variance](https://doc.rust-lang.org/nomicon/subtyping.html)
- [Rust Reference — subtyping](https://doc.rust-lang.org/reference/subtyping.html)
- [Rust Patterns — lifetime patterns](https://www.rust-patterns.com/book/06-lifetime-patterns.html)
