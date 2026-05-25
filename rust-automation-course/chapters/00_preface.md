# Chapter 0: Preface

## Who this book is for

You already write **Java** or **Python** (or both). You do not need a language tutorial that starts with `for` loops. You need a **paradigm map**: how Rust thinks about memory, types, concurrency, and errors — and how to write **idiomatic** Rust instead of “Java with different syntax.”

Automation engineers, backend developers, and curious systems programmers are all welcome. Part III applies the ideas to I/O and hardware-flavoured problems; Parts I–II stand on their own.

## What you will not find here

- A line-by-line clone of *The Rust Book* (excellent; use it as reference)
- Framework churn (Axum vs Actix debates)
- Long proofs about memory models

## What you will find — and why it looks this way

AI is a genuine shift in how we work with code. Classical “programming” — sitting alone and typing every line from a blank file — is no longer the whole job. **Software engineering** is what remains, and in many ways it is stronger: you read implementations, review them critically, shape plans, and turn ideas into precise prompts that something else drafts first. Ex-programmers, in that sense, are now **code readers**, **reviewers**, and **idea implementers**. Success depends less on keystrokes and more on whether you **understand** what the machine is doing, whether you can **feel** when a design is wrong, and whether your **technological thinking** is sharp enough to steer the result.

This book is written for that reality. Each chapter stays close to **fundamentals** — no long academic tours — and gives you **examples to read** (playground snippets you can run and stare at) rather than encyclopedic coverage. At the end of every chapter you will find **Afterparty**: copy-paste prompts for an AI tutor, designed to drill the same concepts from different angles.

Afterparty is not a shortcut around learning Rust. It is practice for the skill you need most now: **interrogate** generated answers, **catch** ownership and type mistakes before they ship, and **build** mental models that prompts alone cannot replace. Use the model as a sparring partner; you stay responsible for correctness and idiomatic style. That combination — tight fundamentals, readable examples, and deliberate AI-assisted review — is why this book exists and why Afterparty is part of its spine, not an appendix gimmick.

Prompts are numbered globally in [appendix/AI_PROMPT_INDEX.md](../appendix/AI_PROMPT_INDEX.md) (200 prompts, **P001–P200**).

Quick references: [Playground Guide](../appendix/PLAYGROUND_GUIDE.md) · [Java/Python/Rust map](../appendix/JAVA_PYTHON_RUST_MAP.md)

## Toolchain: rustup and Cargo

Install Rust with **rustup** (toolchain manager) and build with **Cargo** (package manager, build system, test runner):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo new my_app --bin
cd my_app && cargo run
```

| Concept | In Rust |
|---------|---------|
| Package manager | **Cargo** — deps, build, test, run |
| Manifest | `Cargo.toml` — name, version, edition, dependencies |
| Lockfile | `Cargo.lock` — pinned versions for reproducible builds |
| Entry point | `fn main()` in `src/main.rs` |
| Crate registry | [crates.io](https://crates.io) |

Every binary crate follows the same shape: one `Cargo.toml`, one `src/main.rs`, commands through `cargo run` and `cargo test`. Prefer that workflow over invoking `rustc` directly.

`env!("CARGO_PKG_NAME")` embeds a **compile-time** string from `Cargo.toml` into your binary — not the same as reading OS environment variables at runtime (`std::env::var`).

**Cargo only** — first project locally:

```toml
# Cargo.toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"
```

```rust
// src/main.rs
fn main() {
    println!("Hello from {}", env!("CARGO_PKG_NAME"));
}
```

> **One binary, one manifest.** Commit `Cargo.lock` for applications (automation tools, CLIs); library crates often omit it from version control.

## Conventions

- **Playground** / **Cargo only** tags on every code block — see [STYLE_GUIDE.md](../STYLE_GUIDE.md).
- `?` in snippets means “error propagation”; explained fully in [Chapter 7](07_errors_and_testing.md).

## See also

- [CONTENTS.md](../CONTENTS.md) — full map
- [Chapter 1: Paradigm shift](01_paradigm_shift.md) — start here after this page
