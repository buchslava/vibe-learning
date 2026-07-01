# Playground Guide

## When to use the playground

| Use playground | Use your machine (Chapter 11+) |
|----------------|--------------------------------|
| Chapters 1–10 | Keyboard Snake, saving files |
| Learning syntax step by step | External crates (`crossterm`) |
| Sharing a link with a friend | Projects with many files |

**Playground:** [https://play.rust-lang.org/](https://play.rust-lang.org/)

## How to run a snippet

1. Open the playground link above.
2. Select all default code and delete it.
3. Paste the snippet from the book (include `fn main()`).
4. Click **Run** (or press the run shortcut shown in the UI).
5. Read output in the panel on the right.

## Rules (matches [STYLE_GUIDE.md](../STYLE_GUIDE.md))

1. **One file** with `fn main()` — snippets marked **Playground** are complete programs.
2. **`std` only** — the playground cannot add dependencies from `Cargo.toml`.
3. **No keyboard input** — games use a preset list of moves. Imagine you pressed the keys; the program follows the script.
4. **Output via `println!`** — pictures are made from text characters.

## Scripted input stand-in

When a game would read your keyboard, the book uses arrays like:

```rust
let choices = ["left", "right", "right"];
let moves = [Direction::Right, Direction::Down, Direction::Down];
```

The story tells you what each entry means. On your machine (Chapter 11), you replace the script with real keys.

## Sharing code

1. Paste your working program into the editor.
2. Click **Share** to get a permalink.
3. Bookmark it if you want to return later.

## Edition and channel

Book examples target **Edition 2021**, **stable** — same as the playground default.

## When something fails

| Problem | What to try |
|---------|-------------|
| Red error text | Compare your code to the book character by character |
| No output | Make sure `fn main()` calls `println!` |
| "cannot find" | You may have deleted part of a Trust & Use line — copy it back |

For local projects after Chapter 11:

```bash
cd your_project
cargo run
```

## See also

- [Trust & Use](TRUST_AND_USE.md) — syntax you copied without full explanation
- [Chapter 11](../chapters/11_live_on_your_machine.md) — install Rust and run Snake locally
