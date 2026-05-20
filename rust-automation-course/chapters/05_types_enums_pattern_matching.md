# Chapter 5: Enums and Pattern Matching

## Hook

Java has `enum` for named constants and class hierarchies for variants. Python uses unions informally (`None`, multiple shapes). Rust **`enum`** is a **sum type**: each variant can carry different data — and `match` forces you to handle every case.

## `Option<T>` — no null

```rust
// Playground
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn main() {
    match divide(10.0, 2.0) {
        Some(v) => println!("{}", v),
        None => println!("undefined"),
    }
}
```

`if let Some(x) = expr { ... }` handles one variant briefly.

## `Result<T, E>` — errors as values

```rust
// Playground
fn parse_positive(s: &str) -> Result<i32, &'static str> {
    let n: i32 = s.parse().map_err(|_| "not a number")?;
    if n > 0 { Ok(n) } else { Err("not positive") }
}

fn main() {
    println!("{:?}", parse_positive("42"));
    println!("{:?}", parse_positive("-1"));
}
```

Full `?` and custom errors: [Chapter 7](07_errors_and_testing.md).

## Custom enums

```rust
// Playground
enum Status {
    Idle,
    Running { rpm: u32 },
    Fault { code: u8 },
}

fn describe(s: &Status) -> String {
    match s {
        Status::Idle => "idle".into(),
        Status::Running { rpm } => format!("running at {}", rpm),
        Status::Fault { code } => format!("fault {}", code),
    }
}

fn main() {
    let s = Status::Running { rpm: 1500 };
    println!("{}", describe(&s));
}
```

## Exhaustive `match`

The compiler errors if you skip a variant. Use `_` only when you truly mean “everything else.”

## `match` vs `if` chains

Prefer `match` on enums and `Result`/`Option`. Reserve long `if let` chains for one-off cases.

## Idiom spotlight

> **`match` on `Result` at boundaries, `?` inside.** At `main` or API edge, convert to user-facing messages; inside libraries, propagate with `?`.

## Go deeper

- [Option basics](https://hightechmind.io/rust/) — examples 041–044
- [Result basics](https://hightechmind.io/rust/) — examples 045–048

## See also

- [Chapter 7: Errors](07_errors_and_testing.md)
- [Chapter 6: Traits](06_structs_traits_generics.md)

### Afterparty: AI Lego blocks

1. **Null replacement** — “Translate 5 Java methods returning null into `Option` Rust; explain callsite changes.”
2. **Exhaustive match** — “I have enum with 4 variants; generate match that compiles; then add variant and show compiler error.”
3. **Result railway** — “Chain parse → validate → compute with `?`; I fill blanks, you verify.”
4. **if let vs match** — “When is `if let` clearer than `match`? Give 3 contrasting snippets.”
5. **State machine** — “Model TCP connection states as enum; methods `connect`, `send`, `close` with illegal transition errors.”
6. **Python Union** — “This Python function accepts int | str; design Rust enum + match without dynamic typing.”
