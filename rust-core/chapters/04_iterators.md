# Chapter 4: Iterators

## Hook

**Iterators are a shared pattern, not a Rust quirk.** Walk a sequence once, transform or filter each step, then produce a result — without index loops.

| Stack | How the pattern shows up |
|-------|--------------------------|
| **Java** | `Stream<T>` — `map`, `filter`, `collect` |
| **Python** | `for x in items`, list comprehensions, generators |
| **C# / LINQ** | `IEnumerable`, `.Select()`, `.Where()` |
| **JavaScript** | `Array.prototype.map` / `filter` / `reduce` |
| **SQL** | `SELECT` over rows (set-oriented, not index loops) |
| **Unix** | pipes — one program’s output is the next program’s input stream |
| **Rust** | `Iterator` trait — lazy adapters + consumers |

**Rust’s version:** the **`Iterator`** trait — lazy `.map()` / `.filter()` chains that run only when you `.collect()` or call another consumer. See [Chapter 11](11_collections.md) for the collection types these pipelines walk.

## `for` is syntax sugar

[Chapter 2](02_types.md) introduced `for i in 0..3` and `loop` / `while`. Behind `for`, Rust calls **`IntoIterator`** — “turn this value into something we can walk”:

```rust
// Playground
fn main() {
    for i in 0..3 {
        println!("{}", i); // 0, 1, 2
    }

    let words = vec!["modbus", "tcp"];
    for w in &words {
        println!("{}", w); // borrows each &str
    }
}
```

| What you write | What happens |
|----------------|--------------|
| `for x in 0..10` | range implements `IntoIterator` |
| `for x in &vec` | `vec.iter()` — borrow each element |
| `for x in vec` | `vec.into_iter()` — **consume** the `Vec` |

Ranges (`..`, `..=`) are iterators. So are `.lines()` and `.chars()` on `str` from Chapter 2.

## Three ways to walk a `Vec`

| Method | You get | `Vec` after loop |
|--------|---------|------------------|
| `.iter()` | `Iterator<Item = &T>` | still usable |
| `.iter_mut()` | `Iterator<Item = &mut T>` | still usable, elements may change |
| `.into_iter()` | `Iterator<Item = T>` | **moved** — do not use `v` afterward |

```rust
// Playground
fn main() {
    let mut v = vec![1, 2, 3];

    let sum_borrow: i32 = v.iter().sum();
    println!("sum = {} vec = {:?}", sum_borrow, v);

    for n in v.iter_mut() {
        *n *= 10;
    }
    println!("after mut: {:?}", v);

    let v2 = vec![4, 5];
    for n in v2.into_iter() {
        println!("consumed item {}", n);
    }
    // println!("{:?}", v2); // error: v2 was moved by into_iter
}
```

**Rule of thumb:** default to **`.iter()`** when you only need to read. Use **`.iter_mut()`** to update in place. Use **`into_iter()`** / `for x in vec` when the collection itself should be consumed.

### `iter` vs `into_iter` — what changes

The iterator method picks the **element type** the whole pipeline sees:

| Call | `Item` type (for `Vec<T>`) | Who owns heap data after the chain? |
|------|----------------------------|-------------------------------------|
| `v.iter()` | `&T` | still `v` |
| `v.iter_mut()` | `&mut T` | still `v` |
| `v.into_iter()` | `T` | moved out of `v`; `v` is empty/unusable |

**Common habit in Java/Python-style loops:** iterating a list does not destroy the list. In Rust, `for x in v` is **`into_iter`** — it moves the **whole collection** into the loop, not just the elements you touch in the body.

**Borrow the collection — use it again after the loop:**

```rust
// Playground
fn main() {
    let v = vec![String::from("a"), String::from("b")];

    for s in &v {
        println!("{}", s); // each step: &String (borrow)
    }
    println!("{:?}", v); // ok — v was never moved
}
```

**Consume by value — the vec is gone after the loop**, even when the body does nothing with each item:

```rust
// Playground — does not compile
fn main() {
    let v2 = vec![1, 2, 3];

    for n in v2 {
        // empty body — still consumes v2 via into_iter()
        let _ = n; // i32 is Copy, but the Vec itself is not
    }
    println!("{:?}", v2); // ERROR: v2 moved into the for loop
}
```

