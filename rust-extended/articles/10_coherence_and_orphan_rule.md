# Coherence and the Orphan Rule

You cannot `impl Display for u32` in your app crate — not because the compiler is picky, but because of **coherence** rules that keep trait implementations unique and predictable.

From [Rust Core Chapter 9](../rust-core/chapters/09_modules_paths_crates.md).

---

## 1. Orphan rule in one sentence

You may implement a trait for a type only if **you define the trait or the type** (at least one is local to your crate).

This fails with **E0117** — paste into Playground:

```rust
// Playground — does not compile
use std::fmt;

impl fmt::Display for u32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

fn main() {}
```

**Takeaway:** At least one of {trait, type} must be yours.

---

## 2. Newtype pattern for foreign traits

Wrap the foreign type in a local struct:

```rust
// Playground
use std::fmt;

struct Port(u16);

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "port:{}", self.0)
    }
}
```

Use `.0` or methods to reach the inner value. [Rust Core Chapter 20](../rust-core/chapters/20_production_standards.md) prefers newtypes for domain IDs.

**Takeaway:** Newtype = local type + foreign representation + your trait impls.

---

## 3. Extension traits in your crate

Add methods without wrapping every value:

```rust
// Playground
trait Trimmed {
    fn trimmed(self) -> Self;
}

impl Trimmed for String {
    fn trimmed(self) -> Self {
        self.trim().to_string()
    }
}
```

Callers `use` your trait for the methods to appear. No orphan issue — `String` is local to std but **trait** is yours.

**Takeaway:** Extension traits add behavior; newtypes add type safety.

---

## 4. Overlapping impl pitfalls

Two impls that could apply to the same type → compile error. Specialization is unstable; coherence is strict.

Watch `impl<T> Foo for T` blank impls — they block other impls.

**Takeaway:** One winning impl per (type, trait) pair — plan impl boundaries early.

---

## 5. `From` / `TryFrom` at crate boundaries

Convert at the edge:

```rust
// Playground
struct UserId(u64);

impl From<u64> for UserId {
    fn from(v: u64) -> Self {
        UserId(v)
    }
}
```

Keeps foreign numeric types out of domain APIs.

**Takeaway:** Coherence pushes conversions and display logic through local newtypes.

---

## See also

- [Rust Core → Chapter 9: Modules, paths, and crates](../rust-core/chapters/09_modules_paths_crates.md)
- [Rust Core → Chapter 20: Production standards](../rust-core/chapters/20_production_standards.md) — newtypes
- [Rust Core → Chapter 7: Traits](../rust-core/chapters/07_structs_traits_generics.md)

## Go deeper

- [Rust Book — implementing traits](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type)
- [Rust Reference — orphan rules](https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules)
