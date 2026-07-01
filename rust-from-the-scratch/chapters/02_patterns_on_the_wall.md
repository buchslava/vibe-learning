# Chapter 2: Patterns on the Wall

## Story hook

Deep in the tutorial dungeon, a blank wall waits for decoration. You teach the computer to stamp symbols in rows and columns — your first **picture** made of text.

## What you will build

A hollow square frame:

```text
########
#......#
#......#
#......#
#......#
########
```

---

### Step 1 — One row of dots

**Algorithm:** repeat printing `.` five times on one line.

**Types:** the loop counter is a whole number; `.` is a single **character** (`char`).

**Language:** like writing one sentence of five dots.

```rust
// Playground
fn main() {
    for _ in 0..5 {
        print!(".");
    }
    println!();
}
```

**Expected output:**

```text
.....
```

`for _ in 0..5` runs the body **five times**. `0..5` means 0, 1, 2, 3, 4 — not including 5. We use `_` because we do not need the counter value. `print!` stays on the same line; `println!()` finishes the line.

> **Trust & Use:** `print!` also ends with `!` — same family as `println!`. See [Trust & Use](../appendix/TRUST_AND_USE.md).

---

### Step 2 — A solid rectangle

**Algorithm:** outer loop for rows; inner loop for columns; print `#` each time.

**Types:** row and column counters are numbers; `#` is `char`.

**Language:** reading a paragraph line by line, each line the same width.

```rust
// Playground
fn main() {
    for _ in 0..4 {
        for _ in 0..6 {
            print!("#");
        }
        println!();
    }
}
```

**Expected output:**

```text
######
######
######
######
```

Four rows, six columns — a filled block.

---

### Step 3 — A growing triangle

**Algorithm:** row `r` prints `r + 1` stars; repeat for four rows.

**Types:** `row` is `i32`; `*` is `char`.

**Language:** each line of a poem is one word longer than the last.

```rust
// Playground
fn main() {
    for row in 0..4 {
        for _ in 0..=row {
            print!("*");
        }
        println!();
    }
}
```

**Expected output:**

```text
*
**
***
****
```

`0..=row` includes `row` itself — so row 0 prints one star, row 3 prints four.

---

### Step 4 — Hollow square frame

**Algorithm:** for each row, print `#` at the sides and `.` in the middle; top and bottom rows are all `#`.

**Types:** `row` and `col` are numbers; cells are `char`.

**Language:** a picture frame — thick border, empty center.

```rust
// Playground
fn main() {
    let size = 6;
    for row in 0..size {
        for col in 0..size {
            let edge = row == 0 || row == size - 1 || col == 0 || col == size - 1;
            if edge {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
```

**Expected output:**

```text
######
#....#
#....#
#....#
#....#
######
```

`if edge` chooses the character. `||` means **or** — any true condition makes `edge` true.

---

## Full chapter solution

```rust
// Playground
fn main() {
    let size = 6;
    for row in 0..size {
        for col in 0..size {
            let edge = row == 0 || row == size - 1 || col == 0 || col == size - 1;
            if edge {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
```

---

## See also

Next: [Chapter 3 — Crossroads](03_crossroads.md) — choose paths and change the story.
