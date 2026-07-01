# Chapter 10: Snake Trail

## Story hook

The snake slides across a grid — head first, body trailing. In the playground it follows a **fixed tape** of directions, like a flipbook animation. You watch frames print one after another until the snake eats the food.

## What you will build

```text
--- frame 1 ---
#####
#...#
#.@.#
#...#
#####
--- frame 4 ---
#####
#..@#
#.o.#
#..*#
#####
Game over: ate the food!
```

---

### Step 1 — Static grid with head

**Algorithm:** draw bordered grid; place `@` at head position and `*` at food.

**Types:** head and food are `(i32, i32)` — pairs of coordinates.

**Language:** a comic panel with the hero and a star to collect.

```rust
// Playground
fn print_grid(head: (i32, i32), food: (i32, i32), size: i32) {
    let (hx, hy) = head;
    let (fx, fy) = food;
    for y in 0..size {
        for x in 0..size {
            let wall = x == 0 || y == 0 || x == size - 1 || y == size - 1;
            if x == hx && y == hy {
                print!("@");
            } else if x == fx && y == fy {
                print!("*");
            } else if wall {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    print_grid((2, 2), (3, 3), 5);
}
```

**Expected output:**

```text
#####
#...#
#.@*#
#...#
#####
```

---

### Step 2 — One step to the right

**Algorithm:** move head one cell east; print before and after.

**Types:** coordinates are `i32`.

**Language:** one frame of a walking animation.

```rust
// Playground
fn print_grid(head: (i32, i32), food: (i32, i32), size: i32) {
    let (hx, hy) = head;
    let (fx, fy) = food;
    for y in 0..size {
        for x in 0..size {
            let wall = x == 0 || y == 0 || x == size - 1 || y == size - 1;
            if x == hx && y == hy {
                print!("@");
            } else if x == fx && y == fy {
                print!("*");
            } else if wall {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let food = (3, 3);
    let size = 5;
    let mut head = (2, 2);
    println!("--- before ---");
    print_grid(head, food, size);
    head.0 += 1;
    println!("--- after ---");
    print_grid(head, food, size);
}
```

**Expected output:**

```text
--- before ---
#####
#...#
#.@*#
#...#
#####
--- after ---
#####
#...#
#..@*#
#...#
#####
```

---

### Step 3 — Body follows the head

**Algorithm:** store body segments in a `Vec`; each step, push new head, pop tail unless food is eaten.

**Types:** `body` is `Vec<(i32, i32)>` — list of positions, head at front.

**Language:** a train of cars following the engine.

```rust
// Playground
#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn print_grid(body: &[(i32, i32)], food: (i32, i32), size: i32) {
    let (fx, fy) = food;
    for y in 0..size {
        for x in 0..size {
            let wall = x == 0 || y == 0 || x == size - 1 || y == size - 1;
            let is_head = body.first().map(|(hx, hy)| *hx == x && *hy == y) == Some(true);
            let is_body = body.iter().skip(1).any(|(bx, by)| *bx == x && *by == y);
            if is_head {
                print!("@");
            } else if is_body {
                print!("o");
            } else if x == fx && y == fy {
                print!("*");
            } else if wall {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn step(body: &mut Vec<(i32, i32)>, direction: Direction, food: (i32, i32)) -> bool {
    let (hx, hy) = body[0];
    let (nx, ny) = match direction {
        Direction::North => (hx, hy - 1),
        Direction::South => (hx, hy + 1),
        Direction::East => (hx + 1, hy),
        Direction::West => (hx - 1, hy),
    };
    body.insert(0, (nx, ny));
    let ate = (nx, ny) == food;
    if !ate {
        body.pop();
    }
    ate
}

fn main() {
    let size = 5;
    let food = (3, 3);
    let mut body = vec![(1, 2), (1, 3)];
    let dirs = [Direction::East, Direction::East, Direction::North];

    for (i, d) in dirs.iter().enumerate() {
        println!("--- frame {} ---", i + 1);
        let _ = step(&mut body, *d, food);
        print_grid(&body, food, size);
    }
}
```

**Expected output:**

```text
--- frame 1 ---
#####
#...#
#o@.#
#..*#
#####
--- frame 2 ---
#####
#...#
#.o@#
#..*#
#####
--- frame 3 ---
#####
#..@#
#..o#
#..*#
#####
```

> **Trust & Use:** `Copy` and `Clone` on the enum let you reuse directions safely. See [Trust & Use](../appendix/TRUST_AND_USE.md).

---

### Step 4 — Full auto-play until food

**Algorithm:** run a direction tape; each step move head, grow on food, print frame; stop when food is eaten or wall is hit.

**Types:** `Direction` is copyable; body is `Vec<(i32, i32)>`.

**Language:** a short film of the snake reaching the star.

```rust
// Playground
#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn print_grid(body: &[(i32, i32)], food: (i32, i32), size: i32) {
    let (fx, fy) = food;
    for y in 0..size {
        for x in 0..size {
            let wall = x == 0 || y == 0 || x == size - 1 || y == size - 1;
            let is_head = body.first().map(|(hx, hy)| *hx == x && *hy == y) == Some(true);
            let is_body = body.iter().skip(1).any(|(bx, by)| *bx == x && *by == y);
            if is_head {
                print!("@");
            } else if is_body {
                print!("o");
            } else if x == fx && y == fy {
                print!("*");
            } else if wall {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn hit_wall(head: (i32, i32), size: i32) -> bool {
    let (x, y) = head;
    x <= 0 || y <= 0 || x >= size - 1 || y >= size - 1
}

fn main() {
    let size = 5;
    let food = (3, 3);
    let mut body = vec![(1, 3)];
    let tape = [
        Direction::North,
        Direction::East,
        Direction::East,
        Direction::South,
    ];

    for (i, dir) in tape.iter().enumerate() {
        let (hx, hy) = body[0];
        let next = match dir {
            Direction::North => (hx, hy - 1),
            Direction::South => (hx, hy + 1),
            Direction::East => (hx + 1, hy),
            Direction::West => (hx - 1, hy),
        };

        if hit_wall(next, size) {
            println!("--- frame {} ---", i + 1);
            println!("Game over: hit the wall!");
            break;
        }

        body.insert(0, next);
        if next == food {
            println!("--- frame {} ---", i + 1);
            print_grid(&body, (-1, -1), size);
            println!("Game over: ate the food!");
            break;
        }
        body.pop();

        println!("--- frame {} ---", i + 1);
        print_grid(&body, food, size);
    }
}
```

**Expected output:**

```text
--- frame 1 ---
#####
#...#
#@..#
#..*#
#####
--- frame 2 ---
#####
#...#
#.@.#
#..*#
#####
--- frame 3 ---
#####
#...#
#..@#
#..*#
#####
--- frame 4 ---
#####
#...#
#..o#
#..@#
#####
Game over: ate the food!
```

> **Trust & Use:** `#[derive(Clone, Copy, PartialEq)]` on the enum — use as written; see [Trust & Use](../appendix/TRUST_AND_USE.md).

---

## Full chapter solution

Same as Step 4 above.

---

## See also

Next: [Chapter 11 — Live on Your Machine](11_live_on_your_machine.md) — install Rust and play Snake with the keyboard.
