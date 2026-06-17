# The Sync Prelude: Return an Async Block

`async fn` is the default way to write async Rust — but it hides an important seam. A **sync function that returns an `async` block** runs synchronous setup **immediately at the call site**, then hands back a lazy `Future`. That pattern is often called a **sync prelude**.

This article assumes the [`async fn` / `.await` / `tokio::spawn` mental model from Rust Core Chapter 16](../rust-core/chapters/16_async_tokio.md).

Runnable demo: [`demos/async/check_sync_prelude/`](../demos/async/check_sync_prelude/).

---

## 1. Two shapes, one Future

Both forms return something that implements `Future`. Only **when** the body runs differs.

**`async fn` — entire body is lazy:**

```rust
// Cargo project — tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
use std::future::Future;
use tokio::time::{sleep, Duration};

async fn lazy_job() -> u32 {
    println!("async body: first poll");
    sleep(Duration::from_millis(10)).await;
    42
}

#[tokio::main]
async fn main() {
    println!("main: before call");
    let fut = lazy_job();
    println!("main: have future, not awaited yet");
    let n = fut.await;
    println!("main: result = {}", n);
}
```

**Sync fn returning `async move` — boundary is explicit:**

```rust
// Cargo project
use std::future::Future;
use tokio::time::{sleep, Duration};

fn lazy_job_manual() -> impl Future<Output = u32> {
    println!("sync prelude: runs on call");
    async move {
        println!("async body: first poll");
        sleep(Duration::from_millis(10)).await;
        42
    }
}

#[tokio::main]
async fn main() {
    println!("main: before call");
    let fut = lazy_job_manual();
    println!("main: have future, not awaited yet");
    let n = fut.await;
    println!("main: result = {}", n);
}
```

**Side-by-side timing** — watch where `println!` fires:

| Step | `async fn` | sync fn + async block |
|------|------------|------------------------|
| Caller invokes function | Future created; **body not run** | **Sync prelude runs now** |
| Caller holds future | Still idle | Still idle |
| First `.await` / poll | Async body starts | Async body starts |

With `async fn`, even `println!("async body: first poll")` waits until the first poll. With a sync prelude, `println!("sync prelude: runs on call")` runs **before** the caller receives the future.

An `async fn` desugars roughly to “return an anonymous async block.” The manual form exposes that seam — and opens room for explicit bounds on the returned future (covered in section 4).

**Takeaway:** `async fn` hides the sync/async boundary; a sync return type makes it explicit.

---

## 2. Advantage 1: call-site sync side effects

Use the sync prelude for work that must happen **when the caller invokes the factory**, not when the executor polls the task.

**Validate-then-return** — fail fast before building async state:

```rust
// Cargo project
use std::future::Future;

fn start_poll_job(device_id: u32) -> Result<impl Future<Output = u32>, &'static str> {
    if device_id == 0 {
        return Err("device_id must be non-zero");
    }
    println!("registered job for device {}", device_id);
    Ok(async move {
        // lazy: runs when awaited or spawned
        device_id * 2
    })
}

#[tokio::main]
async fn main() {
    match start_poll_job(0) {
        Err(e) => println!("rejected at call: {}", e),
        Ok(fut) => println!("unexpected ok: {}", fut.await),
    }
    let fut = start_poll_job(7).expect("valid id");
    println!("result = {}", fut.await);
}
```

Invalid input returns `Err` **immediately** — no future, no spawn, no state machine allocated for the async path.

**Double-call edge case** — the prelude runs **every time** you call the function:

```rust
// Cargo project
use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};

static REGISTRATIONS: AtomicUsize = AtomicUsize::new(0);

fn register_and_run() -> impl Future<Output = ()> {
    let n = REGISTRATIONS.fetch_add(1, Ordering::Relaxed) + 1;
    println!("sync prelude: registration #{}", n);
    async move {
        println!("async body for registration #{}", n);
    }
}

#[tokio::main]
async fn main() {
    let a = register_and_run();
    let b = register_and_run();
    a.await;
    b.await;
}
```

