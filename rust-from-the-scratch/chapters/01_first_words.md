# Chapter 1: First Words

## Story hook

You arrive at a game convention. Volunteers hand out name badges printed by a small program. Today, **you** write that program.

## What you will build

```text
+------------------+
|  GAME CON 2026   |
|  Name: Mira      |
|  Level: 7        |
+------------------+
```

---

### Step 1 — Say hello

**Algorithm:** run the program; print one line; stop.

**Types:** none yet — the message is baked into the program.

**Language:** like shouting a single sentence across the room.

```rust
// Playground
fn main() {
    println!("Hello!");
}
```

**Expected output:**

```text
Hello!
```

Every Rust program starts with `fn main()`. That is where execution begins. `println!` prints a line and adds a newline at the end.

> **Trust & Use:** `println!` ends with `!` — it is a **macro**, a shorthand that expands into more code. Use it as shown; see [Trust & Use](../appendix/TRUST_AND_USE.md).

---

### Step 2 — Store a name

**Algorithm:** create a box labeled `name`, put text inside, print a greeting that uses the box.

**Types:** `name` holds text. In Rust we write that as `&str` (a read-only text slice).

**Language:** `name` is a label on a box; the box holds the word **Mira**.

```rust
// Playground
fn main() {
    let name = "Mira";
    println!("Hello, {}!", name);
}
```

**Expected output:**

```text
Hello, Mira!
```

`let` creates a **variable** — a named place for a value. `{}` in the string is filled by `name`.

---

### Step 3 — A score that can change

**Algorithm:** store name and level; print both; increase level; print again.

**Types:** `name` is text (`&str`); `level` is a whole number (`i32`).

**Language:** the level is a number on the badge you can update with a pen.

```rust
// Playground
fn main() {
    let name = "Mira";
    let mut level = 7;
    println!("{} is level {}", name, level);
    level = 8;
    println!("After one quest: level {}", level);
}
```

**Expected output:**

```text
Mira is level 7
After one quest: level 8
```

`mut` means **mutable** — the value may change. Without `mut`, Rust would reject `level = 8`.

---

### Step 4 — The full badge

**Algorithm:** print a border, then name, then level, then a closing border.

**Types:** same as Step 3 — text and integer.

**Language:** like filling in a paper form line by line.

```rust
// Playground
fn main() {
    let name = "Mira";
    let mut level = 7;

    println!("+------------------+");
    println!("|  GAME CON 2026   |");
    println!("|  Name: {}      |", name);
    println!("|  Level: {}        |", level);
    println!("+------------------+");
}
```

**Expected output:**

```text
+------------------+
|  GAME CON 2026   |
|  Name: Mira      |
|  Level: 7        |
+------------------+
```

Change `name` and `level` to your own values and run again.

---

## Full chapter solution

```rust
// Playground
fn main() {
    let name = "Mira";
    let level = 7;

    println!("+------------------+");
    println!("|  GAME CON 2026   |");
    println!("|  Name: {}      |", name);
    println!("|  Level: {}        |", level);
    println!("+------------------+");
}
```

---

## See also

Next: [Chapter 2 — Patterns on the Wall](02_patterns_on_the_wall.md) — draw pictures with loops.
