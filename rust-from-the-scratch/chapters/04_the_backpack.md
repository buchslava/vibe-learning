# Chapter 4: The Backpack

## Story hook

Every adventurer needs a backpack. You cannot see inside the computer's memory, but you **can** print a numbered list of what you carry — and check whether the key is there before you reach the door.

## What you will build

```text
Backpack (3 items):
  1. torch
  2. rope
  3. key
You have the key!
```

---

### Step 1 — Empty pack

**Algorithm:** create an empty list; if length is zero, say so.

**Types:** `pack` is a `Vec<&str>` — a growable list of text items.

**Language:** an empty bag — nothing to show yet.

```rust
// Playground
fn main() {
    let pack: Vec<&str> = Vec::new();
    if pack.len() == 0 {
        println!("Your backpack is empty.");
    }
}
```

**Expected output:**

```text
Your backpack is empty.
```

`Vec::new()` makes an empty vector. `len()` counts items.

---

### Step 2 — Add three items

**Algorithm:** create empty list; push three names; print count.

**Types:** each item is `&str`; the list is `Vec<&str>`.

**Language:** dropping three objects into a bag, one at a time.

```rust
// Playground
fn main() {
    let mut pack: Vec<&str> = Vec::new();
    pack.push("torch");
    pack.push("rope");
    pack.push("key");
    println!("You carry {} items.", pack.len());
}
```

**Expected output:**

```text
You carry 3 items.
```

`push` adds to the **end** of the list.

---

### Step 3 — Numbered inventory

**Algorithm:** loop with index; print each item with its number.

**Types:** `i` is `usize` (unsigned size — good for counting); items are `&str`.

**Language:** reading a packing list: "1. torch, 2. rope, …"

```rust
// Playground
fn main() {
    let pack = vec!["torch", "rope", "key"];
    println!("Backpack ({} items):", pack.len());
    for i in 0..pack.len() {
        println!("  {}. {}", i + 1, pack[i]);
    }
}
```

**Expected output:**

```text
Backpack (3 items):
  1. torch
  2. rope
  3. key
```

> **Trust & Use:** `vec!["torch", "rope", "key"]` builds a vector from a list you give. Copy it exactly. Details in [Trust & Use](../appendix/TRUST_AND_USE.md).

---

### Step 4 — Do I have the key?

**Algorithm:** build inventory; print it; check if `"key"` is in the list.

**Types:** `pack` is `Vec<&str>`; `contains` returns `bool`.

**Language:** patting your pockets before the locked door.

```rust
// Playground
fn main() {
    let pack = vec!["torch", "rope", "key"];
    println!("Backpack ({} items):", pack.len());
    for i in 0..pack.len() {
        println!("  {}. {}", i + 1, pack[i]);
    }
    if pack.contains(&"key") {
        println!("You have the key!");
    } else {
        println!("No key yet.");
    }
}
```

**Expected output:**

```text
Backpack (3 items):
  1. torch
  2. rope
  3. key
You have the key!
```

Remove `"key"` from the list and run again to see the other message.

---

## Full chapter solution

Same as Step 4 above.

---

## See also

Next: [Chapter 5 — Recipes](05_recipes.md) — wrap repeated steps in functions.
