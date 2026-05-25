# Style Guide

## Voice

- **Warm-academic**: precise terms, short sentences, no hype or filler.
- **Brief**: if a paragraph does not teach or orient, cut it.
- **Human**: prefer “you move ownership” over “the value undergoes a move operation.”

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

1. Hook (Java/Python assumption → Rust reality)
2. Core sections with examples
3. **Idiom spotlight** (one boxed habit)
4. **Afterparty: AI Lego blocks** (5–8 numbered prompts, copy-paste ready)
5. **Go deeper** (1–3 hightechmind.io links)
6. **See also** (crosslinks to other chapters)

## Afterparty prompts

Format:

```markdown
### Afterparty: AI Lego blocks

1. **Prompt title** — Full sentence the reader can paste into an AI chat. Reference chapter section if helpful.
```

Prompts should be **active**: quiz, refactor, explain error, compare three approaches, design mini-API.

Every chapter’s prompts are catalogued in [appendix/AI_PROMPT_INDEX.md](appendix/AI_PROMPT_INDEX.md) as **P001–P200**.

## Crosslinks

Use relative paths: `[Chapter 1](chapters/01_paradigm_shift.md#references-borrowing-and-dereferencing)`

## Archival material

`archive/CHAPTER_01_*.md` and `CHAPTER_02_*.md` are source material only; do not edit them. New text lives under `chapters/`.
