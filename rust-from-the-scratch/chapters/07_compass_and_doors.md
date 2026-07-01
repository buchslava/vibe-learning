# Chapter 7: Compass and Doors

## Story hook

You stand on a tiny dungeon map. Walls block the way; doors open when you walk over them. Four compass directions drive each step — we script six moves and print the map after every turn.

## What you will build

```text
--- move 1 ---
.....
.@...
.....
.....
.....
--- move 2 ---
.....
..@..
.....
.....
.....
...
```

---

### Step 1 — Name the four directions

**Algorithm:** define an enum with four variants; store one; print which way it is.

**Types:** `Direction` is an **enum** — a closed list of allowed values.

**Language:** compass points: north, south, east, west — only those words are valid.

```rust
// Playground
#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let step = Direction::East;
    match step {
        Direction::North => println!("Facing north."),
        Direction::South => println!("Facing south."),
        Direction::East => println!("Facing east."),
        Direction::West => println!("Facing west."),
    }
}
```

**Expected output:**

```text
Facing east.
```

`match` picks the branch that fits the value — like a multiple-choice question with exact answers.

---

### Step 2 — Turn direction into movement

**Algorithm:** given a direction, compute how `(x, y)` changes.

**Types:** position is `(i32, i32)` — a pair of integers; deltas are `(i32, i32)`.

**Language:** "east" means one step to the right on the grid.

```rust
// Playground
#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn delta(d: Direction) -> (i32, i32) {
    match d {
        Direction::North => (0, -1),
        Direction::South => (0, 1),
        Direction::East => (1, 0),
        Direction::West => (-1, 0),
    }
}

fn main() {
    let (dx, dy) = delta(Direction::East);
    println!("Move by ({}, {})", dx, dy);
}
```

**Expected output:**

```text
Move by (1, 0)
```

---

### Step 3 — Print the map once

**Algorithm:** build a 5×5 floor with walls on the border; place `@` at player position; print rows.

**Types:** `player` is `(i32, i32)`; each cell is `char`.

**Language:** a bird's-eye sketch on graph paper.

```rust
// Playground
fn print_map(player: (i32, i32), size: i32) {
    let (px, py) = player;
    for y in 0..size {
        for x in 0..size {
            let on_edge = x == 0 || y == 0 || x == size - 1 || y == size - 1;
            if x == px && y == py {
                print!("@");
            } else if on_edge {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    print_map((2, 2), 5);
}
```

**Expected output:**

```text
#####
#...#
#.@.#
#...#
#####
```

---

### Step 4 — Scripted walk with six moves

**Algorithm:** start at center; for each direction in the script, add delta, clamp inside inner floor, print map.

**Types:** `moves` is an array of `Direction`; position stays `(i32, i32)`.

**Language:** watching a chess piece slide across the board move by move.

```rust
// Playground
#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn delta(d: Direction) -> (i32, i32) {
    match d {
        Direction::North => (0, -1),
        Direction::South => (0, 1),
        Direction::East => (1, 0),
        Direction::West => (-1, 0),
    }
}

fn print_map(player: (i32, i32), size: i32) {
    let (px, py) = player;
    for y in 0..size {
        for x in 0..size {
            let on_edge = x == 0 || y == 0 || x == size - 1 || y == size - 1;
            if x == px && y == py {
                print!("@");
            } else if on_edge {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let size = 5;
    let mut pos = (2, 2);
    let moves = [
        Direction::East,
        Direction::North,
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    for (i, direction) in moves.iter().enumerate() {
        println!("--- move {} ---", i + 1);
        let (dx, dy) = delta(*direction);
        let nx = (pos.0 + dx).clamp(1, size - 2);
        let ny = (pos.1 + dy).clamp(1, size - 2);
        pos = (nx, ny);
        print_map(pos, size);
    }
}
```

**Expected output:**

```text
--- move 1 ---
#####
#...#
#..@.#
#...#
#####
--- move 2 ---
#####
#.@.#
#...#
#...#
#####
--- move 3 ---
#####
#.@.#
#...#
#...#
#####
--- move 4 ---
#####
#...#
#.@.#
#...#
#####
--- move 5 ---
#####
#...#
#.@.#
#...#
#####
--- move 6 ---
#####
#...#
#..@.#
#...#
#####
```

`clamp` keeps the player off the wall cells. `.iter().enumerate()` gives move number and direction.

> **Trust & Use:** `#[derive(Clone, Copy)]` on `Direction` — see [Trust & Use](../appendix/TRUST_AND_USE.md).

---

## Full chapter solution

Same as Step 4 above.

---

## See also

Next: [Chapter 8 — Guess the Secret](08_guess_the_secret.md) — loops and hot/cold bars.
