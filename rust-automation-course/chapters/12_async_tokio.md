# Chapter 12: Async Rust and Tokio

## Hook

Java async (CompletableFuture, virtual threads) and Python `asyncio` cooperative tasks differ from Rust: **`async fn` returns a Future** — a state machine polled by an **executor**. Until you `.await`, nothing runs. **Tokio** is the de facto runtime for networking and services.

## `async` / `.await`

```rust
// Playground — conceptual; real async needs runtime (see Cargo lab)
fn main() {
    // Futures need an executor; this chapter's Playground snippet
    // uses blocking stand-in for mental model:
    let tasks = vec!["fetch", "parse", "log"];
    for t in tasks {
        println!("step: {}", t);
    }
}
```

Mental model: `async fn foo()` does not run `foo` immediately; calling `.await` yields control until the future is ready.

## Cargo lab: minimal Tokio

**Cargo only** — `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let h = tokio::spawn(async {
        sleep(Duration::from_millis(50)).await;
        42
    });
    println!("result = {:?}", h.await);
}
```

## Tasks and concurrency

- `tokio::spawn` — concurrent task on runtime
- `tokio::join!` / `select!` — wait on multiple futures
- timeouts: `tokio::time::timeout`

## Async I/O

`tokio::fs`, `tokio::net::TcpListener` mirror sync APIs but `.await` instead of blocking threads — good for many connections, one process.

## Async vs threads

| | OS threads | Async tasks |
|---|------------|-------------|
| Best for | CPU-bound parallel | Many I/O waits |
| Stack cost | higher per thread | many tasks, one thread pool |
| Blocking call in async | blocks executor thread | use `spawn_blocking` |

## Idiom spotlight

> **Do not call blocking `std::thread::sleep` or heavy CPU inside async without `spawn_blocking`.** It stalls the executor.

## Go deeper

- [async fn and .await fundamentals](https://hightechmind.io/rust/) — 321
- [Async channels mpsc](https://hightechmind.io/rust/) — 328
- [Async I/O](https://hightechmind.io/rust/) — 342, 921

## See also

- [Chapter 10: Threads](10_multithreading.md)
- [Chapter 15: I/O](15_io_processes_bits.md)

### Afterparty: AI Lego blocks

1. **Future diagram** — “Draw state machine for async fn with two await points.”
2. **Tokio scaffold** — “Generate minimal Tcp echo server skeleton; I fill body.”
3. **select! scenario** — “Cancel slow request when fast path returns — outline `select!`.”
4. **async vs thread** — “1000 Modbus polls — argue async vs thread pool for latency.”
5. **blocking fix** — “Identify blocking calls in async snippet; suggest `spawn_blocking`.”
6. **Python asyncio** — “Map asyncio gather to Tokio join — API comparison table.”
