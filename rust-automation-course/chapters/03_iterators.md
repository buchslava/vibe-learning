# Chapter 3: Iterators

## Hook

**Iterators are a shared pattern, not a Rust quirk.** Across languages and stacks you see the same idea: walk a sequence once, transform or filter each step, then produce a result — without hand-rolling index math every time.

| Stack | How the pattern shows up |
|-------|--------------------------|
| **Java** | `Stream<T>` — `map`, `filter`, `collect` |
| **Python** | `for x in items`, list comprehensions, generators |
| **C# / LINQ** | `IEnumerable`, `.Select()`, `.Where()` |
| **JavaScript** | `Array.prototype.map` / `filter` / `reduce` |
| **SQL** | `SELECT` over rows (set-oriented, not index loops) |
| **Unix** | pipes — one program’s output is the next program’s input stream |
| **Rust** | `Iterator` trait — lazy adapters + consumers |

The names differ; the habit transfers: **describe the pipeline, not the loop variable.** That is why this chapter sits early in Part I — you will meet the same style in logs, configs, protocol parsing, and [Chapter 8](08_collections_iterators.md) collections work.

**Rust’s version:** the **`Iterator`** trait in `std` — a **lazy** sequence you extend with `.map()`, `.filter()`, and finish with `.collect()`, `.sum()`, and friends.

