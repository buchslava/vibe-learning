# Chapter 10: Smart Pointers and Interior Mutability

## Hook

Some languages treat almost everything as heap references (**Python** names, typical **Java** objects). Rust defaults to **stack + unique ownership**.

When you need heap indirection, shared ownership, or mutation through a shared borrow, reach for **smart pointers** — deliberately, not by default.

Module layout for splitting code across files is [Chapter 9](09_modules_paths_crates.md). This chapter is about **memory patterns**.

## Scope — a brief tour

Smart pointers are a large topic. This chapter covers the **std patterns** you will see in real crates — not every allocator API or custom smart pointer type.

| This chapter covers | Deferred |
|---------------------|----------|
| `Box`, `Rc`, `Arc` | Custom allocators, `NonNull` |
| `Cell`, `RefCell`, `Rc<RefCell<T>>` | `Mutex`/`RwLock` depth → [Chapter 14](14_multithreading.md) |
| `Deref` coercion, `Drop` | Pin/Unpin → [Chapter 16](16_async_tokio.md), [Chapter 18](18_unsafe_and_internals.md) |
| `Weak` (break cycles) | Full graph GC designs |

## `Box<T>` — heap, single owner

`Box` allocates on the heap but keeps **one owner** — like a unique pointer with known size for the type system:

```rust
// Playground
fn main() {
    let b = Box::new(42);
    println!("{}", b);
}
```

Use `Box` for:

- **Trait objects** — `Box<dyn Trait>` when types differ at runtime ([Chapter 7](07_structs_traits_generics.md))
- **Recursive types** — enum variants that contain themselves (e.g. AST nodes)
- **Large blobs** — move heavy data without copying the stack frame

### Recursive enum (why `Box` is required)

A type cannot contain itself with infinite size. Indirection fixes that:

```rust
// Playground
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{:?}", list);
}
```

**What happened:** `Cons` stores `i32` plus a **`Box<List>`** (pointer-sized). The chain terminates at `Nil`. Without `Box`, the compiler rejects infinite size.

### Box edge cases

| Situation | What happens |
|-----------|--------------|
| Move `Box` | ownership moves; old binding unusable |
| `*box_ref` | dereference moves inner value **out** if `T` is not `Copy` |
| Drop last `Box` owner | heap freed immediately |

**Wrong — use after move:**

```rust
// Playground — does not compile
fn main() {
    let a = Box::new(1);
    let b = a;
    println!("{}", a); // ERROR: value moved
}
```

**Move inner value out of `Box`:**

```rust
// Playground
fn main() {
    let b = Box::new(String::from("hi"));
    let s: String = *b; // moves String out; b is empty/unusable
    println!("{}", s);
}
```

## `Rc<T>` and `Arc<T>` — shared ownership

When many parts of the program need to **own** the same data (read-mostly), use reference counting:

| Type | Thread-safe | Use |
|------|-------------|-----|
| `Rc<T>` | no | single-thread graphs, shared config |
| `Arc<T>` | yes | [Chapter 14](14_multithreading.md) shared state |

```rust
// Playground
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a);
    println!("{} refs", Rc::strong_count(&a));
    println!("{} {}", a, b);
}
```

**What happened:** `Rc::clone(&a)` bumps the strong count — it does **not** deep-clone the `String`. Both `a` and `b` point at the same heap buffer.

### `clone` on the smart pointer vs `.clone()` on the inner value

| Call | Effect |
|------|--------|
| `Rc::clone(&a)` | new handle, same allocation, count +1 |
| `(*a).clone()` | deep copy of `T`, new allocation, count unchanged |

**Wrong habit from Java/Python:** calling `.clone()` on the `String` inside when you only needed another handle:

```rust
// Playground
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("config"));
    let b = Rc::clone(&a);     // cheap — shared handle
    let c = (*a).clone();      // expensive — duplicate string on heap
    println!("{} {} {}", a, b, c);
}
```

### Cycles and `Weak`

