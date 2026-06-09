# Preface

## Who these notes are for

You already program in at least one language. You do not need a tutorial that starts with `for` loops. You need a **paradigm map**: how Rust thinks about memory, types, concurrency, and errors — and how to write **idiomatic** Rust instead of transliterating syntax from whatever you know today.

Backend developers, CLI authors, and curious systems programmers are all welcome. **Rust Core** is the language-fundamentals track inside **[Vibe Learning](../README.md)**. Part V applies ideas to std I/O; Parts I–IV stand on their own.

Where tables compare **Java** and **Python**, treat them as **optional anchors** — handy if you know those languages, not required to follow the notes.

## Why Rust is a different bet

Most languages make you pick a poison you already know: stay close to the metal and spend years chasing memory bugs, or buy safety from a **GC** or a **VM** and pay latency, pauses, and opaque runtime behavior. Rust breaks that menu. **Ownership** and **borrow checking** reject huge classes of use-after-free, data race, and aliasing mistakes at **compile time** — with no garbage collector, no mandatory runtime, and no surrender of layout, syscalls, or `no_std` control. That is not a faster C or a stricter Java. It is a different contract between you, the compiler, and the machine.

The contract scales across surfaces that used to demand different stacks. The same language can run on an **ARM64** cloud instance, serve HTTP with **Axum**, embed as a **WebAssembly** module in the browser or beside **Node** via **N-API** / **FFI**, ship a desktop shell with **Tauri**, and still compile firmware for **embedded** targets or a daemon that talks to hardware. You are not maintaining four mental models for four deployment shapes — you are reusing types, tests, and habits.

At runtime Rust stays **lean**: release builds are native **machine code**, not bytecode behind another interpreter. At compile time the language is **expressive**: **algebraic enums**, **traits** instead of inheritance tax, **generics** and **iterators** that optimize like hand-written loops, **macros** when the type system needs a lever. Small binary and rich syntax are not opposites here — zero-cost abstractions and monomorphization are the point.

The industry vote is already in the tree, not in slide decks. **Firefox** shipped memory-safe components from the Servo line; **Windows** and **Android** move security-critical paths to Rust; Chromium hardens parsers at the Rust boundary. In the **Linux kernel**, Rust entered in **6.1** (2022); at the **2025 Kernel Maintainers Summit** it graduated from *experimental* to a **core kernel language** alongside C and assembly — with production drivers and subsystems on the path maintainers expect to keep. You are not learning a hype cycle. You are learning the language those merges already assume.

That is why these notes exist: not to sell Rust in the abstract, but to hand you the paradigm map before the compiler enforces it.

## Production standards — what compile time does not teach

Most Rust material stops when the program builds. Production does not. Reviewers ask whether your error types belong in a public API, whether that `.clone()` hides a borrow smell, whether two `u8` parameters can be swapped without the compiler noticing, and whether a workspace crate still shares dependencies correctly after the last refactor. Those questions rarely appear in language tutorials. They surface in merge requests, on-call threads, and code you maintain for years under enterprise SLAs.

**[Chapter 20: Production Rust standards](20_production_standards.md)** closes that gap. It is a review checklist — typed boundaries, panic-free paths, deliberate allocation, workspace and test conventions — distilled from my own work shipping Rust in **production and enterprise** teams. The patterns there are not generic best-practice quotes recycled from docs. They are habits I apply after every diff: anti-patterns that compiled cleanly but failed in staging, conventions that kept multi-crate repos reviewable, and the bar I use before calling code merge-ready.

Parts I–IV give you the paradigm map. Chapter 20 gives you the production bar. Read it when you are ready to write Rust that survives code review, not just `cargo check`.

## What you will not find here

