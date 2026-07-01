# Chapter 3: Crossroads

## Story hook

The forest path splits. A locked door, a dark cave, a sunny meadow — your choices rewrite the ending. The playground cannot read your keyboard yet, so we **script** your decisions like lines in a play.

## What you will build

```text
Scene 1: You stand at a crossroads.
You go left.
You find a rusty key!
Scene 2: The door is locked.
You use the key. The treasure is yours!
You win!
```

---

### Step 1 — Is the door locked?

**Algorithm:** set a true/false flag; if true, print one message; otherwise print another.

**Types:** `locked` is a **boolean** — only `true` or `false`.

**Language:** a yes/no question with two possible answers.

```rust
// Playground
fn main() {
    let locked = true;
    if locked {
        println!("The door is locked.");
    } else {
        println!("The door swings open.");
    }
}
```

**Expected output:**

```text
The door is locked.
```

Change `locked` to `false` and run again to see the other branch.

---

### Step 2 — Two endings

**Algorithm:** pick a path string; compare it; print the matching ending.

**Types:** `path` is text (`&str`); comparison yields `bool`.

**Language:** "If you went left, then … otherwise …"

```rust
// Playground
fn main() {
    let path = "left";
    if path == "left" {
        println!("You meet a friendly fox.");
    } else {
        println!("You meet a sleeping bear.");
    }
}
```

**Expected output:**

```text
You meet a friendly fox.
```

---

### Step 3 — A script of three choices

**Algorithm:** walk through a list of preset choices; print what happens at each step.

**Types:** `choices` is a fixed array of text slices; `choice` is one `&str`.

**Language:** reading stage directions aloud, one line at a time.

```rust
// Playground
fn main() {
    let choices = ["left", "right", "left"];
    for choice in choices {
        println!("You go {}.", choice);
        if choice == "left" {
            println!("  A coin glints in the grass.");
        } else {
            println!("  Wind rushes through the trees.");
        }
    }
}
```

**Expected output:**

```text
You go left.
  A coin glints in the grass.
You go right.
  Wind rushes through the trees.
You go left.
  A coin glints in the grass.
```

Imagine you picked those directions on a keyboard — the array is your recorded play session.

---

### Step 4 — Mini adventure with health and key

**Algorithm:** start with health and no key; follow scripted moves; update state; print win or lose.

**Types:** `health` is `i32`; `has_key` is `bool`; choices are text.

**Language:** a short story where inventory and health change the ending.

```rust
// Playground
fn main() {
    let mut health = 3;
    let mut has_key = false;
    let choices = ["left", "right", "left"];

    println!("Scene 1: You stand at a crossroads.");

    for choice in choices {
        println!("You go {}.", choice);
        if choice == "left" {
            println!("You find a rusty key!");
            has_key = true;
        } else {
            println!("A thorn bush scratches you.");
            health -= 1;
        }
    }

    println!("Scene 2: The door is locked.");
    if has_key && health > 0 {
        println!("You use the key. The treasure is yours!");
        println!("You win!");
    } else if health <= 0 {
        println!("You are too tired to continue.");
        println!("You lose.");
    } else {
        println!("No key. You turn back.");
        println!("You lose.");
    }
}
```

**Expected output:**

```text
Scene 1: You stand at a crossroads.
You go left.
You find a rusty key!
You go right.
A thorn bush scratches you.
You go left.
You find a rusty key!
Scene 2: The door is locked.
You use the key. The treasure is yours!
You win!
```

`&&` means **and** — both conditions must be true. `-=` subtracts from `health`.

---

## Full chapter solution

Same as Step 4 above.

---

## See also

Next: [Chapter 4 — The Backpack](04_the_backpack.md) — carry a list of items.
