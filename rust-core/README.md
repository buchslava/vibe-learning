# Rust Core

**Part of [Rust Vibe Learning](../README.md)**

A Rust language track for developers **already comfortable programming in another language**. You get the **Rust paradigm** — ownership, traits, concurrency — without framework churn. Examples run on the [Rust Playground](https://play.rust-lang.org/) where possible.

## What makes this book different

1. **Paradigm-first** — ownership, traits, and idioms before application frameworks.
2. **Optional comparison lens** — where helpful, hard ideas are mapped via **Java** and **Python** (GC, interfaces, duck typing, GIL). Know C++, Go, or C# instead? The Rust column still stands on its own.
3. **Playground-friendly** — most examples are one-file **Playground** snippets.
4. **Afterparty** — copy-paste prompts at each chapter end. Use one fresh AI chat per chapter. See [Preface](chapters/preface.md#afterparty-aim-importance-and-how-to-use).

## How to read

| Part | Chapters | Focus |
|------|----------|--------|
| **I** | 1–8 | Paradigm, types, functions, ownership, traits, errors (read [Preface](chapters/preface.md) first) |
| **II** | 9–13 | Modules, smart pointers, collections, closures, std traits |
| **III** | 14–16 | Threads, atomics, async (Tokio) |
| **IV** | 17–18 | Macros, unsafe |
| **V** | 19 | Standard I/O and processes |

Start with [Preface](chapters/preface.md), then [CONTENTS.md](CONTENTS.md). After each chapter, use [appendix/AI_PROMPT_INDEX.md](appendix/AI_PROMPT_INDEX.md). Voice and format: [STYLE_GUIDE.md](STYLE_GUIDE.md).

**Appendices:** [AI Prompt Index](appendix/AI_PROMPT_INDEX.md) · [Playground Guide](appendix/PLAYGROUND_GUIDE.md) · [Java/Python/Rust Map](appendix/JAVA_PYTHON_RUST_MAP.md)

## Repository layout

```
rust-core/
├── README.md           ← you are here
├── CONTENTS.md         ← full table of contents
├── STYLE_GUIDE.md      ← voice, playground rules, prompt format
├── chapters/           ← preface + main text (01–19)
└── appendix/           ← prompt index, playground guide, cheat sheet
```

## Quick start (local)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo new hello_rust --bin && cd hello_rust && cargo run
```

For **Playground** snippets, paste into [play.rust-lang.org](https://play.rust-lang.org/).

## Further patterns

For hundreds of small progressive examples, see [Functional Rust](https://hightechmind.io/rust/). This book links curated examples where they reinforce a topic.
