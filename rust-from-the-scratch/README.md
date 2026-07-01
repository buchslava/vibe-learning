# Rust from the Scratch

**Part of [Vibe Learning](../README.md)**

A beginner book for people who have **never programmed before** — including teenagers. You learn Rust through short story-chapters, each building a small game or scene. Every step compiles and shows a result. No install required until Chapter 11.

## What makes this book different

1. **Trust & Use** — when Rust syntax looks heavy, you use it as written and move on. Full explanations wait in [Trust & Use](appendix/TRUST_AND_USE.md).
2. **Three lenses** — every idea is shown as an algorithm (steps), as types (what each name holds), and as human language (nouns, choices, lists).
3. **Novel chapters** — each chapter is a mini-project with a story hook, not a lecture.
4. **Playground-first** — Chapters 1–10 run in the browser at [play.rust-lang.org](https://play.rust-lang.org/). Chapter 11 moves Snake to your machine.

## How to read

| Part | Chapters | Focus |
|------|----------|--------|
| **I** | Preface + 1–2 | First words, loops, ASCII art |
| **II** | 3–6 | Choices, lists, functions, structs |
| **III** | 7–10 | Maps, guessing, tic-tac-toe, snake |
| **IV** | 11 | Install Rust, keyboard Snake |

Start with [Preface](chapters/preface.md), then [CONTENTS.md](CONTENTS.md). Voice and format: [STYLE_GUIDE.md](STYLE_GUIDE.md).

**Appendices:** [Playground Guide](appendix/PLAYGROUND_GUIDE.md) · [Trust & Use](appendix/TRUST_AND_USE.md)

When you finish, continue with **[Rust Core](../rust-core/)** for ownership, traits, and concurrency.

## Repository layout

```
rust-from-the-scratch/
├── README.md           ← you are here
├── CONTENTS.md         ← full table of contents
├── STYLE_GUIDE.md      ← voice, step format, Trust & Use boxes
├── chapters/           ← preface + main text (01–11)
├── appendix/           ← playground guide, Trust & Use revealed
├── kindle/             ← PDF build (see kindle/README.md)
└── dist/               ← generated Rust-from-the-Scratch-Kindle.pdf
```

## Quick start (no install)

Open [play.rust-lang.org](https://play.rust-lang.org/), paste the first snippet from [Chapter 1](chapters/01_first_words.md), and click **Run**.

Local setup waits until [Chapter 11](chapters/11_live_on_your_machine.md).

## Kindle PDF

```bash
python3 kindle/build.py
```

Output: **`dist/Rust-from-the-Scratch-Kindle.pdf`**. See [kindle/README.md](kindle/README.md) for requirements and upload tips.
