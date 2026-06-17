# Five Deep Rust Facts

Most developers look at Rust and see a strict compiler, memory safety, and a steep learning curve. Under the hood, the model is weirder and more precise than “mutable vs immutable.” These five facts reframe how you read signatures, moves, matches, and generic types.

---

## 1. `&mut T` means exclusive, not “mutable”

Two shared references can mutate the same value — through interior mutability:

```rust
// Playground
use std::cell::Cell;

fn main() {
    let n = Cell::new(0);
    let r1 = &n;
    let r2 = &n;
    r1.set(1);
    r2.set(2);
    println!("{}", n.get());
}
```

Both `r1` and `r2` have type `&Cell<i32>`. Neither is `&mut`. Mutation happens inside `Cell` via copy-in/copy-out for `Copy` types.

Contrast that with an exclusive borrow:

```rust
// Playground — uncomment the second &mut to see the error
fn main() {
    let mut x = 5;
    let r1 = &mut x;
    // let r2 = &mut x; // ERROR: cannot borrow `x` as mutable more than once
    *r1 += 1;
    println!("{}", x);
}
```

Rust’s mutability is a property of the **binding** (`let mut x`), not the reference type alone. You can hold `&T` to something and still change it when the type uses **interior mutability** (`Cell`, `RefCell`, `Mutex`, and others).

The real superpower of `&mut T` is not “you may write here.” It is **exclusivity**: for the lifetime of that borrow, no other pointer may alias that memory — not shared, not mutable. The compiler enforces that at compile time.

Interior mutability moves the same logical rule to runtime (or uses special APIs like `Cell::set`) when you need shared aliases and mutation on one thread. See [Rust Core → Smart pointers and interior mutability](../rust-core/chapters/10_smart_pointers_interior_mutability.md) for `RefCell` and `Rc<RefCell<T>> to go further.

**Takeaway:** When you reach for `&mut`, you usually need **exclusivity**, not just the ability to write.

---

## 2. `std::mem::forget` is completely safe

Leaking memory does not violate Rust’s memory safety guarantees:

```rust
// Playground
use std::mem;

fn main() {
    let s = String::from("leaked on purpose");
    mem::forget(s);
    println!("Program continues — no undefined behavior.");
}
```

The heap buffer for that `String` is never freed. That is a **resource leak**, not undefined behavior (UB). No use-after-free, no double-free.

Safe code could already leak before `forget` existed as a safe function. A classic path is reference cycles with `Rc`:

```rust
// Playground
use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    peer: RefCell<Option<Rc<Node>>>,
}

fn main() {
    let a = Rc::new(Node {
        peer: RefCell::new(None),
    });
    let b = Rc::new(Node {
        peer: RefCell::new(None),
    });
    *a.peer.borrow_mut() = Some(Rc::clone(&b));
    *b.peer.borrow_mut() = Some(Rc::clone(&a));
    println!("Cycle formed — when `a` and `b` drop, memory may leak.");
}
```

When `main` ends, strong counts never reach zero. Memory leaks; the program still exits cleanly. That is why `mem::forget` was made **safe** in Rust 1.0: leaking is unfortunate, but it does not break soundness.

Memory safety in Rust means **no UB on safe code paths** — not “never leak.” Leaks are a resource problem. They can still bite you: `mem::forget` on a thread `JoinHandle` skips joining ([Rust Core → Multithreading](../rust-core/chapters/14_multithreading.md) discusses that pattern). That is a logic bug, not UB.

**Takeaway:** Safe Rust can leak; it cannot introduce undefined behavior through `forget`.

---

## 3. Move semantics are a shallow bitwise copy

Moving a `String` does not clone the heap allocation. The compiler copies the stack representation bit-for-bit and marks the old binding dead:

```rust
// Playground
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // move: ptr, len, cap copied on the stack
    println!("{}", s2);
    // println!("{}", s1); // ERROR: use of moved value: `s1`
}
```

Think of the stack slot before and after:

```
Before move          After move
┌─────────────┐      ┌─────────────┐
│ s1: ptr     │      │ s1: (dead)  │
│     len     │  →   │ s2: ptr     │
│     cap     │      │     len     │
└─────────────┘      │     cap     │
       │             └─────────────┘
       └──────────────────┘  (same heap buffer)
```

No destructor runs on `s1` at the move site. The borrow checker treats `s1` as **uninitialized** after the move. The heap buffer is untouched until the sole owner (`s2`) drops.

For `Copy` types, the same bitwise copy happens, but both bindings stay valid:

```rust
// Playground
fn main() {
    let a = 10_i32;
    let b = a; // bitwise copy — i32 is Copy
    println!("{} {}", a, b);
}
```

`Copy` types opt out of move invalidation. Non-`Copy` types transfer ownership with a cheap stack copy plus a compile-time ledger update. Deep duplication only when you call `.clone()` explicitly. See [Rust Core → Paradigm shift](../rust-core/chapters/01_paradigm_shift.md) for the ownership introduction.

**Takeaway:** A move is a cheap stack copy plus a compile-time ownership transfer, not heap duplication.

---

## 4. Match ergonomics can quietly hide types

When you match on a reference, inner bindings are often references too — even when the pattern looks like it binds owned values:

```rust
// Playground
use std::any::type_name;