`for n in v2` desugars to `IntoIterator::into_iter(v2)` — **`self` takes ownership of the vec**. The loop body never runs `println!`, but the iterator still walks every slot and pulls each `i32` out (a cheap copy for `Copy` types). When the loop ends, **`v2` is moved**, not the individual numbers you skipped printing.

**Same rule with a conditional body** — using only one element does not leave the rest behind:

```rust
// Playground — does not compile
fn main() {
    let v2 = vec![String::from("a"), String::from("b"), String::from("c")];

    for s in v2 {
        if s == "b" {
            println!("{}", s); // only this one is printed
        }
        // "a" and "c" were still moved out of the vec on their turns
        // and dropped here when `s` goes out of scope
    }
    println!("{:?}", v2); // ERROR: v2 moved into the for loop
}
```

Each loop step **moves the next element out of the vec** into `s`. Non-`Copy` values like `String` cannot be put back; unprinted items are dropped at the end of that iteration. The vec is empty and unusable when the loop finishes — same as if you had printed every item.

| Loop form | What moves | Use `v` after the loop? |
|-----------|------------|-------------------------|
| `for x in &v` | nothing — borrows elements | yes |
| `for x in &mut v` | nothing — mutably borrows elements | yes |
| `for x in v` | **the whole `v`** into the iterator; each element on its turn | no |

**Fix when you need the collection back:** iterate a borrow — `for x in &v` — or clone what you need before the loop.

**Wrong pipeline — owned vs borrowed mismatch:**

```rust
// Playground — does not compile; read the explanation below
fn main() {
    let nums = vec![1, 2, 3];
    // Goal: Vec<i32> of doubled values — but iter yields &i32
    let doubled: Vec<i32> = nums.iter().collect();
    // ERROR: expected i32, found &i32 (or similar mismatch on collect)
}
```

**Fix:** either borrow through the chain and copy at the end, or consume:

```rust
// Playground
fn main() {
    let nums = vec![1, 2, 3];
    let doubled: Vec<i32> = nums.iter().map(|&x| x * 2).collect(); // peel & with |&x|
    let also: Vec<i32> = nums.into_iter().map(|x| x * 2).collect(); // nums moved
    println!("{:?} {:?}", doubled, also);
}
```

**Second pass on the same data:**

```rust
// Playground
fn main() {
    let nums = vec![1, 2, 3];
    let sum: i32 = nums.iter().sum();
    println!("{}", sum);
    let max = nums.iter().fold(i32::MIN, |acc, &x| acc.max(x)); // ok — borrow again

    let v = vec![10, 20];
    let total: i32 = v.into_iter().sum();
    // let again: i32 = v.into_iter().sum(); // ERROR: v already moved
    println!("{}", total);
}
```

`into_iter()` **consumes** the collection once. `.iter()` lets you run many consumers (sum, then max, then print) as long as you do not also move `v`.

### `for` forms — quick trap sheet

| You write | Typical mistake |
|-----------|-----------------|
| `for x in v` | Using `v` after the loop — `v` was moved into the loop |
| `for x in &v` | Expecting `x` to be `T` — `x` is `&T` (often fine for `println!`) |
| `for x in &mut v` | Forgetting `*x` to mutate the element |
| `for x in 0..v.len()` with `v[i]` | Out-of-bounds panic if `v` shrinks while looping — prefer `.iter()` |

## Lazy iterators

Adapters return **another iterator** — they do not allocate a full intermediate `Vec` unless you `.collect()`.

```rust
// Playground
fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let doubled_evens: Vec<i32> = nums
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 2)
        .collect();
    println!("{:?}", doubled_evens); // [4, 8]
    println!("nums still {:?}", nums);
}
```

`.filter` and `.map` take **closures** (`|x| ...`). How closures capture variables (`Fn`, `FnMut`, `FnOnce`) is covered in [Chapter 12 — Closures](12_closures.md).

### Why closures see `&&i32` (double reference)

Two `&` layers stack — one from the iterator, one from `.filter()`:

1. **`.iter()`** yields `&i32` — a borrow of each element in the vec.
2. **`.filter()`** passes **`&` + that item** to your closure — it only checks the element, it does not take it.

```
&  +  &i32  =  &&i32
```

That is the whole story for `v.iter().filter(...)`. The table below shows how the same rule plays out for `.map()` (which takes the item directly, without an extra `&`):