Cycles (`A` holds `Rc<B>`, `B` holds `Rc<A>`) leak memory — counts never hit zero. Break cycles with **`Weak`**: a non-owning handle that does not keep the allocation alive.

```rust
// Playground — conceptual pattern (no cycle formed)
use std::rc::{Rc, Weak};

fn main() {
    let strong = Rc::new(42);
    let weak: Weak<i32> = Rc::downgrade(&strong);
    println!("upgrade = {:?}", weak.upgrade()); // Some(42) while strong lives
    drop(strong);
    println!("after drop = {:?}", weak.upgrade()); // None
}
```

Parent/child trees often use `Rc` parent + `Weak` back-pointer to child or parent.

### Rc / Arc edge cases

| Error / symptom | Cause | Fix |
|-----------------|-------|-----|
| `Rc` in `thread::spawn` | not `Send` | `Arc` |
| Memory never freed | `Rc` cycle | `Weak`, redesign graph |
| `strong_count` stays > 0 | leaked clone handle | audit who holds `Rc` |

`Arc` uses atomic ref count — more CPU on clone/drop than `Rc`, but required for thread-safe sharing.

## `Cell` and `RefCell` — interior mutability

Sometimes you need mutation **through a shared reference** — counters in single-thread code, or fields behind `Rc`.

**`Cell`** and **`RefCell`** enforce borrow rules **at runtime** instead of compile time:

| Type | Access | Panics when |
|------|--------|-------------|
| `Cell<T>` | `.get()` / `.set()` for `Copy` types | — |
| `RefCell<T>` | `.borrow()` / `.borrow_mut()` | double `borrow_mut`, borrow while mut borrow alive |

### `Cell` for small `Copy` values

```rust
// Playground
use std::cell::Cell;

fn main() {
    let n = Cell::new(0);
    n.set(n.get() + 1);
    println!("{}", n.get());
}
```

`Cell` has no `.borrow()` — values are copied in and out. Non-`Copy` types belong in `RefCell` (or redesign ownership).

### `RefCell` basics

```rust
// Playground
use std::cell::RefCell;

fn main() {
    let total = RefCell::new(0);
    *total.borrow_mut() += 1;
    *total.borrow_mut() += 2;
    println!("{}", total.borrow());
}
```

### `Rc<RefCell<T>>` — shared mutable graph on one thread

```rust
// Playground
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let counter = Rc::new(RefCell::new(0));
    let c2 = Rc::clone(&counter);
    *counter.borrow_mut() += 1;
    *c2.borrow_mut() += 10;
    println!("{}", counter.borrow());
}
```

Do **not** share `RefCell` across threads — it is not `Sync`. For threads, use `Arc<Mutex<T>>` or channels ([Chapter 14](14_multithreading.md)).

### RefCell edge cases

**Wrong — hold an immutable borrow, then borrow mutably:**

```rust
// Playground — panics at runtime (double borrow)
use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(vec![1, 2, 3]);
    let r = cell.borrow();      // immutable borrow active
    cell.borrow_mut().push(4);  // panic: already borrowed
    drop(r);
}
```

**Fix:** shrink the immutable borrow scope with a nested block, or restructure so borrows do not overlap.

**Wrong — re-enter `borrow_mut` while guard lives:**

```rust
// Playground — panics
use std::cell::RefCell;

fn bump_twice(cell: &RefCell<i32>) {
    let mut g1 = cell.borrow_mut();
    let mut g2 = cell.borrow_mut(); // panic — g1 still alive
    *g1 += 1;
    *g2 += 1;
}
```

**Fix:** finish one mutation, drop the guard, then borrow again — or use a single `borrow_mut` block.

**Compile-time vs runtime:** the borrow checker catches overlapping `&mut` at compile time. `RefCell` moves that check to **runtime** and **panics** on violation — same logic, different moment you find the bug.

| Pattern | Compile-time borrow | `RefCell` |
|---------|----------------------|-----------|
| Overlapping `&mut` | compile error | runtime panic |
| Cost | zero | small bookkeeping |
| Thread-safe | N/A for shared `RefCell` | no — use `Mutex` |

