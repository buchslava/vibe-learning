# Trust & Use — Revealed

During the book you copied syntax and kept going. This appendix explains what those pieces mean. You do not need to memorize everything here — treat it as a reference you can reread after Chapter 11.

---

## Macros (`!` at the end)

Names ending with `!` are **macros** — templates that expand into ordinary Rust code before the program runs.

| Macro | What it does |
|-------|----------------|
| `println!("Hi {}", name)` | Print a line with placeholders filled in |
| `print!(".")` | Print without a newline |
| `format!("{}{}", a, b)` | Build a `String` instead of printing |
| `vec![1, 2, 3]` | Create a `Vec` from a list of values |
| `vec![' '; 9]` | Create a vector with nine copies of one value |

You used macros from day one. They save typing and reduce mistakes.

---

## `#[derive(...)]`

Above a `struct` or `enum`, `#[derive(...)]` asks Rust to **auto-generate** common implementations.

| Trait | Meaning for beginners |
|-------|----------------------|
| `Debug` | Lets you print a value for debugging with `{:?}` |
| `Clone` | `.clone()` makes a duplicate |
| `Copy` | Simple values copy automatically instead of moving (integers, coordinates, enums with `Copy`) |
| `PartialEq` | Lets you compare with `==` |

Example from [Chapter 10](../chapters/10_snake_trail.md):

```rust
#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}
```

Without `Copy`, you would pass directions differently. For small game types, `Copy` keeps code simple.

---

## `Option`: `Some` and `None`

`Option<T>` means **maybe a T, maybe nothing**.

| Value | Meaning |
|-------|---------|
| `Some(42)` | A value is present |
| `None` | No value |

Used in [Chapter 8](../chapters/08_guess_the_secret.md) for win tracking and [Chapter 9](../chapters/09_tic_tac_toe_arena.md) for winner detection:

```rust
match winner(&board) {
    Some(mark) => println!("Winner: {}", mark),
    None => println!("No winner yet."),
}
```

`match` forces you to handle both cases — Rust will not let you forget `None`.

---

## `Result` and `?`

`Result<T, E>` means **success (`Ok`) or failure (`Err`)**. File and terminal operations can fail; `Result` carries that information.

In [Chapter 11](../chapters/11_live_on_your_machine.md):

```rust
fn main() -> io::Result<()> {
    execute!(stdout, Hide)?;
    // ...
    Ok(())
}
```

The `?` suffix means: if this operation failed, return the error from `main` immediately. Otherwise continue.

For small programs, that is cleaner than nested `if let Err(...)`.

---

## Borrowing with `&`

In [Chapter 6](../chapters/06_character_sheet.md) you saw:

```rust
fn print_sheet(hero: &Player) {
    println!("Name: {}", hero.name);
}
```

`&Player` means **borrow for reading** — the function looks at the sheet without taking it away. [Rust Core — Chapter 1](../../rust-core/chapters/01_paradigm_shift.md) explains ownership and borrowing in full.

---

## Helper methods you met

| Method | Role |
|--------|------|
| `.abs()` | Absolute value of a number |
| `.repeat(n)` | Repeat a string or character `n` times |
| `.contains(&item)` | Check if a vector holds a value |
| `.clamp(min, max)` | Keep a number inside a range |
| `.len()` | Count items in a vector or string |
| `.push(x)` | Add to the end of a vector |
| `.pop()` | Remove from the end of a vector |
| `.insert(0, x)` | Insert at the front (Snake head) |

These are ordinary functions tied to a type — Rust calls them **methods**.

---

## What to learn next

You now know variables, control flow, functions, structs, enums, vectors, and small games. The next layer — **ownership**, **traits**, **errors in depth**, and **concurrency** — lives in **[Rust Core](../../rust-core/CONTENTS.md)**.

Read [Rust Core Preface](../../rust-core/chapters/preface.md) when you feel comfortable writing 50–100 lines of Rust on your own.

---

## Quick map: where each idea appeared

| Idea | First chapter |
|------|----------------|
| `println!`, `let`, `mut` | [1 — First Words](../chapters/01_first_words.md) |
| `for`, `if` | [2 — Patterns](../chapters/02_patterns_on_the_wall.md) |
| `bool`, comparisons | [3 — Crossroads](../chapters/03_crossroads.md) |
| `Vec`, `vec!` | [4 — Backpack](../chapters/04_the_backpack.md) |
| `fn`, parameters, return | [5 — Recipes](../chapters/05_recipes.md) |
| `struct` | [6 — Character Sheet](../chapters/06_character_sheet.md) |
| `enum`, `match` | [7 — Compass](../chapters/07_compass_and_doors.md) |
| `while`, `Option` | [8 — Guess](../chapters/08_guess_the_secret.md) |
| 2D grid, game rules | [9 — Tic-Tac-Toe](../chapters/09_tic_tac_toe_arena.md) |
| Game loop, body `Vec` | [10 — Snake Trail](../chapters/10_snake_trail.md) |
| `?`, `crossterm`, Cargo | [11 — Live](../chapters/11_live_on_your_machine.md) |
