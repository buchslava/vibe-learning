# Chapter 6: Structs, Traits, and Generics

## Hook

Java: classes bundle data + inheritance. Python: duck typing (“if it quacks…”). Rust: **structs** for data, **`impl`** for methods, **traits** for shared behaviour — **composition over inheritance**, checked at compile time.

## Structs and methods

```rust
// Playground
struct Sensor {
    id: u32,
    value: f64,
}

impl Sensor {
    fn new(id: u32, value: f64) -> Self {
        Self { id, value }
    }

    fn scaled(&self, factor: f64) -> f64 {
        self.value * factor
    }
}

fn main() {
    let s = Sensor::new(1, 25.0);
    println!("{}", s.scaled(0.5));
}
```

## Traits — interfaces done right

```rust
// Playground
trait Summary {
    fn summarize(&self) -> String;
}

struct Reading { v: f64 }

impl Summary for Reading {
    fn summarize(&self) -> String {
        format!("reading: {}", self.v)
    }
}

fn print_summary(item: &impl Summary) {
    println!("{}", item.summarize());
}

fn main() {
    let r = Reading { v: 3.14 };
    print_summary(&r);
}
```

| Java | Python | Rust |
|------|--------|------|
| `interface` | informal protocol | `trait` |
| `implements` | “has method” | `impl Trait for Type` |
| default interface methods | mixin / ABC | trait default bodies |

## Generics

```rust
// Playground
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in &list[1..] {
        if item > max { max = item; }
    }
    max
}

fn main() {
    let nums = vec![3, 1, 4, 1, 5];
    println!("{}", largest(&nums));
}
```

## Trait bounds and `where`

```rust
// Playground
use std::fmt::Display;

fn show<T: Display>(x: T) {
    println!("{}", x);
}

fn main() {
    show(42);
    show("text");
}
```

## Trait objects (`dyn Trait`)

When you need runtime polymorphism (heterogeneous collection of types):

```rust
// Playground
trait Greeter { fn greet(&self) -> String; }
struct En; impl Greeter for En { fn greet(&self) -> String { "Hello".into() } }
struct Fr; impl Greeter for Fr { fn greet(&self) -> String { "Bonjour".into() } }

fn main() {
    let voices: Vec<Box<dyn Greeter>> = vec![Box::new(En), Box::new(Fr)];
    for v in &voices { println!("{}", v.greet()); }
}
```

Heap allocation (`Box`) required for wide pointers. Prefer **generics** (`impl Trait`) when types are known at compile time — monomorphization, zero cost.

## Idiom spotlight

> **Prefer `impl Trait` parameters over `dyn Trait` unless you need a collection of mixed types.** Static dispatch is the default Rust sweet spot.

## Go deeper

- [Records / structs](https://hightechmind.io/rust/) — example 062
- Archive: [CHAPTER_01 §4](../archive/CHAPTER_01_RUST_BASICS.md)

## See also

- [Chapter 8: Iterators](08_collections_iterators.md)
- [Chapter 12: Async traits](12_async_tokio.md) (advanced)

### Afterparty: AI Lego blocks

1. **Interface port** — “Convert Java interface `Measurable` + two classes to trait + two structs + `impl`.”
2. **Duck typing** — “Python function accepts anything with `.read()`; express as trait bound in Rust generic.”
3. **dyn vs impl** — “Quiz: 4 scenarios — pick `dyn Trait` or `impl Trait` and justify.”
4. **Default trait methods** — “Add default `summary()` on trait; override in one type only.”
5. **Generic bounds** — “Fix compiler error: `T` needs `Display + Clone`; minimal bound set.”
6. **OOP myth** — “Explain in 100 words why Rust has no inheritance and what you do instead.”
