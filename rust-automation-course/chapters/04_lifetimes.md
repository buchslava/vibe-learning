# Chapter 4: Lifetimes

## Hook

In **Java** and **Python**, a garbage collector (or refcounting) keeps heap objects alive while any reference can reach them — you rarely ask whether a pointer is still valid. Rust has no GC: every `&T` must not outlive the value it borrows. **Lifetimes** are the compiler’s way of proving that — usually without you writing any syntax.

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

## What lifetimes are for

**Aim:** guarantee you never use a borrow after its owner is gone.

Chapter 1 said each `&T` must not outlive what it points to. That rule is easy inside one function. It gets hard when you **return** a reference or **store** one in a struct: the caller cannot see your local variables, so the compiler needs a contract on the signature.

**Bad (won’t compile):** return a pointer into memory that dies when the function returns.

```rust
// Playground — uncomment to see the error
fn broken() -> &str {
    let s = String::from("tmp");
    &s // error: `s` dropped here; return would dangle
}
```

**Good:** return a slice that still lives in the caller’s `String`:

```rust
fn first_word(s: &str) -> &str { /* ... */ }  // return borrows from `s`, not from locals
```

A **lifetime** is the span of code where a borrow is valid. You do not set it at runtime — the compiler checks it at compile time. Lifetime syntax on functions and structs answers one question:

> **“Which input (or owner) must still be alive while this reference exists?”**

### How `'a` helps (it’s just a label)

When several references in one signature must live **together**, you give that span a name — `'a`, `'b`, `'input`, whatever reads clearly. Same idea as naming a generic type `T`: the name is for humans and the compiler; **`'a` is not magic.**

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}
```

Read this as: “The returned `&str` is only valid while **both** `x` and `y` are valid — and those two borrows share one lifetime `'a`.” If `s1` is dropped while something still holds the return value from `longest(&s1, &s2)`, the program must not compile.

### What if you omit `<'a>`?

The body does not change — you still return either `x` or `y`. The lifetime is **not** something you execute at runtime; it is a **signature** contract for the compiler. Try writing `longest` without any lifetime parameters:

```rust
fn longest(x: &str, y: &str) -> &str {  // does not compile
    if x.len() >= y.len() { x } else { y }
}
```

Rust rejects this with **missing lifetime specifier** (or similar). Reason: the return might point into `x` **or** into `y`, and those borrows can come from **different owners** with **different lifetimes**. The compiler will not guess.

| With lifetimes | Without lifetimes |
|----------------|-------------------|
| `-> &'a str` tied to `x` and `y` | Compiler does not know which input the return borrows from |
| Caller must keep **both** `s1` and `s2` alive while using the result | No check that you drop the wrong `String` too early |
| Elision cannot help: **two** reference inputs + **one** reference output | Signature is incomplete |

Compare `first_word(s: &str) -> &str`: **one** borrowed input and one returned reference — elision can assume “output lives as long as `s`.” `longest` has **two** borrowed inputs, so elision stops; you must write `'a` (or `'long`, etc.) yourself.

```rust
// Playground
fn main() {
    let s1 = String::from("long");
    let s2 = String::from("x");
    let r = longest(&s1, &s2);
    // drop s1;  // error: r might still point into s1
    println!("{}", r);
}
```

Without `<'a>` on the function, this `main` could compile even when `r` still points at freed memory. Lifetimes exist so that mistake is caught on the **function definition**, before any caller runs.

| Situation | What you are telling the compiler |
|-----------|-----------------------------------|
| `fn first<'a>(s: &'a str) -> &'a str` | Return borrows from `s` only |
| `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str` | Return may point into `x` **or** `y`; both must stay alive |
| `fn pick<'a, 'b>(x: &'a str, y: &'b str) -> &'a str` | Return borrows from `x` only; `y` can die earlier |

The only lifetime name with a **fixed** meaning is **`'static`**: valid for the whole program (e.g. string literals). Do not slap `'static` on normal borrows to silence errors.

**Mental model:** owner = suitcase, `&T` = claim ticket. Lifetimes prove you never read the ticket after the suitcase was thrown away.

## Elision (why you rarely write lifetimes)

Often the compiler infers the contract above — **lifetime elision**. You write the short form; Rust fills in `'a` for you:

