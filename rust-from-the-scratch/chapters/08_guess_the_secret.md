# Chapter 8: Guess the Secret

## Story hook

A vault hides a secret number. You cannot type guesses in the playground yet, so the program tries a fixed list. A **bar chart** made of `#` and `.` shows how close each guess is — hot or cold, visible at a glance.

## What you will build

```text
Secret is 42
Guess 10 -> .......... (far)
Guess 40 -> ############ (close)
Guess 42 -> FOUND!
You win in 3 tries!
```

---

### Step 1 — Rocket countdown

**Algorithm:** start at 3; while greater than zero, print and subtract; then blast off.

**Types:** `n` is `i32`.

**Language:** "3, 2, 1, lift-off" — same rhythm as a launch.

```rust
// Playground
fn main() {
    let mut n = 3;
    while n > 0 {
        println!("{}", n);
        n -= 1;
    }
    println!("Lift-off!");
}
```

**Expected output:**

```text
3
2
1
Lift-off!
```

`while` repeats until the condition is false.

---

### Step 2 — Search with fixed guesses

**Algorithm:** compare each guess to the secret; print hit or miss; stop early on success.

**Types:** all numbers are `i32`.

**Language:** checking numbered lockers until one opens.

```rust
// Playground
fn main() {
    let secret = 42;
    let guesses = [10, 40, 42, 99];
    for guess in guesses {
        if guess == secret {
            println!("Guess {} -> FOUND!", guess);
            break;
        } else {
            println!("Guess {} -> miss", guess);
        }
    }
}
```

**Expected output:**

```text
Guess 10 -> miss
Guess 40 -> miss
Guess 42 -> FOUND!
```

`break` exits the loop early.

---

### Step 3 — Hot/cold bar

**Algorithm:** map distance to bar length — closer guesses fill more `#` slots.

**Types:** `distance` is `i32`; bar is built as text.

**Language:** a thermometer filling up when you get warmer.

```rust
// Playground
fn bar_for(guess: i32, secret: i32) -> String {
    let distance = (guess - secret).abs();
    let slots = 12;
    let filled = (slots - distance.min(slots)).max(0);
    let hot = "#".repeat(filled as usize);
    let cold = ".".repeat((slots - filled) as usize);
    format!("{}{}", hot, cold)
}

fn main() {
    let secret = 42;
    println!("Guess 10 -> {}", bar_for(10, secret));
    println!("Guess 40 -> {}", bar_for(40, secret));
    println!("Guess 42 -> {}", bar_for(42, secret));
}
```

**Expected output:**

```text
Guess 10 -> ............
Guess 40 -> ##########..
Guess 42 -> ############
```

> **Trust & Use:** `.abs()`, `.repeat()`, and `format!` are standard helpers. Copy as shown; see [Trust & Use](../appendix/TRUST_AND_USE.md).

---

### Step 4 — Full scripted game

**Algorithm:** print secret hint; loop guesses with bars; count tries; celebrate win.

**Types:** `tries` is `i32`; optional win uses `Option` — either a number of tries or nothing yet.

**Language:** a game show host calling out each attempt.

```rust
// Playground
fn bar_for(guess: i32, secret: i32) -> String {
    let distance = (guess - secret).abs();
    let slots = 12;
    let filled = (slots - distance.min(slots)).max(0);
    let hot = "#".repeat(filled as usize);
    let cold = ".".repeat((slots - filled) as usize);
    format!("{}{}", hot, cold)
}

fn main() {
    let secret = 42;
    let guesses = [10, 40, 42];
    let mut tries = 0;
    let mut won: Option<i32> = None;

    println!("Secret is {}", secret);

    for guess in guesses {
        tries += 1;
        if guess == secret {
            println!("Guess {} -> FOUND!", guess);
            won = Some(tries);
            break;
        } else {
            println!("Guess {} -> {} (far)", guess, bar_for(guess, secret));
        }
    }

    match won {
        Some(n) => println!("You win in {} tries!", n),
        None => println!("Out of guesses."),
    }
}
```

**Expected output:**

```text
Secret is 42
Guess 10 -> ............ (far)
Guess 40 -> ##########.. (far)
Guess 42 -> FOUND!
You win in 3 tries!
```

> **Trust & Use:** `Some(tries)` and `None` mean "a value exists" or "no value yet." We explain `Option` in [Trust & Use](../appendix/TRUST_AND_USE.md).

---

## Full chapter solution

Same as Step 4 above.

---

## See also

Next: [Chapter 9 — Tic-Tac-Toe Arena](09_tic_tac_toe_arena.md) — a 3×3 board and win rules.
