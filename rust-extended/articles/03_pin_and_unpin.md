# Pin and Unpin: Why Async Needs Them

Every `async fn` desugars to a state machine that may hold **pointers into its own stack frame**. If that frame moves, those pointers dangle. **Pinning** is how Rust keeps self-referential futures sound without giving up safe code.

This article assumes [Rust Core Chapter 16](../rust-core/chapters/16_async_tokio.md) and complements [The Sync Prelude](02_sync_prelude_async_blocks.md).

---

## 1. Why moves break self-references

An async block can borrow from its own locals across `.await` points. Conceptually:

```
State 0: create String "hello"
State 1: hold &self.text while sleeping  ← self-reference
State 2: return self.text.len()
```

If the whole struct **moves** in memory between polls, the internal reference points at the old address. That is use-after-move — undefined behavior.

**Takeaway:** Async state machines are often self-referential; moving them mid-poll is unsound.

---

## 2. `Pin<&mut T>` vs `&mut T`

`Pin<P>` wraps a pointer `P` and promises: safe code will not move the **pointee** through this handle unless `T: Unpin`.

`Future::poll` takes `Pin<&mut Self>` precisely so executors cannot move your future while polling:

```rust
// Conceptual signature from std::future::Future
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
```

You still get shared access via `Pin::as_ref`. The restriction targets **exclusive** access that could move or replace the value.

Runnable demo: [`demos/async/demo_pin/`](../demos/async/demo_pin/).

**Takeaway:** Pin blocks moves through `&mut`; shared refs still work.

---

## 3. `Unpin` and `PhantomPinned`

Most Rust types implement **`Unpin`** automatically — they have no address-sensitive state. For those types, `Pin<&mut T>` behaves like ordinary `&mut T`.

Types that **must not move** opt out by including a **`PhantomPinned`** field (or another `!Unpin` field). Then only `unsafe` code can pin them correctly.

| Type | Typical pinning need |
|------|---------------------|
| `i32`, `String`, `Vec` | `Unpin` — no special care |
| Manual self-referential `Future` | Often `!Unpin` without careful layout |
| Compiler-generated async state | Handled by compiler + `Unpin` when safe |

**Takeaway:** `Unpin` is the default; self-referential types opt out.

---

## 4. `Pin::new_unchecked` contract

Safe `Pin::new` only works when `T: Unpin`. For `!Unpin` values on the stack, you use **`Pin::new_unchecked`** and promise:

1. The value will not move until dropped.
2. You will not expose `&mut T` that could move it.

Violating that contract is **unsafe** and can cause UB. Library code (Tokio, futures crates) hides most of this; app authors rarely call `new_unchecked` directly.

**Takeaway:** Pinning is a proof obligation; `unsafe` marks where you certify it.

---

## 5. When app devs actually need Pin

| Situation | Action |
|-----------|--------|
| Normal `async fn` / `.await` | Ignore Pin — compiler handles it |
| Implement `Future` by hand | Use `Pin` in `poll`; see demo |
| Error mentions `Pin` on spawn | Usually `Send` / capture issue ([article 02](02_sync_prelude_async_blocks.md)) |
| Self-referential struct | Pin + `!Unpin`; read Nomicon |

Generated async code is `Unpin` when the compiler can prove moving is safe. Pin shows up in **API signatures** and **manual futures**, not in every handler.

**Takeaway:** Learn Pin for custom futures and compiler errors — not for every `async fn`.

---

## See also

- [Rust Core → Chapter 16: Async and Tokio](../rust-core/chapters/16_async_tokio.md)
- [Rust Extended → The Sync Prelude](02_sync_prelude_async_blocks.md)
- [Rust Extended → Five Deep Rust Facts §5](01_five_deep_rust_facts.md) — `PhantomData` intro

## Go deeper

- [Rustonomicon — Pin](https://doc.rust-lang.org/nomicon/pin.html)
- [std::pin module docs](https://doc.rust-lang.org/stable/std/pin/)
- [Why is std::pin::Pin so weird? — Sander Saares](https://sander.saares.eu/2024/11/06/why-is-stdpinpin-so-weird/)
- [RFC 2349 — pin API](https://rust-lang.github.io/rfcs/2349-pin.html)