fn show_type<T>(_: T) {
    println!("x is {}", type_name::<T>());
}

fn main() {
    let opt = Some(42);
    match &opt {
        Some(x) => show_type(x), // prints "x is &i32"
        None => {}
    }
}
```

Rust 2018 **match ergonomics** auto-adjusts patterns when the matched value is a reference. You write `Some(x)`; the compiler gives you `x: &i32`. The code reads as if you own the inner value.

Compare with explicit forms that make the reference visible:

```rust
// Playground
fn main() {
    let opt = Some(String::from("hi"));

    // Ergonomics: match &opt, bind &String
    match &opt {
        Some(s) => println!("borrowed: {}", s),
        None => {}
    }

    // Explicit ref in pattern — same effect
    match opt.as_ref() {
        Some(s) => println!("as_ref: {}", s),
        None => {}
    }

    // opt still owned here because we matched on references above
    println!("still have opt: {:?}", opt);
}
```

The footgun appears when you expect to **move** out of a field but matched on a reference:

```rust
// Playground — does not compile
enum Packet {
    Text(String),
    Empty,
}

fn main() {
    let p = Packet::Text(String::from("payload"));
    match &p {
        Packet::Text(s) => println!("{}", s),
        Packet::Empty => {}
    }
    // use p here — still owned, because we matched on &p
}
```

If you wrote `match p` instead, the `String` would move into the arm. Choosing `match &p` vs `match p` is an ownership decision ergonomics can hide. See [Rust Core → Pattern matching](../rust-core/chapters/06_types_enums_pattern_matching.md) for `match &s`, `ref`, and partial-move errors.

**Takeaway:** If you `match` on `&value`, assume inner bindings are references until proven otherwise.

---

## 5. `PhantomData` takes 0 bytes but alters variance

An empty struct and one carrying `PhantomData` can have the same size at runtime:

```rust
// Playground
use std::marker::PhantomData;
use std::mem::size_of;

struct Empty;

struct Holds<T>(PhantomData<T>);

fn main() {
    println!("Empty: {}", size_of::<Empty>());
    println!("Holds<i32>: {}", size_of::<Holds<i32>>());
    println!("Holds<String>: {}", size_of::<Holds<String>>());
}
```

All print `0`. `PhantomData<T>` is a **zero-sized type** — no runtime storage. Yet it tells the compiler how the surrounding type treats `T` for drop check, send/sync, and **variance**.

Raw pointers alone do not carry that information:

```rust
// Playground
use std::marker::PhantomData;

struct RawRef<'a, T>(*const T, PhantomData<&'a T>);

fn main() {
    let x = 42;
    let r = RawRef(&x as *const _, PhantomData);
    println!("{}", unsafe { *r.0 });
}
```

Without `PhantomData<&'a T>`, a struct holding only `*const T` would not participate correctly in lifetime and variance analysis. The compiler would not know this type **borrows** `T` for `'a` rather than owning it.

Variance controls whether a longer lifetime or subtype can be substituted safely. Marker choice matters:

```rust
// Playground — types for intuition; not meant as production APIs
use std::marker::PhantomData;

struct Covariant<'a, T: 'a>(PhantomData<&'a T>);
struct Invariant<T>(PhantomData<fn(T)>);

fn main() {
    let s: &str = "hello";
    let _: Covariant<&str> = Covariant(PhantomData);
    // Covariant over 'a and T when PhantomData holds &T

    let _: Invariant<i32> = Invariant(PhantomData);
    // fn(T) is invariant in T — common for types that could accept or produce T
}
```

You rarely write these markers by hand in application code. Custom containers, iterators, and unsafe wrappers rely on them. See [Rust Core → Unsafe and internals](../rust-core/chapters/18_unsafe_and_internals.md) when you build those abstractions.

**Takeaway:** `PhantomData` is compile-time metadata with no runtime cost.

---

## See also

- [Rust Core → Chapter 1: Paradigm shift](../rust-core/chapters/01_paradigm_shift.md) — ownership and moves
- [Rust Core → Chapter 6: Pattern matching](../rust-core/chapters/06_types_enums_pattern_matching.md) — `match &s`, `ref`, partial moves
- [Rust Core → Chapter 10: Smart pointers and interior mutability](../rust-core/chapters/10_smart_pointers_interior_mutability.md) — `Cell`, `RefCell`, `Rc`
- [Rust Core → Chapter 14: Multithreading](../rust-core/chapters/14_multithreading.md) — `mem::forget` on handles, `Send`/`Sync`
- [Rust Core → Chapter 18: Unsafe and internals](../rust-core/chapters/18_unsafe_and_internals.md) — raw pointers and safe wrappers

## Go deeper

- [`std::mem::forget` docs](https://doc.rust-lang.org/std/mem/fn.forget.html) — official semantics and leak vs UB
- [Match ergonomics RFC](https://rust-lang.github.io/rfcs/2005-match-ergonomics.html) — why `Some(x)` on `&Option<T>` binds `&T`
- [Rustonomicon — PhantomData and variance](https://doc.rust-lang.org/nomicon/phantom-data.html) — full variance rules for custom types
