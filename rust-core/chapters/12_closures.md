# Chapter 12: Closures and the Fn Traits

## Hook

You already used closures in [Chapter 4](04_iterators.md) — `|x| x * 2` inside `.map()`. A **closure** is an anonymous function that can **capture** variables from its scope. Rust classifies captures as **`Fn`**, **`FnMut`**, or **`FnOnce`**. That controls where you can store and call them.

## Scope — a brief tour

Capture modes, `Fn*` traits, storage, and traps — not async closures or thread pools.

| This chapter covers | Deferred |
|---------------------|----------|
| Syntax, `move`, `Fn` / `FnMut` / `FnOnce` | Thread pools, `rayon` → Ch 14 Afterparty |
| `impl Fn` helpers, `Box<dyn Fn()>` storage | HRTB, higher-ranked bounds |
| `.sort_by`, `.retain`, callback registries | Full async `Future` internals → Ch 16 |
| Capture edge cases, `RefCell` + closure | Pin / async trait objects → Ch 16 / 18 |

## Closure syntax

```rust
// Playground
fn main() {
    let factor = 3;
    let scale = |x| x * factor;
    println!("{}", scale(10));
}
```

| Form | Notes |
|------|-------|
| `\|x\| x + 1` | parameter types often inferred |
| `\|x: i32\| -> i32 { x + 1 }` | explicit types when needed |
| `\|\| println!("tick")` | no parameters |

Closures are **expressions** — you can pass them to functions, store them (with trait bounds), or return them (carefully).

## Capture modes

The compiler decides how a closure uses outer variables:

| Capture | Example | Closure trait (typical) |
|---------|---------|-------------------------|
| By shared borrow | read `factor` | `Fn` |
| By mutable borrow | `*count += 1` | `FnMut` |
| By move (ownership) | `move \|\| drop(s)` | `FnOnce` |

```rust
// Playground
fn main() {
    let mut total = 0;
    let mut add = |n| total += n; // FnMut — mutably borrows total
    add(5);
    add(2);
    println!("{}", total);
}
```

## The `move` keyword

`move` forces the closure to **take ownership** of captured values. Use when the closure may **outlive** the current scope — threads, `spawn`, or storing in a struct:

```rust
// Playground
fn main() {
    let label = String::from("worker");
    let print = move || println!("{}", label);
    print();
    // println!("{}", label); // ERROR: label moved into closure
}
```

[Chapter 14](14_multithreading.md) uses `move` on closures passed to `thread::spawn`.

## Fn, FnMut, FnOnce

These are **traits** implemented automatically for closures:

| Trait | Call style | When |
|-------|------------|------|
| `Fn()` | call many times, shared `&self` | read-only capture |
| `FnMut()` | call many times, `&mut self` | mutates captured state |
| `FnOnce()` | consumes `self` on first call | moves captured values out |

**Subtrait chain:** `Fn` implies `FnMut` implies `FnOnce`. A function expecting `FnOnce` accepts any of them; expecting `Fn` is the strictest.

```rust
// Playground
fn call_twice(f: impl Fn(i32) -> i32) {
    println!("{}", f(1));
    println!("{}", f(2));
}

fn main() {
    let double = |x| x * 2;
    call_twice(double);
}
```

## Closures and iterator adapters

`.map`, `.filter`, and `.for_each` take closures. The iterator holds the closure and calls it per item. The closure must match the adapter’s trait bound:

```rust
// Playground
fn main() {
    let nums = vec![1, 2, 3];
    let doubled: Vec<_> = nums.iter().map(|&x| x * 2).collect();
    println!("{:?}", doubled);
}
```

