# MaybeUninit and Uninitialized Memory

Stack and heap memory is not “zero by default” in the sense safe Rust exposes. **`MaybeUninit<T>`** is the supported way to reserve storage and initialize later — with a strict **`assume_init`** contract.

Deferred from [Rust Core Chapter 18](../rust-core/chapters/18_unsafe_and_internals.md).

---

## 1. Uninitialized memory is not `0`

Reading uninitialized bytes is undefined behavior. Safe Rust initializes bindings; unsafe and low-level code must not pretend otherwise.

```rust
// Playground — safe code always initializes
let x = 0_u32; // fully initialized
```

**Takeaway:** Every safe binding starts initialized; bypassing that requires `MaybeUninit` or `unsafe`.

---

## 2. `MaybeUninit<T>` API surface

| Operation | Role |
|-----------|------|
| `uninit()` | Create uninitialized slot |
| `write(value)` | Initialize in place |
| `assume_init()` | Read as `T` — **unsafe** until proven init |
| `assume_init_drop()` | Drop initialized value — **unsafe** |

Demo: [`demos/memory/demo_maybe_uninit/`](../demos/memory/demo_maybe_uninit/) — growable buffer.

**Takeaway:** `MaybeUninit` separates allocation from initialization.

---

## 3. `assume_init` soundness contract

You may call `assume_init` only when:

1. The slot was initialized with `write`, or
2. You copied from another initialized `T`, or
3. You constructed via other documented safe paths.

Violating this is UB — Miri catches it. Paste into a file and run `cargo +nightly miri run`:

```rust
// Miri — undefined behavior
use std::mem::MaybeUninit;

fn main() {
    let slot = MaybeUninit::<String>::uninit();
    let _s = unsafe { slot.assume_init() };
}
```

**Takeaway:** `assume_init` is a proof obligation, like `unsafe`.

---

## 4. Array initialization patterns

`Vec<MaybeUninit<T>>` + length counter avoids double-drop during growth:

- On push: `write` new element, bump len.
- On drop: `assume_init_drop` each initialized slot only.

Pattern used inside `Vec` itself before spare capacity is written.

**Takeaway:** Track “how many slots are init” separately from capacity.

---

## 5. Relation to `mem::zeroed`

`zeroed()` for all `T` is unsound for many types (references, `NonNull`). Prefer `MaybeUninit::uninit()` and explicit init.

**Takeaway:** Do not zero-fill arbitrary `T`; initialize explicitly.

---

## See also

- [Rust Core → Chapter 18: Unsafe and internals](../rust-core/chapters/18_unsafe_and_internals.md)
- [Rust Extended → Miri Workflow](14_miri_workflow.md)
- [Rust Extended → Drop Order](05_drop_order_manually_drop.md)

## Go deeper

- [Rustonomicon — uninitialized memory](https://doc.rust-lang.org/nomicon/uninitialized.html)
- [`MaybeUninit` docs](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html)
- [RFC 1892 — MaybeUninit](https://rust-lang.github.io/rfcs/1892-uninitialized-memory.html)
