# Chapter 13: Metaprogramming

## Hook

Java annotations and reflection generate boilerplate at runtime (or via annotation processors). Python metaclasses twist class creation. Rust metaprogramming is mostly **compile-time**: macros expand source before type checking — zero runtime cost for `macro_rules!` and derive.

## `macro_rules!`

```rust
// Playground
macro_rules! say_hello {
    () => { println!("Hello!"); };
    ($name:expr) => { println!("Hello, {}!", $name); };
}

fn main() {
    say_hello!();
    say_hello!("Automation");
}
```

Macros match **syntax trees** (tokens), not types. Debug with `cargo expand` (nightly/cargo-expand).

## Repetition

```rust
// Playground
macro_rules! vec_str {
    ($($x:expr),*) => {{
        let mut v = Vec::new();
        $( v.push(String::from($x)); )*
        v
    }};
}

fn main() {
    let v = vec_str!["a", "b"];
    println!("{:?}", v);
}
```

## Derive macros

```rust
// Playground
#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }

fn main() {
    println!("{:?}", Point { x: 1, y: 2 });
}
```

`#[derive(...)]` is implemented as procedural macros in the standard library (`serde` adds `Serialize` similarly).

## Proc macros (overview)

Three kinds (usually separate crates):

- **derive** — `#[derive(MyTrait)]`
- **attribute** — `#[route(GET)]`
- **function-like** — `sql!("SELECT ...")`

Authoring proc macros needs `proc-macro = true` and `syn`/`quote` crates — beyond this book’s scope; know they exist.

## `const` and compile-time evaluation

`const fn` and `const` values let more work happen at build time — complementary to macros.

## Idiom spotlight

> **Macros for syntax repetition; generics/traits for logic reuse.** If a macro could be a function, prefer the function.

## Go deeper

- [macro_rules! basics](https://hightechmind.io/rust/) — 411
- [Derive macro concepts](https://hightechmind.io/rust/) — 422

## See also

- [Chapter 6: Traits](06_structs_traits_generics.md)
- [Chapter 14: Unsafe](14_unsafe_and_internals.md)

### Afterparty: AI Lego blocks

1. **Macro vs fn** — “Rewrite macro as generic fn if possible; when impossible, say why.”
2. **derive need** — “List derives I want for config struct loaded from TOML — justify each.”
3. **Hygiene** — “Explain macro hygiene in 60 words with `$crate` mention.”
4. **Debug expand** — “Walk me through `cargo expand` on derive Debug output (conceptual).”
5. **DSL sketch** — “Design tiny `command!` macro for CLI subcommands — tokens only.”
6. **Java annotation** — “Map Lombok `@Data` to Rust derive set — what’s missing?”
