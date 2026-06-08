# Chapter 15: Atomics and Lock-Free Basics

## Hook

Lock-free primitives appear in many runtimes (**Java** `java.util.concurrent.atomic.*`, **Python** atomics behind the GIL in CPython, …). Rust exposes **`std::sync::atomic`** for lock-free counters and flags when mutex overhead is too high. **Memory ordering** is part of the contract.

## Scope — a brief tour

Practical atomics for counters and flags — not lock-free data structures or formal memory-model proofs.

| This chapter covers | Deferred to See also / Afterparty |
|---------------------|-----------------------------------|
| `AtomicBool`, `AtomicUsize`, `load` / `store` / `fetch_add` / `compare_exchange` | Lock-free queues, stacks, hazard pointers |
| Practical **`Ordering`** subset | Full formal memory-model proofs |
| **Data races** + happens-before intuition | Formal verification |
| Shared flags/counters in **threads and async** | `crossbeam`, hand-rolled lock-free DS |
| When **not** to use atomics | SIMD atomics, `portable-atomic` edge cases |

## Where atomics fit — threads, async, and Chapter 14

Atomics solve **concurrent read/write of one machine word** without a **`Mutex` lock**. They appear in **OS-thread** code ([Chapter 14](14_multithreading.md)) and **async** services ([Chapter 16](16_async_tokio.md)). Same types, same rules.

| Context | Shared pattern | Atomics role |
|---------|----------------|--------------|
| **OS threads** ([Ch14 Level 4](14_multithreading.md)) | `Arc<AtomicUsize>` across `thread::spawn` | Hot counter without mutex contention |
| **Async tasks** ([Ch16](16_async_tokio.md)) | `Arc<AtomicBool>` read by many `tokio::spawn` tasks | Cooperative **shutdown** without locking the runtime |
| **Both** | `Arc` wraps the atomic — [`Arc::clone` is cheap](14_multithreading.md) | Metrics: `Relaxed`; flags with dependent data: `Release` / `Acquire` |

```
Ch14:  thread::spawn ──► Arc<AtomicUsize> fetch_add
Ch16:  tokio::spawn   ──► Arc<AtomicBool>  load/store
```

**Key message:** Async does **not** remove concurrency. Tasks on a thread pool still share memory. Atomics are **`Send` + `Sync`**. Wrap them in **`Arc`** when many tasks or threads need the same counter or flag.

Atomics do **not** replace [channels](14_multithreading.md) (message streams) or **`Mutex`** (multi-field invariants). One atomic word is not a lock-free entire struct.

## Data races — why atomics exist

A **data race** (informal): two threads access the **same memory**, at least one **writes**, with **no synchronization**. In C/C++ that is **undefined behavior**.

| Kind | Example | Safe Rust? |
|------|---------|------------|
| **Prevented at compile time** | two `&mut` to same `String` in two threads | **compile error** |
| **Non-atomic shared counter** | `static mut COUNTER` += 1 in two threads | **`unsafe` UB** — do not do this |
| **Atomic operations** | `fetch_add(1, Ordering::Relaxed)` | **defined** — each op is indivisible for that word |
| **Visibility bug** (ordering) | flag set with `Relaxed` but reader never sees prior writes to other fields | **defined atomic op**, **wrong logic** — use Release/Acquire |

**Logic race vs memory race:**

- **`fetch_add`** on `AtomicUsize` does **not lose increments** — hardware makes the RMW atomic.
- Plain **`+= 1`** on shared `mut` (in C or `unsafe` Rust) **can** lose updates — two threads read 5, both write 6.
- **`Relaxed`** is fine for **standalone metrics**; it is **wrong** when a flag must publish **other memory** (config buffer, shutdown state) — see Level 4–5.

**Conceptual anti-pattern (UB — not safe Rust):**

