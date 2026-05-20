# Chapter 0: Preface

## Who this book is for

You already write **Java** or **Python** (or both). You do not need a language tutorial that starts with `for` loops. You need a **paradigm map**: how Rust thinks about memory, types, concurrency, and errors — and how to write **idiomatic** Rust instead of “Java with different syntax.”

Automation engineers, backend developers, and curious systems programmers are all welcome. Part III applies the ideas to I/O and hardware-flavoured problems; Parts I–II stand on their own.

## What you will not find here

- A line-by-line clone of *The Rust Book* (excellent; use it as reference)
- Framework churn (Axum vs Actix debates)
- Long proofs about memory models

What you *will* find: tight chapters, playground snippets, and **Afterparty** prompts — copy-paste questions for an AI tutor that reinforce each section.

## How to use the Afterparty prompts

After each chapter, open your favourite AI assistant and paste one prompt at a time. Treat the model as a **sparring partner**:

1. Read the chapter (20–40 minutes).
2. Run the **Playground** example yourself.
3. Do 2–3 Afterparty prompts — insist on compiler-accurate answers.
4. Optionally follow **Go deeper** links on [Functional Rust](https://hightechmind.io/rust/).

Prompts are numbered globally in [appendix/AI_PROMPT_INDEX.md](../appendix/AI_PROMPT_INDEX.md) (101 prompts, **P001–P101**).

Quick references: [Playground Guide](../appendix/PLAYGROUND_GUIDE.md) · [Java/Python/Rust map](../appendix/JAVA_PYTHON_RUST_MAP.md)

## Suggested pace

| Part | Chapters | Rough time |
|------|----------|------------|
| I | 0–7 | 12–18 h |
| II | 8–14 | 14–20 h |
| III | 15–16 | 6–10 h |

Adjust for depth; concurrency chapters reward repetition.

## Conventions

- **Playground** / **Cargo only** tags on every code block — see [STYLE_GUIDE.md](../STYLE_GUIDE.md).
- `?` in snippets means “error propagation”; explained fully in [Chapter 7](07_errors_and_testing.md).

## See also

- [CONTENTS.md](../CONTENTS.md) — full map
- [Chapter 1: Paradigm shift](01_paradigm_shift.md) — start here after this page

### Afterparty: AI Lego blocks

1. **Learning plan** — “I know Java and Python. Based on this preface, build me a 2-week study plan using only this book’s chapter list; 45 minutes per day.”
2. **Gap check** — “Ask me five quick questions to see if I should skip straight to Chapter 3 (ownership) or read Chapters 1–2 first.”
3. **Prompt practice** — “Give me one sample Afterparty-style question about ownership; I will answer; you grade like a Rust teacher.”
4. **Motivation map** — “List three real projects (automation, CLI, service) where Rust’s ownership model wins over GC; no hype.”
5. **Glossary seed** — “Define in one sentence each: ownership, borrow, trait, async, atomics — I will refine after reading Part I.”