On `.iter()`, items are references — see [Chapter 4 — double reference](04_iterators.md#why-closures-see-&&i32-double-reference).

**Wrong — move closure with borrow after:**

```rust
// Playground — does not compile
fn main() {
    let name = String::from("log");
    let v = vec![1, 2];
    let _ = v.into_iter().map(move |_| name.len()); // name moved into closure
    // println!("{}", name); // ERROR
}
```

## Closures vs function pointers

Named functions can coerce to **`fn()`** (function pointer) when they capture nothing:

```rust
// Playground
fn add_one(x: i32) -> i32 {
    x + 1
}

fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn main() {
    println!("{}", apply(add_one, 5));
}
```

Closures that capture variables are **not** plain `fn()` — they are unique anonymous types. Use `impl Fn` or generics when you need to accept closures.

## Returning closures

Returning `impl Fn()` works when the closure owns its data or only captures `'static` references:

```rust
// Playground
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn main() {
    let add5 = make_adder(5);
    println!("{}", add5(10));
}
```

Returning a closure that borrows local stack variables fails — lifetimes ([Chapter 5](05_lifetimes.md)) forbid dangling borrows.


## Callback registry — storing closures

Filter log lines through pluggable rules. Each rule is a `Box<dyn Fn(&str) -> bool>`:

```rust
// Playground
fn main() {
    let mut rules: Vec<Box<dyn Fn(&str) -> bool>> = Vec::new();
    rules.push(Box::new(|line| line.contains("ERROR")));
    rules.push(Box::new(|line| line.starts_with("WARN")));

    let lines = ["INFO ok", "ERROR timeout", "WARN retry"];
    for line in lines {
        if rules.iter().all(|rule| rule(line)) {
            println!("blocked: {}", line);
        } else if rules.iter().any(|rule| rule(line)) {
            println!("matched: {}", line);
        }
    }
}
```

`Box<dyn Fn(...)>` homogenizes closures into one type for a `Vec`. Each closure may capture different data, but the **signature** must match. Prefer `impl Fn` in helpers that do not need storage. Boxing adds heap allocation and dynamic dispatch.

## Sort and filter with closures

[Chapter 11](11_collections.md) methods take closures when the logic is local:

```rust
// Playground
#[derive(Debug)]
struct SensorReading {
    id: u32,
    value: f64,
    valid: bool,
}

fn main() {
    let mut readings = vec![
        SensorReading { id: 1, value: 22.1, valid: true },
        SensorReading { id: 2, value: -1.0, valid: false },
        SensorReading { id: 3, value: 21.8, valid: true },
    ];

    readings.retain(|r| r.valid);
    readings.sort_by(|a, b| {
        b.value
            .partial_cmp(&a.value)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    println!("{:?}", readings);
}
```

`.retain` needs `FnMut` — it may mutate the vector while iterating. `.sort_by` compares pairs; use `sort_by_key` when you only need one field (`|r| r.id`).

## Generic helpers — `impl Fn` vs boxing

Call a closure twice without boxing:

```rust
// Playground
fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn main() {
    let scale = 3;
    let triple = |n| n * scale;
    println!("{}", apply_twice(triple, 2)); // 18
}
```

`impl Fn` monomorphizes — the compiler generates a specialized copy per closure type. Use `Box<dyn Fn()>` only when you need a homogeneous collection or trait object return type.

## Capture edge cases

**Wrong — closure moves `String` in a loop:**

```rust
// Playground — does not compile
fn main() {
    let labels = vec![String::from("a"), String::from("b")];
    let mut fns: Vec<Box<dyn Fn()>> = Vec::new();
    for label in labels {
        fns.push(Box::new(move || println!("{}", label)));
        // ERROR: label moved into closure, used again next iteration
    }
}
```

**Right — clone before `move`:**

```rust
// Playground
fn main() {
    let labels = vec![String::from("a"), String::from("b")];
    let mut fns: Vec<Box<dyn Fn()>> = Vec::new();
    for label in labels {
        fns.push(Box::new(move || println!("{}", label)));
    }
    for f in &fns {
        f();
    }
}
```

**`RefCell` + `FnMut` — shared counter:**

```rust
// Playground
use std::cell::RefCell;

fn main() {
    let count = RefCell::new(0);
    let mut bump = |n| *count.borrow_mut() += n;
    bump(5);
    bump(2);
    println!("{}", count.borrow());
}
```

Interior mutability ([Chapter 10](10_smart_pointers_interior_mutability.md)) lets a closure mutate state through a shared borrow.

## Thread handoff — `move` on `spawn`

Closures passed to [Chapter 14](14_multithreading.md) `thread::spawn` must own their captures:

```rust
// Playground
use std::thread;

fn main() {
    let label = String::from("worker-1");
    let handle = thread::spawn(move || {
        println!("running {}", label);
    });
    handle.join().expect("thread panicked");
}
```

Without `move`, the closure would borrow `label`. The thread may outlive the stack frame. `move` transfers ownership into the closure.

## Async closure note

Tokio and async code often use `async move { ... }` blocks. Same capture rules apply: use `move` when the task outlives the current scope. Full async trait and `Pin` details are in [Chapter 16](16_async_tokio.md).

```rust
// Cargo only — needs tokio in Cargo.toml: tokio = { version = "1", features = ["rt", "macros"] }
// async fn read_lines() {
//     let path = String::from("config.toml");
//     tokio::spawn(async move {
//         println!("read {}", path);
//     }).await.unwrap();
// }
```

## Java / Python contrast

| | Java | Python | Rust |
|---|------|--------|------|
| Lambda | SAM interfaces | `lambda`, nested `def` | closure + `Fn*` traits |
| Capture | effectively final | full closure | borrow / move checked |
| Store in field | functional interface type | any callable | `Box<dyn Fn()>` or generic `F: Fn()` |

## When the compiler says no

Common errors in this chapter:

| Error (typical) | Cause | Fix |
|-----------------|-------|-----|
| expected `Fn`, found `FnOnce` | closure moves captured var | `move` + redesign, or clone before |
| cannot move out of `...` | `FnOnce` consumed | call once or clone |
| borrowed after move | `move` closure took ownership | restructure scopes |
| `FnMut` in concurrent context | shared mutation | `Mutex`, channels ([Chapter 14](14_multithreading.md)) |
| expected `Fn`, found `FnOnce` | closure consumes captured value on call | clone before capture, or use `FnOnce` bound |
| `FnMut` required | `.retain` / `.sort_by` mutates through closure | change bound to `FnMut` or use `mut` closure |
| type mismatch in `Vec<Box<dyn Fn()>>` | closures with different signatures | same param/return types for every box |


## Idiom spotlight

> **Prefer `impl Fn` in helpers** that only need to call a closure a few times. Use `move` when the closure crosses threads or outlives the stack frame.

## Go deeper

- [The Rust Book — Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Functional Rust — closure topics](https://hightechmind.io/rust/)

## See also

- [Chapter 4: Iterators](04_iterators.md) — adapters that consume closures
- [Chapter 11: Collections](11_collections.md) — `.retain`, `.sort_by`
- [Chapter 7: Traits](07_structs_traits_generics.md) — trait bounds on generics
- [Chapter 14: Multithreading](14_multithreading.md) — `move` + `spawn`

### Afterparty

#### Capture and traits

1. **Fn quiz** — "Four closures: I label each Fn / FnMut / FnOnce; you correct and explain capture."
2. **move drill** — "Thread spawn snippet missing `move` — show compile error and fix."
3. **Iterator chain** — "`.filter` closure that uses `&config` — why `Fn` not `FnMut`?"
4. **RefCell bump** — "Closure mutates `RefCell<u32>` counter — which Fn trait and why?"

#### Types and storage

5. **fn vs closure** — "When can you pass `fn()` vs `impl Fn()` to the same helper?"
6. **Box dyn Fn** — "Store heterogeneous callbacks in a Vec — sketch trait object version."
7. **Return closure** — "Write `make_multiplier(f: f64) -> impl Fn(f64) -> f64` and explain `move`."
8. **Callback registry** — "Three log filters in `Vec<Box<dyn Fn(&str) -> bool>>` — all must match signature; I add a wrong one; you fix."
9. **Loop move trap** — "Building `Vec<Box<dyn Fn()>>` in a `for` loop over `String` — show move error and clone fix."

#### Collections and pipelines

10. **Double reference** — "Fix `.iter().filter(|x| ...)` type error on `Vec<String>` — show `|s|` vs `|&s|` patterns."
11. **sort_by** — "Sort `Vec<(String, u32)>` by count descending with `sort_by` closure."
12. **sort_by_key** — "Same sort with `sort_by_key` — when is key extraction cleaner?"
13. **retain valid** — "Drop invalid `SensorReading` rows in-place with `.retain` — FnMut bound."
14. **for_each vs for** — "Same side-effect loop twice: `for` vs `.for_each(|...|)` — style tradeoffs."

#### Errors and concurrency

15. **Fn bound too strict** — "Helper takes `impl Fn()` but caller passes closure that mutates — fix signature."
16. **Thread move** — "`spawn` closure borrows `String` — show error without `move` and fix."
17. **Send on Box Fn** — "When does `Box<dyn Fn() + Send>` matter for thread pool callbacks?"

#### Capstone

18. **Capstone** — "Pipeline: read lines, filter non-empty, parse `u16`, sum — all with closures; I write; you review Fn bounds."
