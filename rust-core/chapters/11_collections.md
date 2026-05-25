# Chapter 11: Collections

## Hook

Lists and maps are workhorses in most languages (**Python** lists/dicts, **Java** `ArrayList`/`HashMap`, …). Rust’s standard collections are **generic**, **ownership-aware**, and pair with iterator pipelines from [Chapter 4](04_iterators.md). Closures in `.sort_by` and `.retain` appear in [Chapter 12](12_closures.md).

## Scope — a brief tour

Daily `std` collections — not third-party crates or custom allocators.

| This chapter covers | Deferred |
|---------------------|----------|
| `Vec`, `HashMap`, `HashSet`, `VecDeque`, `BTreeMap`/`BTreeSet` | Persistent/immutable collections |
| `entry`, iterators, `.windows` | Full algorithm analysis |
| Ownership with `collect` / `insert` | Database/query engines |

## Choosing a collection

| Type | Keys / order | Typical use |
|------|----------------|-------------|
| `Vec<T>` | indexed, contiguous | default sequence, stack-like growth |
| `VecDeque<T>` | indexed, double-ended | queue, BFS, push/pop both ends |
| `HashMap<K, V>` | unordered keys | fast lookup, counts, caches |
| `HashSet<T>` | unique keys, unordered | membership, dedup |
| `BTreeMap<K, V>` | sorted keys | ordered map, range queries |
| `BTreeSet<T>` | sorted unique | ordered set |

All live in `std::collections` except `Vec` (always available).

**Java / Python contrast**

| Task | Java | Python | Rust |
|------|------|--------|------|
| Growable list | `ArrayList` | `list` | `Vec` |
| Map | `HashMap` | `dict` | `HashMap` |
| Set | `HashSet` | `set` | `HashSet` |
| Queue | `ArrayDeque` | `deque` | `VecDeque` |
| Sorted map | `TreeMap` | `sortedcontainers` (stdlib: none) | `BTreeMap` |

## `Vec<T>`

```rust
// Playground
fn main() {
    let mut v = vec![10, 20, 30];
    v.push(40);
    println!("{:?}", v);
    println!("second = {}", v[1]);
}
```

Indexing with `[i]` **panics** out of bounds. Use `.get(i)` for `Option`.

