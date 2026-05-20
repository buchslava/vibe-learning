# Chapter 3: Ownership and Borrowing

## Hook

Java passes object references; Python passes references to objects (assignment binds names). Rust passes **ownership**, **immutable borrows** (`&T`), or **mutable borrows** (`&mut T`) — and the compiler rejects aliases that would let you observe a mutation while also reading.

## The three rules

1. Each value has exactly one owner.
2. When the owner goes out of scope, the value is dropped.
3. At any time: either **one** `&mut T` **or** any number of `&T`, not both.

## Moves and `Copy`

```rust
// Playground
fn main() {
    let a = 5;
    let b = a; // Copy — i32 is Copy
    println!("{} {}", a, b);

    let s1 = String::from("hi");
    let s2 = s1; // move
    println!("{}", s2);
}
```

Types like integers, `bool`, `char` implement **`Copy`** ( bitwise copy ). `String`, `Vec`, most collections **move**.

## Borrowing

```rust
// Playground
fn len(s: &String) -> usize {
    s.len()
}

fn shout(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut msg = String::from("hello");
    println!("len = {}", len(&msg));
    shout(&mut msg);
    println!("{}", msg);
}
```

## Slices

A **slice** `&[T]` or `&str` borrows part of a collection without taking ownership.

```rust
// Playground
fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

fn main() {
    let v = vec![1, 2, 3];
    println!("{}", sum(&v));
    let s = String::from("rust");
    println!("{}", &s[0..2]); // "ru"
}
```

## When to `clone()`

`.clone()` is explicit and potentially expensive. Prefer borrows when the callee only needs to read or borrow for a short scope.

| Situation | Prefer |
|-----------|--------|
| Read-only access | `&T` |
| In-place update | `&mut T` |
| Independent duplicate | `.clone()` |
| Small `Copy` types | pass by value |

## Common errors (read the message)

- `value moved` — you used a variable after move; use `&` or `.clone()`.
- `cannot borrow as mutable` — another borrow is live; shrink scope with `{ ... }`.
- `borrowed value does not live long enough` — [Chapter 4](04_lifetimes.md).

## Idiom spotlight

> **Shrink borrow scopes.** Put `&mut` uses inside the smallest block possible so the compiler can prove borrows end before the next use.

## Playground: pass by value vs reference

```rust
// Playground
fn by_val(x: u32) { println!("val {}", x); }
fn by_ref(x: &u32) { println!("ref {}", x); }

fn main() {
    let n = 10;
    by_val(n);
    by_ref(&n);
    println!("still have n: {}", n);
}
```

## Go deeper

- Archive: [CHAPTER_01 §2–3](../archive/CHAPTER_01_RUST_BASICS.md)

## See also

- [Chapter 4: Lifetimes](04_lifetimes.md)
- [Chapter 6: Traits](06_structs_traits_generics.md)

### Afterparty: AI Lego blocks

1. **Borrow checker tutor** — “I paste compiler errors; you explain the borrow conflict and show the smallest fix.”
2. **Five snippets** — “Move vs borrow quiz: 5 code fragments, I label ok/error and why.”
3. **Python port** — “This Python function mutates a list passed in; rewrite with `&mut Vec` and explain aliasing rules.”
4. **Java port** — “This Java method stores the passed List in a field; show the Rust ownership split (return owned vs `Arc`).”
5. **Slice drill** — “Given `&[i32]`, write `first` and `rest` without panicking on empty — use `Option`.”
6. **clone audit** — “Review my 30-line Rust snippet; mark unnecessary `.clone()` calls.”