Print order: two prelude lines **before** any async body runs. If registration must be idempotent, enforce that in the prelude — calling the factory twice is two registrations.

The sync prelude fits logging, metrics, opening handles synchronously, or registering callbacks while the caller still holds stack borrows from its own frame.

**Takeaway:** Sync prelude = eager setup and failure at call time; async body = lazy work.

---

## 3. Advantage 2: shared state without borrow pain

Multi-threaded executors may **move** a future between worker threads at each `.await`. Captures that live across yield points must be **`Send`**. The sync prelude is where you **assemble** thread-safe handles; `async move` **owns** them inside the future.

**Arc + `tokio::sync::Mutex` in the prelude:**

```rust
// Cargo project
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

fn start_counter_worker(initial: u32) -> impl Future<Output = u32> + Send {
    let state = Arc::new(Mutex::new(initial));
    println!("sync prelude: Arc and Mutex constructed");
    async move {
        sleep(Duration::from_millis(5)).await;
        let mut guard = state.lock().await;
        *guard += 1;
        *guard
    }
}

#[tokio::main]
async fn main() {
    let fut = start_counter_worker(10);
    println!("main: future returned, prelude already ran");
    println!("result = {}", fut.await);
}
```

`Arc::new` runs in the prelude on the caller’s thread. The future only holds an owned `Arc` — safe to move across `.await` and to pass to `tokio::spawn`.

**Channel setup** — wire endpoints synchronously, spawn with owned halves:

```rust
// Cargo project
use std::future::Future;
use tokio::sync::mpsc;

fn start_pipeline() -> impl Future<Output = ()> + Send {
    let (tx, mut rx) = mpsc::channel::<u32>(4);
    println!("sync prelude: channel created");
    async move {
        tx.send(1).await.expect("send");
        tx.send(2).await.expect("send");
        drop(tx);
        while let Some(n) = rx.recv().await {
            println!("received {}", n);
        }
    }
}

#[tokio::main]
async fn main() {
    start_pipeline().await;
}
```

**Contrast — borrowing the caller’s stack across `.await`:**

```rust
// Cargo project — pattern to avoid in spawned / long-lived futures
async fn borrow_across_await(label: &str) -> usize {
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    label.len() // `label` borrow must survive the await point
}

// Works when awaited directly in the caller's stack frame:
// let n = borrow_across_await("sensor-a").await;

// Breaks with tokio::spawn(borrow_across_await("sensor-a")) — needs 'static + Send
```

An `async fn` that borrows a parameter holds that borrow **inside the future state machine** until the borrow ends. That is fine for direct `.await` in the caller, but awkward for `spawn` or for storing the future. A sync prelude lets you **copy or clone into owned data** (`String`, `Arc<str>`) before returning `async move { ... }`.

**Takeaway:** Build shared handles synchronously; move owned handles into `async move`.

---

## 4. Explicit `Future` bounds: `Send`, `Sync`, lifetimes

Returning `impl Future<Output = T> + Send + 'static` (and optionally other bounds) contracts on the **future type**, not just the output. That is what `async fn` returns under the hood — but the explicit form documents intent and shows up in sync factories and library APIs.

**Document `Send` for spawn:**

```rust
// Cargo project
use std::future::Future;
use tokio::time::{sleep, Duration};

fn background_tick() -> impl Future<Output = ()> + Send + 'static {
    async move {
        sleep(Duration::from_millis(5)).await;
        println!("tick");
    }
}

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(background_tick());
    handle.await.expect("join");
}
```

`tokio::spawn` requires `Future + Send + 'static`. Writing those bounds on the return type tells callers the future is spawn-safe.

**`Send` failure — `Rc` captured across `.await`:**

```rust
// Cargo project — does not compile with + Send
use std::future::Future;
use std::rc::Rc;
use tokio::time::{sleep, Duration};

fn bad_future() -> impl Future<Output = ()> + Send {
    let data = Rc::new(0_i32);
    async move {
        let _d = Rc::clone(&data);
        sleep(Duration::from_millis(1)).await;
        // ERROR: Rc is not Send — future cannot move to another thread
    }
}
```