```rust
// Playground — do not write this; illustrates why atomics exist
// static mut COUNTER: u32 = 0;
// thread::spawn(|| unsafe { COUNTER += 1 });
// thread::spawn(|| unsafe { COUNTER += 1 });
// two unsynchronized writes = data race (undefined behavior)
```

Same job as [Chapter 14 Level 4 `Arc<Mutex<usize>>`](14_multithreading.md) — atomics trade **expressiveness** for **speed** on simple ops.

## Examples: elementary → hard

Work through in order. After each snippet: **run it**, then read **what happened**.

### Level 1 — Elementary: `AtomicBool` shutdown flag

Worker loops until main sets **shutdown** with **`Release`**; worker reads with **`Acquire`**:

```rust
// Playground
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let shutdown = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&shutdown);

    let worker = thread::spawn(move || {
        while !flag.load(Ordering::Acquire) {
            thread::sleep(Duration::from_millis(5));
            // simulate poll work
        }
        println!("worker stopped");
    });

    thread::sleep(Duration::from_millis(20));
    shutdown.store(true, Ordering::Release);
    worker.join().unwrap();
}
```

**What happened:**

- Main sleeps ~20 ms, sets shutdown **`true`**, worker exits loop, prints **`worker stopped`**, **`join`** succeeds.
- **`Release` store** (main) + **`Acquire` load** (worker) = **happens-before**: worker **sees** all memory writes main made **before** the `store(true)`.
- Same pattern works behind **`Arc<AtomicBool>`** in **`tokio::spawn`** ([Chapter 16](16_async_tokio.md)) — tasks poll `load(Acquire)` between `.await` points.

### Level 2 — Elementary: `fetch_add` counter

Lock-free increment — port of [Ch14 Mutex counter](14_multithreading.md):

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
    for h in handles {
        h.join().unwrap();
    }
    println!("{}", n.load(Ordering::Relaxed));
}
```

| Piece | Role |
|-------|------|
| `AtomicUsize` | One shared counter word — no `Mutex` |
| `fetch_add(1, Relaxed)` | Atomic read-modify-write; **`Relaxed`** OK for metrics-only counter |
| `Arc::clone(&n)` | Cheap handle — same atomic on heap ([Chapter 14](14_multithreading.md)) |

**What happened:**

- Prints **`4`** — four threads each added 1; **no lost updates** (contrast non-atomic `static mut`).
- **`Relaxed`** does not synchronize **other** memory — fine here because **only** the atomic is touched.

### Level 3 — Medium: `compare_exchange` — cap a counter

**Compare-and-swap (CAS):** update only if current value matches expected — retry on contention:

```rust
// Playground
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

const MAX: usize = 100;

