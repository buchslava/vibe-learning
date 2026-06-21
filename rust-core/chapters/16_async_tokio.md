# Chapter 16: Async Rust and Tokio

## Hook

Async/await shows up in many stacks (**Java** `CompletableFuture`, **Python** `asyncio`, JavaScript promises, …). Rust feels familiar in shape, different in mechanics.

`async fn` returns a **Future** — lazy work until you `.await` or spawn it.

**Tokio** is the usual runtime for network and concurrent I/O.

## Scope — a brief tour

Practical Tokio intro — not runtime internals or web-framework guides.

| This chapter covers | Deferred to See also / Afterparty |
|---------------------|-----------------------------------|
| `async` / `.await`, Future mental model | Pin/Unpin formalism, `async` trait impls in depth ([Ch7](07_structs_traits_generics.md)) |
| Tokio: `#[tokio::main]`, `spawn`, `join!`, `select!`, `timeout` | `async-std`, embedded runtimes, runtime tuning |
| Blocking footgun + `spawn_blocking` | `block_in_place`, worker thread counts |
| Async I/O overview (`TcpListener`, `fs`) | Production TCP/Modbus stacks — [Chapter 19](19_io_processes_bits.md) |
| Shutdown via `Arc<AtomicBool>` ([Ch15 L6](15_atomics_and_lockfree.md)) | `axum`, `tonic`, stream combinators |

### What this chapter skips — and where to learn it

The table above is honest scope, not a teaser list. Each deferred row is safe to ignore until you hit the trigger in the right column:

