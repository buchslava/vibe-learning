# Style Guide

## Voice

- **Warm-academic**: precise terms, short sentences, no hype or filler.
- **Brief**: if a paragraph does not teach or orient, cut it.
- **Human**: prefer “you move ownership” over “the value undergoes a move operation.”

## Prose

- **One idea per sentence** — aim for 12–22 words; split if you run out of breath reading aloud.
- **Short paragraphs** — two to four sentences, then a list, example, or break.
- **Lead with the point** — state what the reader should know, then nuance.
- **Active voice** — “you borrow”, “the compiler rejects”, not passive stacks.
- **Hooks stay short** — two or three sentences before the first example.
- **Terms stay precise** — simplify grammar, not vocabulary; keep Java/Python tables.
- **Markdown hygiene** — use `` `code` `` for identifiers; reserve **bold** for first mention of a concept, not every keyword.

## Comparisons

When introducing a Rust concept, use a compact table:

| Java | Python | Rust |
|------|--------|------|
| … | … | … |

Do not bash other languages; show *where habits transfer* and *where they break*.

## Code examples

### Tags (required)

Every snippet starts with a line comment or bold label:

- **Playground** — `std` only, single `fn main()`, runs on play.rust-lang.org
- **Cargo only** — needs `Cargo.toml`, filesystem, or external crates

### Playground rules

- No `mod` tree; one file, one `main`
- No `std::fs` / `Command` in playground snippets (use in-memory stand-ins)
- Use `println!` for observable output
- Avoid `unwrap()` in teaching examples unless the topic is panic handling

### Cargo rules

- Show `Cargo.toml` dependency lines when adding crates (`tokio`, `serialport`, etc.)
- State expected stdout where it helps learning

## Chapter skeleton

1. Hook (prior-language habits → Rust reality; optional Java/Python tables when helpful)
2. Optional **Scope — a brief tour** (one boundary sentence + deferred table — see below)
3. Core sections with examples
4. **Idiom spotlight** (one boxed habit)
5. **Go deeper** (1–3 hightechmind.io links)
6. **See also** (crosslinks to other chapters)
7. **Afterparty** (5–8 numbered prompts, copy-paste ready)

## Scope sections

Use when a chapter cannot cover the whole topic. **One sentence** states what you get and what is deferred; then the deferred table. Do not repeat the Hook or pitch Afterparty/Go deeper — those sections always follow.

```markdown
## Scope — a brief tour
Intro to [topic] — not [deferred area].

| This chapter covers | Deferred to See also / Afterparty |
| ... | ... |
```

Skip mermaid “chapter map” diagrams in Scope when the same links appear in **See also**.

## Afterparty prompts

Workflow (starter context, one chat per chapter) lives in [Preface](chapters/preface.md) only. Chapters do **not** repeat tutor instructions.

Format:

```markdown
### Afterparty

1. **Prompt title** — Full sentence the reader can paste into an AI chat. Reference chapter section if helpful.
```

Optional: one short line after the heading if the chapter deliberately skipped a topic (e.g. “Use these for worker pools — not covered above.”). No generic “copy into your AI tutor” prose.

Prompts should be **active**: quiz, refactor, explain error, compare three approaches, design mini-API.

Every chapter’s prompts are catalogued in [appendix/AI_PROMPT_INDEX.md](appendix/AI_PROMPT_INDEX.md) (**P001** onward).

## Compiler error checklists

Keep the error table; intro is one line:

```markdown
## When the compiler says no
Common errors in this chapter:
```

## Crosslinks

Use relative paths: `[Chapter 1](chapters/01_paradigm_shift.md#references-borrowing-and-dereferencing)`
