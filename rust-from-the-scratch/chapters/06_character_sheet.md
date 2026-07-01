# Chapter 6: Character Sheet

## Story hook

Before the boss fight, you open your character sheet: name, health, gold. In Rust, a **struct** is that sheet — one name for a bundle of fields that belong together.

## What you will build

```text
=== Character Sheet ===
Name:   Mira
Health: 8 / 10
Gold:   15
=======================
After battle:
Health: 3 / 10
Gold:   20
```

---

### Step 1 — One field

**Algorithm:** build a `Player` value; print the name field.

**Types:** `Player` is a struct type; `name` is `&str`.

**Language:** a form with one box filled in.

```rust
// Playground
struct Player {
    name: &'static str,
    health: i32,
    gold: i32,
}

fn main() {
    let hero = Player {
        name: "Mira",
        health: 10,
        gold: 15,
    };
    println!("Hero: {}", hero.name);
}
```

**Expected output:**

```text
Hero: Mira
```

`struct` defines the **shape**; `{ name: "Mira", ... }` fills in the fields.

---

### Step 2 — Print the full sheet

**Algorithm:** create hero; print formatted lines for each field.

**Types:** all fields stay as defined — text and two integers.

**Language:** reading a stat card out loud, line by line.

```rust
// Playground
struct Player {
    name: &'static str,
    health: i32,
    gold: i32,
}

fn main() {
    let hero = Player {
        name: "Mira",
        health: 10,
        gold: 15,
    };
    println!("=== Character Sheet ===");
    println!("Name:   {}", hero.name);
    println!("Health: {} / 10", hero.health);
    println!("Gold:   {}", hero.gold);
    println!("=======================");
}
```

**Expected output:**

```text
=== Character Sheet ===
Name:   Mira
Health: 10 / 10
Gold:   15
=======================
```

---

### Step 3 — Take damage and earn gold

**Algorithm:** make hero mutable; subtract health; add gold; print updates.

**Types:** `mut hero` allows field updates; fields keep their types.

**Language:** erasing and rewriting numbers on the paper sheet.

```rust
// Playground
struct Player {
    name: &'static str,
    health: i32,
    gold: i32,
}

fn main() {
    let mut hero = Player {
        name: "Mira",
        health: 10,
        gold: 15,
    };
    hero.health -= 2;
    hero.gold += 5;
    println!("Health: {}", hero.health);
    println!("Gold:   {}", hero.gold);
}
```

**Expected output:**

```text
Health: 8
Gold:   20
```

Use `hero.health`, not `health` alone — the field lives **inside** the struct.

---

### Step 4 — Tiny battle sequence

**Algorithm:** print sheet; apply scripted hits and rewards; print sheet again.

**Types:** same `Player` struct throughout.

**Language:** a combat log updating the same character card.

```rust
// Playground
struct Player {
    name: &'static str,
    health: i32,
    gold: i32,
}

fn print_sheet(hero: &Player) {
    println!("=== Character Sheet ===");
    println!("Name:   {}", hero.name);
    println!("Health: {} / 10", hero.health);
    println!("Gold:   {}", hero.gold);
    println!("=======================");
}

fn main() {
    let mut hero = Player {
        name: "Mira",
        health: 10,
        gold: 15,
    };
    print_sheet(&hero);

    hero.health -= 7; // boss hit
    hero.gold += 5;   // loot

    println!("After battle:");
    print_sheet(&hero);
}
```

**Expected output:**

```text
=== Character Sheet ===
Name:   Mira
Health: 10 / 10
Gold:   15
=======================
After battle:
=== Character Sheet ===
Name:   Mira
Health: 3 / 10
Gold:   20
=======================
```

`&hero` **borrows** the struct for printing without taking ownership.

> **Trust & Use:** The `&` means "look, do not take." Full rules in [Trust & Use](../appendix/TRUST_AND_USE.md) and [Rust Core](../../rust-core/chapters/01_paradigm_shift.md).

---

## Full chapter solution

Same as Step 4 above.

---

## See also

Next: [Chapter 7 — Compass and Doors](07_compass_and_doors.md) — move on a map with enums.
