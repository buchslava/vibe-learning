# Chapter 10: Multithreading

## Hook

Python threads fight the GIL for CPU work; Java threads share the heap with locks. Rust threads are OS threads, but **safe Rust** prevents data races at compile time via `Send` and `Sync` — plus channels and mutexes when sharing is real.

## Spawning threads

```rust
// Playground
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(10));
        42
    });
    println!("joined = {:?}", handle.join());
}
```

`join()` returns `Result` if the thread panicked.

## Channels (`mpsc`)

```rust
// Playground
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(1).unwrap();
        tx.send(2).unwrap();
    });
    for val in rx {
        println!("got {}", val);
    }
}
```

**Message passing** often beats shared mutable state.

## `Mutex<T>` and `Arc`

```rust
// Playground
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..4 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut n = c.lock().unwrap();
            *n += 1;
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("count = {}", *counter.lock().unwrap());
}
```

Lock poisoning: if a thread panics while holding a lock, others see `PoisonError`.

## `Send` and `Sync`

- **`Send`**: safe to move to another thread
- **`Sync`**: safe to share `&T` between threads

Most types are `Send`/`Sync`; `Rc` is not `Send`; raw pointers need care.

## Idiom spotlight

> **Prefer channels for work queues; use `Mutex` for short critical sections.** Long-held locks hurt automation latency.

## Go deeper

- [Mutex basics](https://hightechmind.io/rust/) — 986
- [Thread pool](https://hightechmind.io/rust/) — 923

## See also

- [Chapter 11: Atomics](11_atomics_and_lockfree.md)
- [Chapter 12: Async](12_async_tokio.md)

### Afterparty: AI Lego blocks

1. **Race quiz** — “Which snippets are data races in C++ but rejected by Rust compiler?”
2. **Channel design** — “Worker pool with mpsc: I describe throughput; you sketch thread count + channel shape.”
3. **Mutex vs RwLock** — “Read-heavy sensor cache — pick primitive and why.”
4. **Send fix** — “I try to move `Rc` into thread; show fix with Arc.”
5. **Join panic** — “What happens if spawned thread panics? Handle in main.”
6. **Python GIL** — “Compare this Python threading example to Rust for same task.”
