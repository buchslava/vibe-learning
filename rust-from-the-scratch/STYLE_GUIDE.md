# Style Guide

## Voice

- **Warm and clear**: short sentences, plain words, no slang, no hype.
- **Brief**: if a paragraph does not teach or orient, cut it.
- **Human**: compare programming ideas to everyday language — lists, choices, name tags.

This book does **not** assume Java, Python, or any other language.

## The three lenses

Weave these into prose (not always as three separate headers):

| Lens | Question it answers |
|------|---------------------|
| **Algorithm** | What steps happen, in what order? |
| **Types** | What kind of value does each name hold? |
| **Language** | How would you say this in ordinary speech? |

Example: a variable is a **labeled box** (language), the label holds one **kind** of thing (types), and the program **reads the box then prints it** (algorithm).

## Trust & Use

When syntax is heavy for a first pass — `#[derive]`, `vec![]`, `Some`/`None`, macros — add a short box:

```markdown
> **Trust & Use:** Copy this line exactly. We explain it in [Trust & Use](../appendix/TRUST_AND_USE.md).
```

Never block progress to explain implementation details early.

## Code examples

### Tags (required)

Every snippet starts with a line comment or bold label:

- **Playground** — `std` only, single `fn main()`, runs on play.rust-lang.org
- **Cargo only** — needs `Cargo.toml`, filesystem, or external crates

### Playground rules

- One file, one `main` — no `mod` tree
- No `std::fs` / `Command` / network
- No keyboard input — use hardcoded `choices` or `moves` arrays
- Output via `println!` — ASCII boards, bars, frames
- Edition **2021**, **stable** channel

### Cargo rules (Chapter 11)

- Terminal Snake uses **`crossterm`** for keyboard and screen control
- Show full `Cargo.toml` dependency line when adding the crate
- State expected terminal behavior where it helps

## Chapter skeleton (novel format)

1. **Story hook** — one scene
2. **What you will build** — final output preview in a fenced `text` block
3. **Steps 1…N** — three-lens paragraph → code → **Expected output**
4. **Full chapter solution** — one consolidated snippet
5. **Trust & Use** (if needed)
6. **See also** — link to the next chapter only

## Step block template

```markdown
### Step N — Short title

**Algorithm:** …
**Types:** …
**Language:** …

\`\`\`rust
// Playground
fn main() {
    // ...
}
\`\`\`

**Expected output:**
\`\`\`text
...
\`\`\`
```

## Markdown hygiene

- Use `` `code` `` for identifiers
- Reserve **bold** for first mention of a concept in a section
- Crosslinks: relative paths, e.g. `[Chapter 4](04_the_backpack.md)`