Walk with `.iter()` / `.iter_mut()` / `.into_iter()` — [Chapter 4](04_iterators.md#three-ways-to-walk-a-vec).

**Capacity:** `Vec` over-allocates for amortized push. Use `Vec::with_capacity(n)` when you know size upfront.

### Safe indexing and mutation

| API | On missing index | On success |
|-----|------------------|------------|
| `v[i]` | **panic** | `T` or `&T` |
| `v.get(i)` | `None` | `Some(&T)` |
| `v.get_mut(i)` | `None` | `Some(&mut T)` |

```rust
// Playground
fn main() {
    let v = vec![10, 20];
    println!("{:?}", v.get(5)); // None — no panic
    // println!("{}", v[5]);    // panic if uncommented
}
```

### Vec methods worth knowing

```rust
// Playground
fn main() {
    let mut v = vec![3, 1, 4, 1, 5];
    v.sort();                          // in-place sort
    v.dedup();                         // remove consecutive duplicates (sort first!)
    v.retain(|&x| x % 2 == 0);         // keep evens only
    v.extend([9, 8]);                  // append from iterator
    println!("{:?}", v);
}
```

**What happened:** `dedup` only removes **adjacent** duplicates — sort first if you need global dedup. `retain` takes a closure ([Chapter 12](12_closures.md)).

### Vec edge cases

**Wrong — use `Vec` as a queue with `remove(0)`:**

```rust
// Playground — works but O(n) per pop front
fn main() {
    let mut v = vec![1, 2, 3];
    while !v.is_empty() {
        let front = v.remove(0); // shifts entire buffer left each time
        println!("{}", front);
    }
}
```

**Fix:** use `VecDeque` for FIFO (see below).

**Wrong — hold reference into `Vec`, then mutate:**

```rust
// Playground — does not compile
fn main() {
    let mut v = vec![1, 2, 3];
    let r = &v[0];
    v.push(4); // ERROR: cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("{}", r);
}
```

**Why:** `push` may reallocate. The old reference would dangle. Finish using `r` before mutating.

| Symptom | Cause | Fix |
|---------|-------|-----|
| panic on `[i]` | out of bounds | `.get(i)` or check `len()` |
| slow queue on `Vec` | `remove(0)` is O(n) | `VecDeque` |
| borrow error on `push` | ref into element still alive | shrink ref scope |
| `dedup` leaves dupes | not sorted | `sort` then `dedup` |

## `HashMap<K, V>`

```rust
// Playground
use std::collections::HashMap;

fn main() {
    let mut counts = HashMap::new();
    for word in ["a", "b", "a"] {
        *counts.entry(word).or_insert(0) += 1;
    }
    println!("{:?}", counts);
}
```

Keys need **`Eq + Hash`**. Owned `String` keys are common. Lookup accepts `&str` via **`Borrow`** — `map.get("alice")` works when keys are `String`.

### The `entry` API

`.entry(key)` returns an **`Entry`**. Use it to insert, update, or skip in one lookup:

```rust
// Playground
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("alice", 10);

    let e = scores.entry("bob").or_insert(0);
    *e += 5;

    scores.entry("alice").and_modify(|v| *v += 1).or_insert(0);

    println!("{:?}", scores);
}
```

| Method | Effect |
|--------|--------|
| `or_insert(v)` | insert if missing, return `&mut V` |
| `and_modify(f)` | run `f` on existing value only |
| `or_default()` | insert `Default::default()` if missing |
| `or_insert_with(f)` | lazy insert — call `f` only if missing |

**Lazy insert** avoids work when the key already exists:

```rust
// Playground
use std::collections::HashMap;

fn main() {
    let mut cache = HashMap::new();
    cache.entry("key").or_insert_with(|| {
        println!("building expensive value");
        vec![1, 2, 3]
    });
    cache.entry("key").or_insert_with(|| {
        println!("should not run");
        vec![99]
    });
}
```

**What happened:** the closure runs **once** — on first insert only.

### HashMap edge cases

**`insert` overwrites** — returns the old value if any:

```rust
// Playground
use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    m.insert("port", 502);
    let old = m.insert("port", 503);
    println!("old = {:?}, now = {:?}", old, m.get("port"));
}
```

**Wrong — double lookup (get then insert):**

```rust
// Playground — compiles but wasteful / awkward for mutation
use std::collections::HashMap;

fn bump(m: &mut HashMap<String, i32>, key: &str) {
    if m.contains_key(key) {
        *m.get_mut(key).unwrap() += 1;
    } else {
        m.insert(key.to_string(), 1);
    }
}
```

**Idiomatic:** `*m.entry(key.to_string()).or_insert(0) += 1;`

**Borrow while iterating:** you cannot mutably insert while iterating the same map. Collect keys first, or use `entry` outside the loop.

| Error / symptom | Cause | Fix |
|-----------------|-------|-----|
| key type needs `Hash` | custom struct key | `#[derive(Hash, Eq, PartialEq)]` |
| cannot borrow map mutably | iterator or `get` ref still alive | drop borrows first |
| silent overwrite | `insert` on existing key | check return or use `entry` |
| `f32` as key | floats not `Hash` in std | integer keys or newtype |

## `HashSet<T>`

Unique values with set algebra:

```rust
// Playground
use std::collections::HashSet;

fn main() {
    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [3, 4, 5].into_iter().collect();
    let union: HashSet<_> = a.union(&b).copied().collect();
    let inter: HashSet<_> = a.intersection(&b).copied().collect();
    println!("union {:?} inter {:?}", union, inter);
}
```

| Operation | Method | Notes |
|-----------|--------|-------|
| membership | `set.contains(&x)` | O(1) average |
| dedup | `collect::<HashSet<_>>()` | order lost |
| difference | `a.difference(&b)` | in `a` not in `b` |

**Dedup preserving order** — `HashSet` loses order. For stable unique order, use a `Vec` + `HashSet` seen-set, or sort + `dedup`.

## `VecDeque<T>`

Efficient push/pop at **front and back**:

```rust
// Playground
use std::collections::VecDeque;

fn main() {
    let mut q = VecDeque::new();
    q.push_back(1);
    q.push_back(2);
    q.push_front(0);
    println!("pop front = {:?}", q.pop_front());
    println!("rest = {:?}", q);
}
```

| Operation | `Vec` | `VecDeque` |
|-----------|-------|------------|
| `push_back` | O(1) amortized | O(1) |
| `pop_front` | O(n) with `remove(0)` | O(1) |
| random index | O(1) | O(1) |

Use as a **FIFO queue** or **BFS frontier**. Use `Vec` when you only grow at the end.

## `BTreeMap` and `BTreeSet`

**Sorted** keys — useful for ordered iteration and range scans:

```rust
// Playground
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert(30, "c");
    map.insert(10, "a");
    map.insert(20, "b");
    for (k, v) in &map {
        println!("{} -> {}", k, v);
    }
}
```

Keys need **`Ord`**. Iteration is always **sorted by key**.

### Range queries

```rust
// Playground
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    for k in [100, 150, 200, 250] {
        map.insert(k, format!("reg_{k}"));
    }
    for (k, v) in map.range(120..=220) {
        println!("{} -> {}", k, v);
    }
}
```

**What happened:** prints keys **150** and **200** — inclusive range `120..=220`.

| Need | Pick |
|------|------|
| fastest single lookup | `HashMap` |
| sorted walk or range | `BTreeMap` |
| min/max key always handy | `BTreeMap` (`first_key_value`, `last_key_value`) |

`BTreeMap` is O(log n) per op; `HashMap` is O(1) average. Extra cost buys **order**.

## Iterators and collection methods

Collection methods delegate to iterators — see [Chapter 4](04_iterators.md):

```rust
// Playground
fn main() {
    let nums = vec![1, 2, 3, 4];
    let sum: i32 = nums.iter().sum();
    let evens: Vec<_> = nums.iter().filter(|&&x| x % 2 == 0).collect();
    println!("{} {:?}", sum, evens);
}
```

### `.windows`, `.chunks`, `.enumerate`

```rust
// Playground
fn main() {
    let samples = vec![1.0, 1.1, 2.5, 2.4];
    let rising: Vec<_> = samples
        .windows(2)
        .filter(|w| w[1] > w[0])
        .collect();
    println!("rising pairs {:?}", rising);

    let bytes = vec![0xDE, 0xAD, 0xBE, 0xEF];
    for chunk in bytes.chunks(2) {
        println!("{:02X?}", chunk);
    }
}
```

| Adapter | Yields |
|---------|--------|
| `.windows(n)` | overlapping slices of length `n` |
| `.chunks(n)` | non-overlapping chunks |
| `.enumerate()` | `(index, item)` |

### `sort_by` and custom order

```rust
// Playground
fn main() {
    let mut items = vec![("b", 3), ("a", 1), ("c", 2)];
    items.sort_by(|a, b| a.1.cmp(&b.1)); // sort by numeric field
    println!("{:?}", items);
}
```

Requires **`Ord`** on compared fields or a custom comparator closure ([Chapter 12](12_closures.md)).

## Collecting into collections

`collect()` builds many types when the element type is known:

```rust
// Playground
use std::collections::{HashMap, HashSet};

fn main() {
    let pairs = vec![("a", 1), ("b", 2)];
    let map: HashMap<_, _> = pairs.into_iter().collect();
    let uniq: HashSet<_> = vec![1, 1, 2].into_iter().collect();
    println!("{:?} {:?}", map, uniq);
}
```

**Duplicate keys in `collect` to HashMap:** later pairs **overwrite** earlier ones — know your source data.

**Wrong — ambiguous `collect`:**

```rust
// Playground — does not compile
fn main() {
    let nums = vec![1, 2, 3];
    let v = nums.iter().map(|&x| x * 2).collect();
    // ERROR: type annotations needed
}
```

**Fix:** `let v: Vec<i32> = ...` or `.collect::<Vec<_>>()`.

### `into_iter` vs `iter` when building collections

| Source walk | You get in pipeline | Source after |
|-------------|---------------------|--------------|
| `.iter()` | `&T` | `Vec` still owned |
| `.into_iter()` / owned `vec` | `T` | `Vec` consumed |

Building `HashMap<String, i32>` from owned strings requires **`into_iter`** or cloning keys.

## When the compiler says no

Common errors in this chapter:

| Error (typical) | Cause | Fix |
|-----------------|-------|-----|
| `T` cannot be hashed | key missing `Hash` | derive or newtype |
| `T: Ord` not satisfied | `BTreeMap` key not ordered | derive `Ord` or use `HashMap` |
| cannot borrow as mutable | ref into `Vec`/`HashMap` alive | end borrow scope |
| type annotations needed | ambiguous `collect()` | turbofish or annotate |
| iterator invalidation | mutate map during `for (k,v)` | collect keys first |
| `dedup` ineffective | unsorted `Vec` | `sort_unstable` then `dedup` |

## Idiom spotlight

> **Iterator chains over index loops.** `for i in 0..v.len()` plus `v[i]` is a smell when `.iter().enumerate()` or `.windows(2)` states intent.
>
> **`.entry` for HashMap updates** — one lookup, not `contains_key` + `get_mut` + `insert`.
>
> **Pick the collection for the access pattern** — queue → `VecDeque`; sorted range → `BTreeMap`; default list → `Vec`.

## Go deeper

- [Rust Book — Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [HashMap entry API](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry)
- [Functional Rust — iterator topics](https://hightechmind.io/rust/)

## See also

- [Chapter 4: Iterators](04_iterators.md)
- [Chapter 12: Closures](12_closures.md) — `.sort_by`, `.retain`
- [Chapter 13: Standard traits](13_standard_traits.md) — `Borrow`, `Eq`, `Hash`
- [Chapter 1: Borrowing](01_paradigm_shift.md#references-borrowing-and-dereferencing)
- [Chapter 7: Generics](07_structs_traits_generics.md)

### Afterparty

#### Choosing and comparing collections

1. **Pick collection** — “Five tasks (dedup, sorted range scan, FIFO queue, index by id, min-key lookup) — I pick Vec/HashMap/BTree/VecDeque/HashSet each.”
2. **Hash vs BTree** — “Same 10k insert + range scan workload — when HashMap wins vs BTreeMap; one sentence each.”
3. **Java map** — “Translate `LinkedHashMap` access-order need — what Rust std type fits, what does not?”
4. **Queue anti-pattern** — “Review `while !v.is_empty() { v.remove(0) }` — cost and fix with `VecDeque`.”

#### Vec drills

5. **Loop port** — “Rewrite C-style indexed loop as iterator chain; preserve behavior.”
6. **get vs index** — “Four access patterns — I pick `[i]` vs `.get(i)` vs `.get_mut` vs `if let Some`.”
7. **sort dedup** — “Dedup `[3,1,4,1,5]` wrong vs right — show sort + dedup pipeline.”
8. **retain vs filter** — “Remove evens in-place vs new `Vec` — compare `retain` and `filter().collect()`.”
9. **Borrow push trap** — “Explain `let r = &v[0]; v.push(1)` error; fix with scope.”

#### HashMap and HashSet

10. **entry drill** — “Word frequency from `Vec<&str>` using only `.entry` — no double lookup.”
11. **or_insert_with** — “Lazy cache: expensive `Vec` built once per key — sketch with `or_insert_with`.”
12. **HashMap merge** — “Two maps of scores — merge by max per key; iterator + entry style.”
13. **insert overwrite** — “Track old value on port remap `502 -> 503` using `insert` return.”
14. **Set ops** — “Tags on two records — union, intersection, difference with `HashSet`.”
15. **Stable dedup** — “Unique `String` lines preserving first-seen order — no `HashSet`-only collect.”

#### BTree, windows, collect

16. **BTree range** — “List keys in `BTreeMap<u32, _>` between 100 and 200 inclusive.”
17. **Windows** — “Detect rising edges in `Vec<f64>` with `.windows(2)`; extend to `.windows(3)` for slope.”
18. **chunks vs windows** — “Parse byte stream into 4-byte frames — `chunks(4)` vs `windows(4)` when?”
19. **collect types** — “Three `collect()` calls that need type hints — fix with turbofish.”
20. **Duplicate keys** — “`collect` to HashMap from duplicate-key pairs — predict final map; explain overwrite rule.”

#### Performance and capstone

21. **Performance myth** — “Do Rust iterators optimize to loops? When might they not?”
22. **Capacity hint** — “Read 1M lines into `Vec` — when `with_capacity` matters; rough sizing rule.”
23. **Capstone** — “Design in-memory store: register id `u16` → last reading `f64`, need range scan by id — pick map type, list three API methods, no impl.”