Fix: use `Arc` for cross-task sharing, or drop non-`Send` values **before** the first `.await` so they are not stored in the future state machine.

**`std::sync::Mutex` guard across `.await`:**

```rust
// Cargo project — does not compile with + Send
use std::future::Future;
use std::sync::Mutex;
use tokio::time::{sleep, Duration};

fn guard_across_await() -> impl Future<Output = i32> + Send {
    let m = Mutex::new(0_i32);
    async move {
        let mut guard = m.lock().expect("lock");
        *guard += 1;
        sleep(Duration::from_millis(1)).await; // guard still held — Future not Send
        *guard
    }
}
```

Fixes: scope the guard so it drops before `.await`, do synchronous work in the **sync prelude**, or use [`tokio::sync::Mutex`](../rust-core/chapters/16_async_tokio.md) whose guard is designed for async contexts.

**Lifetime on the returned future:**

```rust
// Cargo project
use std::future::Future;

fn len_when_polled<'a>(s: &'a str) -> impl Future<Output = usize> + Send + 'a {
    async move {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        s.len()
    }
}

#[tokio::main]
async fn main() {
    let text = String::from("hello");
    let n = len_when_polled(&text).await;
    println!("{}", n);
}
```

`+ 'a` ties the future’s lifetime to the borrow. You can `.await` it in the caller’s frame while `text` is alive. **`tokio::spawn` still needs `'static`** — borrowed futures cannot be spawned unless the data is owned inside the future (`async move` with `String`, not `&str`).

**`Sync` (brief):** most futures are **moved** between threads, not shared via `&Future`. You rarely need `Future + Sync`. It matters when multiple threads poll the **same** future concurrently (for example via `Shared`). Day-to-day async code cares about **`Send`**, not `Sync`.

**Takeaway:** Bound the **future**, not just the output type.

---

## 5. When to use which

| Situation | Prefer |
|-----------|--------|
| Simple handler, no setup | `async fn` |
| Eager validation, logging, or resource wiring | sync fn + async block |
| Must document `Send + 'static` for spawn | `impl Future + Send + 'static` |
| Caller stack borrows into async work | prelude clones into owned data + `async move` |
| Forgot to drive the future | unused future — work never runs |

**Unused future footgun:**

```rust
// Cargo project
use std::future::Future;

fn quiet_job() -> impl Future<Output = ()> {
    println!("sync prelude: scheduled");
    async { println!("async body: ran"); }
}

#[tokio::main]
async fn main() {
    quiet_job(); // prelude ran; async body NEVER runs
    println!("main: done");
}
```

The prelude prints; the async body does not — no `.await`, no `spawn`. Same class of bug as calling an `async fn` without `.await` ([Rust Core Ch 16](../rust-core/chapters/16_async_tokio.md)).

The sync prelude is not a replacement for `async fn` everywhere. It is a precision tool when **the call moment** matters or when the **future’s trait bounds** need to be part of the public API.

**Takeaway:** Use sync prelude when the call moment matters or future bounds need to be explicit.

---

## See also

- [Rust Core → Chapter 16: Async and Tokio](../rust-core/chapters/16_async_tokio.md) — executors, `spawn`, `Send`, blocking footguns
- [Rust Core → Chapter 12: Closures](../rust-core/chapters/12_closures.md) — `move` captures (same rules as `async move`)
- [Rust Core → Chapter 14: Multithreading](../rust-core/chapters/14_multithreading.md) — `Send` and `Sync`
- [Rust Core → Chapter 10: Smart pointers](../rust-core/chapters/10_smart_pointers_interior_mutability.md) — `Rc`/`RefCell` vs `Arc`/`Mutex`

## Go deeper

- [Async book — async/await and async blocks](https://rust-lang.github.io/async-book/03_async_await/01_chapter.html)
- [Rust FAQ — future cannot be sent between threads safely](https://www.rustfaq.org/en/error-future-cannot-be-sent-between-threads-safely-how-to-fix/)
- [RFC 3457 — yield-safe generic effects (RPITIT)](https://rust-lang.github.io/rfcs/3457-yield-trait.html) — how `impl Trait` in return position interacts with async
