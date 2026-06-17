# Drop Order and ManuallyDrop

Rust runs destructors automatically — but **order matters** for resources that depend on each other. **`ManuallyDrop`** lets you control when a field's destructor runs.

From [Rust Core Chapter 10](../rust-core/chapters/10_smart_pointers_interior_mutability.md) `Drop` section.

---

## 1. RFC 1857 drop order

| What | Drop order |
|------|------------|
| Local variables | Reverse of declaration |
| Struct fields | Reverse of declaration order |
| Function params | After body, reverse of binding order |
| Tuple fields | Same as struct |

Demo: [`demos/memory/demo_drop_order/`](../demos/memory/demo_drop_order/) prints `drop second` then `drop first`.

**Takeaway:** Last declared local drops first; first declared field drops last.

---

## 2. Struct field surprises

```rust
// Playground
struct Pair {
    first: Resource,
    second: Resource,
}
// second drops before first
```

If `first` must outlive `second` during teardown, declaration order is part of your API.

**Takeaway:** Field order is drop order — document dependencies.

---

## 3. `ManuallyDrop` for controlled teardown

```rust
// Playground
use std::mem::ManuallyDrop;

struct Controlled {
    early: LogOnDrop,
    late: ManuallyDrop<LogOnDrop>,
}

impl Drop for Controlled {
    fn drop(&mut self) {
        unsafe { ManuallyDrop::drop(&mut self.late) };
        // early drops after this impl returns
    }
}
```

Run `cargo run -p demo_drop_order` from the `rust-extended` root (field order, then `ManuallyDrop`).

**Takeaway:** `ManuallyDrop` defers drop until you call `drop` explicitly.

---

## 4. `mem::forget` vs leak vs skip destructor

| Mechanism | Runs `Drop`? | Memory freed? |
|-----------|--------------|---------------|
| Normal scope end | Yes | Yes (for owned heap) |
| `mem::forget` | No | Leaked |
| `ManuallyDrop` until dropped | Deferred | When you drop |

See [Five Deep Rust Facts §2](01_five_deep_rust_facts.md) — forgetting is safe but may leak.

**Takeaway:** `forget` skips drop; `ManuallyDrop` schedules it.

---

## 5. When relying on order is fragile

Refactors that reorder fields or locals change drop order silently. Prefer explicit `close()` methods or scopes for critical teardown (sockets, locks).

**Takeaway:** Do not depend on drop order for correctness unless the type documents it.

---

## See also

- [Rust Core → Chapter 10: Smart pointers](../rust-core/chapters/10_smart_pointers_interior_mutability.md)
- [Rust Extended → Five Deep Rust Facts §2](01_five_deep_rust_facts.md)
- [Rust Extended → Drop Check and PhantomData](11_drop_check_phantom_data.md)

## Go deeper

- [RFC 1857 — stabilized destructors](https://rust-lang.github.io/rfcs/1857-stabilized-destructors.html)
- [Rustonomicon — drop check (order note)](https://doc.rust-lang.org/nomicon/dropck.html)
- [`ManuallyDrop` docs](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html)
