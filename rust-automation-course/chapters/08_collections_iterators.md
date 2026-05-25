# Chapter 8: Collections and Iterators

## Hook

Python lists and dicts are the workhorses; Java has `ArrayList` and `HashMap`. Rust gives **`Vec`** and **`HashMap`** with ownership-aware APIs. Iterator theory — lazy adapters, `iter` vs `into_iter`, consumers — is in [Chapter 3](03_iterators.md); this chapter focuses on **collections** and how to iterate them in practice.

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

Indexing panics out of bounds; use `.get(i)` for `Option`. Walk elements with `.iter()` / `.iter_mut()` / `.into_iter()` — see [Chapter 3](03_iterators.md#three-ways-to-walk-a-vec).

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

Keys need `Eq + Hash`; owned `String` keys are common.

## Iterators in practice

Use [Chapter 3](03_iterators.md) for adapters (`map`, `filter`, `enumerate`) and consumers (`collect`, `sum`). Quick reminder:

```rust
// Playground
fn main() {
    let nums = vec![1, 2, 3, 4];
    let sum: i32 = nums.iter().sum();
    let evens: Vec<_> = nums.iter().filter(|&&x| x % 2 == 0).collect();
    println!("{} {:?}", sum, evens);
}
```

For sliding windows over samples (e.g. rising edges), use `.windows(2)` on a slice or `Vec` — pair with iterator chains from Chapter 3.

## Closures

```rust
// Playground
fn main() {
    let factor = 2;
    let scale = |x| x * factor; // captures factor by borrow
    println!("{}", scale(10));
}
```

Closures implement `Fn`, `FnMut`, or `FnOnce` depending on captures. They power `.map(|x| ...)` and `.filter(|x| ...)` in [Chapter 3](03_iterators.md).

## Idiom spotlight

> **Iterator chains over index loops.** `for i in 0..v.len()` is a smell when `.iter().enumerate()` or `.windows(2)` expresses intent.

## Go deeper

- [Iterator sum/product](https://hightechmind.io/rust/) — 913+
- [List map from scratch](https://hightechmind.io/rust/)

## See also

- [Chapter 3: Iterators](03_iterators.md)
- [Chapter 1: Borrowing](01_paradigm_shift.md#references-borrowing-and-dereferencing)
- [Chapter 6: Generics](06_structs_traits_generics.md)

### Afterparty: AI Lego blocks

1. **Loop port** — “Rewrite this C-style indexed loop as iterator chain; preserve behavior.”
2. **HashMap merge** — “Two maps of scores — merge by taking max per key; iterator style.”
3. **Closure capture** — “Explain `FnOnce` vs `FnMut` for closure storing `String`.”
4. **collect types** — “Why does `collect()` need type hint sometimes? Show turbofish example.”
5. **Windows** — “Detect rising edges in `Vec<f64>` with `.windows(2)` — write snippet.”
6. **Performance myth** — “Do Rust iterators optimize to loops? When might they not?”
