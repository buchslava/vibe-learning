# Chapter 11: Atomics and Lock-Free Basics

## Hook

Java has `java.util.concurrent.atomic.*`. Python hides atomics behind the GIL for CPython. Rust exposes **`std::sync::atomic`** for lock-free counters and flags when mutex overhead is too high — but memory ordering is part of the contract.

## Atomic types

`AtomicBool`, `AtomicI32`, `AtomicUsize`, etc. Operations:

- `load`, `store`
- `fetch_add`, `compare_exchange`

```rust
// Playground
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let n = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..4 {
        let c = Arc::clone(&n);
        handles.push(thread::spawn(move || {
            c.fetch_add(1, Ordering::Relaxed);
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("{}", n.load(Ordering::Relaxed));
}
```

## Memory ordering (practical subset)

| Ordering | Typical use |
|----------|-------------|
| `Relaxed` | counters, stats where races only lose updates visually |
| `Acquire` / `Release` | publish data between threads |
| `SeqCst` | simplest mental model; slightly slower |

For automation **metrics and shutdown flags**, `Relaxed` or `AcqRel` on a single atomic is often enough. Study formal models before writing lock-free queues.

## Atomics vs `Mutex`

| | Atomics | Mutex |
|---|---------|-------|
| Best for | counters, flags | complex invariants |
| Composability | easy ops only | arbitrary critical sections |
| Debugging | subtle ordering bugs | coarser, clearer |

## Idiom spotlight

> **Start with `Mutex`; switch to atomics when profiling shows lock contention on a hot counter.** Do not build custom lock-free structures without need.

## Go deeper

- [Atomic types — lock-free primitives](https://hightechmind.io/rust/) — 452

## See also

- [Chapter 10: Multithreading](10_multithreading.md)
- [Chapter 12: Async](12_async_tokio.md)

### Afterparty: AI Lego blocks

1. **Ordering quiz** — “For shutdown flag + published config pointer, which orderings? Justify briefly.”
2. **Counter port** — “Replace Mutex counter with AtomicUsize; discuss lost updates with Relaxed.”
3. **ABA problem** — “Explain ABA in 80 words for compare_exchange — no full queue impl.”
4. **Java AtomicInteger** — “Map Java atomic increment to Rust fetch_add snippet.”
5. **When not** — “Three cases atomics are the wrong tool; prefer channels or Mutex.”
6. **Fence intuition** — “Draw happens-before arrow diagram for Release store + Acquire load.”