| Chain | `Item` | `.filter` closure gets | `.map` closure gets |
|-------|--------|------------------------|---------------------|
| `v.iter()` | `&i32` | `&&i32` | `&i32` |
| `v.into_iter()` | `i32` | `&i32` | `i32` |

**Three ways to write the same filter** — pick one style and stay consistent:

| Closure param | Type of binding | Works for `% 2`? |
|---------------|-----------------|------------------|
| `\|x\|` | `x: &&i32` | yes — `*`/`%` auto-deref through references |
| `\|&x\|` | `x: &i32` | yes — one layer peeled |
| `\|&&x\|` | `x: i32` | yes — two layers peeled; most explicit |

Mixing `.iter()` with a closure that expects owned `i32` without peeling any reference layer is a frequent compile error.

The `Iterator` trait lives in `std` and defines `.next() -> Option<Item>`. Calling `.iter()` on standard types covers most code — when you need a custom walk, see [Implementing Iterator](#implementing-iterator) below and [Chapter 7 — associated types](07_structs_traits_generics.md#associated-types-and-supertraits).

## Common adapters

| Adapter | Effect |
|---------|--------|
| `.map(f)` | transform each item |
| `.filter(p)` | keep items where `p` is true |
| `.enumerate()` | `(index, item)` pairs |
| `.zip(other)` | pair items until one iterator ends |
| `.take(n)` / `.skip(n)` | first `n` / skip first `n` |
| `.chain(other)` | append another iterator |

```rust
// Playground
fn main() {
    let a = ['x', 'y', 'z'];
    let b = [1, 2, 3];

    for (i, ch) in a.iter().enumerate() {
        println!("{}: {}", i, ch);
    }

    for (ch, n) in a.iter().zip(b.iter()) {
        println!("{} {}", ch, n);
    }

    let first_two: Vec<_> = a.iter().take(2).collect();
    println!("{:?}", first_two);
}
```

On text, remember [Chapter 2](02_types.md) helpers: `"a\nb\n".lines()`, `"hi".chars()` — each returns an iterator over parts of the string.

### Adapter edge cases (empty, short, uneven)

| Situation | What happens |
|-----------|----------------|
| `.zip` on unequal lengths | stops when **either** iterator ends — silent truncation |
| `.take(100)` on 3 items | yields 3, no error |
| `.skip(10)` on 3 items | yields nothing |
| `.chain` | fused walk; still lazy until a consumer runs |

```rust
// Playground
fn main() {
    let ids = vec![1u16, 2];
    let vals = vec![10u16, 20, 30]; // longer than ids
    let pairs: Vec<_> = ids.iter().zip(vals.iter()).collect();
    println!("{:?}", pairs); // [(1, 10), (2, 20)] — third value dropped

    let empty: Vec<i32> = vec![];
    let n: i32 = empty.iter().sum(); // 0 — not an error
    let hit = empty.iter().find(|&&x| x > 0); // None
    println!("sum={} find={:?}", n, hit);
}
```

**Wrong — two mutable walks at once:**

```rust
// Playground — does not compile
fn main() {
    let mut v = vec![1, 2, 3];
    for a in v.iter_mut() {
        for b in v.iter_mut() {
            // ERROR: cannot borrow `v` as mutable more than once
            *a += *b;
        }
    }
}
```

Rust forbids overlapping `&mut` borrows, including through nested `iter_mut()` loops. Fix: index by position, use `.enumerate()`, or collect indices first — patterns from [Chapter 1](01_paradigm_shift.md#references-borrowing-and-dereferencing).

## Consumers — finishing the pipeline

A **consumer** drains the iterator and produces a value (or side effect).

| Consumer | Result |
|----------|--------|
| `.collect()` | build `Vec`, `String`, `HashMap`, … |
| `.sum()` / `.product()` | numeric fold |
| `.count()` | number of items |
| `.find(p)` | `Option<Item>` — first match |
| `.any(p)` / `.all(p)` | `bool` |
| `.fold(init, f)` | single accumulated value |

```rust
// Playground
fn main() {
    let nums = vec![3, 1, 4, 1, 5];
    let sum: i32 = nums.iter().sum();
    let max = nums.iter().fold(i32::MIN, |acc, &x| acc.max(x));
    let has_even = nums.iter().any(|&x| x % 2 == 0);
    println!("sum={} max={} has_even={}", sum, max, has_even);
}
```

### `collect` and type hints

`collect()` can build many collection types. Sometimes Rust needs help:

```rust
// Playground
fn main() {
    let nums = vec![1, 2, 3];
    let doubled: Vec<i32> = nums.iter().map(|&x| x * 2).collect();
    let also: Vec<i32> = nums.iter().map(|&x| x * 2).collect::<Vec<_>>();
    println!("{:?} {:?}", doubled, also);
}
```

`Vec<_>` asks the compiler to infer the element type. Prefer an explicit binding type (`let v: Vec<i32> = ...`) when `collect` is the only clue.

**Wrong — `collect` without enough type context:**

```rust
// Playground — does not compile
fn main() {
    let nums = vec![1, 2, 3];
    let doubled = nums.iter().map(|&x| x * 2).collect();
    // ERROR: type annotations needed — many types implement FromIterator
}
```

**Wrong — collecting references, then dropping the owner:**

`drop(value)` (from `std::mem::drop`) **ends the value's lifetime early** — it runs cleanup (same as when a variable goes out of scope at `}`) and frees the owned data. Values normally drop automatically at scope end; you call `drop` only when you need to release something *before* the closing brace. Here the example tries to destroy `lines` while `tags` still holds `&str` slices pointing into it:

```rust
// Playground — does not compile
fn main() {
    let lines: Vec<String> = vec![String::from("PORT=502")];
    let tags: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    drop(lines); // explicit early destroy — would free the strings `tags` points at
    // println!("{:?}", tags); // would dangle — often fails earlier:
    // tags borrows `lines`, so drop(lines) is ERROR while tags is alive
}
```

Owned collection after parsing is the safe default: `.map(|s| s.clone())` or collect into `Vec<String>` before dropping the source buffer. Lifetimes formalize this in [Chapter 5](05_lifetimes.md).

### Consumer edge cases

| Call | Empty input | Notes |
|------|-------------|-------|
| `.sum()` | `0` for numbers | type must implement `Sum` |
| `.product()` | `1` for multiplication | |
| `.count()` | `0` | |
| `.find(...)` | `None` | not a panic — use `if let` / `match` |
| `.max()` / `.min()` | `None` | |
| `.any` / `.all` | `false` / `true` | empty `all` is vacuously true |

```rust
// Playground
fn main() {
    let ports: Vec<u16> = vec![];
    let ok = ports.iter().all(|&p| p >= 1 && p <= 65535);
    println!("all in range? {}", ok); // true on empty — know your domain rule

    let logs = vec!["ok", "ERROR: timeout"];
    if let Some(line) = logs.iter().find(|&&s| s.contains("ERROR")) {
        println!("first error line: {}", line);
    }
}
```

## Implementing Iterator

Most code uses `.iter()` on collections. When you own the walk logic — numeric ranges, non-empty lines, frame parsing — implement `Iterator` yourself.

### Inclusive range counter

Walk integers `1..=5` without building a `Vec` first — useful for sample indices, tick counts, or any stepped sequence:

```rust
// Playground
struct RangeCounter {
    next: u16,
    end: u16,
}

impl Iterator for RangeCounter {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next > self.end {
            return None;
        }
        let n = self.next;
        self.next += 1;
        Some(n)
    }
}

fn main() {
    let values: Vec<u16> = RangeCounter { next: 1, end: 5 }.collect();
    let sum: u16 = RangeCounter { next: 1, end: 4 }.sum(); // 1 + 2 + 3 + 4
    println!("values={:?} sum={}", values, sum);
}
```

`type Item = u16` is an **associated type** — see [Chapter 7](07_structs_traits_generics.md#associated-types-and-supertraits). Adapters like `.map` and `.sum` use it to know what flows through the pipeline.

### Non-empty line iterator

Skip blank lines while walking config text:

```rust
// Playground
struct NonEmptyLines<'a> {
    inner: std::str::Lines<'a>,
}

impl<'a> Iterator for NonEmptyLines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.inner.next()?;
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                return Some(trimmed);
            }
        }
    }
}

fn main() {
    let cfg = "PORT=502\n\nHOST=127.0.0.1\n";
    let keys: Vec<_> = NonEmptyLines { inner: cfg.lines() }
        .filter_map(|line| line.split('=').next())
        .collect();
    println!("{:?}", keys);
}
```

State lives in the struct fields. `next` returns `None` when the inner iterator is exhausted — that is how every iterator signals "done."

### Implementing Iterator edge cases

**Empty range — immediate `None`:**

```rust
// Playground
struct RangeCounter {
    next: u16,
    end: u16,
}

impl Iterator for RangeCounter {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next > self.end {
            return None;
        }
        let n = self.next;
        self.next += 1;
        Some(n)
    }
}

fn main() {
    let scan = RangeCounter { next: 5, end: 4 }; // empty range: start already past end
    let n = scan.count();
    println!("{}", n); // 0
}
```

**Infinite range — always pair with `.take(n)`:**

```rust
// Playground
struct Counter {
    n: u64,
}

impl Iterator for Counter {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let v = self.n;
        self.n += 1;
        Some(v)
    }
}

fn main() {
    let first_five: Vec<_> = Counter { n: 0 }.take(5).collect();
    println!("{:?}", first_five); // [0, 1, 2, 3, 4]
}
```

**`IntoIterator` vs `Iterator`:** collections implement `IntoIterator` so `for x in vec` works. Your type can implement both — `Iterator` for `.next()`, and `IntoIterator` with `type Item = Self; type IntoIter = Self` when consuming `self` in a `for` loop is natural.

## When the compiler says no

Common errors in this chapter:

| Error message (typical) | Likely cause | Smallest fix |
|-------------------------|--------------|--------------|
| value used after move | `for x in v` or `into_iter()` then `v` again | `for x in &v` or `.iter()` |
| cannot borrow as mutable more than once | nested `iter_mut` | one mut borrow at a time; restructure |
| type annotations needed for `collect` | ambiguous target collection | `let x: Vec<T> = ...` or turbofish |
| expected `T`, found `&T` in `collect` | `.iter()` but collecting owned values | `|&x|` in `map` or use `into_iter()` |
| borrow of moved value | `.into_iter().sum()` then reuse `v` | clone input or use `.iter()` |
| lifetimes / may not live long enough | `Vec<&str>` pointing into dropped `String` | own the strings |

Read the **first** error top-down; iterator chains confuse the borrow checker, but the fix is usually “wrong walk mode (`iter` vs `into_iter`)” or “references outlive owner.”

## Java / Python contrast

| | Java | Python | Rust |
|---|------|--------|------|
| Lazy pipeline | `Stream` (often from collection) | generator / itertools | `Iterator` adapters |
| Eager list comp | — | `[f(x) for x in xs]` | `.iter().map(f).collect()` |
| Consume collection | stream from list | `for x in list` | `into_iter()` moves |
| Index loop | `for (int i=0; …)` | `for i in range(len)` | prefer `.iter().enumerate()` |

## Idiom spotlight

> **Prefer iterator chains over index loops.** Use `.iter()`, `.enumerate()`, or `.windows(2)` instead of `for i in 0..v.len()` with `v[i]`. Sliding windows: [Chapter 11](11_collections.md).

## Go deeper

- [The Rust Book — Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Functional Rust — iterator topics](https://hightechmind.io/rust/)

## See also

- [Chapter 1: Ownership and borrowing](01_paradigm_shift.md#references-borrowing-and-dereferencing) — `iter` vs `into_iter`
- [Chapter 2: Types and expressions](02_types.md) — ranges, `for`, string iterators
- [Chapter 5: Lifetimes](05_lifetimes.md) — borrows inside iterator chains
- [Chapter 7: Structs, traits, and generics](07_structs_traits_generics.md#associated-types-and-supertraits) — `type Item`, associated types
- [Chapter 11: Collections](11_collections.md) — `Vec`, `HashMap`, `.windows`
- [Chapter 12: Closures](12_closures.md) — `Fn` traits for adapters

### Afterparty



#### `for`, ranges, and three walk modes

1. **for desugar quiz** — “Four `for` loops (`0..n`, `&vec`, `vec`, `&mut vec`). I say which calls `iter`, `into_iter`, or range; you correct and show owner state after the loop.”
2. **iter vs into_iter** — “Give 4 snippets using `Vec`; I predict whether `v` is usable after the loop; you explain move vs borrow.”
3. **iter_mut drill** — “Task: double every element in `Vec<f64>` in place without allocating a new `Vec`. I write the loop; you review `*n` and borrow rules.”
4. **Range vs collect** — “When is `for i in 0..n` better than `(0..n).collect::<Vec<_>>()`? Give two automation examples (retry count vs materializing indices).”

#### Adapters and lazy pipelines

5. **Adapter chain** — “Task: parse lines, trim, keep non-empty, parse as `u16`. I sketch `.lines().map(...).filter(...).collect()`; you refine.”
6. **zip pairs** — “Two `Vec<u16>`: register IDs and values. Build `Vec<(u16, u16)>` with `.zip()`; I write it; you handle length mismatch policy.”
7. **take and skip** — “Paginate a log: skip first 100 lines, take next 20. I use `.skip().take()` on `.lines()`; you show one pitfall if the source is not recomputed.”
8. **chain iterators** — “Concatenate header rows and body rows (two `&[u8]` slices) without copying bytes into one array first — sketch with `.iter().chain()`.”
9. **enumerate vs index** — “Same sum-over-evens task twice: index `for` vs `.enumerate()`. Compare readability and bounds-check risk.”
10. **Lazy vs eager** — “Explain when `filter().map()` allocates vs when `.collect()` forces work. One example with `println!` in `map` showing evaluation order.”

#### Consumers and types

11. **collect turbofish** — “Three `collect()` calls that fail without hints — I add type annotation or turbofish; you verify.”
12. **find and Option** — “Wire scan: `Vec<&str>` of lines, find first containing `ERROR`. I return `Option<&str>` with `.find()`; you contrast with `.filter().next()`.”
13. **fold vs sum** — “Compute max and count in one pass with `.fold()` vs calling `.max()` and `.len()` separately — when is fold worth it?”
14. **any and all** — “Validate a batch: all ports in 1..=65535, any line starts with `#`. I write `.all()` / `.any()` predicates; you fix one double-reference mistake.”

#### Errors, ownership, and automation

15. **Moved Vec mistake** — “Show code that does `for x in v` then uses `v` again. I explain the error and fix with `.iter()` or clone; you rank fixes.”
16. **Borrow in chain** — “Snippet: build `Vec<&str>` from `String` lines then drop the `String`. I explain why it fails; you fix with owned `String` or different lifetime design.”
17. **Modbus-style scan** — “List of raw register values `Vec<u16>`: filter evens, map to `f64` scale 0.1, sum. I write the iterator chain; you check overflow and types.”
18. **Zero-cost check** — “Does `nums.iter().filter(...).map(...).sum()` allocate intermediate `Vec`s? Answer for `--release` and what to measure conceptually.”

#### Compile errors and edge cases

19. **Trap sheet drill** — “Give 6 snippets mixing `for x in v`, `for x in &v`, `into_iter`, and `collect` type errors. I predict compile ok or fail and why; you show the fixed line.”
20. **&&i32 decoder** — “One `.iter().filter().map()` chain: I label the closure parameter type at each step; you correct and show `|x|`, `|&x|`, and `|&&x|` versions that compile.”
21. **Empty iterator policy** — “Three tasks (sum, find, all) on possibly empty `Vec`. I state the result and whether it is a domain bug; you correct (e.g. empty `all` is true).”
22. **zip truncation** — “IDs len 5, values len 100 — I write zip collect; you explain silent loss and sketch `zip` + length check or `enumerate` on the longer vec.”

#### Custom iterators

23. **RangeCounter impl** — "Implement `Iterator` for integers 1..=5; collect to `Vec` and sum with `.sum()` — show `type Item` and `fn next` only."
24. **Skip blanks** — "Config string with empty lines — write `NonEmptyLines` iterator that trims and skips `''`; collect keys before `=`."
25. **Infinite take** — "Counter from 0 without end — why must you `.take(n)` before `.collect()`? Show hang vs bounded collect."
26. **IntoIterator pair** — "Same struct: implement `Iterator` and `IntoIterator` so both `scan.next()` and `for p in scan` work — minimal impl blocks."
27. **Stateful parser** — "Byte buffer iterator yielding complete 4-byte frames; partial frame stays in struct — sketch `next()` state machine."
28. **Capstone iterator** — "CSV line iterator: split fields, parse col 2 as `u16`, filter > 0 — custom struct + one consumer chain."

