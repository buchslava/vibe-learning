# Chapter 4: Lifetimes

## Hook

Java’s garbage collector keeps objects alive while references exist. Rust has no GC: references must not outlive the data they point to. **Lifetimes** are the compiler’s way of proving that — usually without you writing any syntax.

## References always have a lifetime

Every `&T` is valid for some span of code. If you return a reference to a local variable, that reference dies when the stack frame ends — the compiler rejects it.

```rust
// Playground — this pattern is OK: reference does not outlive owner
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}

fn main() {
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("{}", word);
}
```

## Elision (why you rarely write lifetimes)

The compiler assigns elided lifetimes on common patterns:

```rust
fn len(s: &str) -> usize  // same as fn len<'a>(s: &'a str) -> usize
```

Three rules cover most function signatures; when they fail, you add explicit `'a`.

## Named lifetimes on structs

Store references in structs only when you tie them to **some other data’s** lifetime:

```rust
// Playground
struct Excerpt<'a> {
    text: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Once upon a time...");
    let first = novel.split('.').next().unwrap_or("");
    let e = Excerpt { text: first };
    println!("{}", e.text);
}
```

`Excerpt` cannot outlive `novel` — the struct carries `'a`.

## When the compiler says no

Typical fixes:

- Return an **owned** `String` instead of `&str`.
- Take owned data as parameter and return owned data.
- Use `Rc`/`Arc` when shared ownership is real ([Chapter 9](09_smart_pointers_modules.md)).

## Java / Python contrast

| | Java | Python | Rust |
|---|------|--------|------|
| dangling pointer | rare (GC + JIT) | possible C extensions | compile error |
| return inner ref | object on heap OK | OK if object lives | must tie lifetimes |
| self-referential struct | awkward | awkward | needs pins/advanced |

## Idiom spotlight

> **Return owned types from public APIs** unless you are building a zero-copy internal API and can document lifetime ties. `String` and `Vec` are easy; `&str` in returns often fights the borrow checker for newcomers.

## Playground: explicit lifetime on two inputs

```rust
// Playground
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("longer");
    let s2 = String::from("x");
    println!("{}", longest(&s1, &s2));
}
```

## Go deeper

- [Functional Rust — pattern matching & types](https://hightechmind.io/rust/by-topic.html)

## See also

- [Chapter 3: Ownership](03_ownership_borrowing.md)
- [Chapter 6: Traits](06_structs_traits_generics.md)

### Afterparty: AI Lego blocks

1. **Error archaeology** — “I paste a ‘lifetime may not live long enough’ error; walk me through owner vs reference diagram.”
2. **Return type choice** — “For API `fn title(book: &Book) -> ???` compare `&str` vs `String` trade-offs for a library.”
3. **Struct lifetime** — “Design `ConfigParser` holding `&str` slices into input buffer — when is it sound vs use owned `String`?”
4. **Elision quiz** — “Add explicit lifetimes to 4 function signatures where elision fails.”
5. **Java analogy** — “Compare Rust lifetimes to Java stack locals vs heap references — 120 words, accurate only.”
6. **Fix mine** — “I return `&String` built inside function; show three idiomatic fixes ranked by simplicity.”
