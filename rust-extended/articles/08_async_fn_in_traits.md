# async fn in Traits and RPITIT

Gateway traits often need `connect().await` and `poll().await`. Before Rust 1.75 you boxed futures or wrote manual associated types. **Native async fn in traits** and **RPITIT** (return-position `impl Trait` in traits) are now the standard path — with limits on `dyn Trait`.

This article assumes [Rust Core Chapter 7](../rust-core/chapters/07_structs_traits_generics.md) and [Chapter 16](../rust-core/chapters/16_async_tokio.md).

---

## 1. Before and after native async traits

**Old pattern — boxed future:**

```rust
// Playground — conceptual; needs Pin + Box in real code
use std::future::Future;
use std::pin::Pin;

trait Gateway {
    fn connect(&self) -> Pin<Box<dyn Future<Output = u32> + Send + '_>>;
}
```

**Modern — async fn in trait (Rust 1.75+):**

```rust
// Cargo project — see demos/async/demo_async_trait/
trait Gateway: Send + Sync {
    async fn connect(&self) -> Result<u32, &'static str>;
}
```

The compiler generates an opaque future type per impl. Runnable demo: [`demos/async/demo_async_trait/`](../demos/async/demo_async_trait/).

**Takeaway:** Prefer native async traits over hand-rolled `Pin<Box<dyn Future>>` for new code.

---

## 2. `Send` bounds and spawning

`tokio::spawn` needs `Future + Send + 'static`. Async trait methods inherit the same constraints when you spawn them:

```rust
// Cargo project
async fn run<G: Gateway + Send + 'static>(gw: G) {
    let _ = gw.connect().await;
}

#[tokio::main]
async fn main() {
    tokio::spawn(run(MockGateway { device_id: 1 }));
}
```

Non-`Send` captures (`Rc`, mutex guards across `.await`) break spawning — see [The Sync Prelude §4](02_sync_prelude_async_blocks.md).

**Takeaway:** Trait + async + spawn ⇒ plan for `Send + 'static` on the impl type.

---

## 3. RPITIT — opaque futures in traits

**RPITIT** lets trait methods return `impl Future<Output = T>` (or other `impl Trait`) in the trait definition. Async fn in traits is sugar atop this machinery ([RFC 3457](https://rust-lang.github.io/rfcs/3457-yield-trait.html)).

Benefits:

- Concrete future type per impl — no heap allocation by default.
- Compiler checks yield safety across `.await`.

**Takeaway:** RPITIT is how async traits stay zero-cost at the type level.

---

## 4. Why `dyn AsyncTrait` is still limited

Trait objects need a known vtable layout. Async methods return **different opaque future types** per impl — hard to objectify. You still use:

- Generics: `fn run<G: Gateway>(g: G)`
- Sync prelude factory returning `impl Future + Send` ([article 02](02_sync_prelude_async_blocks.md))
- Boxing at the boundary when you truly need `dyn`

GATs on traits are also not object-safe ([article 09](09_gats_lending_iterators.md)).

**Takeaway:** Async traits shine with generics; `dyn` async traits remain awkward — design around that.

---

## 5. Sync prelude as escape hatch

When you need a named factory with explicit bounds:

```rust
// Cargo project
use std::future::Future;

fn connect_job(id: u32) -> impl Future<Output = u32> + Send + 'static {
    async move {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        id
    }
}
```

Combine trait objects for **sync** interface methods and associated async factories where needed.

**Takeaway:** Sync prelude + explicit `Future` bounds when the public API must document spawn safety.

---

## See also

- [Rust Core → Chapter 7: Structs, traits, and generics](../rust-core/chapters/07_structs_traits_generics.md)
- [Rust Core → Chapter 16: Async and Tokio](../rust-core/chapters/16_async_tokio.md)
- [Rust Extended → The Sync Prelude](02_sync_prelude_async_blocks.md)

## Go deeper

- [RFC 3185 — async fn in traits](https://rust-lang.github.io/rfcs/3185-async-closures-and-async-fn-in-traits.html)
- [RFC 3457 — yield-safe generic effects (RPITIT)](https://rust-lang.github.io/rfcs/3457-yield-trait.html)
- [Rust Reference — trait object safety](https://doc.rust-lang.org/reference/items/traits.html#object-safety)
