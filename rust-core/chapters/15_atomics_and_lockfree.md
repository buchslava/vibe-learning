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

## Memory ordering — `Ordering` cheat sheet

Every atomic operation takes an **`Ordering`**. It answers two questions:

1. **Is this read/write on the atomic word indivisible?** — yes for **all** orderings.
2. **Do other threads see my non-atomic writes in a predictable order?** — only for **some** orderings.

| Ordering | Valid on | What it synchronizes | Plain language |
|----------|----------|----------------------|----------------|
| **`Relaxed`** | `load`, `store`, RMW | **This atomic word only** — no ordering for other memory | “Count reliably; I don’t publish other data through this op.” |
| **`Acquire`** | `load`, RMW (success) | Reads **after** this load see writes that happened **before** a **`Release`** on the **same** atomic | Reader: “When I see the new flag/version, I see the data the writer prepared.” |
| **`Release`** | `store`, RMW (success) | Writes **before** this store are visible to threads that later **`Acquire`**-load the same atomic | Writer: “I finish writing config, **then** I publish the flag/version.” |
| **`AcqRel`** | RMW only (`fetch_add`, `compare_exchange`, …) | **Both** — successful RMW releases prior writes and acquires prior releases | “Update and publish (or consume and publish) in one atomic step.” |
| **`SeqCst`** | all ops | **Global** total order among all `SeqCst` ops — strongest, easiest mental model, often slightly slower | “I want one strict worldwide timeline when in doubt.” |

**Handoff pattern** (used in Levels 1 and 4):

```
writer:  write config buffer  →  atomic.store(..., Release)
reader:  atomic.load(Acquire)  →  read config buffer   // sees writer's data
```

**`compare_exchange` has two orderings** (Level 3):

```rust
counter.compare_exchange_weak(current, current + 1, Ordering::AcqRel, Ordering::Relaxed)
//                                                  ^ success          ^ failure (retry path)
```

Use **`Relaxed`** on failure when you only loop and retry; use **`AcqRel`** on success when the RMW participates in a handoff.

**Quick pick:**

| Scenario | Ordering |
|----------|----------|
| Packet / poll / frame counter — nothing else depends on it | `Relaxed` |
| Shutdown flag, version number, “data ready” bit + other shared fields | `Release` (writer) + `Acquire` (reader) |
| CAS / `fetch_add` that caps or publishes in one step | `AcqRel` on success |
| Low-frequency flag, want simplest model | `SeqCst` everywhere on that atomic |

Official reference: [std::sync::atomic::Ordering](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html).

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

**Two different problems atomics address:**

| Problem | Non-atomic `counter += 1` | `fetch_add` on `AtomicUsize` |
|---------|---------------------------|------------------------------|
| **Lost updates** | Thread A and B both read `5`, both write `6` → count is **6**, not **7** | Hardware **read-modify-write** is one indivisible step — **no lost increments** |
| Where it shows up | C shared `mut`, `static mut` + `unsafe` in Rust | `Atomic*` with any `Ordering` |

**Ordering is a separate issue:** even when increments are not lost, **`Relaxed`** only guarantees atomicity of **that one word**. It does **not** guarantee that other threads see writes you made **before** updating a flag or counter.

| Use case | Ordering |
|----------|----------|
| Standalone metric (packet count, sample tally) | `Relaxed` — usually enough |
| Flag that means “other data is ready” (config buffer, shutdown) | `Release` on writer, `Acquire` on reader — see Levels 4–5 |

So: **`fetch_add` fixes lost increments**; **`Release`/`Acquire` fix visibility** of *other* memory. You need both ideas when a flag publishes state beyond the atomic itself.

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

**What happened — step by step:**

| Step | Who | Action |
|------|-----|--------|
| 1 | main | Creates `shutdown = false`, spawns worker with a clone of the same `Arc` |
| 2 | worker | Loops: `load(Acquire)` → still `false` → sleep 5 ms, repeat |
| 3 | main | Sleeps 20 ms, then `store(true, Release)` |
| 4 | worker | Next `load(Acquire)` → `true` → exits loop, prints **`worker stopped`** |
| 5 | main | `join()` returns — worker finished cleanly |

