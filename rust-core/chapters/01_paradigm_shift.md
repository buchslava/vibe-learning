# Chapter 1: Paradigm Shift

## Hook

Many mainstream languages let you **share** data freely: references, object graphs, mutable globals — **Java** and **Python** are familiar examples. Rust says: **one owner at a time**, checked at compile time.

That rule replaces a GC and prevents most data races. Learn to read compiler errors as hints, not obstacles.

## Compiled vs interpreted


|             | Java                | Python                   | Rust                           |
| ----------- | ------------------- | ------------------------ | ------------------------------ |
| Execution   | Bytecode on JVM     | Interpreter / bytecode   | Native machine code            |
| Type checks | Mostly compile-time | Runtime                  | Compile-time                   |
| Memory      | GC                  | GC (refcount + cycle GC) | Ownership + deterministic drop |


Rust has **no GC**. When a value’s owner goes out of scope, `drop` runs immediately. No stop-the-world pauses — valuable for long-running loops and low-latency tools.

## Ownership vs garbage collection

**Python:** `b = a` makes two names point at one list; mutating through `b` affects `a`.

**Java:** references to the same object; GC reclaims when unreachable.

**Rust:** `let s2 = s1` for a `String` **moves** ownership. `s1` is invalid afterward unless you borrow or clone.

```rust
// Playground
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // move
    println!("{}", s2);
    // println!("{}", s1); // would not compile
}
```

Think of a move as **renaming** the value, not copying.

### Move edge cases (what the compiler catches)

| Situation | Java / Python | Rust |
|-----------|---------------|------|
| Reassign name to new value | old object GC’d when unreachable | old owner **dropped** when `let s1 = ...` overwrites `s1` |
| Pass to function by value | reference still in caller | **move** — caller binding dead unless returned |
| Use after move | often still works | **compile error** |

```rust
// Playground — uncomment one failing line at a time
fn main() {
    let s1 = String::from("plc_a");
    let s2 = s1; // move: s1 → s2
    // println!("{}", s1); // ERROR: use of moved value: `s1`

    let mut log = String::from("start");
    log = String::from("replaced"); // drop old "start" buffer, bind new owner
    println!("{}", log);
}
```

**Wrong — use heap value after moving into a function:**

```rust
// Playground — does not compile
fn consume(s: String) {
    println!("{}", s);
}

fn main() {
    let label = String::from("plc1");
    consume(label);
    println!("{}", label); // ERROR: borrow of moved value: `label`
}
```

**Idiomatic fixes:** borrow for read (`consume_borrow(&label)`), take and return (`fn transform(s: String) -> String`), or `.clone()` when you truly need two owners (sparingly).

## Zero-cost abstractions

Rust’s iterators, traits, and generics are designed to compile down to code as tight as hand-written C **when you use release builds** (`cargo build --release`). You do not pay for “elegant” APIs at runtime the way heavy OOP patterns can cost in Java.

## Fearless concurrency (preview)

The same borrow rules that prevent use-after-free also prevent **data races** in safe Rust: you cannot mutate shared state from two threads without synchronization (`Mutex`, channels, atomics — Part III). The compiler enforces this; you do not rely on discipline alone.

## Stack and heap

Ownership makes more sense once you see **where** values live. Rust uses two main memory regions:

- **Stack** — per-function frames, very fast allocate/deallocate (pointer bump). Size known at compile time.
- **Heap** — shared pool for data whose size is unknown or may grow. Slower; requires an explicit allocator and later **free** (in Rust: `drop` when the owner leaves scope).

