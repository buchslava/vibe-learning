# Rust for the Rest of Us

An innovative Rust book for developers who already speak **Java** or **Python** — and want to learn the **Rust paradigm** without wading through filler.

## What makes this book different

1. **Paradigm-first** — ownership, traits, and idioms before frameworks.
2. **Dual lens** — every hard idea is mapped from both Java (GC, interfaces, inheritance) and Python (references, duck typing, GIL).
3. **Playground-friendly** — most examples run on [Rust Playground](https://play.rust-lang.org/) with one click.
4. **Afterparty: AI Lego blocks** — curated prompts at the end of each chapter. Use them with any AI assistant to drill, refactor, and connect ideas *after* you read.

Think of the prompts as LEGO bricks: each one snaps onto what you just learned and builds toward fluency.

## How to read

| Part | Chapters | Focus |
|------|----------|--------|
| **I** | 0–7 | Paradigm, types, ownership, traits |
| **II** | 8–14 | Collections, concurrency, async, macros |
| **III** | 15–16 | I/O, processes, automation capstone |

Start with [CONTENTS.md](CONTENTS.md). After each chapter, use prompts from [appendix/AI_PROMPT_INDEX.md](appendix/AI_PROMPT_INDEX.md). Authors and contributors: see [STYLE_GUIDE.md](STYLE_GUIDE.md).

**Appendices:** [AI Prompt Index](appendix/AI_PROMPT_INDEX.md) · [Playground Guide](appendix/PLAYGROUND_GUIDE.md) · [Java/Python/Rust Map](appendix/JAVA_PYTHON_RUST_MAP.md)

## Repository layout

```
rust-automation-course/
├── README.md           ← you are here
├── CONTENTS.md         ← full table of contents
├── STYLE_GUIDE.md      ← voice, playground rules, prompt format
├── chapters/           ← main text (00–16)
├── appendix/           ← prompt index, playground guide, cheat sheet
└── archive/            ← original course chapters (reference only)
```

## Quick start (local)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo new hello_rust --bin && cd hello_rust && cargo run
```

For playground-only snippets, paste code from any chapter marked **Playground** into [play.rust-lang.org](https://play.rust-lang.org/).

## Further patterns

For hundreds of small, progressive examples (especially functional style), see [Functional Rust](https://hightechmind.io/rust/). This book links curated examples from that site where they reinforce a topic.
