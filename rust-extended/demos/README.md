# Demos — working Cargo projects

Runnable demos grouped by article category. From the [rust-extended](../) root:

```bash
cargo run -p demo_pin
cargo run -p five_deep_facts
cargo build          # all crates
```

## Sets

| Set | Path | Crates |
|-----|------|--------|
| **Mindset** | [mindset/](mindset/) | `five_deep_facts` |
| **Async** | [async/](async/) | `check_sync_prelude`, `demo_pin`, `demo_cancel_safety`, `demo_async_trait` |
| **Type system** | [type-system/](type-system/) | `demo_variance`, `demo_hrtb`, `demo_gats`, `demo_coherence` |
| **Memory & unsafe** | [memory/](memory/) | `demo_drop_order`, `demo_drop_check`, `demo_niche`, `demo_maybe_uninit`, `demo_miri` |

Each crate has one working binary in `src/main.rs`. Compile-fail and Miri counterexamples live as **Playground snippets in the articles**, not in this tree.