```rust
fn len(s: &str) -> usize
// means: fn len<'a>(s: &'a str) -> usize  — no return reference, so no tie needed

fn first_word(s: &str) -> &str
// means: return borrows from `s` (elision ties input and output)
```

Three **elision rules** cover most everyday signatures. When they do not apply — several references in and out, ambiguous return — you write `'a` / `'b` explicitly, with any names you like:

```rust
fn longest<'long>(x: &'long str, y: &'long str) -> &'long str {
    if x.len() >= y.len() { x } else { y }
}
```

## Named lifetimes on structs

You have not met **structs** in depth yet ([Chapter 6](06_structs_traits_generics.md) covers `struct`, `impl`, and traits). For lifetimes, you only need this much:

A **struct** groups named fields into one type — like a tuple with labels, or a small Java/Python class that holds data:

```rust
struct Point {
    x: f64,
    y: f64,
}
```

When a field is a **borrow** (`&str`, `&[u8]`, …) instead of an owned value (`String`, `Vec`), the struct does not own that memory. It only holds a pointer into someone else’s buffer. The same question applies as for `longest`:

> **“Which owner must still be alive while this struct exists?”**

Put `'a` on the **struct** (not just on a function) when a field stores a reference:

```rust
// Playground
struct Excerpt<'a> {
    text: &'a str,  // borrows from outside; struct does not own the bytes
}

fn main() {
    let novel = String::from("Call me Ishmael. Once upon a time...");
    let first = novel.split('.').next().unwrap_or("");
    let e = Excerpt { text: first };
    println!("{}", e.text);
} // `e` and `first` dropped here; then `novel` can be dropped
```

Read `Excerpt<'a>` like `longest<'a>`: **`'a` is the lifetime of the borrow stored in `text`.** The struct instance may not outlive the data `text` points into — here, `novel`.

| Piece | Role |
|-------|------|
| `novel: String` | **Owner** of the heap text |
| `first: &str` | Borrow into `novel` |
| `e: Excerpt<'a>` | Bundles that borrow; `'a` ties `e.text` to `novel`’s lifetime |

### What if the struct omits `<'a>`?

```rust
struct Excerpt {   // does not compile
    text: &str,
}
```

Same idea as `fn longest(x: &str, y: &str) -> &str` without lifetimes: the compiler sees a reference inside the type but **no contract** for how long it is valid. Error: **missing lifetime specifier** on `text`.

**Practical rule:** struct fields that are references almost always need a lifetime parameter on the struct. Fields that are owned (`String`, `u32`, `Vec<…>`) do not.

```rust
// Playground — uncomment `drop` to see the error
fn main() {
    let novel = String::from("Call me Ishmael.");
    let first = novel.split('.').next().unwrap_or("");
    let e = Excerpt { text: first };
    // drop(novel);  // error: `e.text` still borrows `novel`
    println!("{}", e.text);
}
```

Prefer **owned** fields in public APIs (`String` instead of `&str`) until you need zero-copy views. Methods on structs (`impl Excerpt { ... }`) come in [Chapter 6](06_structs_traits_generics.md).

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

## Playground: two inputs, one shared lifetime

When the return must be valid for **either** argument, both references share one lifetime parameter (here `'a`; any name works):

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

- [Chapter 1: Ownership and borrowing](01_paradigm_shift.md#references-borrowing-and-dereferencing)
- [Chapter 3: Iterators](03_iterators.md) — `.iter()` borrows reinforce lifetime thinking
- [Chapter 6: Structs, traits, and generics](06_structs_traits_generics.md) — full `struct` / `impl` coverage (this chapter only needs borrowed fields + `'a`)

### Afterparty: AI Lego blocks

1. **Error archaeology** — “I paste a ‘lifetime may not live long enough’ error; walk me through owner vs reference diagram.”
2. **Return type choice** — “For API `fn title(book: &Book) -> ???` compare `&str` vs `String` trade-offs for a library.”
3. **Struct lifetime** — “Design `ConfigParser` holding `&str` slices into input buffer — when is it sound vs use owned `String`?”
4. **Elision quiz** — “Add explicit lifetimes to 4 function signatures where elision fails.”
5. **Java analogy** — “Compare Rust lifetimes to Java stack locals vs heap references — 120 words, accurate only.”
6. **Fix mine** — “I return `&String` built inside function; show three idiomatic fixes ranked by simplicity.”
