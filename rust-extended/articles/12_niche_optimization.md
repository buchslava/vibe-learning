# Niche Optimization and Enum Size

`Option<&T>` is the same size as `&T` — not magic, **niche optimization**. The compiler stores `None` in bit patterns that are **invalid** for `&T` (such as null).

Not covered in Rust Core — useful for APIs and FFI layout reasoning.

---

## 1. `Option<&T>` same size as `&T`

```rust
// Playground
use std::mem::size_of;

fn main() {
    println!("&i32:         {}", size_of::<&i32>());
    println!("Option<&i32>: {}", size_of::<Option<&i32>>());
}
```

References exclude null; `None` uses that niche.

Demo: [`demos/memory/demo_niche/`](../demos/memory/demo_niche/).

**Takeaway:** Null (or other invalid values) encodes `None` without extra bytes.

---

## 2. Niches as invalid ranges, not bit tricks

Each type has valid value ranges:

- `bool`: 0 and 1 — niche 2..=255
- `char`: Unicode scalar range — niche outside it
- `NonZeroU8`: 1..=255 — zero is niche for `Option<NonZeroU8>`

The compiler merges niches across enums when layout analysis allows — not guaranteed for all nested shapes.

**Takeaway:** Think in valid/invalid ranges, not manual bit packing.

---

## 3. Nested enums and compiler limits

Complex enums may not shrink `Option<Outer>` even when inner variants have niches — layout passes trade size for branch cost ([rust-lang#125363](https://github.com/rust-lang/rust/issues/125363)).

Do not rely on `size_of` in public API guarantees except with `#[repr(C)]` and documented layout.

**Takeaway:** Niche filling is best-effort — measure if size matters.

---

## 4. `UnsafeCell` suppresses niche

Interior mutability signals possible concurrent mutation — optimizer conservatively disables some niche reuse ([PR #68491](https://github.com/rust-lang/rust/pull/68491)).

Demo prints larger `Option<Wrapped>` when `Wrapped` contains `UnsafeCell`.

**Takeaway:** `UnsafeCell` in a field can cost you niche optimization.

---

## 5. `NonZero*` and `NonNull` APIs

Std types expose niches intentionally:

```rust
// Playground
use std::mem::size_of;
use std::num::NonZeroU8;

fn main() {
    assert_eq!(size_of::<Option<NonZeroU8>>(), size_of::<u8>());
}
```

Use them for compact `Option` in structs sent over the wire (with explicit layout docs).

**Takeaway:** Standard newtypes document niches — use them for compact options.

---

## See also

- [Rust Core → Chapter 6: Enums](../rust-core/chapters/06_types_enums_pattern_matching.md)
- [Rust Extended → MaybeUninit](13_maybe_uninit.md)

## Go deeper

- [Rust Reference — type layout](https://doc.rust-lang.org/reference/type-layout.html)
- [GitHub #125363 — Arc niche limits](https://github.com/rust-lang/rust/issues/125363)
- [HN — surprising enum size optimization](https://news.ycombinator.com/item?id=43616649)
