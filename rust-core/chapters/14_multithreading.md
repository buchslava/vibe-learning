# Chapter 14: Multithreading

## Hook

Concurrency models differ by language. **Python** threads fight the GIL for CPU work. **Java** threads share the heap with locks. Rust threads are **OS threads**. **Safe Rust** blocks many data races at compile time via `Send` and `Sync`. Use channels and mutexes when sharing is real.

## Scope — a brief tour

Intro to OS threads, channels, and `Arc<Mutex<T>>` — not pools, IPC, or lock-free depth.

| This chapter covers | Deferred to See also / Afterparty |
|---------------------|-----------------------------------|
| `thread::spawn`, `join`, `move`, `thread::scope` (brief) | detached threads, custom pools |
| `mpsc` channels | bounded channels, backpressure, worker pools |
| `Arc<Mutex<T>>`, `RwLock`, `OnceLock` (brief) | `Condvar`, `Barrier`, `LazyLock` depth |
| `Send` / `Sync` basics | `RefCell` across threads, pinning |
| One worker sketch (poll + channel) | `rayon`, custom thread pools, cross-process IPC |
| — | Lock-free atomics → [Chapter 15](15_atomics_and_lockfree.md) |
| — | Async concurrency → [Chapter 16](16_async_tokio.md) |

## What multithreading is

A **thread** is an independent call stack scheduled by the OS. Your program can run multiple threads at once — or interleaved on one CPU.

| Idea | Plain language |
|------|----------------|
| **Thread** | Its own stack; runs code in parallel with other threads |
| **Why bother** | Overlap **I/O wait** (serial port + network) or parallelize **isolated** work units |
| **Rust twist** | Ownership and borrowing apply **across threads** — the compiler rejects many sharing mistakes that become data races in C++/Java |
| **Two safe patterns** | **Message passing** (`mpsc` channels) or **shared state** (`Arc<Mutex<T>>`) |

```
main thread:  spawn worker ──► worker runs ──► join() waits ──► use result
```

[`Arc`](10_smart_pointers_interior_mutability.md) exists largely because threads need **shared ownership** — `Rc` is single-threaded only.

## `Send` and `Sync` — the thread boundary rules

When data crosses a thread boundary, the compiler checks two marker traits:

| Trait | Meaning (simplified) |
|-------|----------------------|
| **`Send`** | Owning value may be **moved** to another thread |
| **`Sync`** | Shared **`&T`** may be used from multiple threads safely |

Most types you write are both `Send` and `Sync`. Common exceptions:

| Type | `Send`? | `Sync`? | Why |
|------|---------|---------|-----|
| `Rc<T>` | no | no | ref count not thread-safe |
| `RefCell<T>` | yes* | no | interior mutability not thread-safe |
| `Mutex<T>` | yes if `T: Send` | yes if `T: Send` | lock enforces exclusive access |

\*Moving `RefCell` to another thread is allowed; sharing `&RefCell` across threads is not.

`thread::spawn` requires a **`Send`** closure with **`'static`** captures. Captured data must outlive the spawn and be safe to transfer. That is why `Rc` inside a spawned thread fails but `Arc` works ([Chapter 10](10_smart_pointers_interior_mutability.md)).

## Examples: elementary → hard

Work through the levels in order. After each snippet: **run it**, then read **what happened**.

### Level 1 — Elementary: spawn and join

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

**What happened:**