**Why atomics and these orderings?**

**Not a plain `bool`:** two threads reading/writing the same `bool` without synchronization is a **data race** (undefined behavior in C; blocked or unsafe in Rust). `AtomicBool` makes each load/store safe.

**Not `Relaxed` (for this pattern):** `Relaxed` only promises “this one flag updates correctly.” It does **not** promise that the worker sees **other** data main wrote earlier (logs flushed, config updated, port number set). For a **stop signal that goes with other setup**, use a matched pair:

| Who | Code | Simple meaning |
|-----|------|----------------|
| main | `store(true, Release)` | “I finished my other work — **now** I turn on shutdown.” |
| worker | `load(Acquire)` | “If I see shutdown on, I also see that other work.” |

**Picture it like a doorbell:**

1. Main prepares the package (writes config, flushes logs, …).
2. Main rings the bell — `store(true, Release)`.
3. Worker hears the bell — `load(Acquire)`.
4. Worker opens the door and uses the package.

Steps 1–3 must stay in that order across threads. **`Release` + `Acquire`** is how Rust asks the CPU for that guarantee. (Formal name: **happens-before** — main’s earlier writes **happen before** the worker’s reads that follow the `Acquire`.)

**In this tiny example** the flag is the only shared value, so `Relaxed` would often still stop the loop. We show **`Release` / `Acquire`** because real services almost always mean “stop **and** read the state I prepared first” — Level 4 adds a **config + version** to make that explicit.

Same layout with **`Arc<AtomicBool>`** inside **`tokio::spawn`** ([Chapter 16](16_async_tokio.md)): tasks call `load(Acquire)` between `.await` points instead of `thread::sleep`.

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

- Counter starts at **98**; four threads race to increment; at most **2** succeed → prints **`capped at 100`** (never exceeds `MAX`).

**Reading `compare_exchange_weak`:**

```rust
counter.compare_exchange_weak(current, current + 1, Ordering::AcqRel, Ordering::Relaxed)
//                            ^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^
//                            expected   new value    success order     failure order
```

| Argument | Value here | Meaning |
|----------|------------|---------|
| **expected** | `current` (from prior `load`) | “I think the counter is still this.” |
| **new** | `current + 1` | “If so, write this instead.” |
| **success** ordering | `AcqRel` | If swap succeeds — atomic RMW with acquire+release semantics |
| **failure** ordering | `Relaxed` | If swap fails — only the atomic word is consulted; no extra sync |

**One CAS attempt, atomically:**

```
if counter == expected { counter = new; Ok(expected) }
else                   { Err(actual_value_now) }
```

That check-and-write is **one indivisible step** — unlike `load` then `store` separately, where another thread can sneak in between.

**Why the loop retries:**

| Failure reason | What happened |
|----------------|---------------|
| **Contention** | Another thread incremented between your `load` and CAS — `Err(actual)`; re-read and try again |
| **Cap reached** | `current >= MAX` — exit before CAS |
| **Spurious** (`_weak` only) | Hardware may fail even when values match — **retry**, same as a lost race |

Use **`compare_exchange_weak` in loops** (cheaper on some CPUs). Use **`compare_exchange`** (strong) when you need at most one attempt and no spurious failure.

**Trace from 98:** two threads can reach 99 and 100; the rest see `current >= MAX` or lose CAS races once value is 100 — cap holds without a mutex.

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

**What happened — trace the run:**

