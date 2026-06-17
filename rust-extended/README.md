# Rust Extended

**Part of [Vibe Learning](../README.md)**

Deep-cut articles for developers who already know Rust basics — or who finished (or skimmed) **[Rust Core](../rust-core/)**. Each article stands alone; pick any topic from [CONTENTS.md](CONTENTS.md).

## Prerequisites

Comfort with ownership, types, pattern matching, and smart pointers from Rust Core Parts I–II. Start with the [Rust Core preface](../rust-core/chapters/preface.md) if you are new to the track.

## How to read

Articles live in [`articles/`](articles/). Most code snippets are tagged **Playground** and run on [play.rust-lang.org](https://play.rust-lang.org/) as a single file. Async and Pin topics use **Cargo only** snippets; runnable copies live under [`demos/`](demos/).

Voice and format follow [Rust Core → Style Guide](../rust-core/STYLE_GUIDE.md).

## Categories

| Category | Articles | Read when… |
|----------|----------|------------|
| **Mindset** | 01 | reframing ownership, matches, moves |
| **Async depth** | 02, 03, 04, 08 | after Ch 16; compiler mentions Pin or `select!` drops work |
| **Type system** | 06, 07, 09, 10 | HRTB / GAT / variance errors after Ch 5 or 7 |
| **Memory & unsafe** | 05, 11, 12, 13, 14 | custom containers, drop errors, Miri |

Suggested path (optional): **01 → 02 → 03 → 04** (async), then **06 → 07 → 11** (types/lifetimes), then pick by error message.

## Demos

Cargo projects are grouped by category under [`demos/`](demos/):

| Set | Path | Crates |
|-----|------|--------|
| Mindset | [`demos/mindset/`](demos/mindset/) | `five_deep_facts` |
| Async | [`demos/async/`](demos/async/) | `check_sync_prelude`, `demo_pin`, `demo_cancel_safety`, `demo_async_trait` |
| Type system | [`demos/type-system/`](demos/type-system/) | `demo_variance`, `demo_hrtb`, `demo_gats`, `demo_coherence` |
| Memory & unsafe | [`demos/memory/`](demos/memory/) | `demo_drop_order`, `demo_drop_check`, `demo_niche`, `demo_maybe_uninit`, `demo_miri` |

From the `rust-extended` root (virtual workspace):

```bash
cargo run -p demo_pin
cargo run -p five_deep_facts
```

Or `cd` into any crate directory and run `cargo run` there. Each crate has one working binary in `src/main.rs`. Compile-fail and Miri counterexamples live as **Playground snippets in the articles**.

## Repository layout

```
rust-extended/
├── Cargo.toml          ← workspace (all demo crates)
├── README.md           ← you are here
├── CONTENTS.md         ← categorized article index
├── INSPIRATION.md      ← external project links (not articles)
├── .check_all.rs       ← article 01 playground copy (also five_deep_facts)
├── articles/           ← standalone deep dives
└── demos/              ← category sets of Cargo projects
    ├── mindset/
    ├── async/
    ├── type-system/
    └── memory/
```