- Main spawns a child thread; child sleeps 10 ms, returns `42`.
- **`joined = Ok(42)`** — `join()` waits for the child and wraps its return value in `Ok`.
- If the child **panics**, `join()` returns **`Err(...)`** (not a normal return) — see [Chapter 8 — panic and unwind](08_errors_and_testing.md#panic-unwind-and-why-it-is-not-result) and Level 6 below.

### Level 2 — Elementary: `move` into the thread

Data used inside the closure must be **owned** or **`'static`**. Usually you **`move`** values into the new thread:

```rust
// Playground
use std::thread;

fn main() {
    let msg = String::from("poll");
    let handle = thread::spawn(move || {
        println!("{msg}");
    });
    handle.join().unwrap();
}
```

**What happened:**

- Prints **`poll`** — the child owns `msg` inside its closure.
- **`move`** transfers ownership from `main` into the thread; `msg` is **not** usable in `main` after spawn.

**Wrong — use `msg` after `move`:**

```rust
// Playground — does not compile
use std::thread;

fn main() {
    let msg = String::from("poll");
    let handle = thread::spawn(move || println!("{msg}"));
    println!("{msg}"); // ERROR: borrow of moved value: `msg`
    handle.join().unwrap();
}
```

Fix: **`msg.clone()`** before spawn if both sides need a copy. That **duplicates the `String` on the heap**. Large buffers can get costly. Cheaper patterns: return data via **`join()`** or a **channel**, or share with **`Arc`** (Level 4) when many threads need one allocation.

### Level 3 — Medium: `mpsc` channel

**Multi-producer, single-consumer** — send messages instead of sharing mutable state:

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

**What happened:**

- Prints **`got 1`** then **`got 2`** — order preserved (FIFO).
- **`move`** on the closure: **`tx`** ownership moves to the worker; main keeps **`rx`**.
- When all **`tx`** senders are **dropped**, the `for val in rx` loop **ends** — channel closed.
- **Message passing** often beats shared mutable state for work queues and command streams.

### Level 4 — Medium: `Arc<Mutex<T>>` shared counter

When threads must **mutate the same data**, wrap it in **`Mutex`** (exclusive access) and **`Arc`** (shared ownership):

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
    for h in handles {
        h.join().unwrap();
    }
    println!("count = {}", *counter.lock().unwrap());
}
```

| Piece | Role |
|-------|------|
| `Arc` | Reference-counted handle — many threads hold the **same** allocation |
| `Mutex` | Only one thread mutates inner `T` at a time |
| `lock().unwrap()` | Block until lock acquired; **panic** if mutex is **poisoned** |
| `Arc::clone(&counter)` | Cheap handle copy — points at same `Mutex` |

**`Arc::clone` vs cloning the inner value** — easy to confuse:

| Call | What it copies | Cost |
|------|----------------|------|
| **`Arc::clone(&counter)`** | Pointer + atomic ref-count only — same heap `Mutex` | **Cheap** — O(1), no data duplicate |
| **`(*counter.lock().unwrap()).clone()`** | The **inner** `T` when `T: Clone` (e.g. whole `String`, `Vec`) | **Expensive** — full deep copy of protected data |

In the loop, **`Arc::clone(&counter)`** gives each thread a **handle** to the **one** shared counter. Calling **`.clone()` on the inner value** copies the data under the lock. That is the wrong tool for “many threads, one counter.” See [Chapter 10 — `Arc` vs deep clone](10_smart_pointers_interior_mutability.md).

**What happened:**

- Prints **`count = 4`** — each of four threads incremented once; no lost updates.
- Without **`Mutex`**, sharing `&mut` across threads would not compile — Rust blocks the data race at compile time.
- **Lock poisoning:** if a thread **panics while holding the lock**, others get **`PoisonError`** on `lock()` — the lock may be inconsistent ([Chapter 8](08_errors_and_testing.md)).

### Level 5 — Hard: `Send` trap — `Rc` cannot enter a thread

Types must implement **`Send`** to be **moved** into another thread. **`Rc`** is not `Send`:

```rust
// Playground — does not compile
use std::rc::Rc;
use std::thread;

fn main() {
    let data = Rc::new(0);
    thread::spawn(move || {
        println!("{data}");
    });
    // ERROR: `Rc<i32>` cannot be sent between threads safely
}
```

**What happened:**

- Compiler **rejects** the spawn — `Rc` ref-count is not thread-safe.
- **Fix:** use **`Arc::new(0)`** (atomic ref-count) — same pattern as Level 4.

```rust
// Playground
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(0);
    let d = Arc::clone(&data);
    thread::spawn(move || println!("{d}")).join().unwrap();
    println!("main still has {data}");
}
```

**What happened:** compiles and prints **`0`** twice — `Arc` is **`Send` + `Sync`** when the inner type allows it. **`Arc::clone(&data)`** is cheap (ref-count bump); main and the worker share **one** `i32` on the heap, not two copies.

### Level 6 — Hard: join without `unwrap` + sensor poll worker

**6a. Handle panics at the boundary** ([Chapter 8](08_errors_and_testing.md) — don't `unwrap` production join in unattended services):

```rust
// Playground
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        // simulate failure: panic!("device fault");
        100
    });
    match handle.join() {
        Ok(reading) => println!("ok {reading}"),
        Err(_) => eprintln!("worker panicked — log and continue supervisor loop"),
    }
}
```

**What happened:**

- **`Ok(100)`** → prints **`ok 100`**.
- If you uncomment the panic → **`Err(_)`** arm runs; **main survives** — contrast with panic in the worker killing the whole process if unhandled.

**6b. Poll worker sketch — poll thread, main logs readings:**

```rust
// Playground
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for id in 1..=3 {
            thread::sleep(Duration::from_millis(5));
            tx.send(format!("sensor-{id}: {:.1}", 20.0 + id as f64)).unwrap();
        }
        // tx dropped here — channel closes
    });

    for reading in rx {
        println!("log {}", reading);
    }
    println!("poll loop ended");
}
```

**What happened:**

- Prints **`log sensor-1: 21.0`**, **`sensor-2: 22.0`**, **`sensor-3: 23.0`**, then **`poll loop ended`**.
- Worker **produces**; main **consumes** — classic **message-passing work queue** without shared mutable state.
- Dropping **`tx`** when the worker finishes closes the channel; main's `for reading in rx` exits cleanly.

## `RwLock` — many readers, rare writers

Sensor cache: many threads read, one thread reloads config:

```rust
// Playground
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

