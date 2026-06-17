# Miri: Finding UB Before Production

The Rust compiler rejects most unsound **safe** code, but `unsafe` and some layout tricks slip through. **Miri** interprets your program and detects undefined behavior at runtime — in tests and small binaries.

Introduced briefly in [Rust Core Chapter 18](../rust-core/chapters/18_unsafe_and_internals.md).

---

## 1. What Miri catches

| Class | Example |
|-------|---------|
| Use-after-free | Return reference to local |
| Uninitialized read | `assume_init` too early |
| Stacked / tree borrows violations | Invalid `unsafe` pointer use |
| Data races | (with `-Zmiri-preemption-rate`) |

Demo (safe): [`demos/memory/demo_miri/`](../demos/memory/demo_miri/) — `cargo run -p demo_miri`.

Stack reference UB (paste into a file, then Miri):

```rust
// Miri — undefined behavior
fn dangling() -> &'static i32 {
    let x = 42;
    &x
}

fn main() {
    let _r = dangling();
}
```

**Takeaway:** Miri finds UB that compiles cleanly.

---

## 2. Running Miri

One-time setup:

```bash
rustup +nightly component add miri
```

Run a small harness:

```bash
cargo +nightly miri run --manifest-path path/to/Cargo.toml
```

Miri prints the UB location and a backtrace.

**Takeaway:** `cargo +nightly miri run` / `miri test` on unsafe-heavy code paths.

---

## 3. When to run Miri

- After writing or reviewing `unsafe` blocks.
- Custom allocators, `MaybeUninit` buffers ([article 13](13_maybe_uninit.md)).
- FFI boundary smoke tests.
- Before merging library code others will build on.

Not required for every CRUD handler — targeted use saves time.

**Takeaway:** Miri is for unsafe, layout, and concurrency experiments — not every binary.

---

## 4. Limitations

- Nightly-only tool; some platform APIs unsupported.
- Slow compared to native runs — use small harnesses.
- Does not replace code review or sanitizers in production CI for all targets.

Pair with `cargo test` on stable for logic; Miri for soundness.

**Takeaway:** Miri complements tests; it does not replace them.

---

## 5. Pair with article 13

The safe demo passes Miri:

```bash
cargo +nightly miri run -p demo_maybe_uninit
```

The `assume_init` counterexample from [article 13](13_maybe_uninit.md) should fail under Miri when pasted into its own harness.

**Takeaway:** Keep Miri snippets in articles or tests — demos stay working-only.

---

## See also

- [Rust Core → Chapter 18: Unsafe and internals](../rust-core/chapters/18_unsafe_and_internals.md)
- [Rust Extended → MaybeUninit](13_maybe_uninit.md)
- [Rust Extended → Drop Check and PhantomData](11_drop_check_phantom_data.md)

## Go deeper

- [Miri README](https://github.com/rust-lang/miri)
- [Rust Blog — introducing Miri](https://blog.rust-lang.org/inside-rust/2020/05/07/Miri.html)
