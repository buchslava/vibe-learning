# Cancellation Safety in Async Rust

In async Rust, **cancellation is dropping the future**. No special API — if a branch loses `tokio::select!` or you stop polling, the state machine is destroyed mid-flight. That is powerful and easy to get wrong.

This article builds on [Rust Core Chapter 16](../rust-core/chapters/16_async_tokio.md) and [Pin and Unpin](03_pin_and_unpin.md).

---

## 1. Cancellation = drop the future

A future is a state machine. **Drop** means:

- Execution stops at the current state.
- All owned fields are dropped recursively.
- Child futures inside the parent are dropped too.

The future may be dropped **before the first poll** — sync prelude code in [article 02](02_sync_prelude_async_blocks.md) still ran, but the async body never started.

**Takeaway:** Not awaiting is often cancellation; side effects may be partial or skipped.

---

## 2. `tokio::select!` drops losers

`select!` polls branches concurrently; the **first to complete** wins. Other branches are **dropped immediately**.

```rust
// Cargo project — tokio with "macros", "rt-multi-thread", "time", "sync"
tokio::select! {
    _ = tokio::time::sleep(Duration::from_millis(10)) => {
        println!("timeout");
    }
    msg = rx.recv() => {
        println!("got {:?}", msg);
    }
}
```

If the timeout wins, the `recv` future vanishes — even if it was mid-operation.

Runnable demos: [`demos/async/demo_cancel_safety/`](../demos/async/demo_cancel_safety/).

**Takeaway:** `select!` winners cancel losers; plan for dropped branches.

---

## 3. Cancel-safe vs cancel-unsafe APIs

A future is **cancel-safe** if dropping it at any `.await` leaves the system in a valid state.

| API / pattern | Cancel-safe? | Why |
|---------------|--------------|-----|
| `mpsc::Receiver::recv` | Yes | No partial message |
| `Interval::tick` (reuse same interval) | Yes | Next tick is fresh |
| `read_exact` / `write_all` mid-buffer | **No** | Unknown bytes transferred |
| Loop building local `Vec` inside future | **No** | Partial progress lost |

Sync Rust `write_all` is fine; async `write_all().await` split across awaits is **not** cancel-safe if the future can be dropped mid-write.

**Takeaway:** Idempotent or all-or-nothing APIs survive cancellation; partial I/O does not.

---

## 4. Fixes — state outside, spawn, resume

**Move state out of the losing branch:**

```rust
// Cargo project
let mut collected = Vec::new();
loop {
    tokio::select! {
        _ = timeout => break,
        Some(n) = rx.recv() => collected.push(n),
    }
}
```

**Spawn cancel-unsafe work** so the task owns it until completion — dropping a `JoinHandle` does **not** cancel the task by default; use `handle.abort()` explicitly.

**Pin and resume** the same future across `select!` rounds instead of recreating it — see [Oxide RFD 400](https://rfd.shared.oxide.computer/rfd/0400).

**Takeaway:** Keep durable state outside cancelable futures; spawn or pin when you must finish.

---

## 5. Tasks vs futures

| Handle | Drop behavior |
|--------|---------------|
| Future (local) | Dropped → cancelled now |
| `JoinHandle` | Dropped → task **keeps running** |
| `JoinHandle::abort()` | Task cancelled at next `.await` |

Do not assume dropping a handle cleans up background work.

**Takeaway:** Tasks and futures have different cancellation semantics.

---

## See also

- [Rust Core → Chapter 16: Async and Tokio](../rust-core/chapters/16_async_tokio.md)
- [Rust Extended → The Sync Prelude](02_sync_prelude_async_blocks.md)
- [Rust Extended → Pin and Unpin](03_pin_and_unpin.md)

## Go deeper

- [Oxide RFD 400 — cancel safety](https://rfd.shared.oxide.computer/rfd/0400)
- [Cancelling async Rust — sunshowers](https://sunshowers.io/posts/cancelling-async-rust/)
- [Cancellation and async state machines — holk](https://blog.theincredibleholk.org/blog/2023/11/08/cancellation-async-state-machines/)
- [Tokio — `select!` docs](https://docs.rs/tokio/latest/tokio/macro.select.html)