fn main() {
    let cache: Arc<RwLock<HashMap<String, f64>>> =
        Arc::new(RwLock::new(HashMap::from([("temp".into(), 22.5)])));

    let reader = Arc::clone(&cache);
    let t = std::thread::spawn(move || {
        let map = reader.read().expect("lock poisoned");
        println!("read temp={}", map["temp"]);
    });
    t.join().unwrap();

    {
        let mut map = cache.write().expect("lock poisoned");
        map.insert("temp".into(), 23.1);
    }
    let map = cache.read().unwrap();
    println!("after reload temp={}", map["temp"]);
}
```

`.read()` allows concurrent readers. `.write()` exclusive for updates. If a thread panics while holding a lock, the lock is **poisoned** — `.expect` or match on the `Result`.

## `OnceLock` — initialize once

Lazy global config without hand-rolled double-checked locking:

```rust
// Playground
use std::sync::OnceLock;

struct Settings {
    port: u16,
}

static CONFIG: OnceLock<Settings> = OnceLock::new();

fn settings() -> &'static Settings {
    CONFIG.get_or_init(|| Settings { port: 502 })
}

fn main() {
    println!("port={}", settings().port);
    println!("same={}", settings().port);
}
```

First call runs the closure; later calls return the same reference. For stack-local one-time init, `std::sync::LazyLock` (Rust 1.80+) works similarly without `static`.

## `thread::scope` — borrow stack data into threads

Parallel checksum over chunks — each thread borrows `&chunks[i]` safely:

```rust
// Playground
use std::thread;

fn checksum(chunk: &[u8]) -> u32 {
    chunk.iter().map(|&b| b as u32).sum()
}

