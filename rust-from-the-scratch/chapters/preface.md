# Preface — Rust from the Scratch

You are about to learn **programming** — how to write instructions a computer follows. You will use **Rust**, a language that checks your work before anything runs. That sounds strict, but for a beginner it is helpful: Rust points to mistakes like a patient editor.

No prior coding experience is assumed. If you can read and follow steps, you can do this.

## What programming is

Think of a recipe. It lists ingredients and steps: chop, stir, bake. A program is the same kind of thing — steps for the machine. The computer does not guess what you meant. It follows the recipe exactly.

Rust gives each ingredient a **type** (number, text, true/false) so the computer knows what fits where. That is one reason we chose Rust for this book: the rules are visible from day one.

## Why Rust for your first language

Many courses start with a "easier" language and save strict typing for later. We start with Rust on purpose. Its rules match how careful thinking works:

- Names mean one thing at a time.
- Steps happen in order.
- Choices have clear branches.

You will not learn everything about Rust here. You **will** learn enough to build small games and read the screen output they produce.

## Trust & Use

Rust sometimes shows syntax that looks mysterious — lines starting with `#`, words like `vec!`, or markers like `Some` and `None`.

**Trust & Use** means: copy those lines as written, run the program, watch the result, and move on. We explain them later in [Trust & Use](../appendix/TRUST_AND_USE.md). You do not need to understand every symbol on first sight — the same way you can follow a map legend before you know how cartography works.

## Three ways to read every idea

Throughout the book, each concept appears from three angles:

1. **Algorithm** — the steps the computer takes, in order.
2. **Types** — what kind of value each name holds (number, text, list, …).
3. **Language** — how the idea matches normal speech (a name tag, a fork in the road, a backpack of items).

You do not need three separate labels in your head. Just notice that a program is always **something stored**, **something done**, and **something said**.

## How this book is organized

Each chapter is a **short novel** — a small scene or game. You build it in **Steps**. Every Step is a complete program you paste into the [Rust Playground](https://play.rust-lang.org/) and run. There is no install until [Chapter 11](../chapters/11_live_on_your_machine.md).

Chapters 1–2 introduce words and pictures. Chapters 3–6 add choices, lists, recipes (functions), and character sheets (structs). Chapters 7–10 are game novels: a map, a guessing game, tic-tac-toe, snake. Chapter 11 puts Snake on your own computer with real keyboard control.

Open [CONTENTS.md](../CONTENTS.md) for the full list.

## Your first run

When you are ready, go to [Chapter 1: First Words](01_first_words.md). You will print a name badge — your first program.

If a Step fails to compile, compare your code to the book one character at a time. Rust's error messages name the line — that is the compiler helping you.

Welcome. Let's write something that runs.
