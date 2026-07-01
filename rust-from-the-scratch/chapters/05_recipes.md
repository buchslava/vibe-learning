# Chapter 5: Recipes

## Story hook

You drew walls in Chapter 2 and walked forest paths in Chapter 3. The same patterns appear again and again. A **function** is a named recipe — write it once, use it whenever you need that dish.

## What you will build

A triangle from a recipe, then a tiny adventure scene built from two recipes:

```text
*
**
***
****
--- adventure ---
You go left.
  A coin glints in the grass.
You go right.
  Wind rushes through the trees.
```

---

### Step 1 — One-line recipe

**Algorithm:** define a function that prints; call it from `main`.

**Types:** the function returns nothing — Rust writes that as `()`.

**Language:** a card in the kitchen that says "say hello."

```rust
// Playground
fn greet() {
    println!("Welcome, traveler.");
}

fn main() {
    greet();
}
```

**Expected output:**

```text
Welcome, traveler.
```

Functions can sit **above** `main`. Rust reads the whole file before running.

---

### Step 2 — Recipe with a width parameter

**Algorithm:** pass a number; loop that many times; print dots.

**Types:** `width` is `i32`; the function still returns `()`.

**Language:** "draw a line this long" — the number is the instruction.

```rust
// Playground
fn dot_line(width: i32) {
    for _ in 0..width {
        print!(".");
    }
    println!();
}

fn main() {
    dot_line(5);
    dot_line(3);
}
```

**Expected output:**

```text
.....
...
```

---

### Step 3 — Recipe that returns a number

**Algorithm:** roll damage from a fixed "dice" value; return it; print in `main`.

**Types:** parameter and return are `i32`.

**Language:** a function is a question: "how much damage?" — answer is a number.

```rust
// Playground
fn roll_damage(base: i32) -> i32 {
    base + 2
}

fn main() {
    let hit = roll_damage(3);
    println!("You deal {} damage.", hit);
}
```

**Expected output:**

```text
You deal 5 damage.
```

`-> i32` says the function **returns** an integer.

---

### Step 4 — Triangle and adventure recipes together

**Algorithm:** `print_triangle` draws stars; `describe_move` prints one path outcome; `main` calls both.

**Types:** height is `i32`; choice is `&str`.

**Language:** two recipe cards — one for art, one for story.

```rust
// Playground
fn print_triangle(height: i32) {
    for row in 0..height {
        for _ in 0..=row {
            print!("*");
        }
        println!();
    }
}

fn describe_move(choice: &str) {
    println!("You go {}.", choice);
    if choice == "left" {
        println!("  A coin glints in the grass.");
    } else {
        println!("  Wind rushes through the trees.");
    }
}

fn main() {
    print_triangle(4);
    println!("--- adventure ---");
    describe_move("left");
    describe_move("right");
}
```

**Expected output:**

```text
*
**
***
****
--- adventure ---
You go left.
  A coin glints in the grass.
You go right.
  Wind rushes through the trees.
```

---

## Full chapter solution

Same as Step 4 above.

---

## See also

Next: [Chapter 6 — Character Sheet](06_character_sheet.md) — group related fields in a struct.
