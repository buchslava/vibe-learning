# Playground Guide

## When to use the playground

| Use playground | Use Cargo locally |
|----------------|-----------------|
| Learning syntax, ownership, types | Files, `Command`, serial ports |
| Quick experiments (< 50 lines) | External crates (`tokio`, `serialport`) |
| Sharing a link in chat or docs | Integration tests, multi-file crates |

**Playground:** [https://play.rust-lang.org/](https://play.rust-lang.org/)

## Rules (matches [STYLE_GUIDE.md](../STYLE_GUIDE.md))

1. **One file** with `fn main()` — no `mod` tree in book snippets marked **Playground**.
2. **`std` only** — playground cannot add `Cargo.toml` dependencies.
3. **No filesystem** — use `Cursor<&[u8]>`, `Vec<u8>`, or `&str` instead of `File::open`.
4. **No subprocess** — simulate with strings or skip to Cargo lab.
5. **Output via `println!`** — playground has no CLI args; hardcode sample input.

## Sharing code

1. Paste the snippet into the editor.
2. Click **Share** to get a permalink.
3. In your notes, store: `Playground: <url>` next to the chapter example.

## Edition and channel

Book examples target **Edition 2021**, **stable** channel — same as playground default.

## Playground stand-ins

| Real API | Playground substitute |
|----------|----------------------|
| `File::open` | `Cursor::new(bytes)` or `include_str!` in local Cargo only |
| `Command::output` | Print mock stdout string |
| `tokio::spawn` | Describe in comment; run Cargo lab |
| Serial port | Encode/decode bytes in memory |

## Local run for Cargo labs

```bash
cd your_project
cargo run
cargo test
cargo run --example name   # if using examples/
```

## Troubleshooting

| Problem | Fix |
|---------|-----|
| `cannot find crate` | You need a Cargo lab — add dependency to `Cargo.toml` |
| Playground timeout | Infinite loop or huge print; reduce data |
| Different output on Mac/Linux | Line endings or `echo` vs `cmd` — expected for process labs |

## See also

- [CONTENTS.md](../CONTENTS.md)
- [AI Prompt Index](AI_PROMPT_INDEX.md)