- A line-by-line clone of [*The Rust Book*](https://doc.rust-lang.org/book/) (excellent; use it as reference)
- Framework churn (Axum vs Actix debates)
- Long proofs about memory models
- Long prosaic-style text (density over narrative padding)

## What these notes are (and are not)

These are **lecture notes**, not a textbook. Treat each chapter like dense slides from a course: a map of ideas, runnable examples, and pointers for self-study — not exhaustive reference pages you read cover to cover once and shelve.

**These notes are:**

- paradigm maps for programmers coming from any mainstream language
- optional **Java** / **Python** comparison tables where they clarify a habit
- **Playground** snippets you can run, break, and fix
- edge-case drills through compiler errors — Rust is about restrictions; fluency is understanding the compiler's logic, not just syntax
- hooks for drill and review (Afterparty prompts, crosslinks, optional deep dives)

**These notes are not:**

- a hand-holding tutorial that explains every keyword in order
- a substitute for the official book or compiler documentation

AI drafts code; you still read, review, and steer. Success depends on three skills:

- you **understand** what the machine is doing
- you **feel** when a design is wrong
- your **technological thinking** is sharp enough to steer the result

These notes are written for that workflow.

## Try it in the Rust Playground

Most examples are tagged **Playground** — paste them into the online editor and click **Run**:

**[https://play.rust-lang.org/](https://play.rust-lang.org/)**

Tweak values, remove a semicolon, borrow after a move. The compiler errors are part of the lesson. You do not need a local install to start Part I.

| Use the playground | Use Cargo locally |
|--------------------|-------------------|
| Syntax, ownership, types | Files, `Command`, serial ports |
| Quick experiments (< 50 lines) | External crates (`tokio`, `serialport`) |
| Sharing a link in chat or notes | Integration tests, multi-file crates |

Rules for playground snippets: one file with `fn main()`, **`std` only**, no filesystem or subprocess APIs. Full details: [Playground Guide](../appendix/PLAYGROUND_GUIDE.md).

## How to work through a chapter

1. **Read once for the map** — skim headings and tables; do not memorize every line.
2. **Run the Playground example** — confirm output, then break it on purpose.
3. **Prompt what is still fuzzy** — open a **new chat for this chapter**, paste the [starter context](#how-to-use), then a micro-prompt or Afterparty drill.
4. **Move on** — revisit later via [AI Prompt Index](../appendix/AI_PROMPT_INDEX.md) IDs (**P001** onward).

## Afterparty: aim, importance, and how to use

At the end of every chapter you will find **Afterparty** — numbered, copy-paste prompts for an AI tutor.

These notes give you a **map**, not an encyclopedia. No book — not even a full reference — can hold everything Rust can do or everything you might need in production. The space of possible knowledge is larger than any single track. Memorizing it all is neither required nor realistic.

**Afterparty** is how you choose your depth. Each prompt is a Lego block for **practice**: one angle on the chapter — a quiz, an error to decode, a comparison, a mini-design. Paste it, push back, verify in the playground, stop when it clicks — or keep going until it sticks. **You** set the pace; the book does not pretend to exhaust the topic.

Compare that to **Skills Lego blocks** — capability you assemble over time outside any one chapter: reading a crate, wrapping FFI safely, designing an error enum for a gateway, tuning Tokio under load. Those skills are **optional extensions**. They stack from prompts, side projects, and real code — not from reading cover to cover. Afterparty starts the stack; Skills blocks are what you add when a job or curiosity demands more.

This track ships **prompts**, not a fixed skill tree. That is intentional: the capacity available to you (AI tutor, docs, crates, production code) is greater than what fits in these pages. Prompts are the interface. Depth is your decision.

### Aim

Afterparty drills the same concepts from different angles: quizzes, error decoding, comparisons, mini-design exercises. Each prompt is a complete sentence you paste into a chat — no setup required.

### Importance

Afterparty is not a shortcut around learning Rust. It builds the skill that matters in an AI-assisted workflow:

- **interrogate** generated answers instead of accepting them
- **catch** ownership and type mistakes before they ship
- **stress-test** mental models that prompts alone cannot replace

Use the model as a sparring partner. You stay responsible for correctness and idiomatic style.

### How to use

**One chapter, one chat.** Start a fresh conversation for each chapter. Keep Afterparty prompts in that thread; open a new chat when you move on — old ownership context will confuse async topics.

**Session flow** (after [reading and running examples](#how-to-work-through-a-chapter)):

1. Paste the **starter context** below once.
2. Paste **one** Afterparty prompt; verify any code before the next.
3. Push back on vague answers; ask for exact compiler errors.
4. Repeat step 2 in the same chat until done.

**Starter context — paste once at the top of each chapter chat:**

```text
You are my Rust tutor. I am working through Rust Core.
I already program in: [your language(s) — e.g. Python, C++, Go].

Answer rules:
- Rust edition 2021, stable toolchain unless I ask otherwise
- Prefer single-file Playground snippets (fn main, std only) unless I say Cargo
- Quote compiler errors literally; explain ownership/borrow moves step by step
- Do not invent crates or APIs; if unsure, say so
- I will verify your code — keep snippets short and runnable

Current chapter: [number and title — e.g. Chapter 1 Paradigm shift]
Topics I just read: [one line from the chapter hook, or paste a section heading]
```

**When to start a new chat instead of continuing:**

| Situation | Action |
|-----------|--------|
| Finished the chapter’s Afterparty | New chat for the next chapter |
| Model repeats wrong advice after you corrected it twice | New chat + shorter starter context |
| You jumped to a unrelated topic (e.g. Tokio while still on lifetimes) | New chat scoped to one chapter |
| Context is huge and answers feel generic | New chat; paste only the section you are stuck on |

**What good answers look like:** concrete Rust code, named compiler errors (`E0382`, `E0502`), and “this fails because …” — not hand-wavy “Rust is strict about memory.” **Your job** is to run the snippet and say “that did not compile — here is the error” until the explanation matches reality.

### Where to find prompts

Each chapter ends with its own block. All prompts are numbered globally in [appendix/AI_PROMPT_INDEX.md](../appendix/AI_PROMPT_INDEX.md) (**P001** onward) so you can reuse them later without rereading the chapter.

## When something is unclear, prompt it

You do **not** need to wait for Afterparty. Any paragraph, table row, compiler error, or half-understood example is fair game — especially the parts that feel unclear after a first read.

**Micro-prompt recipe:**

1. **Quote** the passage, snippet, or error text.
2. **State** what you expected to happen.
3. **Ask** for a concrete answer: the exact compiler message, one sentence with an analogy from a language you know (e.g. Java or Python), or a fixed 5-line snippet.

Example you can paste today:

> “In [Chapter 2](02_types.md), the notes say to prefer `&str` in parameters and `String` when storing. I am building a config parser. Explain why that split matters — give one function signature for `parse_line` and show what breaks if I use `String` everywhere.”

You remain the **reader and reviewer**. The model drafts explanations; you verify against the playground or `cargo check`. That habit turns lecture notes into understanding faster than rereading the same paragraph five times.

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

> **One binary, one manifest.** Commit `Cargo.lock` for applications (CLIs, services); library crates often omit it from version control.

## Conventions

How the notes are formatted — not Rust syntax itself.

| Convention | Meaning |
|------------|---------|
| **Playground** / **Cargo only** | Every code block is tagged. **Playground** = one `fn main()`, `std` only, [play.rust-lang.org](https://play.rust-lang.org/). **Cargo only** = needs `Cargo.toml`, filesystem, subprocess, or external crates. Details: [STYLE_GUIDE.md](../STYLE_GUIDE.md), [Playground Guide](../appendix/PLAYGROUND_GUIDE.md). |
| **Chapter shape** | Hook → sections with examples → **Idiom spotlight** → **Go deeper** → **See also** → **Afterparty**. |
| **Levels** | Many chapters label examples **Level 1 … N**. Run the snippet, then read **What happened** before the next level. |
| **Crosslinks** | Relative paths (`08_errors_and_testing.md`). Chapter numbers match [CONTENTS.md](../CONTENTS.md). |
| **Java / Python columns** | Optional in tables — skip if you do not need the analogy. |
| **`unwrap()` in examples** | Avoided in teaching snippets unless the section is about panic or `Result` ([Chapter 8](08_errors_and_testing.md)). |
| **Prompt IDs** | Afterparty lines are catalogued as **P001+** in [AI Prompt Index](../appendix/AI_PROMPT_INDEX.md) for reuse across chats. |
| **Edition** | Rust **2021**, stable toolchain, unless a note says otherwise. |

## Reading ahead

Some syntax appears in examples before its dedicated chapter:

- **`?`** — “If this fails, return the error to the caller.” Explained fully in [Chapter 8: Errors and testing](08_errors_and_testing.md). Until then, read it as a placeholder, not something you must master on day one.

## See also

- [CONTENTS.md](../CONTENTS.md) — full map
- [Playground Guide](../appendix/PLAYGROUND_GUIDE.md) — rules and stand-ins
- [AI Prompt Index](../appendix/AI_PROMPT_INDEX.md) — all Afterparty prompts (**P001+**)
- [Java/Python/Rust map](../appendix/JAVA_PYTHON_RUST_MAP.md) — optional one-page cheat sheet
- [Chapter 1: Paradigm shift](01_paradigm_shift.md) — start here after this page