fn main() {
    let chunks: Vec<[u8; 4]> = vec![[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];

    thread::scope(|s| {
        let handles: Vec<_> = chunks
            .iter()
            .map(|chunk| s.spawn(|| checksum(chunk)))
            .collect();

        let total: u32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
        println!("total={}", total);
    });
}
```

`scope` guarantees threads join before returning — so borrows from `chunks` cannot dangle. Plain `spawn` cannot borrow local `Vec` elements without `'static` data.

### Sync primitive edge cases

**RwLock vs Mutex:** use `RwLock` when reads dominate; `Mutex` is simpler when writes are frequent or critical sections are tiny.

**Poisoned lock:** if a writer panics, `read()` returns `Err`. Log and rebuild cache, or use `into_inner()` on the poison error to recover the inner value.

## Techniques at a glance

Popular primitives — one line each. Details for starred rows live in Afterparty or linked chapters.

| Technique | One-line use | Where |
|-----------|--------------|-------|
| `thread::spawn` / `join` | Start OS thread; wait for result | Levels 1–2, 6 |
| `move` closures | Transfer ownership into thread | Levels 2–4 |
| `mpsc` | Message passing, work queues | Levels 3, 6b |
| `Arc<Mutex<T>>` | Shared mutable state, short locks | Level 4 |
| **`Send`** | Safe to **move** `T` to another thread | Level 5 |
| **`Sync`** | Safe to share **`&T`** across threads | Level 4 (`Arc` shares `&Mutex`) |
| `RwLock` | Many readers, rare writers | Section above |
| `rayon` / thread pools | CPU-bound parallelism | Afterparty / Go deeper |
| atomics | Lock-free counters, flags | [Chapter 15](15_atomics_and_lockfree.md) |
| async / `.await` | Many concurrent I/O waits | [Chapter 16](16_async_tokio.md) |

**`Send` / `Sync` in practice:**

- Most plain types (`i32`, `String`, `Vec`) are **`Send`** and **`Sync`**.
- **`Rc`**, **`RefCell`** — not **`Sync`** (single-thread interior mutability).
- **Raw pointers** — neither unless you enforce safety yourself (`unsafe`).
- **`Arc<Mutex<T>>`** — share **`Sync`** handle; mutation only inside **`lock()`**.

## Idiom spotlight

> **Prefer channels for work queues and command streams; use `Mutex` for short critical sections.** Long-held locks hurt poll latency (a Modbus cycle can miss its slot).
>
> **I/O-bound services:** one thread blocked on serial read, another on network publish — overlap waits without fighting the GIL.
>
> **Handle `join()` like `Result`** at the supervisor boundary — worker panic is data, not necessarily process death.

## Go deeper

- [The Rust Book — Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Mutex basics](https://hightechmind.io/rust/) — 986
- [Thread pool](https://hightechmind.io/rust/) — 923
- [Arc threads](https://hightechmind.io/rust/) — 109

## See also

- [Chapter 8: Errors and panic](08_errors_and_testing.md) — `join()` after worker panic, why `unwrap` in loops is risky
- [Chapter 10: Arc and smart pointers](10_smart_pointers_interior_mutability.md) — `Rc` vs `Arc`
- [Chapter 12: Closures](12_closures.md) — `move` and `Fn` traits for `spawn`
- [Chapter 15: Atomics](15_atomics_and_lockfree.md) — lock-free counters, `OnceLock` vs hand-rolled init
- [Chapter 16: Async](16_async_tokio.md) — when threads are not the right tool

### Afterparty

Use these for worker pools and topics not covered above.

#### Spawn, move, join

1. **Race quiz** — “Which snippets are data races in C++ but rejected by Rust compiler?”
2. **Move fix** — “Show spawn without `move` that fails; fix with `move` or `clone`.”
3. **Join panic** — “Worker panics; rewrite main to `match join()` and keep supervisor alive.”
4. **Detached threads** — “Why is `mem::forget(handle)` after spawn dangerous? Better pattern?”

#### Channels

5. **Channel design** — “Worker pool with mpsc: I describe throughput; sketch thread count + channel shape.”
6. **Drop tx footgun** — “Main exits before worker sends — diagram who holds `tx`/`rx`.”
7. **Multiple producers** — “Clone `tx` to two workers; main receives merged stream — sketch code.”
8. **Bounded vs unbounded** — “When does unbounded `mpsc` blow memory in automation? Bounded alternative?”

#### Mutex and shared state

9. **Mutex vs RwLock** — “Read-heavy sensor cache — pick primitive and why.”
10. **Hold lock briefly** — “Refactor bad code that calls network I/O while holding `Mutex` lock.”
11. **Deadlock sketch** — “Two mutexes, lock order A then B vs B then A — show hang scenario.”
12. **Poison recovery** — “Thread panics holding lock; show `PoisonError` and `into_inner()` recovery.”
13. **Send fix** — “I try to move `Rc` into thread; show fix with `Arc`.”
14. **RefCell trap** — “Why `Arc<RefCell<T>>` is not `Sync`; fix pattern for shared mutation.”

#### Production and automation

15. **PLC gateway layout** — “Sketch poll thread + command channel + main supervisor; no code over 40 lines.”
16. **Lock latency** — “Modbus cycle 20 ms — max time holding mutex for register cache update?”
17. **Level ladder recap** — “Explain Levels 1–6 in one paragraph each for a Java teammate.”

#### RwLock, OnceLock, and scope

18. **RwLock cache** — "Read-heavy sensor map — sketch `Arc<RwLock<HashMap>>`; when does write starve readers?"
19. **Mutex vs RwLock** — "Same cache with 50% writes — pick Mutex or RwLock and justify in two sentences."
20. **OnceLock init** — "`get_or_init` fails second init with different value — show `OnceLock` behaviour."
21. **Scope borrow** — "Parallel sum over `Vec<[u8;512]>` with `thread::scope` — why plain `spawn` fails on `&chunk`."
22. **Poison recovery** — "Writer thread panics holding `RwLock` — show poisoned `read()` error and recovery options."
23. **Capstone sync** — "Design: lazy config (`OnceLock`), shared cache (`RwLock`), scoped batch workers — list types only."