Nothing in the chain runs until you ask for a result — unlike eager list comprehensions that build intermediate lists in Python. In `--release` builds, these pipelines often compile to the same machine code as a hand-written loop ([Chapter 1](01_paradigm_shift.md#zero-cost-abstractions)).

You will use `Vec` and `HashMap` heavily in [Chapter 8](08_collections_iterators.md). This chapter is the **iteration machinery** that works on those types (and on ranges, strings, slices).

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

**Java / Python habit:** looping a list does not destroy the list. In Rust, `for x in v` is **`into_iter`** — equivalent to consuming `v`. That surprises newcomers and is a common compile-error source.

```rust
// Playground — uncomment ONE wrong line at a time to see the error
fn main() {
    let v = vec![String::from("a"), String::from("b")];

    for s in &v {
        println!("{}", s); // borrows each &String
    }
    println!("{:?}", v); // ok — v still owned here

    let v2 = vec![1, 2, 3];
    for n in v2 {
        println!("{}", n); // moves each i32 out of v2
    }
    // println!("{:?}", v2); // ERROR: value used after move
}
```

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

`.filter` and `.map` take **closures** (`|x| ...`). How closures capture variables (`Fn`, `FnMut`, `FnOnce`) is covered in [Chapter 8 — Closures](08_collections_iterators.md#closures).

### Why closures see `&&i32` (double reference)

On `v.iter()`, each `Item` is `&i32`. Adapter methods like `.filter()` pass **`&Item`** to your closure — so the parameter is `&&i32`.

| Closure param | Meaning |
|---------------|---------|
| `|x|` | `x` is `&&i32`; `*x` coerces to `i32` in expressions |
| `|&x|` | destructures one layer → `x: &i32` |
| `|&&x|` | destructures to `x: i32` |

Pick one style and stay consistent in a chain. Mixing `.iter()` with a closure that expects owned `i32` without peeling references is a frequent compile error.

The `Iterator` trait lives in `std` and defines `.next() -> Option<Item>`. You rarely implement it yourself until [Chapter 6](06_structs_traits_generics.md); calling `.iter()` on standard types is enough for most automation code.

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

```rust
// Playground — does not compile
fn main() {
    let lines: Vec<String> = vec![String::from("PORT=502")];
    let tags: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    drop(lines);
    // println!("{:?}", tags); // would dangle — often fails earlier:
    // tags borrows `lines`, so drop(lines) is ERROR while tags is alive
}
```

Owned collection after parsing is the safe default: `.map(|s| s.clone())` or collect into `Vec<String>` before dropping the source buffer. Lifetimes formalize this in [Chapter 4](04_lifetimes.md).

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
    let ok = ports.iter().all(|&&p| p >= 1 && p <= 65535);
    println!("all in range? {}", ok); // true on empty — know your domain rule

    let logs = vec!["ok", "ERROR: timeout"];
    if let Some(line) = logs.iter().find(|&&s| s.contains("ERROR")) {
        println!("first error line: {}", line);
    }
}
```

## When the compiler says no (iterator checklist)

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

> **Iterator chains over index loops.** `for i in 0..v.len()` plus `v[i]` is a smell when `.iter()`, `.enumerate()`, or `.windows(2)` states intent. Sliding pairs over samples live in [Chapter 8](08_collections_iterators.md).

## Go deeper

- [The Rust Book — Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Functional Rust — iterator topics](https://hightechmind.io/rust/)

## See also

- [Chapter 1: Ownership and borrowing](01_paradigm_shift.md#references-borrowing-and-dereferencing) — `iter` vs `into_iter`
- [Chapter 2: Types and expressions](02_types.md) — ranges, `for`, string iterators
- [Chapter 4: Lifetimes](04_lifetimes.md) — borrows inside iterator chains
- [Chapter 6: Structs, traits, and generics](06_structs_traits_generics.md) — `Iterator` trait and custom types
- [Chapter 8: Collections](08_collections_iterators.md) — `Vec`, `HashMap`, closures, `.windows`

### Afterparty: AI Lego blocks

Copy a prompt into your AI tutor after running the Playground examples. Insist on compiler-accurate answers.

#### Cross-language and mental model

1. **Loop port** — “Rewrite this C-style indexed loop as an iterator chain; preserve behavior and types.”
2. **Stack pattern map** — “I name a task in Java (`Stream`), Python (comprehension), and SQL. You show the idiomatic Rust iterator chain for each — one line per language plus Rust.”
3. **Python comp port** — “Translate `[x * 2 for x in nums if x > 0]` into a Rust `Vec` pipeline with `.iter().filter().map().collect()`; explain lazy vs eager.”
4. **Unix pipe analogy** — “In ≤120 words, map `cmd1 | cmd2` to Rust iterator adapters on in-memory data; where does `.collect()` fit?”

#### `for`, ranges, and three walk modes

5. **for desugar quiz** — “Four `for` loops (`0..n`, `&vec`, `vec`, `&mut vec`). I say which calls `iter`, `into_iter`, or range; you correct and show owner state after the loop.”
6. **iter vs into_iter** — “Give 4 snippets using `Vec`; I predict whether `v` is usable after the loop; you explain move vs borrow.”
7. **iter_mut drill** — “Task: double every element in `Vec<f64>` in place without allocating a new `Vec`. I write the loop; you review `*n` and borrow rules.”
8. **Range vs collect** — “When is `for i in 0..n` better than `(0..n).collect::<Vec<_>>()`? Give two automation examples (retry count vs materializing indices).”

#### Adapters and lazy pipelines

9. **Adapter chain** — “Task: parse lines, trim, keep non-empty, parse as `u16`. I sketch `.lines().map(...).filter(...).collect()`; you refine.”
10. **zip pairs** — “Two `Vec<u16>`: register IDs and values. Build `Vec<(u16, u16)>` with `.zip()`; I write it; you handle length mismatch policy.”
11. **take and skip** — “Paginate a log: skip first 100 lines, take next 20. I use `.skip().take()` on `.lines()`; you show one pitfall if the source is not recomputed.”
12. **chain iterators** — “Concatenate header rows and body rows (two `&[u8]` slices) without copying bytes into one array first — sketch with `.iter().chain()`.”
13. **enumerate vs index** — “Same sum-over-evens task twice: index `for` vs `.enumerate()`. Compare readability and bounds-check risk.”
14. **Lazy vs eager** — “Explain when `filter().map()` allocates vs when `.collect()` forces work. One example with `println!` in `map` showing evaluation order.”

#### Consumers and types

15. **collect turbofish** — “Three `collect()` calls that fail without hints — I add type annotation or turbofish; you verify.”
16. **find and Option** — “Wire scan: `Vec<&str>` of lines, find first containing `ERROR`. I return `Option<&str>` with `.find()`; you contrast with `.filter().next()`.”
17. **fold vs sum** — “Compute max and count in one pass with `.fold()` vs calling `.max()` and `.len()` separately — when is fold worth it?”
18. **any and all** — “Validate a batch: all ports in 1..=65535, any line starts with `#`. I write `.all()` / `.any()` predicates; you fix one double-reference mistake.”

#### Errors, ownership, and automation

19. **Moved Vec mistake** — “Show code that does `for x in v` then uses `v` again. I explain the error and fix with `.iter()` or clone; you rank fixes.”
20. **Borrow in chain** — “Snippet: build `Vec<&str>` from `String` lines then drop the `String`. I explain why it fails; you fix with owned `String` or different lifetime design.”
21. **Modbus-style scan** — “List of raw register values `Vec<u16>`: filter evens, map to `f64` scale 0.1, sum. I write the iterator chain; you check overflow and types.”
22. **Zero-cost check** — “Does `nums.iter().filter(...).map(...).sum()` allocate intermediate `Vec`s? Answer for `--release` and what to measure conceptually.”

#### Compile errors and edge cases

23. **Trap sheet drill** — “Give 6 snippets mixing `for x in v`, `for x in &v`, `into_iter`, and `collect` type errors. I predict compile ok or fail and why; you show the fixed line.”
24. **&&i32 decoder** — “One `.iter().filter().map()` chain: I label the closure parameter type at each step; you correct and show `|x|`, `|&x|`, and `|&&x|` versions that compile.”
25. **Empty iterator policy** — “Three tasks (sum, find, all) on possibly empty `Vec`. I state the result and whether it is a domain bug; you correct (e.g. empty `all` is true).”
26. **zip truncation** — “IDs len 5, values len 100 — I write zip collect; you explain silent loss and sketch `zip` + length check or `enumerate` on the longer vec.”