In many GC-managed languages (Java, Python, C#, …) you rarely think about stack vs heap: the runtime puts much of the data on the heap and cleans up later. Rust puts **small, fixed-size data on the stack by default** and uses the heap only when the type needs it (`String`, `Vec`, and similar).


|                      | Java                                 | Python              | Rust                                     |
| -------------------- | ------------------------------------ | ------------------- | ---------------------------------------- |
| `int` / small number | often stack (local) or heap (object) | heap (`PyObject`)   | stack (`i32`, `Copy`)                    |
| growable text        | heap (`String` object)               | heap (`str` object) | stack struct **pointing at** heap buffer |
| who frees heap?      | GC                                   | GC                  | owner’s `drop` at end of scope           |
| local variable cost  | reference push                       | name binding        | often zero-cost stack slot               |


### Stack: fast and scoped

Each function call gets a **stack frame**. Locals like `i32`, `bool`, and tuples of `Copy` types sit there. When the function returns, the frame is popped — no separate “free” step.

```rust
// Playground
fn stack_demo() -> i32 {
    let a: i32 = 10;
    let b: i32 = 32;
    a + b // a and b die when this function returns
}

fn main() {
    println!("{}", stack_demo());
}
```

`a` and `b` die when `stack_demo` returns — their stack slots vanish with the frame. You still see **42** because `a + b` is not asking the caller to keep `a` or `b`. It **computes a new value**, and that value becomes the **return value** before the frame is destroyed.

Order matters: Rust evaluates `a + b`, places **42** in the return slot (for `i32`, a bitwise **copy**), then pops the frame and drops `a` and `b`. `main` receives the copied **42** and passes it to `println!`. The locals are gone; the **result** escaped via `return`.

“Die” means the **bindings** and their stack slots are destroyed — not that every value computed inside the function disappears. Only what you **return** (or otherwise move out) survives past the closing brace. With heap-backed types like `String`, the same rule applies: the owner binding dies, but the data can live on if ownership **moves** into the caller’s binding.

Nested blocks shrink live ranges — useful for [borrowing](#references-borrowing-and-dereferencing) below:

```rust
// Playground
fn main() {
    let outer = 1;
    {
        let inner = 2;
        println!("{} {}", outer, inner);
    } // inner dropped here
    println!("outer still live: {}", outer);
}
```

### Heap: growable data with an owner

Types like `String` and `Vec` store **metadata on the stack** (pointer, length, capacity) and **payload on the heap**. The stack part is small and fixed; the heap part can grow.

Mental picture for `let s = String::from("hi")`:

```
stack                          heap
┌──────────────┐              ┌───┬───┐
│ s: String    │──ptr────────▶│ h │ i │
│  len: 2      │              └───┴───┘
│  cap: 2      │
└──────────────┘
```

When `s` goes out of scope, Rust runs `drop` on `String`, which frees the heap buffer. No GC scan — cost is **predictable** and tied to scope.

```rust
// Playground
fn main() {
    let x: i32 = 42;            // entirely on stack
    let s = String::from("hi"); // stack handle → heap bytes
    let r = &s;                 // borrow: no heap copy — see [References](#references-borrowing-and-dereferencing)
    println!("{} {}", x, r);
} // s dropped here → heap "hi" freed
```

### Moves and the heap

Assigning one `String` to another **moves** the stack handle (pointer/len/cap) to the new owner; the heap buffer is **not** copied. That is why `s1` becomes invalid after `let s2 = s1` — there is only one owner responsible for freeing that heap memory.

```rust
// Playground
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // move handle; heap buffer stays in place
    println!("{}", s2);
    // println!("{}", s1); // error: s1 no longer owns the heap data
}
```

Java would copy a **reference** (two refs, one object). Python would bind another name to the same object. Rust **transfers responsibility** — fewer copies, stricter rules.

### `Copy` on stack vs heap-backed types

#### *Why Rust splits the world this way*

*Rust’s default is **move** (transfer ownership). **Copy** is a narrow, opt-in exception — not an arbitrary rule:*

> ***Every value has exactly one owner who is responsible for cleaning up any resources it holds.***

*If assignment silently duplicated a* `String` *or* `Vec` *handle, you would get **two owners for one heap allocation**. When both go out of scope, Rust would free the same memory twice — a classic double-free. C and C++ leave that to discipline; Rust makes the dangerous case **illegal at compile time**.*

*`Copy` is allowed only when a bitwise duplicate is **semantically identical** to the original: the type is self-contained, has no* `Drop` *that must run once, and duplicating it never shares external memory. Integers and floats are just stack bits — cheap and safe to copy. Heap-backed types **own** external memory, so the compiler **moves** unless you explicitly* `.clone()`*.*

*Rust treats ownership transfer as the normal case (handing someone a key) and silent duplication as a special case for values that cannot alias. Assignment feels “strict” compared to languages with implicit sharing because Rust prefers **provable correctness** over silent aliasing.*

#### Which types are `Copy` vs move?

**`Copy` types** (assignment duplicates bits; both variables stay valid):


| Category              | Examples                                                                               |
| --------------------- | -------------------------------------------------------------------------------------- |
| Integer scalars       | `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize` |
| Floating point        | `f32`, `f64`                                                                           |
| Other scalars         | `bool`, `char`                                                                         |
| Unit type             | `()`                                                                                   |
| Fixed-size arrays     | `[T; N]` when `T: Copy` (e.g. `[u8; 4]`)                                               |
| Tuples                | `(T1, T2, …)` when every element is `Copy`                                             |
| References            | `&T`, `&mut T` (the reference itself is copied; it still *points to* the same data)    |
| Raw pointers          | `*const T`, `*mut T`                                                                   |
| Function pointers     | `fn(...) -> ...`                                                                       |
| `NonNull`, `NonZero*` | Many niche std types that wrap a plain integer                                         |


**Move types** (assignment transfers ownership; the source is invalid afterward unless you borrow):


| Category                        | Examples                                                                   |
| ------------------------------- | -------------------------------------------------------------------------- |
| Growable / owned heap data      | `String`, `Vec<T>`, `Box<T>`, `HashMap<K, V>`, `HashSet<T>`, `BTreeMap`, … |
| Shared / interior mutability    | `Rc<T>`, `Arc<T>`, `RefCell<T>`, `Mutex<T>`, `RwLock<T>`                   |
| Most user-defined structs/enums | Unless you add `#[derive(Copy, Clone)]` **and** every field is `Copy`      |
| Types with custom `Drop`        | File handles, network sockets, anything that must run cleanup once         |


**Rules of thumb:**

1. **Primitives on the stack → usually `Copy`.**
2. **Anything that owns heap memory or runs `Drop` → move by default.**
3. **`Copy` and `Drop` are mutually exclusive** — a type cannot implement both.
4. **`.clone()`** is the explicit, potentially expensive deep copy when you truly need two independent heap values.

You can check any type in the playground or docs: `Copy` is a trait; if a type implements it, assignment copies; otherwise it moves.

```rust
// Playground
fn main() {
    let a = 5;
    let b = a; // Copy — two independent i32 on stack
    println!("{} {}", a, b);

    let v1 = vec![1, 2, 3];
    let v2 = v1; // move — v1 invalid; one owner for heap array
    println!("{:?}", v2);

    let s1 = String::from("hi");
    let s2 = s1.clone(); // explicit deep copy — both valid, two heap buffers
    println!("{} {}", s1, s2);
}
```

### References, borrowing, and dereferencing

Moves and `.clone()` are not the only way to use data. A **reference** is a borrow: you get access without becoming the owner. Rust has two safe reference forms:

| Type | Access | Alias rule (preview) |
|------|--------|----------------------|
| `&T` | read-only | many `&T` borrows at once |
| `&mut T` | read + write | **one** `&mut` at a time, no overlapping `&T` |

Use the **`&`** operator to **create** a reference from an owner:

```rust
// Playground
fn main() {
    let s = String::from("sensor_a");
    let r = &s; // r: &String — immutable borrow; s still owns the heap buffer

    let mut count = 0;
    let m = &mut count; // m: &mut i32 — exclusive borrow for mutation
    *m += 1; // writes through m into count (1)

    // Works: after *m += 1, the mutable borrow of count is over (last use of m)
    println!("{} {}", r, count); // sensor_a 1
}
```

Walk through the two borrows separately:

- **`r` and `s`:** `r` is a read-only handle to `s`. The heap buffer still has one owner (`s`); `r` only lets you read it. Many immutable borrows of the same value can coexist.
- **`m` and `count`:** `count` must be `mut` before you can take `&mut count`. The `*m += 1` line **dereferences** `m` and updates the owner’s slot in place — `count` becomes `1` without moving it.

The closing `println!` uses `count` directly, not `m`. That compiles because the **mutable borrow ended** when `m` was last used (`*m += 1`). The binding `m` still exists, but until you use `m` again, `count` is free for a read.

**Try adding `m` to the same `println!` — it fails:**

```rust
// Playground — does not compile
fn main() {
    let s = String::from("sensor_a");
    let r = &s;

    let mut count = 0;
    let m = &mut count;
    *m += 1;

    // ERROR E0502: cannot borrow `count` as immutable because it is also borrowed as mutable
    println!("{} {} {}", r, m, count);
}
```

`println!` expands to code that must **use every argument** for the duration of the call. Passing `m` (`&mut i32`) **re-opens** the exclusive borrow of `count`. Passing `count` in the same macro asks for an **immutable** read of the owner while that mutable borrow is active. Rust rejects the overlap — you cannot read and exclusively borrow `count` at once.

Fixes (smallest first):

1. **Read through one path** — `println!("{} {} {}", r, *m, count)` still fails for the same reason (`count` plus active `m`). Use `println!("{} {}", r, *m)` or `println!("{} {}", r, count)` — not both `m` and `count`.
2. **End the borrow with a block** — `{ let m = &mut count; *m += 1; }` then `println!("{} {}", r, count)`.
3. **Drop `m` explicitly** — `drop(m);` before printing `count` (less idiomatic than a block; [Chapter 10](10_smart_pointers_interior_mutability.md) covers `drop` in depth).

**Java / Python:** two variables can reach the same object; neither owns it in the Rust sense. **Rust:** the **owner** (`s`, `count`) stays put; `r` and `m` are temporary handles the compiler checks ([Chapter 5](05_lifetimes.md)).

References are **`Copy`**: `let r2 = r1` copies the **pointer**, not the heap data. There is still **one owner** of the buffer.

#### Dereferencing with `*`

If `r: &i32`, then `*r` is the `i32` **at** that address. The unary **`*`** operator **dereferences** — it follows the reference to the underlying value.

| Operator | Reads as | Example |
|----------|----------|---------|
| `&` | “borrow this” | `&s`, `&mut n` |
| `*` | “value behind this reference” | `*m`, `*m += 1` |

Rust **auto-derefs** in many everyday spots: `println!("{}", r)` works with `&i32`, and method calls like `r.abs()` reach through the reference for you. You still write explicit `*` when you need the **inner value** in an expression — especially **assigning or mutating through `&mut T`**:

```rust
// Playground
fn bump(n: &mut i32) {
    *n += 1; // without *, you would try to reassign the reference itself
}

fn main() {
    let x = 10;
    let r = &x;
    let sum = x + *r; // explicit: add the i32 behind r
    println!("{}", sum);

    let mut ticks = 0;
    bump(&mut ticks);
    println!("ticks = {}", ticks);
}
```

**Read-only borrow:** `*r` gives you a copy when `T: Copy` (like `i32`). You cannot write `*r = 5` if `r: &i32` — mutating requires `&mut`.

**Wrong — mutate through `&`:**

```rust
// Playground — does not compile
fn main() {
    let x = 10;
    let r = &x;
    // *r = 20; // ERROR: cannot assign to `*r`, which is behind a `&` reference
    println!("{}", r);
}
```

**Mutable borrow:** `*m = value` updates the **owner’s** data in place. That is how a function increments a caller’s counter or fills a buffer without taking ownership.

```rust
// Playground
fn main() {
    let mut frame = [0u8; 4];
    let header = &mut frame[0..2];
    header[0] = 0xDE; // slice methods auto-deref; index writes through &mut
    header[1] = 0xAD;
    println!("{:02X?}", frame);
}
```

#### References vs raw pointers (name only)

The type table above lists **raw pointers** `*const T` and `*mut T`. They also use `*` to dereference, but only inside **`unsafe`** code — common in embedded and FFI, not day-one application Rust. Safe code uses `&T` / `&mut T`; the compiler enforces borrow rules instead of trusting you.

**Rules of thumb:**

1. **Need read-only access without moving?** → `&T`
2. **Need in-place mutation of the owner?** → `&mut T` (one at a time)
3. **Callee should take ownership** (store, send, transform-and-return)? → pass by value — **move**
4. **Caller must reuse the same allocation** (loop buffer, counter, in-place edit)? → `&mut T`

#### Move vs `&mut`: who keeps the value? (rules 3–4)

These look similar in other languages — “pass something to a function and let it change” — but Rust splits them on **ownership**.

| | **Move** (`fn f(s: String)`) | **Mutable borrow** (`fn f(s: &mut String)`) |
|---|------------------------------|---------------------------------------------|
| Ownership | Transfers to the callee | Stays with the caller |
| Caller after the call | Original binding is **invalid** | Original binding is **still valid**, often changed |
| Heap buffer | Same buffer, new owner | Same buffer, same owner |
| Callee’s job | Own, transform, drop, or **return** the value | Temporarily mutate, then **give control back** |
| Typical use | “Take this and finish with it” | “Tweak my value in place” |

**Move:** the function becomes the owner. If it takes `String` by value, the caller’s `label` is gone after the call — like handing over the key. The callee may mutate and drop it, or return it to transfer ownership.

**`&mut`:** the caller keeps ownership. The function gets a **loan** — exclusive access for the call — to edit the existing data. When the call returns, the borrow ends and the caller reads the updated value.

```rust
// Playground
fn consume(mut s: String) {
    s.push('!'); // callee owns s; mutates its own local owner
} // s dropped here unless returned

fn append_bang(s: &mut String) {
    s.push('!'); // mutates caller's buffer through the borrow
}

fn main() {
    let label = String::from("plc1");

    // Move path: ownership leaves main
    let moved = label;
    consume(moved);
    // println!("{}", moved); // error: moved value

    // Mut borrow path: caller keeps label
    let mut label = String::from("plc1");
    append_bang(&mut label); // lend exclusive access for one call
    println!("{}", label);   // plc1! — same owner, updated heap data
}
```

Use a move when the callee **should** take ownership — sending a message on a channel, storing in a struct field, or returning a transformed value. After `consume(moved)`, `main` no longer frees that buffer.

Use `&mut` when the caller must **reuse** the same allocation — a `String` label in a loop, a reusable `Vec<u8>` frame buffer, or a tick counter. `&mut` mutates in place with no handoff and no `.clone()`.

For stack `Copy` types like `i32`, “move” is a bitwise copy — both sides can still use their copy. The distinction matters most for **heap-backed** owners (`String`, `Vec`, …) where move means the source name dies.

5. **Need to assign or arithmetic on the pointee?** → often explicit `*`
6. **Need an independent duplicate?** → `.clone()` or pass by value — not a longer-lived borrow

### Borrow checker edge cases

Rust’s alias rules are strict. These patterns look reasonable if you come from a language with implicit sharing (Java, Python, JavaScript, …). The compiler rejects them **before** run time.

**1. Immutable borrow blocks mutation of the owner**

```rust
// Playground — does not compile
fn main() {
    let mut s = String::from("plc");
    let r = &s;           // shared read loan starts
    s.push('!');          // ERROR: cannot borrow `s` as mutable while `r` is live
    println!("{}", r);
}
```

**Fix:** end the read borrow before mutating — nested block, or `println!` first so `r` is not used after `push`:

```rust
// Playground
fn main() {
    let mut s = String::from("plc");
    {
        let r = &s;
        println!("{}", r);
    } // r ends here
    s.push('!');
    println!("{}", s);
}
```

**2. Only one `&mut` at a time**

```rust
// Playground — does not compile
fn main() {
    let mut v = vec![1, 2, 3];
    let a = &mut v;
    let b = &mut v; // ERROR: cannot borrow `v` as mutable more than once
    a.push(4);
    println!("{:?}", b);
}
```

**3. `&` and `&mut` cannot overlap**

```rust
// Playground — does not compile
fn main() {
    let mut n = 0;
    let r = &n;
    let m = &mut n; // ERROR: cannot borrow as mutable while immutable borrow exists
    println!("{} {}", r, m);
}
```

**4. Cannot move while borrowed**

```rust
// Playground — does not compile
fn main() {
    let s = String::from("plc");
    let r = &s;
    let moved = s; // ERROR: cannot move `s` because it is borrowed
    println!("{}", r);
}
```

References are `Copy`, but they **point at** an owner. Moving the owner while a borrow is active would leave a dangling `&` — same class of bug as use-after-free, ruled out at compile time.

**5. Returning a reference to a local (preview)**

```rust
// Playground — does not compile
fn broken() -> &str {
    let s = String::from("tmp");
    &s // ERROR: `s` does not live long enough — returned ref would dangle
}
```

The owner dies at the end of `broken`; the caller cannot hold `&str` afterward. Return owned `String`, or borrow from the caller’s data ([Chapter 5](05_lifetimes.md)).

**6. Two `&mut` element borrows on one `Vec`**

```rust
// Playground — does not compile
fn main() {
    let mut frame = vec![0u8; 4];
    let a = &mut frame[0];
    let b = &mut frame[1]; // ERROR: second mutable borrow of `frame`
    *a = 1;
    *b = 2;
    println!("{:?}", frame);
}
```

Even when indices differ, `&mut frame[i]` borrows the **whole** `Vec` mutably — the compiler does not track “this element only.” Two element refs at once are two mutable borrows of the same owner.

**Fix — mutate by index (no simultaneous refs):**

```rust
// Playground
fn main() {
    let mut frame = vec![0u8; 4];
    frame[0] = 1;
    frame[1] = 2;
    println!("{:?}", frame);
}
```

**Fix — `split_at_mut` for two non-overlapping subslices:**

```rust
// Playground
fn main() {
    let mut frame = vec![0u8; 4];
    let (left, right) = frame.split_at_mut(2);
    left[0] = 1;
    right[0] = 2;
    println!("{:?}", frame);
}
```

Each subslices borrow is disjoint; the compiler can prove they do not alias.

**Related trap — `&mut` to whole `Vec` while an element borrow is still live:**

```rust
// Playground — does not compile
fn main() {
    let mut frame = vec![0u8; 4];
    let a = &mut frame[0];
    let whole = &mut frame; // ERROR: `frame` already borrowed through `a`
    whole.push(5);
    *a = 1;
}
```

End the element borrow (drop `a`’s scope) before `push` or other calls that need `&mut` to the entire collection.

### When the compiler says no

Common errors in this chapter:

| Error (typical wording) | You probably did | Smallest fix |
|-------------------------|------------------|--------------|
| use of moved value | `let s2 = s1`, or passed `String` by value | borrow `&s1`, or clone, or use return value |
| borrow of moved value | used binding after `consume(s)` | `consume(&s)` or receive owned return |
| cannot borrow as **mutable** more than once | two `&mut` to same value | one mut borrow; restructure loop |
| cannot borrow as mutable while **immutable** borrow is active | `let r = &s` then `s.push` | shrink `r`’s scope with `{ }` |
| cannot move out of … because it is borrowed | move owner while `&owner` exists | drop borrows first, then move |
| cannot assign to `*r` behind `&` reference | `*r = 5` on `&i32` | use `&mut` |
| `does not live long enough` | return `&` to local | return `String` or borrow from caller |

Read errors **top to bottom** — the first note is usually the root cause; later notes are often follow-on noise.

### Why this matters for long-running programs

Control loops and serial parsers often run for hours. **Stack allocation is essentially free**. Heap allocation is fine when you need it, but freeing at a known scope avoids GC pauses and makes worst-case latency easier to reason about — the same theme as [ownership vs GC](#ownership-vs-garbage-collection) above.

## Idiom spotlight

> **Let the type system carry intent.** Prefer `Result` and `Option` over sentinel values (`null`, `-1`, magic strings). Formalized in [Chapter 6](06_types_enums_pattern_matching.md) and [Chapter 8](08_errors_and_testing.md).

## Go deeper

- [The Rust Book — Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Functional Rust — Option basics](https://hightechmind.io/rust/)

## See also

- [Chapter 4: Iterators](04_iterators.md) — `iter` vs `into_iter` and borrow errors in chains
- [Chapter 5: Lifetimes](05_lifetimes.md) — when references must be named in types
- [Chapter 14: Multithreading](14_multithreading.md)

### Afterparty

#### Ownership, moves, and `drop`

1. **Move drill** — “Give 6 tiny Rust snippets mixing `String`, `Vec`, and `i32`. I predict `ok` or compile error; you reveal answers, quote the error message, and give the smallest fix.”
2. **Use-after-move decoder** — “Show one `println!` after a move that fails to compile. I explain the error in plain English; you refine my explanation and show the three fixes: borrow, `.clone()`, or restructure scope.”
3. **Return transfers ownership** — “Write a function `fn take(s: String) -> String` and a `main` that calls it. I trace who owns the heap buffer at each line, including after the return.”
4. **Move into a call** — “Snippet: `process(build_label())` where `build_label() -> String`. Explain when the heap allocation happens, when ownership moves into `process`, and when `drop` runs if `process` takes `String` by value.”
5. **Scope and drop** — “Give a nested-block snippet with two `String`s and one `Vec`. I mark the exact line where each heap buffer is freed vs where stack slots disappear; you correct and explain drop order.”
6. **Single-owner principle** — “State Rust’s rule ‘every value has exactly one owner’ in one sentence, then give one `String` example where breaking that rule would double-free. Use the key-handoff metaphor.”

#### Stack, heap, and memory layout

7. **Stack vs heap quiz** — “For 10 declarations (`i32`, `bool`, `[u8; 4]`, `String`, `Vec<i32>`, `&str`, `&String`, `(i32, f64)`, `Box<i32>`, `()`), I say stack-only, heap involved, or both; you draw the pointer picture for any I miss.”
8. **String layout sketch** — “For `let s = String::from("hi")`, I describe stack fields (ptr, len, cap) and heap bytes; you correct and extend to `let v = vec![1, 2, 3]`.”
9. **Stack frame drill** — “I give a 3-function call chain with `i32` locals and one `String` passed by value. Trace stack frame push/pop, when each slot dies, and when the `String` heap buffer is dropped.”
10. **Nested block live ranges** — “Snippet with `{ let inner = ... }` inside `main`. I list which bindings are alive on each line; you explain why shorter live ranges matter for borrowing later.”
11. **Borrow without heap copy** — “Show `let s = String::from("x"); let r = &s;` — I explain what is on stack vs heap and why `r` does not duplicate the buffer; you add one line that would fail because of move/borrow conflict.”

#### References and dereferencing (`&` and `*`)

12. **`&` vs `&mut` pick** — “Five tasks (log a label, increment a tick counter, parse into a temp struct, share read-only config, swap two buffers in place). I choose pass-by-value, `&T`, or `&mut T`; you explain owner count and alias rules.”
13. **Create the borrow** — “Given `let mut n = 10;` and `let s = String::from(\"plc\");`, I write the types and expressions for one `&` and one `&mut` borrow; you verify the owner is still usable afterward.”
14. **When is `*` required?** — “Four expressions mixing `&i32`, `&mut i32`, and `println!`. I mark where explicit `*r` is needed vs where auto-deref handles it; you correct with compiled examples.”
15. **Mutate through `*`** — “Fill in `fn reset(n: &mut u32) { ... }` and a `main` that calls it. I must use `*` to zero the caller’s value; you show a broken version that tries to reassign `n` instead.”
16. **Reference copy vs move** — “After `let r1 = &s; let r2 = r1;` vs `let s2 = s1;`: I compare heap owner count, which bindings stay valid, and whether heap data was copied.”
17. **Type of `*r`** — “Bindings `r: &i32`, `m: &mut i32`, `b: Box<i32>`. I name the type of `*r`, `*m`, and whether `*m = 5` mutates the owner; you extend with one `&T` where `*r = 5` fails.”
18. **Borrow blocks mutation** — “Snippet: immutable `let r = &s;` then `s.push('!')`. I explain the compile error; you fix by shrinking `r`’s scope with a nested block.”
19. **Automation counter** — “Sketch a 1 kHz loop with `ticks: u64` and a helper `fn maybe_rollover(t: &mut u64)`. I write the call site and one `*t` mutation inside the helper; you review without full thread code.”

#### Move vs mutable borrow

20. **Move or `&mut` quiz** — “Six tasks (append to a reusable log line, send a message on a channel, fill a frame `Vec<u8>` in a loop, store a device name in a struct, transform-and-return a label, increment a counter). I pick `fn take(T)` move vs `fn tweak(&mut T)`; you explain who owns the heap data after the call.”
21. **After the call** — “Two functions: `consume(String)` and `append_bang(&mut String)`. I trace which bindings in `main` are valid **after** each call and whether the heap buffer was dropped, reused, or returned.”
22. **Fix the signature** — “Show code that moves a `String` into a helper but then tries to `println!` it in `main`. I explain the error and choose the smallest fix: change to `&mut String`, restructure with a return value, or `.clone()` — you rank the idiomatic options.”
23. **Loop reuse** — “A serial parser reuses `let mut buf = Vec::with_capacity(256)` every tick. I explain why `parse_frame(buf)` (move) is wrong and write `parse_frame(&mut buf)` instead; you add one line showing the caller reading updated length after parse.”
24. **Transform-and-return** — “Task: uppercase a `String` and give it back. I write `fn upper(s: String) -> String` and call site; you contrast with an anti-pattern that takes `&mut String` when ownership should transfer.”
25. **`i32` vs `String` move** — “Same pattern with `fn add_one(n: i32)` and `fn add_bang(s: String)`: I explain why the caller can still use `n` after the call but not `s`; you map each to Copy vs heap move.”
26. **Call it twice** — “Snippet that needs to pass the same `String` to two helpers in one scope. I show why two by-value calls fail, then fix with `&str`/`&mut` borrows or one move plus `.clone()`; you flag the smell.”

#### `Copy`, move, and `.clone()`

27. **Copy eligibility quiz** — “Quiz me on 12 types (`i32`, `String`, `&str`, `Vec<i32>`, `[u8; 8]`, `(i32, String)`, `Box<i32>`, `fn()`, `bool`, `char`, `Rc<i32>`, struct with only `i32` fields). Copy, move, or ‘Copy only if derived’? Cite the rule each time.”
28. **Copy vs Drop paradox** — “Why can’t a type be both `Copy` and `Drop`? Use a `File` or socket handle: show what goes wrong if assignment bitwise-copies the handle.”
29. **Semantic copy test** — “For `i32`, `&T`, and `String`: if I duplicate the stack bits, is the result always semantically identical? When does it fail, and what does Rust do instead?”
30. **Double-free thought experiment** — “Hypothetical: `String` were `Copy`. Walk line-by-line through `let a = ...; let b = a; }` end of block — show both drops freeing the same address. Contrast with real move semantics.”
31. **When to derive Copy** — “I put `#[derive(Copy, Clone)]` on a struct with a `String` field — explain the error. Then give three struct shapes: safe to derive `Copy`, must stay move-only, and where `.clone()` belongs in the public API.”
32. **Reference is Copy** — “Four snippets: `let r2 = r1` for `&String` vs `let s2 = s1` for `String`, plus one with `&mut`. I predict ok/error; you count owners of the heap buffer after each line.”
33. **`.clone()` judgment** — “Five scenarios (store config `String`, pass into fn twice, cache key in a map, loop append, return from fn). For each, say move, borrow, or `.clone()` — and flag when `.clone()` is a design smell.”

