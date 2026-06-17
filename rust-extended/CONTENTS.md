# Rust Extended — Contents

Standalone articles. Read in any order, or follow a category path below.

## By category

| Category | # | Article | Focus |
|----------|---|---------|-------|
| **Mindset** | 1 | [Five Deep Rust Facts](articles/01_five_deep_rust_facts.md) | Exclusive borrows, safe leaks, moves, match ergonomics, `PhantomData` intro |
| **Async depth** | 2 | [The Sync Prelude](articles/02_sync_prelude_async_blocks.md) | Sync setup at call site, shared state, explicit `Future` bounds |
| **Async depth** | 3 | [Pin and Unpin](articles/03_pin_and_unpin.md) | Self-referential futures, `Unpin`, why async needs pinning |
| **Async depth** | 4 | [Cancellation Safety](articles/04_cancellation_safety.md) | Drop = cancel, `select!` losers, cancel-safe APIs |
| **Async depth** | 8 | [async fn in Traits](articles/08_async_fn_in_traits.md) | Native async traits, RPITIT, `Send` bounds |
| **Type system** | 6 | [Variance and Subtyping](articles/06_variance_and_subtyping.md) | Covariance table, `'static` <: `'a`, struct field rules |
| **Type system** | 7 | [HRTB and Fn Traits](articles/07_hrtb_and_fn_traits.md) | `for<'a>`, closure lifetime polymorphism |
| **Type system** | 9 | [GATs and Lending Iterators](articles/09_gats_lending_iterators.md) | Generic associated types, borrow-from-self iteration |
| **Type system** | 10 | [Coherence and Orphan Rule](articles/10_coherence_and_orphan_rule.md) | Newtype pattern, extension traits |
| **Memory & unsafe** | 5 | [Drop Order and ManuallyDrop](articles/05_drop_order_manually_drop.md) | RFC 1857 order, controlled teardown |
| **Memory & unsafe** | 11 | [Drop Check and PhantomData](articles/11_drop_check_phantom_data.md) | Sound generic drop, marker types |
| **Memory & unsafe** | 12 | [Niche Optimization](articles/12_niche_optimization.md) | `Option` size, invalid value ranges |
| **Memory & unsafe** | 13 | [MaybeUninit](articles/13_maybe_uninit.md) | Uninitialized memory, `assume_init` contract |
| **Memory & unsafe** | 14 | [Miri Workflow](articles/14_miri_workflow.md) | Finding UB before production |

## Full index

| # | Demo | Article |
|---|------|---------|
| 1 | [demos/mindset/five_deep_facts/](demos/mindset/five_deep_facts/) | [Five Deep Rust Facts](articles/01_five_deep_rust_facts.md) |
| 2 | [demos/async/check_sync_prelude/](demos/async/check_sync_prelude/) | [The Sync Prelude](articles/02_sync_prelude_async_blocks.md) |
| 3 | [demos/async/demo_pin/](demos/async/demo_pin/) | [Pin and Unpin](articles/03_pin_and_unpin.md) |
| 4 | [demos/async/demo_cancel_safety/](demos/async/demo_cancel_safety/) | [Cancellation Safety](articles/04_cancellation_safety.md) |
| 5 | [demos/memory/demo_drop_order/](demos/memory/demo_drop_order/) | [Drop Order and ManuallyDrop](articles/05_drop_order_manually_drop.md) |
| 6 | [demos/type-system/demo_variance/](demos/type-system/demo_variance/) | [Variance and Subtyping](articles/06_variance_and_subtyping.md) |
| 7 | [demos/type-system/demo_hrtb/](demos/type-system/demo_hrtb/) | [HRTB and Fn Traits](articles/07_hrtb_and_fn_traits.md) |
| 8 | [demos/async/demo_async_trait/](demos/async/demo_async_trait/) | [async fn in Traits](articles/08_async_fn_in_traits.md) |
| 9 | [demos/type-system/demo_gats/](demos/type-system/demo_gats/) | [GATs and Lending Iterators](articles/09_gats_lending_iterators.md) |
| 10 | [demos/type-system/demo_coherence/](demos/type-system/demo_coherence/) | [Coherence and Orphan Rule](articles/10_coherence_and_orphan_rule.md) |
| 11 | [demos/memory/demo_drop_check/](demos/memory/demo_drop_check/) | [Drop Check and PhantomData](articles/11_drop_check_phantom_data.md) |
| 12 | [demos/memory/demo_niche/](demos/memory/demo_niche/) | [Niche Optimization](articles/12_niche_optimization.md) |
| 13 | [demos/memory/demo_maybe_uninit/](demos/memory/demo_maybe_uninit/) | [MaybeUninit](articles/13_maybe_uninit.md) |
| 14 | [demos/memory/demo_miri/](demos/memory/demo_miri/) | [Miri Workflow](articles/14_miri_workflow.md) |

## Suggested reading paths

- **After Rust Core Ch 16 (async):** 02 → 03 → 04 → 08
- **Lifetime / trait bound errors:** 06 → 07 → 09 → 10
- **Unsafe or custom containers:** 05 → 11 → 12 → 13 → 14
- **Quick mindset reset:** 01 alone

Project-tutorial links (not articles): [INSPIRATION.md](INSPIRATION.md).