fn try_inc(counter: &AtomicUsize) {
    loop {
        let current = counter.load(Ordering::Relaxed);
        if current >= MAX {
            return;
        }
        if counter
            .compare_exchange_weak(current, current + 1, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
        {
            return;
        }
        // spurious failure — retry
    }
}

fn main() {
    let n = Arc::new(AtomicUsize::new(98));
    let mut handles = vec![];

    for _ in 0..4 {
        let c = Arc::clone(&n);
        handles.push(thread::spawn(move || try_inc(&c)));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("capped at {}", n.load(Ordering::Relaxed));
}
```

**What happened:**

- Starts at **98**; four threads race to increment; at most **2** succeed → prints **`capped at 100`** (never exceeds `MAX`).
- **`compare_exchange_weak`** may fail spuriously — **always retry** in a loop.
- **`AcqRel`** on success: typical for RMW that publishes/consumes in one step.

### Level 4 — Medium: Release/Acquire publish pattern

Worker updates **config**, then bumps **version**; reader observes version with **`Acquire`** before reading config:

```rust
// Playground
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let config = Arc::new(Mutex::new(502u16));
    let version = Arc::new(AtomicUsize::new(0));

    let cfg_w = Arc::clone(&config);
    let ver_w = Arc::clone(&version);
    let writer = thread::spawn(move || {
        {
            let mut p = cfg_w.lock().unwrap();
            *p = 8080;
        } // release lock before publishing
        ver_w.fetch_add(1, Ordering::Release);
    });

    let cfg_r = Arc::clone(&config);
    let ver_r = Arc::clone(&version);
    let reader = thread::spawn(move || {
        loop {
            let seen = ver_r.load(Ordering::Acquire);
            if seen > 0 {
                let p = *cfg_r.lock().unwrap();
                println!("reader saw version {} port {}", seen, p);
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }
    });

    writer.join().unwrap();
    reader.join().unwrap();
}
```

**What happened:**

- Writer sets port **8080**, then **`fetch_add(1, Release)`** publishes.
- Reader **`load(Acquire)`** on version → guaranteed to see port **8080**, not stale **502** from before the write.
- Happens-before chain:

```
writer:  *config = 8080  ──►  version Release+1  ──►  reader Acquire load  ──►  read config
```

**Wrong — `Relaxed` on version only (visibility bug story):**

Using **`Relaxed`** for both version load and store can let the reader see a new version **without** the config write on some architectures. **Release/Acquire** fixes the handoff. Metrics-only atomics do not need this; **publish patterns do**.

### Level 5 — Hard: when `Relaxed` lies — ordering footgun

| Scenario | Ordering | Verdict |
|----------|----------|---------|
| Frames-processed counter | `Relaxed` load/store | **OK** — standalone stat |
| Shutdown flag + prior log flush | `Relaxed` both sides | **Risky** — reader may not see flushed logs |
| Shutdown flag + prior log flush | `Release` store / `Acquire` load | **Correct** publish pattern |
| “I want simple mental model” | `SeqCst` everywhere | **OK** at low frequency; slightly slower |

**Sophisticated edge case — atomic flag + non-atomic payload:**

```rust
// Playground — conceptual bug pattern (do not ship)
use std::sync::atomic::{AtomicBool, Ordering};

struct BadHandoff {
    ready: AtomicBool,
    port: u16, // NOT atomic — paired with ready
}

// Writer: port = 8080; ready.store(true, Ordering::Relaxed);
// Reader:  if ready.load(Relaxed) { use port }  // may read stale port without Release/Acquire
```

Fix: **`Release`** after writing `port`, **`Acquire`** before reading `port` — or protect both under **`Mutex`** ([Chapter 14](14_multithreading.md)).

**What happened (conceptually):** the atomic op is well-defined. **Your program logic** is wrong if ordering does not establish happens-before between the flag and other fields.

### Level 6 — Hard: gateway sketch

**Metrics** (`Relaxed`) + **shutdown** (`Release`/`Acquire`) — same struct works for threads today, async tasks tomorrow:

```rust
// Playground
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Gateway {
    polls: AtomicUsize,
    shutdown: AtomicBool,
}

fn main() {
    let gw = Arc::new(Gateway {
        polls: AtomicUsize::new(0),
        shutdown: AtomicBool::new(false),
    });

    let worker_gw = Arc::clone(&gw);
    let worker = thread::spawn(move || {
        while !worker_gw.shutdown.load(Ordering::Acquire) {
            thread::sleep(Duration::from_millis(5));
            worker_gw.polls.fetch_add(1, Ordering::Relaxed);
        }
    });

    thread::sleep(Duration::from_millis(25));
    println!("metrics polls={}", gw.polls.load(Ordering::Relaxed));
    gw.shutdown.store(true, Ordering::Release);
    worker.join().unwrap();
    println!("supervisor done");
}
```

**What happened:**

- Worker increments **`polls`** several times with **`Relaxed`** (metrics-only).
- Supervisor prints poll count, sets **`shutdown`** with **`Release`**, worker observes **`Acquire`** and exits.
- In **async** ([Chapter 16](16_async_tokio.md)): replace `thread::spawn` with `tokio::spawn`, same `Arc<Gateway>` — atomics unchanged.

## Memory ordering (practical subset)

| Ordering | Typical use | Example |
|----------|-------------|---------|
| **`Relaxed`** | Standalone counters, stats | frames processed, bytes read, poll count |
| **`Release`** | Publish — “data is ready” | store shutdown `true` after flushing state |
| **`Acquire`** | Observe — “I see your prior writes” | worker checks shutdown flag |
| **`AcqRel`** | Read-modify-write (CAS, `fetch_add` handoff) | Level 3 cap counter |
| **`SeqCst`** | Simplest global order; slightly slower | low-frequency flags when unsure |

**When `Relaxed` is wrong:** any atomic that **guards other memory** (config version, shutdown + shared buffer) needs **Release/Acquire** (or **`Mutex`**).

**ABA (one line):** `compare_exchange` on **pointers** can succeed spuriously if the same bit pattern reappears after free/realloc. Relevant for lock-free queues. Use Afterparty for depth — not production queues here.

## Atomics vs `Mutex`

| | Atomics | `Mutex` ([Ch14](14_multithreading.md)) |
|---|---------|----------------------------------------|
| Best for | counters, flags, CAS on one word | multi-field invariants, `Vec` updates |
| Composability | single-word ops | arbitrary critical sections |
| Debugging | subtle ordering bugs | coarser, clearer |
| Overhead | no lock for hot counter | lock + possible contention |

**Decision sketch:**

```
one word, simple op (count, flag)?     → atomic
several fields must stay consistent?   → Mutex
stream of messages / commands?         → channel (Ch14)
custom lock-free queue?                → crate or Afterparty — not hand-roll
```

**Port exercise:** swap [Ch14 Level 4 `Arc<Mutex<usize>>`](14_multithreading.md) for [Level 2](#level-2--elementary-fetch_add-counter) `AtomicUsize` when profiling shows lock contention on a hot path.

## Edge cases and compiler traps

| Trap | Symptom | Idiom |
|------|---------|-------|
| Non-atomic shared `mut` | lost counts, UB in C/`unsafe` | `Atomic*` or `Mutex` |
| `Relaxed` for publish flag | stale reads of non-atomic fields | `Release` / `Acquire` |
| CAS without retry loop | spurious `compare_exchange_weak` failure | `loop` until `is_ok()` or give up |
| Atomics for large struct | wrong tool — one word only | `Mutex` or channel |
| False sharing (advanced) | cache line ping-pong between cores | pad/separate hot atomics — Afterparty |
| ABA on pointer CAS | wrong lock-free pop | epoch / RCU — Afterparty |
| Confusing panic with race | thread died vs data race | [Ch8 `join`](08_errors_and_testing.md) vs atomics |

**Wrong — `compare_exchange` once, no retry:**

```rust
// Playground — may silently fail to increment under contention
use std::sync::atomic::{AtomicUsize, Ordering};

fn bad_inc(n: &AtomicUsize) {
    let c = n.load(Ordering::Relaxed);
    let _ = n.compare_exchange(c, c + 1, Ordering::Relaxed, Ordering::Relaxed);
    // spurious failure → lost increment
}
```

**Wrong — expect `Mutex`-less `Vec` push from many threads:**

```rust
// Playground — does not compile
use std::sync::Arc;
use std::thread;

fn main() {
    let v = Arc::new(vec![1, 2, 3]);
    let v2 = Arc::clone(&v);
    thread::spawn(move || {
        // v2.push(4); // ERROR: cannot borrow as mutable
    });
}
```

Use **`Mutex<Vec<_>>`**, a **channel**, or a dedicated concurrent crate — not a lone atomic.

## Idiom spotlight

> **Start with `Mutex`; switch to atomics when profiling shows lock contention on a hot counter or flag.** Use **`Relaxed`** for metrics-only words; use **`Release`/`Acquire`** when another thread must see your **prior non-atomic writes**.
>
> Same **`Arc<Atomic*>`** behind **OS threads** ([Chapter 14](14_multithreading.md)) and **async tasks** ([Chapter 16](16_async_tokio.md)). Do not hand-roll lock-free queues without study.

## Go deeper

- [Atomic types — lock-free primitives](https://hightechmind.io/rust/) — 452
- [Rust nomicon — atomics](https://doc.rust-lang.org/nomicon/atomics.html)

## See also

- [Chapter 14: Multithreading](14_multithreading.md) — `Arc`, `Mutex`, threads
- [Chapter 16: Async](16_async_tokio.md) — tasks sharing `Arc<AtomicBool>`
- [Chapter 8: Errors and panic](08_errors_and_testing.md) — `join()` after panic vs race bugs

### Afterparty

Use these for lock-free structures and formal memory-model topics not covered above.

#### Concepts and placement

1. **Threads vs async atomics** — “Same `Arc<AtomicBool>` in `thread::spawn` vs `tokio::spawn` — what differs, what stays the same?”
2. **Data race vs logic race** — “Define both; classify lost `static mut` increment vs `fetch_add`.”
[-]3. **Scope honesty** — “List 5 topics Ch15 skips and where to learn each.”
4. **Ch14 port** — “Rewrite Mutex counter (Ch14 L4) as `AtomicUsize`; when is each better?”

#### Memory ordering

5. **Ordering quiz** — “Shutdown flag + published config pointer — pick orderings; justify.”
6. **When Relaxed lies** — “Metrics OK, config handoff broken — show Release/Acquire fix.”
7. **Fence intuition** — “Draw happens-before for Release store + Acquire load (Level 4).”
[-]8. **SeqCst default** — “When is paying for SeqCst worth it in a PLC gateway?”
9. **Relaxed polls vs version** — “Why is `Relaxed` OK for Level 2 `polls` but wrong for Level 4 `version`? One sentence each.”

#### CAS and compare_exchange

10. **Counter port** — “Cap counter with CAS loop; explain spurious `compare_exchange_weak` failure.”
11. **ABA problem** — “Explain ABA in 80 words for pointer `compare_exchange` — no full queue.”
12. **Retry loop** — “Show single-shot CAS anti-pattern; fix with loop.”
13. **Double-checked locking** — “Show broken lazy init with atomics; fix with `OnceLock` or explain why not hand-roll.”

#### Races and debugging

14. **Race quiz** — “Mark 6 snippets: safe atomic, UB `static mut`, ordering bug, needs Mutex.”
15. **Visibility story** — “Writer updates `port` then `Relaxed` flag — what can reader see?”
[-]16. **Panic vs race** — “Thread panicked in worker — is it a data race? Link Ch8.”

#### Atomics vs Mutex vs channels

17. **When not** — “Three cases atomics are the wrong tool; prefer channels or Mutex.”
18. **Mutex vs RwLock** — “Read-heavy sensor cache — atomic counter vs RwLock vs Mutex.”
19. **Vec push** — “Why many threads cannot `push` to shared `Vec`; three fixes.”
20. **Profile-first** — “Gateway uses `Mutex` for counter; profiler shows lock contention — minimal atomic refactor.”
21. **Channel vs atomic flag** — “When is `crossbeam`/bounded channel + shutdown better than lone `AtomicBool`?”

#### Production and Java port

22. **Gateway metrics** — “Design `Gateway { polls, shutdown }` for async; orderings per field.”
23. **False sharing** — “Two hot atomics on same cache line — problem and mitigation in 60 words.”
[-]24. **Java AtomicInteger** — “Map Java `incrementAndGet` to Rust `fetch_add` + orderings.”
25. **Lock-free queue** — “Why this chapter says don’t hand-roll; name crates/patterns instead.”

[-]#### Capstone

[-]26. **Level ladder recap** — “Explain Levels 1–6 in one paragraph each for a Java teammate.”