| Deferred topic | Learn it when… | Start here |
|----------------|----------------|------------|
| **Pin / Unpin** | You implement a manual `Future`, build self-referential async state, or read compiler errors mentioning `Pin` | [Rustonomicon — Pin](https://doc.rust-lang.org/nomicon/pin.html); return here after [Chapter 7](07_structs_traits_generics.md) traits |
| **`async fn` in traits** | A gateway trait needs async methods (`connect`, `poll_device`) | [Chapter 7](07_structs_traits_generics.md) — `async fn` in traits, `Send` bounds, `dyn` limits |
| **Other runtimes / tuning** | Tokio defaults starve your workload or you target `no_std` / embedded | [`async-std`](https://docs.rs/async-std) for API comparison; [Embassy](https://embassy.dev/) for embedded; [Tokio tuning](https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html) for worker thread counts |
| **`block_in_place`** | CPU-heavy or blocking work must run *on* an async worker thread, not the blocking pool | [Tokio — `block_in_place`](https://docs.rs/tokio/latest/tokio/task/fn.block_in_place.html) after you have measured executor latency (Level 5) |
| **Production TCP / Modbus stacks** | One echo connection works; you need timeouts, retry, framing, and logging in production | [Chapter 19](19_io_processes_bits.md) sync I/O first, then port the poll loop async |
| **`axum`, `tonic`, streams** | Raw `TcpListener` + `spawn` is understood; you want HTTP/gRPC routes or stream pipelines | [axum](https://docs.rs/axum), [tonic](https://github.com/hyperium/tonic), [`tokio_stream`](https://docs.rs/tokio-stream) combinators |
| **`tokio::sync::mpsc` + cancellation hygiene** | Tasks need async message passing, or `select!` drops work mid-flight | Tokio sync docs; Level 4 caveat + **Cancellation hygiene** in Afterparty |

You do **not** need every row to ship a Level 6 supervisor. Add depth when a concrete project forces the question.

## What async is

**Async** is cooperative concurrency: one OS thread hosts many **tasks**, each pausing at `.await` so the executor runs others. Unlike [Chapter 14](14_multithreading.md) threads (OS-preempted), async tasks **yield voluntarily** — which makes blocking calls inside them dangerous.

The [Rust Book — Async, Await, Futures, and Streams](https://doc.rust-lang.org/book/ch17-00-async-await.html) covers the std mental model: **futures**, **tasks**, and **executors**. This chapter applies that model with **Tokio** — a third-party runtime the book uses in examples but does not standardize.

| Idea | Plain language |
|------|----------------|
| **`async fn`** | Returns a **Future** — a state machine describing work; **does not run** until polled |
| **`.await`** | Pause **this** task until the step is ready; the **executor** runs other tasks |
| **Executor (Tokio)** | Thread pool + scheduler that **polls** futures to completion |
| vs **OS thread** ([Chapter 14](14_multithreading.md)) | Threads are preempted by the OS; async **cooperates** — a blocking call in one task can stall many |
| **Shared state** | Same concurrency rules: `Arc`, atomics ([Chapter 15](15_atomics_and_lockfree.md)), **`tokio::sync::Mutex`** — not `std::sync::Mutex` held across `.await` |

When a task hits `.await`, control returns to the executor until the waited-on step is ready:

```
caller .await ──► executor runs other tasks ──► future ready ──► caller resumes
```

### Level 0 — Mental model (sync stand-in, not async)

Before adding Tokio, anchor the **names** (`fetch`, `parse`, `log`) with ordinary sequential Rust. Real async needs a **runtime**; this Playground snippet is **not async** — it runs one step after another on one thread:

```rust
// Playground — conceptual; NOT async (no executor)
fn main() {
    let tasks = vec!["fetch", "parse", "log"];
    for t in tasks {
        println!("step: {}", t);
    }
}
```

**What happened:** prints **`step: fetch`**, **`parse`**, **`log`** in order — one thread, no concurrency. An `async fn` would **not** interleave until `.await` yields to Tokio.

All Tokio examples below are **Cargo projects**. Add this dependency once:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

## Examples: elementary → hard

The levels below build from “start a runtime” to a small async supervisor. Work through in order; after each snippet, read **what happened** before moving on.

### Level 1 — Elementary: minimal Tokio spawn

Start here: boot Tokio with `#[tokio::main]`, spawn a child task, and **await** its join handle so `main` does not exit early.

```rust
// Cargo project
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(50)).await;
        42
    });
    println!("result = {:?}", handle.await);
}
```

**What happened:**

- **`#[tokio::main]`** starts the Tokio runtime and runs `async fn main`.
- Child task sleeps 50 ms, returns **`42`**; **`handle.await`** waits for **`Ok(42)`**.
- Without **`.await`** on the handle, `main` could exit before the task finishes — dropped join handle **aborts** the task.

### Level 2 — Elementary: sequential vs concurrent `.await`

Writing two `.await`s in one function does **not** run them in parallel. Compare sequential awaits with **`tokio::join!`** to see real async concurrency:

```rust
// Cargo project
use tokio::time::{sleep, Duration, Instant};

async fn sequential() {
    sleep(Duration::from_millis(100)).await;
    sleep(Duration::from_millis(100)).await;
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    sequential().await;
    println!("sequential ~{} ms", start.elapsed().as_millis());

    let start = Instant::now();
    tokio::join!(
        sleep(Duration::from_millis(100)),
        sleep(Duration::from_millis(100)),
    );
    println!("join! ~{} ms", start.elapsed().as_millis());
}
```

**What happened:**

- **`sequential`** ≈ **200 ms** — second sleep starts after the first completes.
- **`tokio::join!`** ≈ **100 ms** — both sleeps run **concurrently** on the executor.
- For background work that outlives the caller, use **`tokio::spawn`** (Level 1).

### Level 3 — Medium: `join!` and `timeout`

Services often wait on **multiple** device polls at once and must **give up** when hardware is slow. `join!` waits for all branches; `timeout` caps how long one future may run:

```rust
// Cargo project
use tokio::time::{sleep, timeout, Duration};

async fn fast_poll() -> u16 {
    sleep(Duration::from_millis(10)).await;
    502
}

async fn slow_poll() -> u16 {
    sleep(Duration::from_millis(500)).await;
    8080
}

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(fast_poll(), fast_poll());
    println!("join ports {} {}", a, b);

    match timeout(Duration::from_millis(50), slow_poll()).await {
        Ok(port) => println!("slow ok {}", port),
        Err(_) => eprintln!("slow poll timed out"),
    }
}
```

**What happened:**

- **`join!`** runs both **`fast_poll`** tasks concurrently → prints **`502 502`** quickly.
- **`timeout(50ms, slow_poll())`** returns **`Err(_)`** — slow poll needs 500 ms. This is the **deadline** pattern for Modbus/serial timeouts.

### Level 4 — Medium: `select!` — first branch wins

When you want the **fastest** response and can abandon slower paths, `select!` races branches and **cancels** the losers:

```rust
// Cargo project
use tokio::time::{sleep, Duration};

async fn fast_cache_hit() -> &'static str {
    sleep(Duration::from_millis(5)).await;
    "cached"
}

async fn slow_device_read() -> &'static str {
    sleep(Duration::from_millis(200)).await;
    "device"
}

#[tokio::main]
async fn main() {
    let source = tokio::select! {
        v = fast_cache_hit() => v,
        v = slow_device_read() => v,
    };
    println!("source = {}", source);
}
```

**What happened:**

- Prints **`source = cached`** — fast branch completes first; **slow branch is cancelled** (dropped future).
- **Caveat:** cancelled work may stop mid-flight — ensure partial I/O is safe; see Afterparty for cancellation hygiene.

### Level 5 — Hard: blocking footgun + `spawn_blocking`

Tokio runs many async tasks on a **small** set of OS worker threads. When a task hits `.await`, it **steps aside** so the worker can run other tasks. **Blocking** code never steps aside — it holds the worker until it finishes.

| What you call | What actually happens |
|---------------|------------------------|
| `thread::sleep(100ms)` inside `async fn` | Worker thread is **stuck** for 100 ms. Every other task on that worker **waits**. |
| `tokio::time::sleep(100ms).await` | Task releases the worker. Other tasks run during the wait. |
| `std::fs::read` / heavy CPU on hot path | Same as `thread::sleep` — blocks the worker. |
| `tokio::task::spawn_blocking(...)` | Moves blocking work to a **separate** thread pool. Async workers stay free. |

**Restaurant analogy:** one waiter (worker) serves many tables (tasks). `.await` = “I’ll check back when the kitchen is ready.” `thread::sleep` = waiter stands frozen at one table while others go ignored.

Under load, a few `thread::sleep` or `std::fs::read` calls in async handlers can slow **every** connection — not just the one that blocked.

The snippet below runs two experiments with `tokio::join!`: first with `std::thread::sleep` (bad), then with `tokio::time::sleep` (good), then `spawn_blocking` for work you cannot make async.

```rust
// Cargo project
use std::thread;
use std::time::Duration as StdDuration;
use tokio::time::{sleep, Duration, Instant};

async fn bad_blocking() {
    thread::sleep(StdDuration::from_millis(100)); // stalls executor worker
}

async fn good_async_sleep() {
    sleep(Duration::from_millis(100)).await; // yields — other tasks run
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    tokio::join!(bad_blocking(), sleep(Duration::from_millis(10)));
    println!("bad path elapsed ~{} ms (10 ms task delayed)", start.elapsed().as_millis());

    let start = Instant::now();
    tokio::join!(good_async_sleep(), sleep(Duration::from_millis(10)));
    println!("good path elapsed ~{} ms", start.elapsed().as_millis());

    let n = tokio::task::spawn_blocking(|| {
        thread::sleep(StdDuration::from_millis(50));
        99
    })
    .await
    .unwrap();
    println!("spawn_blocking -> {}", n);
}
```

Quick reference for what belongs inside an async task:

| Call | In async task? | Effect |
|------|----------------|--------|
| `std::thread::sleep` | **Bad** | blocks executor worker — other tasks wait |
| `tokio::time::sleep().await` | **Good** | yields until deadline |
| `std::fs::read` on hot path | **Risky** | blocks — prefer `tokio::fs` or **`spawn_blocking`** |
| `tokio::task::spawn_blocking` | **Good** | runs closure on blocking thread pool |

**What happened:**

- **`bad_blocking` + 10 ms task** — prints elapsed ≈ **100+ ms**. The 10 ms sleep **started late** because `thread::sleep` hogged the worker. Two “concurrent” tasks ran **one after another**.
- **`good_async_sleep` + 10 ms task** — prints elapsed ≈ **100 ms**. Both tasks **shared** the worker during waits; wall time is ~max(100, 10), not 100 + 10.
- **`spawn_blocking`** — prints **`spawn_blocking -> 99`**. The 50 ms `thread::sleep` ran on a **blocking** thread; async workers were not stuck.

**Rule of thumb:** in [Chapter 14](14_multithreading.md), `thread::sleep` on its own OS thread only blocks **that** thread. Inside Tokio async code, it blocks a **shared** worker — so it hurts **all** tasks on that worker.

### Level 6 — Hard: async gateway supervisor

This mirrors [Chapter 15 Level 6](15_atomics_and_lockfree.md), but the worker is a **Tokio task** instead of an OS thread:

| Piece | Role |
|-------|------|
| **`Gateway`** | Shared state: poll counter + shutdown flag |
| **`Arc<Gateway>`** | Lets `main` and the worker **both** hold the same struct safely |
| **`run_worker`** | Loop: sleep 5 ms → increment `polls` → repeat until shutdown |
| **`tokio::spawn`** | Starts the worker **in the background** without blocking `main` |
| **`shutdown` flag** | `main` sets `true`; worker sees it and exits the loop cleanly |

**`main` is the supervisor**; **`run_worker` is the poll loop** on a device gateway. Supervisor runs for 25 ms, prints poll count, then stops the worker and waits for a clean exit.

```
main                          worker (spawned task)
 │                                  │
 ├─ create Arc<Gateway>             │
 ├─ spawn run_worker ──────────────►│ loop: if !shutdown
 │                                  │   sleep 5ms (.await — yields!)
 │                                  │   polls += 1
 ├─ sleep 25ms (.await)             │   (keeps looping…)
 ├─ print polls count               │
 ├─ shutdown = true ───────────────►│ sees shutdown, exits loop
 ├─ await join handle ◄─────────────┤ returns Ok(())
 └─ print "supervisor done"          │
```

**Why atomics again?** Same as Chapter 15: `polls` is a **metric** (`Relaxed` is fine). `shutdown` is a **signal** — `Release` when main writes, `Acquire` when the worker reads.

**Why `sleep(...).await` in the worker?** Each 5 ms pause **yields** to Tokio. `thread::sleep` here would block a worker on every iteration (Level 5 footgun).

```rust
// Cargo project
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

struct Gateway {
    polls: AtomicUsize,
    shutdown: AtomicBool,
}

async fn run_worker(gw: Arc<Gateway>) -> Result<(), &'static str> {
    while !gw.shutdown.load(Ordering::Acquire) {
        sleep(Duration::from_millis(5)).await;
        gw.polls.fetch_add(1, Ordering::Relaxed);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let gw = Arc::new(Gateway {
        polls: AtomicUsize::new(0),
        shutdown: AtomicBool::new(false),
    });

    let worker_gw = Arc::clone(&gw);
    let handle = tokio::spawn(async move { run_worker(worker_gw).await });

    sleep(Duration::from_millis(25)).await;
    println!("metrics polls={}", gw.polls.load(Ordering::Relaxed));

    gw.shutdown.store(true, Ordering::Release);
    handle.await.map_err(|_| "worker join failed")??;
    println!("supervisor done");
    Ok(())
}
```

**What happened (summary):**

1. Worker polls concurrently with `main` while supervisor sleeps 25 ms.
2. **`polls.load(Relaxed)`** prints something like **`metrics polls=4`** (~25 ms ÷ 5 ms per loop).
3. **`shutdown.store(true, Release)`** stops the worker after the current iteration.
4. **`handle.await`** waits for the task to finish. The **`??`** applies `?` to both the join result and `run_worker`’s `Result`.
5. **`supervisor done`** — no dangling background task.

**Compared to Chapter 15:** replace `thread::spawn` + `thread::sleep` with `tokio::spawn` + `tokio::time::sleep().await`. Atomics and shutdown pattern stay the same — only the runtime changes.

**`main() -> Result`** ([Chapter 8](08_errors_and_testing.md)): errors bubble to one place instead of scattering `unwrap()` in the supervisor.

### Level 7 — Medium: bounded concurrency with `Semaphore`

Unbounded `tokio::spawn` on thousands of devices can overwhelm databases or serial buses. A **`Semaphore`** caps how many in-flight operations run at once:

```rust
// Cargo project
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

async fn probe_device(id: u32) -> u32 {
    sleep(Duration::from_millis(20)).await;
    id
}

#[tokio::main]
async fn main() {
    let sem = Arc::new(Semaphore::new(2)); // at most 2 probes at once
    let mut handles = Vec::new();

    for id in 0..5u32 {
        let permit = Arc::clone(&sem);
        handles.push(tokio::spawn(async move {
            let _permit = permit.acquire().await.unwrap(); // held until task finishes
            probe_device(id).await
        }));
    }

    for h in handles {
        println!("id {}", h.await.unwrap());
    }
}
```

**What happened:**

- Five tasks are spawned, but **`acquire().await`** blocks when two permits are already taken.
- The **`_permit`** guard releases the slot when the task ends — no manual `release`.
- Same pattern protects MongoDB cursors, HTTP fan-out, and Modbus batch reads.

### Level 8 — Medium: batch spawn and `await??`

Fan out independent async work with **`tokio::spawn`**, then collect results. Each **`handle.await`** yields **`Result<T, JoinError>`** — use **`??`** when the inner task also returns **`Result`** ([Chapter 8](08_errors_and_testing.md)):

```rust
// Cargo project
use tokio::time::{sleep, Duration};

async fn fetch_reading(seed: u32) -> Result<f64, &'static str> {
    sleep(Duration::from_millis(10)).await;
    if seed == 3 {
        Err("device offline")
    } else {
        Ok(seed as f64 * 1.5)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seeds = [1u32, 2, 3];
    let mut handles = Vec::new();

    for seed in seeds {
        handles.push(tokio::spawn(fetch_reading(seed)));
    }

    for handle in handles {
        match handle.await? {
            Ok(v) => println!("reading {}", v),
            Err(e) => eprintln!("skip: {}", e),
        }
    }
    Ok(())
}
```

| Layer | Type | `?` / `??` |
|-------|------|------------|
| Outer | `JoinError` from panicked/cancelled task | first `?` on `handle.await` |
| Inner | `Result<T, E>` from your async fn | second `?` when you want to propagate |

Use **`match`** on the inner `Result` when one failure should not abort the whole batch (as above for seed `3`).

### Pipeline sketch — reader task, channel, workers

Long-running services decouple **read throughput** from **write latency** with bounded channels. One task pulls documents/frames; workers consume batches:

```rust
// Cargo project — outline
use tokio::sync::mpsc;

async fn reader_task(tx: mpsc::Sender<Vec<u8>>) -> Result<(), String> {
    for i in 0u8..3 {
        tx.send(vec![i]).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

async fn worker_task(mut rx: mpsc::Receiver<Vec<u8>>) -> Result<(), String> {
    while let Some(batch) = rx.recv().await {
        println!("process {} bytes", batch.len());
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let (tx, rx) = mpsc::channel(4); // backpressure when full
    let worker = tokio::spawn(worker_task(rx));
    reader_task(tx).await?; // dropping tx closes the channel
    worker.await.unwrap()?;
    Ok(())
}
```

**Bounded capacity** (`4` here) applies backpressure: a fast reader blocks on `send().await` when workers fall behind — preferable to unbounded memory growth.

### Polling loop — long-running HTTP or job status

APIs that return **202 + poll URL** fit a **`loop`**, **`sleep().await`**, and exit on success or timeout:

```rust
// Cargo project
use tokio::time::{sleep, timeout, Duration, Instant};

async fn job_ready(tick: u32) -> Option<&'static str> {
    if tick >= 3 {
        Some("done")
    } else {
        None
    }
}

async fn poll_until_ready(max_wait: Duration) -> Result<&'static str, &'static str> {
    let start = Instant::now();
    let mut tick = 0;
    loop {
        if let Some(status) = job_ready(tick).await {
            return Ok(status);
        }
        if start.elapsed() >= max_wait {
            return Err("timed out waiting for job");
        }
        tick += 1;
        sleep(Duration::from_millis(50)).await;
    }
}

#[tokio::main]
async fn main() {
    match timeout(Duration::from_secs(1), poll_until_ready(Duration::from_secs(5))).await {
        Ok(Ok(s)) => println!("status = {}", s),
        Ok(Err(e)) => eprintln!("{}", e),
        Err(_) => eprintln!("outer timeout"),
    }
}
```

Wrap the whole poll in **`timeout(...)`** ([Level 3](#level-3--medium-join-and-timeout)) so a stuck remote cannot loop forever.

## Techniques at a glance

Use this table as a cheat sheet while reading Tokio code elsewhere. Each row maps to a level above.

| Technique | One-line use | Level |
|-----------|--------------|-------|
| `async fn` / `.await` | lazy futures; yield points | 0–2 |
| `#[tokio::main]` | start Tokio runtime | 1 |
| `tokio::spawn` | concurrent background task | 1, 6, 8 |
| `tokio::join!` | wait for **all** branches | 2–3 |
| `tokio::select!` | **first** ready branch | 4 |
| `tokio::time::timeout` | deadline / device timeout | 3, poll loop |
| `spawn_blocking` | sync I/O or `thread::sleep` off executor | 5 |
| `Semaphore` | cap in-flight async work | 7 |
| `mpsc` channel | decouple reader and workers | pipeline sketch |
| `async_trait` | async methods on `dyn Trait` | testing boundary |
| `Arc<AtomicBool>` | shutdown flag across tasks | 6, Ch15 |

**Mutex choice:** use **`tokio::sync::Mutex`** if the lock may be held across `.await`. Holding **`std::sync::MutexGuard`** across `.await` makes the future **`!Send`** — the compiler rejects it (see edge cases below).

## Async I/O (brief)

Networking and file I/O are where async pays off: many connections wait on kernel I/O while a small thread pool keeps working. Tokio mirrors std I/O with **async** APIs — `.await` instead of blocking the thread:

| Sync ([Ch19](19_io_processes_bits.md)) | Async (Tokio) |
|----------------------------------------|---------------|
| `std::fs::read` | `tokio::fs::read` |
| `TcpListener::accept` | `tokio::net::TcpListener::accept().await` |

The sketch below accepts one connection, reads bytes, and echoes them back — a minimal pattern before a multi-connection server:

```rust
// Cargo project — outline
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let (mut socket, _) = listener.accept().await?;
    let mut buf = [0u8; 64];
    let n = socket.read(&mut buf).await?;
    socket.write_all(&buf[..n]).await?;
    Ok(())
}
```

Many connections → **one process**, **many tasks** — each `accept` can `tokio::spawn` a handler. Full protocols and error handling: [Chapter 19](19_io_processes_bits.md) + Afterparty.

## Async vs OS threads

You already have OS threads from [Chapter 14](14_multithreading.md). Async is not a replacement — it is a different tool for **many concurrent waits** on a small thread pool:

| | OS threads ([Ch14](14_multithreading.md)) | Async tasks (Tokio) |
|---|-------------------------------------------|---------------------|
| Best for | CPU-bound parallel work | Many **I/O waits** (TCP, timers) |
| Stack / memory | higher per thread | many tasks on thread pool |
| Blocking call | blocks one thread only | **blocks executor worker** — hurts all tasks on that worker |
| Shared shutdown | `Arc<AtomicBool>` ([Ch15](15_atomics_and_lockfree.md)) | same atomics behind `tokio::spawn` |

When choosing for a device gateway or network service:

```
many idle TCP/serial waits?     → async
CPU-bound parallel compute?     → threads (or rayon — Ch14 Afterparty)
one simple PLC poll loop?       → single thread may suffice
1000 connections, one process?  → async
```

## Edge cases and compiler traps

These mistakes show up constantly in real Tokio code. The table lists the symptom and the fix; the snippets below are **wrong on purpose** so you recognize them in the wild.

| Trap | Symptom | Idiom |
|------|---------|-------|
| Forgot `.await` | logic never runs; unused future warning | always `.await` or `spawn` |
| No runtime | `async fn main` won't run futures | `#[tokio::main]` |
| `std::thread::sleep` in async | latency spikes for all tasks | `tokio::time::sleep` / `spawn_blocking` |
| `MutexGuard` across `.await` | `Future not Send` compile error | `tokio::sync::Mutex`; drop guard before await |
| Blocking `std::fs` on hot path | executor starvation | `tokio::fs` / `spawn_blocking` |
| `select!` cancels losing branch | partial work / leaked state | scope cancellation; abort handles — Afterparty |

**Wrong — `std::sync::MutexGuard` across `.await`:**

```rust
// Cargo project — does not compile
use std::sync::Mutex;
use tokio::time::{sleep, Duration};

async fn bad(m: Mutex<i32>) {
    let _guard = m.lock().unwrap();
    sleep(Duration::from_millis(10)).await;
    // ERROR: future cannot be sent between threads safely (MutexGuard not Send)
}

// fn main() { tokio::runtime::Runtime::new().unwrap().block_on(bad(Mutex::new(0))); }
```

**Wrong — drop future without `.await`:**

```rust
// Cargo project
async fn important() {
    println!("never runs if not awaited");
}

#[tokio::main]
async fn main() {
    important(); // warning: unused Future — does NOT run
    // important().await; // correct
}
```

## Idiom spotlight

One paragraph to carry into production code:

> **Async for many I/O waits; threads for CPU-heavy or a single simple poll loop.** Never block the executor — **`tokio::time::sleep`**, not **`thread::sleep`**. Share shutdown with **`Arc<AtomicBool>`** ([Chapter 15](15_atomics_and_lockfree.md)). Bubble errors with **`?`** and **`main() -> Result`** at the boundary ([Chapter 8](08_errors_and_testing.md)).
>
> **Cap fan-out with `Semaphore`**. **Decouple stages with bounded `mpsc`**. **`async_trait`** for swappable I/O in tests.

## Go deeper

When this chapter’s ladder is not enough, these links cover fundamentals, channels, I/O, and the official Tokio walkthrough:

- [The Rust Book — Async, Await, Futures, and Streams](https://doc.rust-lang.org/book/ch17-00-async-await.html)
- [async fn and .await fundamentals](https://hightechmind.io/rust/) — 321
- [Async channels mpsc](https://hightechmind.io/rust/) — 328
- [Async I/O](https://hightechmind.io/rust/) — 342, 921
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)

## See also

Related chapters — read these when you need threads, atomics, errors, sync I/O, or async traits in depth:

- [Chapter 14: Multithreading](14_multithreading.md) — when threads beat async
- [Chapter 15: Atomics](15_atomics_and_lockfree.md) — `Arc<AtomicBool>` shutdown in async
- [Chapter 8: Errors](08_errors_and_testing.md) — `Result` boundary in `main`
- [Chapter 19: I/O](19_io_processes_bits.md) — sync I/O and processes
- [Chapter 7: Traits](07_structs_traits_generics.md) — async traits (advanced)

### Afterparty

Use these for runtime internals and web-framework topics not covered above.

#### Concepts and mental model

Prompts to solidify how futures, polling, and scope fit together:

1. **Future diagram** — “Draw state machine for `async fn` with two `.await` points.”
2. **Poll vs await** — “Who polls the Future — caller, executor, or Tokio runtime?”
3. **Executor vs thread** — “One paragraph: cooperative async vs preemptive OS threads.”
4. **Forgotten await** — “Show unused Future bug; fix with `.await` or `spawn`.”

#### Tokio basics

Runtime setup, spawn lifecycle, and error boundaries:

5. **Tokio scaffold** — “Minimal `#[tokio::main]` + `spawn` + join handle — explain each line.”
6. **Spawn vs await** — “What if `main` drops join handle without `.await`?”
7. **Join handle `??`** — “Explain the `??` on `handle.await` in Level 6 — outer join `Result` vs inner `run_worker` `Result`.”

#### Level 5–6 drills

Reinforce blocking vs yielding and the async gateway supervisor:

8. **Bad join timing** — “Walk through Level 5 bad path: why does a 10 ms task wait ~100 ms when paired with `thread::sleep`?”
9. **Ch15 port** — “Compare Level 6 to [Ch15 L6](15_atomics_and_lockfree.md) — what changes with `tokio::spawn`, what stays the same?”
10. **Supervisor timeline** — “Trace Level 6 step by step: when does the worker stop, and why must it use `tokio::time::sleep` not `thread::sleep`?”

#### Concurrency primitives

When to wait for all vs first, deadlines, and cancellation:

11. **join vs select** — “Two slow tasks: when `join!` vs `select!`? One automation example each.”
12. **select! scenario** — “Cancel slow request when fast path returns — full `select!` sketch.”
13. **Timeout drill** — “Wrap Modbus read `async fn` with 100 ms timeout; handle `Elapsed`.”
14. **Cancellation hygiene** — “What runs when `select!` drops the losing branch? Modbus read vs cache hit example.”

#### Blocking and I/O

Keeping the executor healthy while doing real device and file work:

15. **Blocking fix** — “Audit async snippet with `thread::sleep`, `std::fs::read`; fix each.”
16. **spawn_blocking** — “When `tokio::fs` vs `spawn_blocking` for config file read?”
17. **Tcp echo** — “Expand Async I/O outline to multi-connection echo with `spawn` per accept.”

#### Async vs threads

Architecture choices for gateways and connection counts:

18. **async vs thread** — “1000 Modbus polls — argue async vs thread pool for latency.”
19. **200 TCP connections** — “One process — async vs thread-per-connection memory story.”
20. **When thread enough** — “Single serial port poll — justify thread loop over Tokio.”

#### Atomics and shared state (Ch15)

Bridging lock-free flags from Chapter 15 into async tasks:

21. **Async sleep in worker** — “Why must Level 6’s poll loop use `tokio::time::sleep().await` on every iteration, not `thread::sleep`?”
22. **tokio Mutex** — “Rewrite bad `std::sync::MutexGuard` across await with `tokio::sync::Mutex`.”