| Step | Thread | Code | Effect |
|------|--------|------|--------|
| 1 | main | `Arc::new(Gateway { polls: 0, shutdown: false })` | One shared struct on the heap |
| 2 | main | `Arc::clone` → `worker_gw`, `thread::spawn` | Worker gets its own handle to the **same** `Gateway` |
| 3 | worker | `load(Acquire)` → `false` | Keep polling |
| 4 | worker | `sleep(5ms)`, `polls.fetch_add(1, Relaxed)` | Bump metric only — repeats each loop (~5 ms apart) |
| 5 | main | `sleep(25ms)` | Lets worker run several poll cycles |
| 6 | main | `polls.load(Relaxed)` → prints **`metrics polls=…`** (often **4–5**, timing-dependent) | Supervisor reads stats — **`Relaxed`** OK: counter is standalone |
| 7 | main | `shutdown.store(true, Release)` | “Stop now” — **`Release`** publishes the shutdown signal |
| 8 | worker | next `load(Acquire)` → `true` | Sees shutdown — **`Acquire`** pairs with main’s **`Release`** |
| 9 | worker | loop exits, thread ends | No more `fetch_add` |
| 10 | main | `join()` → **`supervisor done`** | Clean shutdown |

**Why two different orderings in one struct?**

| Field | Ordering | Reason |
|-------|----------|--------|
| `polls` | `Relaxed` on `fetch_add` / `load` | Only counts polls — no other memory is “published” through this counter |
| `shutdown` | `Acquire` (worker) / `Release` (main) | Stop signal — same doorbell pattern as Level 1; use when shutdown implies “main is done setting up” |

Both fields sit in one `Gateway`, but each atomic picks the ordering that matches its job — see [Ordering cheat sheet](#memory-ordering--ordering-cheat-sheet).

**Async note:** swap `thread::spawn` for `tokio::spawn` ([Chapter 16](16_async_tokio.md)); keep `Arc<Gateway>` and the same `load`/`store`/`fetch_add` calls between `.await` points.

## Memory ordering — recap

Full table and handoff patterns are in [Memory ordering — `Ordering` cheat sheet](#memory-ordering--ordering-cheat-sheet) at the top. After the examples, the practical rule is:

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

- [The Rust Book — Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html) — mutex-first baseline; atomics go deeper below
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
3. **Ch14 port** — “Rewrite Mutex counter (Ch14 L4) as `AtomicUsize`; when is each better?”

#### Memory ordering

4. **Ordering quiz** — “Shutdown flag + published config pointer — pick orderings; justify.”
5. **When Relaxed lies** — “Metrics OK, config handoff broken — show Release/Acquire fix.”
6. **Fence intuition** — “Draw happens-before for Release store + Acquire load (Level 4).”
7. **Relaxed polls vs version** — “Why is `Relaxed` OK for Level 2 `polls` but wrong for Level 4 `version`? One sentence each.”

#### CAS and compare_exchange

8. **Counter port** — “Cap counter with CAS loop; explain spurious `compare_exchange_weak` failure.”
9. **ABA problem** — “Explain ABA in 80 words for pointer `compare_exchange` — no full queue.”
10. **Retry loop** — “Show single-shot CAS anti-pattern; fix with loop.”
11. **Double-checked locking** — “Show broken lazy init with atomics; fix with `OnceLock` or explain why not hand-roll.”

#### Races and debugging

12. **Race quiz** — “Mark 6 snippets: safe atomic, UB `static mut`, ordering bug, needs Mutex.”
13. **Visibility story** — “Writer updates `port` then `Relaxed` flag — what can reader see?”

#### Atomics vs Mutex vs channels

14. **When not** — “Three cases atomics are the wrong tool; prefer channels or Mutex.”
15. **Mutex vs RwLock** — “Read-heavy sensor cache — atomic counter vs RwLock vs Mutex.”
16. **Vec push** — “Why many threads cannot `push` to shared `Vec`; three fixes.”
17. **Profile-first** — “Gateway uses `Mutex` for counter; profiler shows lock contention — minimal atomic refactor.”
18. **Channel vs atomic flag** — “When is `crossbeam`/bounded channel + shutdown better than lone `AtomicBool`?”

#### Production and Java port

19. **Gateway metrics** — “Design `Gateway { polls, shutdown }` for async; orderings per field.”
20. **False sharing** — “Two hot atomics on same cache line — problem and mitigation in 60 words.”
21. **Lock-free queue** — “Why this chapter says don’t hand-roll; name crates/patterns instead.”


