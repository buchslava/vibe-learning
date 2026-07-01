# Chapter 11: Live on Your Machine

## Story hook

You watched Snake follow a script in the playground. Now you install Rust on your computer and **drive** with the arrow keys. Same grid, same rules — real control.

## What you will build

A terminal Snake game: arrow keys move the head, `q` quits, `@` is the head, `o` is the body, `*` is food.

**Cargo only** — this chapter needs files on disk and the `crossterm` crate.

---

### Step 1 — Install Rust

**Algorithm:** download the installer, run it, open a new terminal, verify versions.

**Types:** no Rust code yet — shell commands only.

**Language:** setting up a workshop before building furniture.

Visit [https://rustup.rs](https://rustup.rs) and follow the instructions for your system. On macOS or Linux, the one-line installer is:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When it finishes, **close and reopen** your terminal, then verify:

```bash
rustc --version
cargo --version
```

**Expected output (versions will differ):**

```text
rustc 1.87.0 (...)
cargo 1.87.0 (...)
```

If both commands print a version, you are ready.

---

### Step 2 — Hello from Cargo

**Algorithm:** create a project folder, build, run the default program.

**Types:** Cargo manages the project; `main.rs` holds `fn main()`.

**Language:** naming a notebook and writing the first page.

```bash
cargo new snake_live
cd snake_live
cargo run
```

Open `src/main.rs` — it already contains a hello program. **Expected output:**

```text
   Compiling snake_live v0.1.0 (...)
    Finished `dev` profile target(s) in ...
     Running `target/debug/snake_live`
Hello, world!
```

---

### Step 3 — Add crossterm and draw the grid

**Algorithm:** add dependency; clear screen; print a bordered grid each frame.

**Types:** `Grid` holds size; coordinates stay `(i32, i32)`.

**Language:** preparing the stage before the actors enter.

Edit `Cargo.toml` — add under `[dependencies]`:

```toml
crossterm = "0.28"
```

Replace `src/main.rs` with:

```rust
// Cargo only
use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

fn print_grid(head: (i32, i32), food: (i32, i32), size: i32) -> io::Result<()> {
    let (hx, hy) = head;
    let (fx, fy) = food;
    for y in 0..size {
        for x in 0..size {
            let wall = x == 0 || y == 0 || x == size - 1 || y == size - 1;
            let ch = if x == hx && y == hy {
                '@'
            } else if x == fx && y == fy {
                '*'
            } else if wall {
                '#'
            } else {
                '.'
            };
            print!("{}", ch);
        }
        println!();
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Hide)?;
    execute!(stdout, Clear(ClearType::All))?;
    print_grid((2, 2), (3, 3), 8)?;
    execute!(stdout, Show)?;
    Ok(())
}
```

Run:

```bash
cargo run
```

**Expected output:** an 8×8 grid in the terminal with `@` near the center and `*` on the floor.

> **Trust & Use:** `?` after a fallible operation means "stop and report error if this failed." See [Trust & Use](../appendix/TRUST_AND_USE.md).

---

### Step 4 — Playable Snake

**Algorithm:** game loop — read key, update snake, detect food and walls, redraw until quit or game over.

**Types:** `body: Vec<(i32, i32)>` — same model as Chapter 10.

**Language:** the flipbook from Chapter 10, but you turn the pages with arrow keys.

Replace `src/main.rs` with the full game:

```rust
// Cargo only
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn print_grid(body: &[(i32, i32)], food: (i32, i32), size: i32) -> io::Result<()> {
    let (fx, fy) = food;
    for y in 0..size {
        for x in 0..size {
            let wall = x == 0 || y == 0 || x == size - 1 || y == size - 1;
            let is_head = body.first().map(|(hx, hy)| *hx == x && *hy == y) == Some(true);
            let is_body = body.iter().skip(1).any(|(bx, by)| *bx == x && *by == y);
            let ch = if is_head {
                '@'
            } else if is_body {
                'o'
            } else if x == fx && y == fy {
                '*'
            } else if wall {
                '#'
            } else {
                '.'
            };
            print!("{}", ch);
        }
        println!();
    }
    Ok(())
}

fn hit_wall(head: (i32, i32), size: i32) -> bool {
    let (x, y) = head;
    x <= 0 || y <= 0 || x >= size - 1 || y >= size - 1
}

fn hit_self(body: &[(i32, i32)]) -> bool {
    let (hx, hy) = body[0];
    body.iter().skip(1).any(|(x, y)| *x == hx && *y == hy)
}

fn main() -> io::Result<()> {
    let size = 12;
    let mut body = vec![(size / 2, size / 2)];
    let mut food = (size - 2, size / 2);
    let mut dir = Direction::East;
    let tick = Duration::from_millis(180);

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let mut last_tick = Instant::now();
    let mut game_over = false;
    let mut message = "Arrow keys move. Q quits.".to_string();

    loop {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Up => dir = Direction::North,
                        KeyCode::Down => dir = Direction::South,
                        KeyCode::Left => dir = Direction::West,
                        KeyCode::Right => dir = Direction::East,
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick && !game_over {
            let (hx, hy) = body[0];
            let next = match dir {
                Direction::North => (hx, hy - 1),
                Direction::South => (hx, hy + 1),
                Direction::East => (hx + 1, hy),
                Direction::West => (hx - 1, hy),
            };

            if hit_wall(next, size) || hit_self(&body) {
                game_over = true;
                message = "Game over! Press Q.".to_string();
            } else {
                body.insert(0, next);
                if next == food {
                    food = (size - 2, (food.1 + 3) % (size - 2) + 1);
                    message = "Yum! Keep going.".to_string();
                } else {
                    body.pop();
                }
            }
            last_tick = Instant::now();
        }

        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;
        print_grid(&body, food, size)?;
        println!();
        println!("{}", message);
        stdout.flush()?;
    }

    execute!(stdout, Show, LeaveAlternateScreen)?;
    Ok(())
}
```

Run:

```bash
cargo run
```

**Expected behavior:**

- A 12×12 grid appears.
- Arrow keys change direction; the snake moves every ~180 ms.
- Eating `*` grows the snake and moves food.
- Hitting `#` or your own body shows **Game over! Press Q.**
- **Q** returns to the normal terminal.

Compare this file to [Chapter 10](10_snake_trail.md) — the grid drawing and body logic are the same ideas with keyboard and timing added.

---

## Full chapter solution

Step 4 source is the complete game. Keep the project:

```bash
cd snake_live
cargo run
```

---

## See also

- [Trust & Use](../appendix/TRUST_AND_USE.md) — macros, `?`, and `derive` explained
- [Rust Core — Preface](../../rust-core/chapters/preface.md) — the next book when you are ready for ownership and traits
