# Chapter 2: Types and Expressions

## Hook

Rust’s type system is not ceremony. The compiler uses it to check ownership, catch bugs, and keep your code predictable **before** you run anything. This chapter maps the built-in types, how inference works, and the expression-style control flow you will see in every idiomatic crate.

## Integer types

Rust has **fixed-width** signed and unsigned integers. Pick the width that matches your domain (protocol field, array index, counter).

| Type | Width | Typical use |
|------|-------|-------------|
| `i8` … `i128` | 8–128 bit signed | Binary protocols, embedded registers |
| `u8` … `u128` | 8–128 bit unsigned | Bytes, sizes, hashes |
| `isize` / `usize` | pointer-sized | indexing, `len()`, collection sizes |

```rust
// Playground
fn main() {
    let port: u16 = 502;           // Modbus-style port — explicit width
    let count = 42;                // infers i32 by default
    let byte = 0xFFu8;
    let big: i64 = 1_000_000;
    println!("{} {} {} {}", port, count, byte, big);
}
```

**Defaults:** integer literals infer `i32` unless you suffix (`42u64`) or annotate.

For wire formats and hardware, **always** choose an explicit width. Do not rely on “whatever the platform uses.”

**Overflow:** in debug builds, arithmetic that overflows **panics**. In release, integers wrap (two’s complement).

Use `checked_add`, `saturating_add`, or `wrapping_add` when the policy matters.

## Floating point, `bool`, and `char`

| Type | Notes |
|------|-------|
| `f32`, `f64` | IEEE floats; `f64` is the default inference |
| `bool` | exactly `true` or `false` — not an integer in safe Rust |
| `char` | one **Unicode scalar value** (4 bytes), e.g. `'🦀'` |
| `()` | **unit type** — “no meaningful value”; used as empty return |

```rust
// Playground
fn main() {
    let pi: f64 = 3.14159;
    let ok = true;
    let crab = '🦀';
    let unit: () = (); // only value of type ()
    println!("{} {} {} {:?}", pi, ok, crab, unit);
}
```

`char` is not a UTF-8 byte — it is a single code point.