## `Deref` and deref coercion

Smart pointers implement **`Deref`** so you can call methods on the inner value:

```rust
// Playground
fn main() {
    let s = Box::new(String::from("hi"));
    println!("len = {}", s.len()); // Deref: Box<String> -> &String -> &str for .len()
}
```

**Deref coercion** converts references at call sites:

| From | Often coerces to |
|------|------------------|
| `&String` | `&str` |
| `&Box<T>` | `&T` |
| `&Rc<T>` | `&T` |

That is why `fn f(s: &str)` accepts `&String` and `Box<String>` after appropriate refs.

### Deref edge cases

- Coercion applies to **references**, not owned values moving into `fn take(s: String)`.
- Chains stop at the target type the function expects — no infinite deref ladder in safe code.
- **`Deref` to `str` for custom newtypes** is a common pattern for string-like wrappers ([Chapter 13](13_standard_traits.md)).

## `Drop` — cleanup on scope end

When an owner goes out of scope, Rust runs **`drop`** automatically ([Chapter 1](01_paradigm_shift.md#ownership-vs-garbage-collection)).

| Type | When heap / cleanup runs |
|------|--------------------------|
| `Box<T>` | when `Box` dropped |
| `Rc` / `Arc` | when **strong count** hits 0 |
| Custom `Drop` | before memory freed, on last owner |

Custom cleanup:

```rust
// Playground
struct Logger;

impl Drop for Logger {
    fn drop(&mut self) {
        println!("Logger shut down");
    }
}

fn main() {
    let _log = Logger;
    println!("working...");
} // prints "Logger shut down" here
```

**Never panic in `Drop`** — a panicking destructor during unwind aborts the process ([Chapter 8](08_errors_and_testing.md)).

### Drop order edge cases

Locals drop in **reverse declaration order** (last declared, first dropped):

```rust
// Playground
struct Tag(&'static str);

impl Drop for Tag {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let _a = Tag("a");
    let _b = Tag("b");
    println!("running");
}
// prints: running, then drop b, then drop a
```

**Rc drop:** inner `T` drops only when the **last** `Rc`/`Arc` handle goes away — not when one clone is dropped.

```rust
// Playground
use std::rc::Rc;

struct Loud(i32);
impl Drop for Loud {
    fn drop(&mut self) {
        println!("drop Loud({})", self.0);
    }
}

fn main() {
    let a = Rc::new(Loud(1));
    let b = Rc::clone(&a);
    drop(a);
    println!("b still alive");
} // drop Loud(1) here when b goes out of scope
```

## Choosing a pointer (quick table)

| Need | Type |
|------|------|
| heap, one owner | `Box<T>` |
| shared read, one thread | `Rc<T>` |
| shared read, many threads | `Arc<T>` |
| mutate through shared ref, one thread | `RefCell<T>` / `Rc<RefCell<T>>` |
| mutate across threads | `Arc<Mutex<T>>` or channels |
| break `Rc` cycles | `Weak<T>` |

## When the compiler says no (checklist)

| Error (typical) | Cause | Fix |
|-----------------|-------|-----|
| `Rc` cannot be sent between threads | not `Send` | `Arc` |
| `RefCell` cannot be shared between threads | not `Sync` | `Arc<Mutex<T>>` |
| `already borrowed` (runtime) | overlapping `RefCell` borrows | shrink scope, restructure |
| recursive type has infinite size | direct self in enum | `Box`, `Rc`, or `Arc` |
| value moved out of `Box` | `*b` moved `T` | clone inner or keep in `Box` |
| `T: Copy` not satisfied for `Cell` | non-Copy in `Cell` | `RefCell` or owned mutation |

## Idiom spotlight

> **Reach for `Box`/`Arc` when ownership is genuinely shared or recursive — not by default.** Most data should stay owned or borrowed.
>
> **`Rc::clone` is not `.clone()` on the data.** Count handles; deep-copy only when semantics require a duplicate.
>
> **`RefCell` trades compile-time errors for runtime panics** — use for single-thread patterns; prefer compile-time borrows when you can.

## Go deeper

- [Smart pointers — Rust Book](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Interior mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
- [Reference cycles with Weak](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)

## See also

- [Chapter 9: Modules](09_modules_paths_crates.md) — crate layout
- [Chapter 12: Closures](12_closures.md) — `Fn` traits with captured handles
- [Chapter 14: Multithreading](14_multithreading.md) — `Arc`, `Mutex`, `Send`/`Sync`
- [Chapter 7: Trait objects](07_structs_traits_generics.md) — `Box<dyn Trait>`
- [Chapter 1: Ownership](01_paradigm_shift.md) — drop and move

### Afterparty: AI Lego blocks

Copy a prompt into your AI tutor after running the Playground examples. Insist on compiler-accurate answers.

#### Box and heap ownership

1. **Box why** — “When is `Box<[T]>` better than `Vec<T>` on the stack? Two cases.”
2. **Recursive list** — “Draw memory for `Cons(1, Box::new(Cons(2, Nil)))` — stack vs heap boxes.”
3. **Move out of Box** — “What happens after `let s = *box_string`? When is that idiomatic vs keeping the `Box`?”
4. **Trait object box** — “Three plugin types implement `Plugin` — sketch `Vec<Box<dyn Plugin>>` factory; why not `Vec<Plugin>`?”

#### Rc, Arc, and Weak

5. **Rc cycle** — “Explain why `Rc` cycles leak; contrast with Python reference cycles and `Weak` fix.”
6. **Handle vs deep clone** — “Audit snippet with `Rc<String>` and both `Rc::clone` and `(*rc).clone()` — label cost of each.”
7. **Arc vs Rc** — “Thread spawn with `Rc` — show error; fix with `Arc`; explain atomic count overhead in one sentence.”
8. **Weak upgrade** — “Parent/child with `Rc` parent and `Weak` child back-ref — I sketch types; you explain cycle break.”
9. **strong_count debug** — “`strong_count` stays at 2 after I thought I dropped all refs — list 5 places handles hide.”

#### RefCell and interior mutability

10. **RefCell trap** — “Show double `borrow_mut` panic; fix with scoped borrows.”
11. **Immutable then mut** — “Hold `let r = cell.borrow()` and call `borrow_mut` — explain panic; fix with nested block.”
12. **Cell vs RefCell** — “Counter `u32` vs `Vec` cache — I pick `Cell` or `RefCell` each; you correct.”
13. **Rc RefCell graph** — “Two nodes share `Rc<RefCell<Node>>` — one updates field, one reads — sketch borrow rules on one thread.”
14. **Compile vs runtime** — “Same overlapping-mut pattern: show compile error with `&mut` and runtime panic with `RefCell` side by side.”

#### Deref, Drop, and threads

15. **Deref coercion** — “Why does `fn takes_str(s: &str)` accept `&String`, `&Box<String>`, and `&Rc<String>`? Trace steps.”
16. **Drop order** — “Three `Drop` structs in one function — I predict print order; you confirm reverse declaration rule.”
17. **Rc last handle** — “Two `Rc` clones dropped at different times — when does inner `Drop` run? Step through with println in `Drop`.”
18. **Drop panic** — “Explain double-panic abort if `Drop` panics during unwind — link to Ch8.”
19. **Arc Mutex sketch** — “Diagram thread-safe cache with `Arc<Mutex<HashMap>>` — no full code.”
20. **Pick pointer** — “Five scenarios (AST node, thread cache, GUI callback graph, config string, plugin list) — I pick Box/Rc/Arc/RefCell/Weak; you grade.”

#### Capstone

21. **Java heap map** — “Map Java ‘everything is reference’ to Rust ownership — when `Arc`, when plain `&`, when neither.”
22. **Refactor to smart ptr** — “I paste struct with `Box`, `Rc`, or raw `Vec` tree — you suggest minimal smart pointer fix and justify.”
23. **Leak hunt** — “Sketch `Rc` cycle in observer pattern; refactor one edge to `Weak` and explain count after each drop.”
