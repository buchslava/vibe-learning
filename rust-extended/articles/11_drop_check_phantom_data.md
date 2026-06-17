# Drop Check and PhantomData

Generic types with **`Drop`** must not access data that might already be destroyed. The **drop checker (dropck)** enforces that generic parameters **outlive** the container — unless you mark ownership with **`PhantomData`**.

Expands [Five Deep Rust Facts §5](01_five_deep_rust_facts.md) and [Variance and Subtyping](06_variance_and_subtyping.md).

---

## 1. Sound generic drop rule

**Big rule:** For a generic type to soundly implement `Drop`, its type/lifetime parameters must strictly outlive the dropping value — unless the destructor proves it never accesses them.

The classic failure:

```rust
// Playground — does not compile
struct Inspector<'a, T: 'a> {
    part: &'a T,
}

impl<'a, T> Drop for Inspector<'a, T> {
    fn drop(&mut self) {}
}
```

`T` could be dropped before `Inspector` — dropck rejects.

Demo: [`demos/memory/demo_drop_check/`](../demos/memory/demo_drop_check/) — `Owns<T>` with `PhantomData<T>` passes dropck.

**Takeaway:** Drop + generic parameters ⇒ dropck asks “can `Drop` touch expired data?”

---

## 2. The Inspector anti-pattern

Storing `&'a T` in a type that implements `Drop` over `'a`/`T` looks innocent but implies the destructor might read through the reference after `T` is gone.

Fixes:

- Remove `Drop` if you do not need it.
- Own the data (`T` not `&T`).
- Use correct `PhantomData` markers for unsafe wrappers.

**Takeaway:** Borrowed fields + custom `Drop` trigger dropck — by design.

---

## 3. `PhantomData<T>` vs `PhantomData<fn(T)>`

| Marker | Variance over `T` | Typical meaning |
|--------|-------------------|-----------------|
| `PhantomData<T>` | covariant | owns `T` logically |
| `PhantomData<&T>` | covariant in `'a`, covariant in `T` | borrows `T` |
| `PhantomData<fn(T)>` | invariant | may accept or produce `T` |

Demo: [`demos/memory/demo_drop_check/`](../demos/memory/demo_drop_check/) — `Owns<T>` with `PhantomData<T>` passes dropck.

**Takeaway:** Phantom markers tell dropck and variance how you treat unused type parameters.

---

## 4. `#[may_dangle]` (unstable)

Library authors may use unstable `#[may_dangle]` on parameters the destructor **never** accesses. Do not use in application code without understanding the unsafety proof.

See [RFC 1327](https://rust-lang.github.io/rfcs/1327-dropck-param-eyepatch.html).

**Takeaway:** `may_dangle` is an expert escape hatch — prefer ownership modeling.

---

## 5. Links to variance and unsafe

- Variance table: [article 06](06_variance_and_subtyping.md)
- Raw pointers in safe wrappers: [Rust Core Chapter 18](../rust-core/chapters/18_unsafe_and_internals.md)
- Zero-sized markers intro: [article 01 §5](01_five_deep_rust_facts.md)

**Takeaway:** Dropck, variance, and `PhantomData` are one system for safe generic containers.

---

## See also

- [Rust Core → Chapter 18: Unsafe and internals](../rust-core/chapters/18_unsafe_and_internals.md)
- [Rust Extended → Five Deep Rust Facts](01_five_deep_rust_facts.md)
- [Rust Extended → Variance and Subtyping](06_variance_and_subtyping.md)

## Go deeper

- [Rustonomicon — drop check](https://doc.rust-lang.org/nomicon/dropck.html)
- [Rustonomicon — PhantomData](https://doc.rust-lang.org/nomicon/phantom-data.html)
- [RFC 1327 — dropck eyepatch](https://rust-lang.github.io/rfcs/1327-dropck-param-eyepatch.html)