Text strings use `str` / `String` (below); ownership of those buffers is [Chapter 1](01_paradigm_shift.md#ownership-vs-garbage-collection).

## Inference and annotations

The compiler infers types when unambiguous. Add annotations or suffixes when it is not:

```rust
// Playground
fn main() {
    let xs = vec![1, 2, 3];     // Vec<i32>
    let ys: Vec<u8> = Vec::new(); // empty vec needs a hint
    let n = 10u32;
    let ratio = n as f64 / 3.0;  // cast integer to float
    println!("{:?} {:?} {}", ys, xs, ratio);
}
```

**Casts** use `as` between numeric types (`u16 as u32`). **Conversions** that can fail (string parsing, narrowing) use methods like `.parse()` or `TryFrom` — covered in [Chapter 8](08_errors_and_testing.md).

## Tuples and arrays

**Tuples** group fixed types; access by index (`.0`, `.1`):

```rust
// Playground
fn main() {
    let pair: (u16, &str) = (502, "modbus");
    println!("port {} proto {}", pair.0, pair.1);
}
```

**Arrays** `[T; N]` have compile-time fixed length and stack storage when `T` is `Copy`:

```rust
// Playground
fn main() {
    let frame: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    println!("len {} first {:02X}", frame.len(), frame[0]);
}
```

Growable `Vec<T>` and ownership rules for slices come back in [Chapter 1](01_paradigm_shift.md#references-borrowing-and-dereferencing) and [Chapter 11](11_collections.md).

## Slices

A **slice** is a **view** into contiguous memory: pointer plus length, no ownership.

The type is written `&[T]` — a borrow of `[T]`. `[T]` is **unsized**: the compiler does not know its length at compile time.

| Form | Meaning |
|------|---------|
| `&[T]` | borrowed view of `T` elements |
| `&[u8]` | byte buffer view — wire payloads, file chunks, `Vec<u8>` data |
| `&str` | UTF-8 **text** slice — same fat-pointer idea, different element type (`u8` with UTF-8 invariant) |

Create slices from arrays, vectors, or other slices with **range syntax**:

```rust
// Playground
fn main() {
    let frame: [u8; 6] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    let payload: &[u8] = &frame[2..5]; // bytes at indices 2, 3, 4
    let all: &[u8] = &frame[..];      // whole array as slice

    let nums = vec![10, 20, 30, 40];
    let mid = &nums[1..3];             // &[i32] borrowing the Vec's buffer

    println!("payload {:?} all {:?} mid {:?}", payload, all, mid);
}
```

**Indexing:** `[i]` on a slice is a direct access. Out-of-range indices **panic** at runtime.

Prefer `.get(i) -> Option<&T>` when the index comes from user input, parsed fields, or loop math you have not proven safe.

```rust
// Playground
fn main() {
    let frame: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    let i = 2;
    match frame.get(i) {
        Some(byte) => println!("byte {:02X}", byte),
        None => println!("index {} out of range", i),
    }
}
```

Half-open ranges match `for` loops: `&xs[0..3]` includes indices 0, 1, 2. An empty slice is valid: `&xs[0..0]`.

## Strings and UTF-8

Rust strings are **UTF-8** byte sequences, not fixed-width characters. That keeps them compact and compatible with the web and most modern protocols.

It also means **byte index ≠ character index**.

| Type | Role |
|------|------|
| `str` | unsized UTF-8 text — always used as `&str` (borrowed slice) |
| `&str` | borrowed view into valid UTF-8 (literal, substring, or `String` borrow) |
| `String` | owned, growable UTF-8 buffer on the heap |

```rust
// Playground
fn main() {
    let literal: &str = "fixed";            // stored in read-only segment
    let owned = String::from("growable");   // heap-backed owner
    let also: &str = &owned;                // borrow the String as &str

    let mut label = String::new();
    label.push_str("port=");
    label.push_str("502");

    println!("{} {} {}", literal, owned, also);
    println!("{}", label);
}
```

**Literals** have type `&str`. **`String::from`**, **`.to_string()`**, and **`format!`** allocate when you need an owned buffer.

**`push_str`** / **`push`** extend a `String` in place. Use them when building output in a loop — log lines, CSV rows, protocol text fields.

### `&str` vs `String` in APIs

| Callee needs | Parameter type |
|--------------|----------------|
| Read or compare text only | `&str` |
| Store text beyond the call | `String` (or return owned from caller) |
| Accept literal or owned caller data | `&str` — callers pass `"literal"` or `&my_string` |

```rust
// Playground
fn log_label(label: &str) {
    println!("label={}", label);
}

fn store_tag(tag: String) -> String {
    tag // owner returns owned text
}

fn main() {
    let name = String::from("sensor_a");
    log_label("startup");      // &str literal
    log_label(&name);          // &String coerces to &str
    let saved = store_tag(name);
    println!("{}", saved);
}
```

Rust **coerces** `&String` to `&str` at call sites. So `fn f(s: &str)` is the default for read-only string parameters.

Take `String` when the function must **own** the data — for example, to store in a struct, spawn a thread, or insert into a map keyed by owned text.

### Length, iteration, and slicing text

- **`.len()`** on `&str` / `String` is **byte** length, not grapheme or `char` count.
- **`.chars()`** iterates Unicode scalar values (`char`); use when you care about characters.
- **`.bytes()`** iterates raw `u8` values — useful next to binary parsers.
- **`.lines()`** splits on line endings — common for config and serial-line protocols.

```rust
// Playground
fn main() {
    let s = "naïve"; // 'ï' is two UTF-8 bytes
    println!("bytes={} chars={}", s.len(), s.chars().count());

    for ch in s.chars() {
        print!("{} ", ch);
    }
    println!();

    for line in "a\nb\n".lines() {
        println!("line: {}", line);
    }
}
```

**String slicing** uses the same range syntax as byte slices. Ranges must fall on **UTF-8 character boundaries**.

Slicing at a bad byte index **panics**:

```rust
// Playground
fn main() {
    let s = "hello";
    let hello = &s[0..5]; // ok — ASCII, one byte per char

    let emoji = "hi 🦀";
    let hi = &emoji[0..2]; // ok
    // let bad = &emoji[0..3]; // panic — splits the crab emoji
    println!("{} | {}", hello, hi);
}
```

At protocol and config boundaries, treat **`&[u8]`** as opaque bytes on the wire. Treat **`&str`** as validated human-readable text — config keys, error messages, JSON string fields after parsing.

Do not cast arbitrary bytes to `&str` without validation. Use **`std::str::from_utf8`** or **`String::from_utf8`** when converting ([Chapter 8](08_errors_and_testing.md)).

### Common `&str` helpers (no allocation)

| Method | Use |
|--------|-----|
| `.starts_with` / `.ends_with` | protocol prefixes, file extensions |
| `.strip_prefix` / `.strip_suffix` | peel `"CMD:"` off a serial line |
| `.split` / `.split_once` | CSV-ish fields, key=value pairs |
| `.trim` | whitespace around user input |
| `.parse::<T>()` | `"502".parse::<u16>()` → `Result` |

```rust
// Playground
fn main() {
    let line = "  PORT=502  ";
    let trimmed = line.trim();
    if let Some(rest) = trimmed.strip_prefix("PORT=") {
        match rest.parse::<u16>() {
            Ok(port) => println!("port {}", port),
            Err(e) => println!("bad port: {}", e),
        }
    }
}
```

## Variables and mutability

```rust
// Playground
fn main() {
    let x = 10;      // immutable binding
    let mut y = 20;  // mutable binding
    y += x;
    println!("{}", y);
}
```

Default **`let` is immutable**. Use `mut` only where the value changes — it documents intent and helps the borrow checker later.

## Control flow as expressions

```rust
// Playground
fn main() {
    let n = 7;
    let label = if n % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", n, label);

    for i in 0..3 {
        println!("i = {}", i);
    }

    let mut sum = 0;
    let mut k = 0;
    while k < 3 {
        sum += k;
        k += 1;
    }
    println!("sum = {}", sum);
}
```

- **`if`** is an expression — both branches must return the same type.
- **`0..3`** is half-open (0, 1, 2); **`0..=3`** is inclusive (0 through 3).
- **`loop`**, **`while`**, and **`for`** cover the usual iteration shapes; `for` over a range is idiomatic for indexed passes. Iterator pipelines (`map`, `filter`, `collect`) are [Chapter 4](04_iterators.md).

## `match` preview

Full power in [Chapter 6](06_types_enums_pattern_matching.md); here, matching integers:

```rust
// Playground
fn main() {
    let code = 404;
    let msg = match code {
        200 => "ok",
        404 => "not found",
        _ => "other",
    };
    println!("{}", msg);
}
```

`_` is the wildcard arm. `match` must be **exhaustive** — every possible value covered (easy for integers with `_`; enums need every variant).

## Idiom spotlight

> **Name your widths at boundaries.** Inside a function, inference is fine. At protocol edges, file headers, and public APIs, use explicit types (`u16`, `[u8; 8]`). That way refactors cannot silently change layout.

> **Borrow text, own when storing.** Prefer `&str` and `&[u8]` in function parameters. Return or store `String` / `Vec<u8>` only when the data must outlive the caller. At binary/text boundaries, keep **`&[u8]` for bytes** and **`&str` for validated UTF-8**. Do not mix them silently.

## Go deeper

- [The Rust Book — Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [The Rust Book — String slices](https://doc.rust-lang.org/book/ch04-03-slices.html)

## See also

- [Preface](preface.md) — rustup and Cargo setup
- [Chapter 1: Ownership and borrowing](01_paradigm_shift.md#references-borrowing-and-dereferencing)
- [Chapter 4: Iterators](04_iterators.md)
- [Chapter 6: Enums and pattern matching](06_types_enums_pattern_matching.md)

### Afterparty: AI Lego blocks

Rust-only drills aligned with this chapter. Copy a prompt into your AI tutor; answer out loud before reading the reply.

#### Scalars and inference

1. **Integer pick** — “Quiz me: for 8 scenarios (Modbus port, byte buffer, collection index, money cents, timestamp millis, hash output, loop counter, enum discriminant), I pick `u8`/`u16`/`u32`/`u64`/`i32`/`usize`; you correct and explain overflow risk.”
2. **Suffix or annotate** — “Give 5 snippets where integer inference is ambiguous or wrong; I add a suffix or type annotation; you verify.”
3. **Overflow policy** — “Three arithmetic scenarios in a control loop. For each, recommend plain `+`, `checked_add`, `saturating_add`, or `wrapping_add`; explain panic vs wrap in debug/release.”
4. **`char` vs byte** — “Explain why `'A'` is not the same type as `65u8`. Show one valid `char` literal and one invalid escape; mention UTF-8 only when discussing `str`/`String`.”

#### Compound types and text

5. **Tuple vs array** — “When do I use `(u16, u16)` vs `[u16; 2]` for a coordinate pair? Give one idiomatic example of each.”
6. **Array bounds** — “Snippet with `[u8; 4]` and an index from user input. I explain compile-time vs runtime safety; you show bounds checking with `.get()` vs `[i]`.”
7. **`&str` vs `String` API** — “Five function signatures for logging labels. I choose `&str` or `String` for each; you explain ownership and allocation cost.”
8. **Parse drill** — “Give `let s = \"502\";` — I write two ways to get `u16` (unwrap version and proper `Result` version); you grade idiomatic error handling.”

#### Slices and string idioms

9. **Slice from array** — “Given `[u8; 8]` with a 2-byte header and 6-byte payload, I write range expressions for `header`, `payload`, and `full` as `&[u8]`; you check half-open bounds.”
10. **`.get` vs `[i]`** — “Three indexing scenarios (fixed compile-time index, user-supplied index, loop variable). I pick `[i]` or `.get(i)` for each; you explain panic risk.”
11. **`&[u8]` vs `&str`** — “Four automation scenarios (Modbus PDU bytes, JSON string field after parse, serial line with ASCII command, raw hex dump). I pick byte slice or text slice; you explain validation.”
12. **UTF-8 length trap** — “For `\"naïve\"` and `\"hi 🦀\"`, I predict `.len()` vs `.chars().count()`; you show one bad string slice that panics and the safe fix (`.chars()` or careful byte ranges).”
13. **Build a `String`** — “Task: assemble `\"port=502;host=plc1\"` from parts in a loop. I choose `push_str`, `format!`, or `+`; you rank by allocations and idiomatic style.”
14. **Coercion quiz** — “Five call sites passing `&str`, `String`, `&String`, and a literal into `fn log(msg: &str)`. I say ok or what coercion happens; you quote the effective type.”
15. **Text helpers** — “Give a serial line `CMD:READ,ADDR=10`. I rewrite using `trim`, `strip_prefix`, and `split_once` — no manual byte indexing; you nitpick.”
16. **Slice signature drill** — “Four functions: sum numeric slice, validate UTF-8 wire bytes, store tag for later, peek first 4 header bytes. I write parameter types (`&[u8]`, `&str`, `String`, etc.); you review.”
17. **Empty slice** — “Explain whether `&frame[0..0]` is valid, what `.len()` returns, and how `if slice.is_empty()` reads in a parser guard.”
18. **Lines and config** — “Multiline config string with comments and blank lines. I sketch a loop using `.lines()` and `.trim().starts_with('#')`; you extend with one `strip_prefix` for `KEY=` rows.”

#### Control flow and `match`

19. **Range quiz** — “For half-open vs inclusive ranges, I predict output of four `for` loops using `..` and `..=`; you correct with printed values.”
20. **`if` expression types** — “Show an `if` where branches return different types and fail to compile. I explain the error; you fix with a unified type (same type in both arms or wrap in enum preview).”
21. **match warm-up** — “Extend a `match` on HTTP status codes with 3xx, 4xx, 5xx groupings using range patterns; I write arms; you check exhaustiveness.”
22. **Loop choice** — “Four iteration tasks (infinite retry with break, consume iterator, index 0..len, early exit on condition). I pick `loop`/`while`/`for`; you confirm idiomatic choice.”

#### Reading and reviewing code

23. **Type read-through** — “Paste a 20-line `main` with mixed types. I annotate every binding with its inferred or explicit type without running the compiler; you correct.”
24. **Protocol struct sketch** — “Design a `[u8; N]` frame parser header: field names, types, and which values must be explicit width. No full impl — types only.”
25. **Review nitpick** — “Show unidiomatic code using `mut` everywhere and magic `i32` for port numbers. I rewrite with minimal `mut` and `u16`; you review.”
26. **String smell audit** — “Paste 25 lines mixing `&String` parameters, `.clone()` on literals, and `[..]` on non-ASCII text. I flag smells and rewrite with `&str`, borrows, and safe slicing; you grade.”
27. **Frame parser sketch** — “I describe a 12-byte header + variable payload protocol. You ask only type/signature questions; I answer with `&[u8]`, ranges, and where `String` appears (if at all).”
