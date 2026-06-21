# Chapter 7: Structs, Traits, and Generics

## Hook

Rust uses **structs** for data, `impl` for methods, **traits** for shared behaviour, and **enums** for closed alternatives. **Java** bundles data with inheritance; **Python** relies on duck typing (“if it quacks…”) — optional comparison points. Rust favors composition over inheritance, all checked at compile time.

## Structs and methods

```rust
// Playground
struct Sensor {
    id: u32,
    value: f64,
}

impl Sensor {
    fn new(id: u32, value: f64) -> Self {
        Self { id, value }
    }

    fn scaled(&self, factor: f64) -> f64 {
        self.value * factor
    }
}

fn main() {
    let s = Sensor::new(1, 25.0);
    println!("{}", s.scaled(0.5));
}
```

## Traits — interfaces done right

If you know **Java** or **Python**, a **trait** names a **capability** any type can opt into — shared behaviour **without inheritance**, checked at compile time. The table below maps the habit; then the examples.

**Why traits are a win**


| Benefit               | Java / Python pain                                  | Rust trait answer                                                           |
| --------------------- | --------------------------------------------------- | --------------------------------------------------------------------------- |
| Checked contracts     | interface optional; duck typing fails at runtime    | missing method = **compile error**                                          |
| No hierarchy tax      | deep trees, fragile `super` chains                  | flat types + `impl` blocks                                                  |
| Cross-crate reuse     | can’t add methods to `String` / third-party classes | implement **your** trait for **your** wrapper (orphan rule applies)         |
| Performance           | interface dispatch / dynamic checks                 | **static dispatch** by default (`impl Trait` → monomorphization, no vtable) |
| Optional polymorphism | always reference types / ABCs                       | `dyn Trait` only when you need mixed-type collections                       |


You will use traits everywhere: `Display`, `Clone`, `Iterator`, custom `Measurable` / `Summary` in automation code. Start with `impl Trait for MyType` and `fn f(x: &impl Trait)`; reach for `dyn Trait` when you need [mixed types in one collection](#dyn-trait--mixed-types-in-one-collection) (full detail in [Trait objects](#trait-objects-dyn-trait) below).

```rust
// Playground
trait Summary {
    fn summarize(&self) -> String;
}

struct Reading { v: f64 }

impl Summary for Reading {
    fn summarize(&self) -> String {
        format!("reading: {}", self.v)
    }
}

fn print_summary(item: &impl Summary) {
    println!("{}", item.summarize());
}

fn main() {
    let r = Reading { v: 3.14 };
    print_summary(&r);
}
```


| Java                      | Python            | Rust                  |
| ------------------------- | ----------------- | --------------------- |
| `interface`               | informal protocol | `trait`               |
| `implements`              | “has method”      | `impl Trait for Type` |
| default interface methods | mixin / ABC       | trait default bodies  |


## Enums, structs, and traits together

[Chapter 6](06_types_enums_pattern_matching.md) gave you `enum` + exhaustive `match`. This section covers **how structs and traits attach to enums** — Rust’s substitute for a class hierarchy or a Python `Union` of unrelated types.

The idiomatic shape for automation and protocol code:

1. `enum` — closed set of states or message kinds (compiler tracks every variant).
2. `struct` — per-variant payload when a variant carries real data.
3. `trait` — shared behaviour across *different* types (`Reading`, `Alarm`, …), or a uniform API over one enum.
4. `impl` — inherent methods on the enum *and/or* trait implementations that `match` inside.

```rust
// Playground
trait Measurable {
    fn value(&self) -> f64;
    fn unit(&self) -> &'static str;
}

struct Temperature {
    celsius: f64,
}

struct Pressure {
    bar: f64,
}

enum SensorReading {
    Temp(Temperature),
    Press(Pressure),
    Skipped { reason: String },
}

impl Measurable for SensorReading {
    fn value(&self) -> f64 {
        match self {
            SensorReading::Temp(t) => t.celsius,
            SensorReading::Press(p) => p.bar,
            SensorReading::Skipped { .. } => f64::NAN,
        }
    }

    fn unit(&self) -> &'static str {
        match self {
            SensorReading::Temp(_) => "°C",
            SensorReading::Press(_) => "bar",
            SensorReading::Skipped { .. } => "n/a",
        }
    }
}

impl SensorReading {
    fn is_valid(&self) -> bool {
        !matches!(self, SensorReading::Skipped { .. })
    }
}

fn log(item: &impl Measurable) {
    println!("{} {}", item.value(), item.unit());
}

fn main() {
    let r = SensorReading::Temp(Temperature { celsius: 21.5 });
    log(&r);
    println!("valid={}", r.is_valid());
}
```

**What `!matches!(self, SensorReading::Skipped { .. })` does**

- `matches!(value, pattern)` — standard-library **macro** ([Chapter 17: Metaprogramming](17_metaprogramming.md)) that expands to a `match` returning `true` or `false`. It is syntax sugar for “does this value fit this pattern?” without binding variables you do not need.
- `SensorReading::Skipped { .. }` — matches the `Skipped` struct variant and **ignores** the `reason` field (`..` = “other fields don’t matter for this test”). Same pattern token as in [Chapter 6](06_types_enums_pattern_matching.md).
- `!` — negates the result: `Temp` and `Press` → `true`; `Skipped { .. }` → `false`.

Equivalent without the macro:

```rust
// Playground
enum SensorReading {
    Temp,
    Press,
    Skipped { reason: String },
}

fn is_valid_longhand(r: &SensorReading) -> bool {
    match r {
        SensorReading::Skipped { .. } => false,
        _ => true,
    }
}

fn main() {
    println!("{}", is_valid_longhand(&SensorReading::Temp));
}
```

Use `matches!` for readable guards and filters; use full `match` when each arm returns different data. More on declarative macros: [Chapter 17 — `macro_rules!`](17_metaprogramming.md#macro_rules).

### Two `impl` blocks on one type


| Block                                       | Role                                                       | Java / Python analogy    |
| ------------------------------------------- | ---------------------------------------------------------- | ------------------------ |
| `impl SensorReading { ... }`                | **Inherent** methods — always available on `SensorReading` | methods on the class     |
| `impl Measurable for SensorReading { ... }` | **Trait** impl — call only where `Measurable` is required  | interface implementation |


Both on the same `enum` is normal. Put variant-specific logic in inherent methods. Put cross-type contracts on traits.

### Struct-in-enum vs enum-in-struct


| Layout                         | Sketch                                   | Reach for it when                                          |
| ------------------------------ | ---------------------------------------- | ---------------------------------------------------------- |
| **Struct inside enum variant** | `enum Msg { Data(Frame), Ping }`         | variants own *different* shapes; `match` is the dispatcher |
| **Enum field inside struct**   | `struct Device { kind: Kind, addr: u8 }` | all rows share the same fields; tag only selects behaviour |
| **Unit variants only**         | `enum Mode { Auto, Manual }`             | no payload — traits/methods ignore inner data              |


```rust
// Playground — shared metadata + tagged kind
enum DeviceKind {
    Modbus,
    OpcUa,
}

struct Device {
    name: String,
    kind: DeviceKind,
}

impl Device {
    fn default_port(&self) -> u16 {
        match self.kind {
            DeviceKind::Modbus => 502,
            DeviceKind::OpcUa => 4840,
        }
    }
}

fn main() {
    let d = Device {
        name: "plc-1".into(),
        kind: DeviceKind::Modbus,
    };
    println!("{}:{}", d.name, d.default_port());
}
```

Prefer **enum + struct variants** when each variant would have been a subclass with different fields. Prefer **struct + enum field** when every instance shares the same columns and only behaviour differs.

### Trait on enum: `match` is mandatory

Unlike a single `struct`, an `enum` trait body almost always uses `match self` (or `match &self`) — one arm per variant. That mirrors [Chapter 6](06_types_enums_pattern_matching.md) exhaustiveness. Add a variant, and the compiler lists every `impl` and `match` you must update.

**Default trait methods** still work — override only where a variant differs:

```rust
// Playground
trait HasCode {
    fn code(&self) -> u8;
    /// Shared default formatting — call this from overrides to avoid recursion.
    fn default_label(&self) -> String {
        format!("code {}", self.code())
    }
    fn label(&self) -> String {
        self.default_label()
    }
}

enum Fault {
    OverTemp(u8),
    CommLost,
}

impl HasCode for Fault {
    fn code(&self) -> u8 {
        match self {
            Fault::OverTemp(c) => *c,
            Fault::CommLost => 0xFF,
        }
    }

    fn label(&self) -> String {
        match self {
            Fault::CommLost => "communication lost".into(),
            other => other.default_label(), // trait default body, not this override
        }
    }
}

fn main() {
    println!("{}", Fault::CommLost.label());
    println!("{}", Fault::OverTemp(0x0A).label()); // default: "code 10"
}
```

#### Calling the trait’s default from your override

You want two behaviours in one `label` override:

| Variant | Desired output |
|---------|----------------|
| `CommLost` | custom string — `"communication lost"` |
| `OverTemp` | shared default — `"code {n}"` from the trait |

The trait exposes **`default_label`** for the shared path. In the override, `other.default_label()` calls **that trait method’s body** — not your `label` override:

| Call in `impl HasCode for Fault` | What runs |
|----------------------------------|-----------|
| `other.default_label()` | trait default: `format!("code {}", other.code())` |
| `other.label()` | **this override again** → infinite recursion on `OverTemp` |

That is why the trait names the fallback `default_label` instead of reusing `label`. Same idea as Java’s `HasCode.super.label()` — call the default implementation, not your override.

**Footgun:** never write `other.label()` inside `fn label` unless you mean to recurse. Use `other.default_label()`, or inline `format!("code {}", other.code())`.

**Wrong — “call the override again” (infinite recursion on `OverTemp`):**

```rust
// Playground — stack overflow at runtime for OverTemp
impl HasCode for Fault {
    fn label(&self) -> String {
        match self {
            Fault::CommLost => "communication lost".into(),
            other => other.label(), // ERROR path: calls THIS override again, forever
        }
    }
}
```

The `OverTemp` arm above is the bug — `other.label()` never reaches the trait default.

#### One `impl` block per trait (not `impl A, B for T`)

**No** — Rust does not allow multiple traits in one impl header:

```rust
// Playground — does not compile
// impl HasCode, Display for Fault { ... }
// ERROR: expected `for`, found `,`
```

A type may implement **many** traits, but each trait gets its **own** `impl` block:

```rust
// Playground
use std::fmt;

trait HasCode {
    fn code(&self) -> u8;
}

enum Fault {
    OverTemp(u8),
    CommLost,
}

impl HasCode for Fault {
    fn code(&self) -> u8 {
        match self {
            Fault::OverTemp(c) => *c,
            Fault::CommLost => 0xFF,
        }
    }
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fault 0x{:02X}", <Self as HasCode>::code(self))
    }
}

fn main() {
    println!("{}", Fault::CommLost);
}
```

Where **multiple traits appear together** is on **bounds**, not on `impl`:


| Location          | Example                               | Meaning                                         |
| ----------------- | ------------------------------------- | ----------------------------------------------- |
| Generic parameter | `fn show<T: HasCode + Display>(x: T)` | `T` must implement **both** traits              |
| `where` clause    | `where T: HasCode + Display`          | same, spelled out                               |
| Trait inheritance | `trait Loud: Display { ... }`         | every `Loud` type must also implement `Display` |


So: `impl HasCode for Fault` and `impl Display for Fault` are two separate blocks; `T: HasCode + Display` means “T implements both.”

### `#[derive(...)]` on enums and structs

Enums and structs in the same model often share derives:

```rust
// Playground
#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    Stop,
    SetSpeed(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SetSpeedLog {
    at: u64,
    rpm: u32,
}

impl Command {
    /// Turn a command into a timestamped log row when it carries speed data.
    fn to_log(&self, at: u64) -> Option<SetSpeedLog> {
        match self {
            Command::SetSpeed(rpm) => Some(SetSpeedLog { at, rpm: *rpm }),
            Command::Stop => None,
        }
    }
}

fn main() {
    let c = Command::SetSpeed(1500);
    assert_eq!(c, Command::SetSpeed(1500));

    let log = c.to_log(1_700_000_000).expect("SetSpeed maps to a log row");
    assert_eq!(log, SetSpeedLog { at: 1_700_000_000, rpm: 1500 });
    println!("{:?}", log);
}
```

**Two shapes for one domain:**

| Type | Role | Example |
|------|------|---------|
| `Command` (enum) | wire / control message | `Stop`, `SetSpeed(1500)` |
| `SetSpeedLog` (struct) | persisted audit row | `{ at: timestamp, rpm: 1500 }` |

`Command::to_log` converts between them: `SetSpeed(rpm)` → `Some(SetSpeedLog { ... })`; `Stop` → `None` (nothing to log).

Both types carry the same `#[derive(Debug, Clone, PartialEq, Eq)]` — commands and the rows they produce usually need the same test/debug tooling. `#[derive]` generates trait impls at compile time (not runtime annotations); details in [Chapter 17: Derive attributes](17_metaprogramming.md#derive-attributes).

**Derive constraint:** `PartialEq`/`Eq` on an enum requires every payload type to support it. A variant with `f64` breaks `Eq` — use integers, drop `Eq`, or model floats differently (see edge cases below).

### `dyn Trait` — mixed types in one collection

So far, traits use **static dispatch**: `fn print_summary(item: &impl Summary)` and `impl Measurable for SensorReading` are resolved at **compile time** — the compiler generates specialized code per concrete type (no vtable).

Use **`dyn Trait`** when several **different struct types** must sit in the **same collection** or pass through one function parameter type. **Yes — this is Rust’s runtime polymorphism**, closest to a **Java interface reference** or Python duck typing through a shared protocol — but Rust makes you opt in explicitly; it is not the default for every trait call.

| Approach | Type in signature | Dispatch | Typical use |
|----------|-------------------|----------|-------------|
| `impl Trait` / generics | `&impl Measurable` | static — monomorphized | helper called with one concrete type at a time |
| `&dyn Trait` | borrowed trait object | dynamic — vtable lookup | `&[&dyn Measurable]` — stack values, mixed types |
| `Box<dyn Trait>` | owned trait object on heap | dynamic | `Vec<Box<dyn Measurable>>` — store plug-ins for process lifetime |

**Java habit vs Rust:**

| Java | Rust (`dyn`) | Same idea? |
|------|--------------|------------|
| `interface Measurable { double value(); }` | `trait Measurable { fn value(&self) -> f64; }` | shared contract |
| `class TempSensor implements Measurable` | `impl Measurable for TempSensor` | type opts in |
| `Measurable s = new TempSensor();` | `let s: Box<dyn Measurable> = Box::new(TempSensor { ... });` | variable typed as interface/trait, holds concrete type |
| `List<Measurable> sensors = List.of(new TempSensor(), new PressSensor());` | `Vec<Box<dyn Measurable>> = vec![Box::new(...), Box::new(...)];` | mixed concrete types in one collection |
| `s.value()` — virtual dispatch via vtable | `s.value()` on `&dyn Measurable` — vtable lookup at runtime | **yes — dynamic dispatch** |

**Key differences from Java:**

| Java | Rust |
|------|------|
| Interface references are the **usual** polymorphism style | **`impl Trait` / generics first** — `dyn` only when you need type erasure in a collection or factory |
| Every object reference is already a pointer | `dyn Measurable` is **unsized** (`!Sized`) — must live behind `&`, `Box`, `Arc`, … |
| `List<Measurable>` homogenizes references automatically | `Vec<Box<dyn Measurable>>` — **`Box` required** because `TempSensor` and `PressSensor` have different sizes |
| Inheritance + `implements` | **no inheritance** — flat structs + separate `impl` blocks |
| Most interface methods work in collections | only **object-safe** traits become `dyn` (see below) |

A **`dyn Measurable`** value is a **fat pointer** (like a Java object reference carrying type info for virtual calls): address of the concrete sensor **plus** a vtable pointer for `impl Measurable`. Because each struct has a different size, `dyn Measurable` cannot sit on the stack by itself — hence `&dyn` or `Box<dyn>`.

```rust
// Playground
trait Measurable {
    fn value(&self) -> f64;
}

struct TempSensor { celsius: f64 }
struct PressSensor { bar: f64 }

impl Measurable for TempSensor {
    fn value(&self) -> f64 { self.celsius }
}
impl Measurable for PressSensor {
    fn value(&self) -> f64 { self.bar }
}

fn average(sensors: &[&dyn Measurable]) -> f64 {
    let sum: f64 = sensors.iter().map(|s| s.value()).sum();
    sum / sensors.len() as f64
}

fn main() {
    let t = TempSensor { celsius: 20.0 };
    let p = PressSensor { bar: 1.2 };

    // borrow different struct types through one trait interface
    println!("avg = {}", average(&[&t, &p]));

    // owning heterogeneous Vec — each element is Box<dyn Measurable>
    let bank: Vec<Box<dyn Measurable>> = vec![
        Box::new(TempSensor { celsius: 22.0 }),
        Box::new(PressSensor { bar: 0.9 }),
    ];
    for s in &bank {
        println!("{}", s.value());
    }
}
```

**Object-safe traits only:** not every trait can become `dyn Trait`. Methods must use `&self` (or `&mut self`) — the vtable lists instance methods with a fixed shape. Traits with associated functions without `self`, generic methods, or `-> Self` returns often fail — see the edge case below and [Object safety](#object-safety--not-every-trait-can-be-dyn).

For a **closed set** of variants (`SensorReading::Temp | Press | …`), an **`enum` + `match`** is usually better than `Vec<Box<dyn _>>` — no vtable, exhaustiveness checking. Use `dyn` when the set of concrete types is **open** (plug-ins, drivers loaded at runtime).

### Enums + traits edge cases and compiler traps

**Wrong — non-exhaustive `match` in trait impl (add a variant, forget an arm):**

```rust
// Playground — does not compile
enum Status { Idle, Running, Fault }

trait Label {
    fn label(&self) -> &'static str;
}

impl Label for Status {
    fn label(&self) -> &'static str {
        match self {
            Status::Idle => "idle",
            Status::Running => "running",
            // Status::Fault => ...  // ERROR: non-exhaustive patterns
        }
    }
}
```

**Wrong — partial move: extract payload, then reuse `self`:**

```rust
// Playground — does not compile
enum Packet {
    Raw(String),
    Empty,
}

impl Packet {
    fn dump(self) {
        if let Packet::Raw(s) = self {
            println!("{}", s.len());
        }
        // ERROR: use of partially moved value: `self`
        // Packet::Empty arm never ran, but `Raw` variant moved `self`
        let empty = matches!(self, Packet::Empty);
        println!("empty={}", empty);
    }
}
```

Fix: take `&self`, clone the `String` when you need ownership, or handle all variants in one `match`.

**Wrong — `match self` by value in one method, then call another method on `self`:**

```rust
// Playground — does not compile
impl Packet {
    fn describe(self) -> String {
        match self {
            Packet::Raw(s) => format!("raw {} bytes", s.len()),
            Packet::Empty => "empty".into(),
        }
    }

    fn tag(self) -> &'static str {
        match self {
            Packet::Raw(_) => "RAW",
            Packet::Empty => "EMPTY",
        }
    }

    fn full(self) -> String {
        format!("{}: {}", self.tag(), self.describe()) // ERROR: `self` moved
    }
}
```

Idiomatic fix: `fn full(&self) -> String` and match on references, or merge into a single `match`.

**Wrong — orphan rule (external trait + external type):**

```rust
// Playground — does not compile
// impl std::fmt::Display for Vec<u8> { ... }
// ERROR: impl doesn't apply to type defined outside of crate
```

You can `impl Display for YourEnum`, or wrap `Vec<u8>` in a newtype `struct Frame(pub Vec<u8>);` and implement there.

**Wrong — trait not object-safe (cannot build `dyn Trait`):**

The working `Measurable` trait above uses `fn value(&self)`. This trait adds `fn read() -> f64` **without `self`** — the vtable cannot list it, so `dyn` fails:

```rust
// Playground — does not compile
trait RawReading {
    fn value(&self) -> f64;
    fn read() -> f64; // no `self` — not object-safe
}

// let items: Vec<Box<dyn RawReading>> = vec![...];
// ERROR: trait `RawReading` is not dyn compatible
```

**Fix:** make every callable method take `&self`, split static helpers into free functions, or skip `dyn` and use an `enum` / generics instead. More traps (`-> Self`, generic methods, `dyn` by value) — [Object safety](#object-safety--not-every-trait-can-be-dyn) below.

**Wrong — `Eq` on enum with `f64` payload:**

```rust
// Playground — does not compile
#[derive(Eq, PartialEq)]
enum Sample {
    Analog(f64),
    Digital(bool),
}
// ERROR: the trait bound `f64: Eq` is not satisfied
```

Use integer fixed-point, ordered floats (`ordered-float`), or derive only `PartialEq`.


| Trap                       | Symptom                                    | Idiom                                                                            |
| -------------------------- | ------------------------------------------ | -------------------------------------------------------------------------------- |
| New enum variant           | errors in every `match` / trait `impl`     | treat as feature — update all arms                                               |
| Partial move in `match`    | “use of partially moved value”             | `match &self`, clone, or one combined `match`                                    |
| Trait impl without `match` | wrong for multi-variant enums              | one arm per variant                                                              |
| `dyn Trait` collection     | “not dyn compatible”                       | object-safe trait, or enum of variants instead of heterogenous `Vec<Box<dyn _>>` |
| Orphan rule                | “impl doesn't apply to type outside crate” | newtype wrapper or own trait                                                     |


### Idiom spotlight (enums + traits)

> **Model closed sets as `enum`, not strings.** Put data in struct variants; implement traits with exhaustive `match`.
>
> **New variant → compiler updates your checklist** — no forgotten `else` when the PLC adds a fault code.
>
> **Prefer `&self` on enums** unless you intentionally consume the value.

## Generics

```rust
// Playground
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in &list[1..] {
        if item > max { max = item; }
    }
    max
}

fn main() {
    let nums = vec![3, 1, 4, 1, 5];
    println!("{}", largest(&nums));
}
```

## Trait bounds and `where`

```rust
// Playground
use std::fmt::Display;

fn show<T: Display>(x: T) {
    println!("{}", x);
}

fn main() {
    show(42);
    show("text");
}
```

## Trait objects (`dyn Trait`)

[Above](#dyn-trait--mixed-types-in-one-collection) introduced `&dyn Trait` and `Box<dyn Trait>`. This section expands on fat pointers, factories, `impl` vs `dyn` vs `enum`, and object-safety traps.

**Generics and `impl Trait`** pick the concrete type at **compile time** — monomorphized, no vtable.

**`dyn Trait`** is runtime polymorphism through a vtable — like Java interface references or Python duck typing at runtime. Prefer `impl Trait` when types are known at compile time; use `dyn Trait` for mixed-type collections.

### What you actually store

A trait object is a **wide pointer** (fat pointer):

| Part | Points to |
|------|-----------|
| Data pointer | the concrete value (`En`, `Fr`, …) |
| Vtable pointer | that type’s `impl Greeter` method table |

Because size varies by concrete type, `dyn Greeter` is **dynamically sized** (`!Sized`). It almost always lives **behind a pointer**:

| Form | Owns value? | Typical use |
|------|-------------|-------------|
| `&dyn Trait` | no — borrow | pass one of several types into `fn notify(x: &dyn Trait)` |
| `&mut dyn Trait` | no — mutable borrow | plug-in you mutate in place |
| `Box<dyn Trait>` | yes — heap | `Vec<Box<dyn Trait>>` of mixed types |
| `Arc<dyn Trait>` | yes — shared | callbacks/handlers shared across threads (`Send + Sync`) |

```rust
// Playground
trait Greeter {
    fn greet(&self) -> String;
}

struct En;
struct Fr;

impl Greeter for En {
    fn greet(&self) -> String { "Hello".into() }
}
impl Greeter for Fr {
    fn greet(&self) -> String { "Bonjour".into() }
}

fn announce(g: &dyn Greeter) {
    println!("{}", g.greet());
}

fn main() {
    announce(&En); // no Box — stack value, borrowed as trait object
    announce(&Fr);

    let voices: Vec<Box<dyn Greeter>> = vec![Box::new(En), Box::new(Fr)];
    for v in &voices {
        println!("{}", v.greet());
    }
}
```

**Why `Vec` needs `Box<dyn Greeter>`:**

A `Vec` stores elements **back-to-back in memory**. Every slot must be the **same byte width** so the CPU can jump to index `i` with `base + i * stride`.

| What you try | Problem |
|--------------|---------|
| `Vec<En>` mixed with `Fr` | `En` and `Fr` may have **different sizes** — one array cannot hold both inline |
| `Vec<dyn Greeter>` | `dyn Greeter` is **unsized** — Rust does not know how many bytes one slot needs |
| `Vec<Box<dyn Greeter>>` | **works** — each slot is exactly one `Box` (same size every time) |

`Box::new(En)` and `Box::new(Fr)` move the concrete values to the **heap**. The `Vec` only stores identical `Box<dyn Greeter>` handles — each a **fat pointer** (data address + vtable address), same width for every element:

```
Vec slot:  [ Box<dyn> | Box<dyn> | Box<dyn> ]   ← fixed-size slots
                |          |
                v          v
             heap En    heap Fr                 ← different sizes OK off-heap
```

That is what “homogenize” means here: the collection stores **uniform handles**, not the varying struct bodies. Java does this implicitly — every `Measurable` reference in an `ArrayList` is one pointer width. Rust makes the heap indirection explicit with `Box`.

### When `dyn Trait` is idiomatic

| Situation | Prefer | Why |
|-----------|--------|-----|
| Function called with **one of a few types**, known when writing code | `fn f(x: &impl Greeter)` | static dispatch, zero vtable |
| **`Vec` / registry of plug-ins** loaded at runtime | `Vec<Box<dyn Handler>>` | one collection, mixed concrete types |
| **Return type depends on runtime input** (factory) | `Box<dyn Greeter>` | caller only knows the trait |
| **Closed protocol** (fixed set of variants) | `enum` + `match` | no vtable; exhaustiveness — see [enums + traits](#enums-structs-and-traits-together) |
| Shared handler across threads | `Arc<dyn AlarmSink + Send + Sync>` | cheap clone, thread-safe ref count |

**Automation sketch — alarm sinks (open set of outputs):**

```rust
// Playground
trait AlarmSink {
    fn emit(&self, code: u8, msg: &str);
}

struct LogSink;
struct MetricsSink;

impl AlarmSink for LogSink {
    fn emit(&self, code: u8, msg: &str) {
        println!("[{}] {}", code, msg);
    }
}
impl AlarmSink for MetricsSink {
    fn emit(&self, code: u8, msg: &str) {
        println!("metric alarm_code={} len={}", code, msg.len());
    }
}

fn raise(sinks: &[&dyn AlarmSink], code: u8, msg: &str) {
    for s in sinks {
        s.emit(code, msg);
    }
}

fn main() {
    let log = LogSink;
    let metrics = MetricsSink;
    raise(&[&log, &metrics], 0x07, "over-temp");
}
```

Borrowed `&dyn AlarmSink` avoids heap allocation when callers already own `LogSink` / `MetricsSink` on the stack. Use a **`Vec<Box<dyn AlarmSink>>`** when sinks are constructed dynamically and stored for the process lifetime.

**Factory returning erased type:**

```rust
// Playground
trait Greeter {
    fn greet(&self) -> String;
}
struct En;
struct Fr;
impl Greeter for En {
    fn greet(&self) -> String {
        "Hello".into()
    }
}
impl Greeter for Fr {
    fn greet(&self) -> String {
        "Bonjour".into()
    }
}

fn greeter_for(lang: &str) -> Box<dyn Greeter> {
    match lang {
        "fr" => Box::new(Fr),
        _ => Box::new(En),
    }
}

fn main() {
    let g = greeter_for("fr");
    println!("{}", g.greet());
}
```

**Why `-> Box<dyn Greeter>` and not `-> impl Greeter`?**

`impl Greeter` in return position means: “this function returns **one** concrete type, picked at compile time — the same type on **every** return path.”

| `match` arm | Concrete type returned |
|-------------|------------------------|
| `"fr" => ...` | `Fr` |
| `_ => ...` | `En` |

Two different types → no single hidden concrete type → `-> impl Greeter` **does not compile**.

| Return type | Works here? | Why |
|-------------|-------------|-----|
| `-> impl Greeter` | no | each arm must return the **same** struct (`En` *or* `Fr`, not both) |
| `-> Box<dyn Greeter>` | yes | erases to one pointer type; runtime picks `En` or `Fr` |
| `-> GreeterKind` (enum) | yes | closed set — `enum { En(En), Fr(Fr) }` + `match` at call site |

Use **`Box<dyn Trait>`** when the caller only needs the trait interface. Use an **`enum`** when the variant set is fixed in your crate and you want exhaustiveness without heap allocation.

### `impl Trait` vs `dyn Trait` vs `enum`

```rust
// Playground — same behaviour, three Rust styles

trait Driver {
    fn poll(&self) -> u32;
}
struct Modbus;
struct OpcUa;
impl Driver for Modbus { fn poll(&self) -> u32 { 502 } }
impl Driver for OpcUa { fn poll(&self) -> u32 { 4840 } }

// 1) Static dispatch — best default
fn port_static(d: &impl Driver) -> u32 { d.poll() }

// 2) Runtime erasure — plug-in list
fn ports_dynamic(drivers: &[&dyn Driver]) -> Vec<u32> {
    drivers.iter().map(|d| d.poll()).collect()
}

// 3) Closed sum — best when variants are fixed in your crate
enum Device {
    Modbus,
    OpcUa,
}
impl Device {
    fn poll(&self) -> u32 {
        match self {
            Device::Modbus => Modbus.poll(),
            Device::OpcUa => OpcUa.poll(),
        }
    }
}

fn main() {
    assert_eq!(port_static(&Modbus), 502);
    assert_eq!(ports_dynamic(&[&Modbus, &OpcUa]), vec![502, 4840]);
    assert_eq!(Device::OpcUa.poll(), 4840);
}
```

### Object safety — not every trait can be `dyn`

A trait is **`dyn` compatible** (object-safe) only if the vtable can list a fixed set of methods. Common **object-unsafe** patterns:

| Trait shape | Why it breaks `dyn` |
|-------------|---------------------|
| `fn read() -> f64` — no `self` | no receiver to dispatch on |
| `fn convert<T>(&self, x: T)` — generic method | vtable cannot hold infinite monomorphizations |
| `fn clone(&self) -> Self` | return type is concrete `Self`, unknown at call site |
| `trait Foo: Sized` | trait object itself is `!Sized` |

**Wrong — not object-safe:**

```rust
// Playground — does not compile
trait Reader {
    fn read() -> f64; // no `self`
}

// fn load(r: &dyn Reader) { ... }
// ERROR: the trait `Reader` is not dyn compatible … consider marking as `#[allow(...)]`
```

**Wrong — generic method on trait:**

```rust
// Playground — does not compile
trait Parse {
    fn parse<T: std::str::FromStr>(&self, s: &str) -> Option<T>;
}
// let p: &dyn Parse = &MyParser;
// ERROR: trait `Parse` is not dyn compatible
```

**Fix for `-> Self`:** return a boxed trait object or associated type with a bound, or use static dispatch (`impl Clone` on generic `T` instead of `dyn Clone`-style patterns). Standard library `Clone` is not dyn-compatible for this reason. You clone concrete types or use `Box<dyn Display>` etc. for other traits.

Cross-reference: the same object-safety trap appears in [enums + traits edge cases](#enums--traits-edge-cases-and-compiler-traps) above.

### More edge cases and compiler traps

**Wrong — `dyn Trait` by value on the stack:**

```rust
// Playground — does not compile
trait Greeter { fn greet(&self) -> String; }
struct En;
impl Greeter for En { fn greet(&self) -> String { "Hi".into() } }

// let g: dyn Greeter = En; // ERROR: the size for values of type `dyn Greeter` cannot be known
```

Use `Box<dyn Greeter>`, `&dyn Greeter`, etc.

**Wrong — heterogeneous `Vec` without homogenizing pointer:**

```rust
// Playground — does not compile
// let v: Vec<dyn Greeter> = vec![En, Fr];
// ERROR: the size for values of type `dyn Greeter` cannot be known
```

**Lifetime on borrowed trait objects** — the reference must not outlive the concrete value:

```rust
// Playground — OK inside one function: `en`/`fr` live long enough
trait Greeter {
    fn greet(&self) -> String;
}
struct En;
struct Fr;
impl Greeter for En {
    fn greet(&self) -> String {
        "Hello".into()
    }
}
impl Greeter for Fr {
    fn greet(&self) -> String {
        "Bonjour".into()
    }
}

fn pick_in_place(use_fr: bool) {
    let en = En;
    let fr = Fr;
    let g: &dyn Greeter = if use_fr { &fr } else { &en };
    println!("{}", g.greet());
}

fn main() {
    pick_in_place(true);
}
```

**Returning `&dyn Greeter` from locals — two compile errors:**

Inside one function, borrowing locals works — `pick_in_place` above keeps `en`/`fr` alive for the whole call. Returning that borrow to the **caller** fails: `en`/`fr` are dropped when `broken` returns, so the trait object would dangle.

```rust
// Playground — does not compile (include Greeter / En / Fr from above, or full copy below)
trait Greeter {
    fn greet(&self) -> String;
}
struct En;
struct Fr;
impl Greeter for En { fn greet(&self) -> String { "Hello".into() } }
impl Greeter for Fr { fn greet(&self) -> String { "Bonjour".into() } }

fn broken(use_fr: bool) -> &dyn Greeter {
    let en = En;
    let fr = Fr;
    if use_fr { &fr } else { &en }
    // ERROR E0106: missing lifetime specifier on `&dyn Greeter`
    // Rust asks: how long does this borrow live? You must name a lifetime.
}

fn main() {}
```

If you add a lifetime (`fn broken<'a>(...) -> &'a dyn Greeter`), the compiler reaches the real problem:

```
ERROR E0515: cannot return value referencing local variable `en` / `fr`
```

The return type promises a borrow, but `en` and `fr` are **owned by `broken` and destroyed at `}`** — nothing for the caller to borrow from.

| Situation | Works? | Pattern |
|-----------|--------|---------|
| Use `&dyn Greeter` inside the same function | yes | `pick_in_place` — locals outlive the borrow |
| Return `&dyn Greeter` to caller from locals | no | E0106, then E0515 |
| Return owned trait object to caller | yes | `-> Box<dyn Greeter>` — `greeter_for` above |

**Fix:** when the factory **creates** the value, return **`Box<dyn Greeter>`** (or pass a caller-owned buffer in). Use **`&dyn Greeter`** only when borrowing something the **caller** already owns.

**Auto traits on trait objects:** `Box<dyn AlarmSink + Send + Sync>` when the handler crosses thread boundaries; omit when single-threaded.

| Trap | Symptom | Idiom |
|------|---------|-------|
| Unsized `dyn` | “size cannot be known at compile time” | `&`, `Box`, `Arc`, never bare `dyn` value |
| Non-object-safe trait | “trait `X` is not dyn compatible” | fix trait API, or use `enum` / generics |
| `-> impl Trait` factory | arms return different types | `Box<dyn Trait>` or `enum` |
| Closed protocol | stringly plug-in names | `enum` beats `dyn` for speed + exhaustiveness |
| Dangling `&dyn` | borrow checker on return | `Box<dyn Trait>` or borrow from caller’s value |

## Trait decomposition — small interfaces

Large “god traits” are hard to mock and test. Split I/O boundaries into **focused traits** that each do one job. The same orchestration code runs in production, CLI, and tests by swapping implementations:

```rust
// Playground
use std::io::{self, Read};

trait ConfigLoader {
    fn load_text(&self, path: &str) -> io::Result<String>;
}

trait PortValidator {
    fn validate(&self, port: u16) -> Result<(), &'static str>;
}

struct FileConfigLoader;

impl ConfigLoader for FileConfigLoader {
    fn load_text(&self, path: &str) -> io::Result<String> {
        std::fs::read_to_string(path)
    }
}

struct PrivilegedPortGuard;

impl PortValidator for PrivilegedPortGuard {
    fn validate(&self, port: u16) -> Result<(), &'static str> {
        if port < 1024 {
            Err("port below 1024")
        } else {
            Ok(())
        }
    }
}

fn startup_port<L: ConfigLoader, V: PortValidator>(
    loader: &L,
    validator: &V,
    path: &str,
) -> io::Result<u16> {
    let text = loader.load_text(path)?;
    let port: u16 = text.trim().parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    validator.validate(port).map_err(|msg| io::Error::new(io::ErrorKind::InvalidInput, msg))?;
    Ok(port)
}

fn main() {
    let loader = FileConfigLoader;
    let guard = PrivilegedPortGuard;
    // startup_port(&loader, &guard, "port.txt") in a real project
    let _ = (&loader as &dyn ConfigLoader, &guard as &dyn PortValidator);
    println!("traits split for test doubles");
}
```

| Split | Role |
|-------|------|
| `ConfigLoader` | where bytes come from (file, env, mock) |
| `PortValidator` | business rule isolated from I/O |
| generic orchestrator | one function, many backend pairs |

In async services, the same split applies with `async fn` traits and the **`async_trait`** crate — see [Chapter 8 — trait mocks](08_errors_and_testing.md#trait-based-mocks--test-orchestration-not-io).

## Transform traits — parse once, map to domain

Keep **wire format** separate from **domain records**. A parser yields raw rows; a transformer maps them to what you persist:

```rust
// Playground
struct RawReading {
    tag: String,
    value: f64,
}

struct StoredReading {
    tag: String,
    value_milli: i64,
}

trait ToStored {
    fn to_stored(&self) -> StoredReading;
}

impl ToStored for RawReading {
    fn to_stored(&self) -> StoredReading {
        StoredReading {
            tag: self.tag.clone(),
            value_milli: (self.value * 1000.0).round() as i64,
        }
    }
}

fn ingest(rows: &[RawReading]) -> Vec<StoredReading> {
    rows.iter().map(|r| r.to_stored()).collect()
}

fn main() {
    let raw = RawReading {
        tag: "temp".into(),
        value: 22.5,
    };
    println!("{:?}", ingest(&[raw]).len());
}
```

Each source (Modbus, OPC-UA, CSV) can implement the same transform trait on its row type. The runner stays generic over “anything that becomes `StoredReading`”.

## Associated types and supertraits

You already implement traits with methods. Many std traits also declare an **associated type** — one output type fixed per `impl`, not a separate generic parameter on the trait itself.

### `Iterator::Item` — one output type per iterator

[Chapter 4 — Implementing Iterator](04_iterators.md#implementing-iterator) defines a port scanner. The associated type pins what `.next()` yields:

```rust
// Playground
struct PortScan {
    next_port: u16,
    end: u16,
}

impl Iterator for PortScan {
    type Item = u16; // associated type — fixed for this impl

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_port > self.end {
            return None;
        }
        let p = self.next_port;
        self.next_port += 1;
        Some(p)
    }
}

fn main() {
    let ports: Vec<u16> = PortScan { next_port: 502, end: 505 }.collect();
    let sum: u16 = PortScan { next_port: 502, end: 504 }.sum();
    println!("ports={:?} sum={}", ports, sum);
}
```

`type Item = u16` tells every adapter (`.map`, `.sum`, `.collect`) what flows through the pipeline. Change it to `String` and the same `impl` body stops compiling — callers and adapters depend on that one associated type.

### Custom trait with `type Output`

When a trait's return type varies **per implementor** but stays **one type per implementor**, use an associated type instead of a generic parameter on the trait:

```rust
// Playground
trait Summarizable {
    type Output;
    fn summarize(&self) -> Self::Output;
}

struct TempReading(f64);
struct PressReading(f64);

impl Summarizable for TempReading {
    type Output = String;
    fn summarize(&self) -> String {
        format!("temp={:.1}C", self.0)
    }
}

impl Summarizable for PressReading {
    type Output = String;
    fn summarize(&self) -> String {
        format!("press={:.0}kPa", self.0)
    }
}

fn log_reading<T: Summarizable>(r: &T)
where
    T::Output: std::fmt::Display,
{
    println!("{}", r.summarize());
}

fn main() {
    log_reading(&TempReading(22.5));
    log_reading(&PressReading(101.3));
}
```

### Associated type vs generic trait parameter

| Style | Example | When |
|-------|---------|------|
| Associated type | `trait Iterator { type Item; … }` | **One** output type per implementor (`u16` ports, `&str` lines) |
| Generic param | `trait Storage<T> { fn get(&self) -> T; }` | Caller picks `T` at each call site — many operations per implementor |
| Both in std | `FromStr` with `type Err` | Error type is fixed per parsed type, like `Item` for iterators |

`Iterator` could have been `trait Iterator<T> { fn next(&mut self) -> Option<T>; }` — but then every function taking "some iterator" would need an extra type parameter. Associated types keep call sites clean.

### Supertraits — stacking requirements

A **supertrait** requires another trait as a prerequisite. Your type must implement both:

```rust
// Playground
use std::fmt::Display;

trait Loggable: Display {
    fn log(&self) {
        println!("[LOG] {}", self);
    }
}

struct Port(u16);
impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Port({})", self.0)
    }
}
impl Loggable for Port {}

fn emit(p: &impl Loggable) {
    p.log();
}

fn main() {
    emit(&Port(502));
}
```

`Loggable: Display` means "anything loggable must also be displayable." The default method can call `Display` formatting inside the trait.

### UFCS when two traits share a method name

**Wrong — ambiguous method call:**

```rust
// Playground — does not compile
trait A { fn id(&self) -> u32; }
trait B { fn id(&self) -> u32; }
struct Tag;
impl A for Tag { fn id(&self) -> u32 { 1 } }
impl B for Tag { fn id(&self) -> u32 { 2 } }

fn main() {
    let t = Tag;
    // println!("{}", t.id()); // ERROR: multiple applicable items in scope
    println!("A={} B={}", A::id(&t), B::id(&t));
}
```

Use **UFCS** — `TraitName::method(&self)` — when two traits define the same method name.

## Idiom spotlight

> **Default: `impl Trait` / generics (static dispatch).** Use **`dyn Trait`** for runtime plug-in lists or return-type erasure — and **`enum`** when the variant set is closed.
>
> **`&dyn Trait`** when callers own values; **`Box<dyn Trait>`** for owning heterogeneous collections.
>
> **Split large traits** into small getters/savers; **enum-dispatch** fixed backends; **transform traits** bridge wire format → domain.

## Go deeper

- [The Rust Book — Structs and Methods](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [The Rust Book — Generics, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [The Rust Book — Trait Objects](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)
- [Records / structs](https://hightechmind.io/rust/) — example 062

## See also

- [Chapter 6: Enums and pattern matching](06_types_enums_pattern_matching.md) — `match`, exhaustiveness, `Option`/`Result`
- [Chapter 4: Iterators](04_iterators.md#implementing-iterator) — custom `Iterator` and `type Item`
- [Chapter 11: Collections](11_collections.md)
- [Chapter 17: Derive attributes](17_metaprogramming.md#derive-attributes) — `#[derive]` syntax, std/ecosystem/custom
- [Chapter 16: Async traits](16_async_tokio.md) (advanced)

### Afterparty

#### Structs and inherent `impl`

1. **new vs default** — “When is `Sensor::new` idiomatic vs `Default` + field update? One automation example each.”
2. **Method receiver** — “Same logic three ways: `fn f(self)`, `fn f(&self)`, `fn f(&mut self)` on a struct; I predict what calls compile.”



#### Enums, structs, and traits together

3. **Enum trait impl** — “Add variant `Fault` to `Status`; show every compile error until trait `impl` and inherent methods match.”
4. **Struct vs enum layout** — “Modbus/OpcUa device registry: justify `struct Device { kind: Enum }` vs `enum Device { Modbus(...), OpcUa(...) }`; sketch types.”
5. **SensorReading port** — “Python `Union[TempReading, PressReading, Skipped]` → Rust enum + struct payloads + `Measurable` trait; show `match` in impl.”
6. **Two impl blocks** — “Same type: add inherent `is_valid()` and trait `Summary`; explain which call sites see which methods.”
7. **Partial move fix** — “Given `enum Packet { Raw(String), Empty }`, reproduce ‘partially moved value’ and rewrite with `&self` or one `match`.”
8. **matches! drill** — “Rewrite `!matches!(self, Skipped { .. })` as longhand `match`; then back to `matches!`; link to macro_rules conceptually.”

#### Default trait methods and UFCS

9. **Default override** — “Trait `HasCode` with default `label()`; override for one enum variant only; use `HasCode::label(self)` for the rest.”
10. **Recursion trap** — “Show infinite recursion when override calls `self.label()` instead of `HasCode::label(self)`; fix it.”
11. **One trait per impl** — “Show `impl HasCode, Display for T` compile error; split into two blocks; add `fn show<T: HasCode + Display>(x: T)`.”

#### `#[derive]` and shared models

12. **Command + log row** — “`enum Command` + `struct SetSpeedLog` + `to_log()`; list which derives each type needs and why.”
13. **Eq on floats** — “Add `Analog(f64)` variant; show `#[derive(Eq)]` failure; fix with integer fixed-point or `PartialEq` only.”

#### Generics and trait bounds

14. **Generic bounds** — “Fix compiler error: `T` needs `Display + Clone`; minimal bound set on `fn duplicate_and_print<T>(x: T)`.”
15. **largest pitfalls** — “Why does `largest` need non-empty slice? Add `Option` return or document panic; compare to Java generics erasure story.”
16. **where clause** — “Rewrite `fn f<T: A + B + C>(x: T)` with a `where` block; same signature, longer trait list.”

#### `impl Trait` vs `dyn Trait` vs `enum`

17. **dyn vs impl quiz** — “Four scenarios (plug-in Vec, single helper fn, closed protocol, factory return) — pick `impl`, `dyn`, or `enum` each time.”
18. **AlarmSink registry** — “Implement `&[&dyn AlarmSink]` for log + metrics; then refactor to `enum Sink` if set is closed — compare trade-offs.”
19. **Factory return** — “`greeter_for(lang) -> Box<dyn Greeter>` vs `-> impl Greeter` — show why `impl` fails when arms return `En` and `Fr`.”
20. **Driver three ways** — “Same `poll()` behaviour with `&impl Driver`, `&[&dyn Driver]`, and `enum Device`; benchmark story without running code.”
21. **Box vs borrow** — “When is `&dyn Trait` enough vs `Box<dyn Trait>` required? Vec of mixed types + dangling return examples.”

#### Object safety and `dyn` traps

22. **Object safety audit** — “Mark each trait dyn-safe or not: no-`self` method, generic method, `-> Self`, `trait Foo: Sized`.”
23. **Reader fix** — “Trait with `fn read() -> f64` fails as `dyn`; redesign for `&dyn Reader` or use enum.”
24. **Clone not dyn** — “Why no `Box<dyn Clone>` pattern for heterogenous clone list; suggest enum or generic `T: Clone` instead.”
25. **Send + Sync** — “Alarm handler shared across threads: write type as `Arc<dyn AlarmSink + Send + Sync>`; what breaks if handler holds `Rc`?”
26. **Unsized trap** — “Show three snippets that fail: `let g: dyn Greeter = En`, `Vec<dyn Greeter>`, returning `&dyn` from locals; fix each.”

#### Orphan rule and cross-crate patterns

27. **Orphan rule** — “Why `impl Display for Vec<u8>` fails; fix with newtype `struct Frame(pub Vec<u8>)` + `impl Display for Frame`.”
28. **External trait** — “Wrap third-party struct; implement your trait on the wrapper; call from automation main.”

#### Capstone drills

29. **Checklist drill** — “Match 8 Chapter 7 compiler errors to snippets (non-exhaustive match, partial move, orphan, not dyn compatible, unsized, moved self, Eq+f64, multi-trait impl).”
30. **PLC message model** — “Design full model: enum frames, struct payloads, two traits, one `dyn` registry for sinks, derive list — no code over 80 lines.”
31. **Refactor story** — “Start with `Vec<Box<dyn Driver>>`; protocol closes to two devices; refactor to `enum`; list what the compiler now catches.”

#### Associated types and supertraits

32. **Item type quiz** — "Change `PortScan`'s `type Item` from `u16` to `(u16, bool)` — list every call site in a `.map().collect()` chain that breaks and why."
33. **Summarizable design** — "Add `Summarizable` with `type Output = String` for three sensor structs; one `fn report(r: &impl Summarizable)` — no duplicate return types in the trait."
34. **Associated vs generic** — "Same cache API twice: `trait Get<T>` vs `trait Get { type Value; }` — when is each painful at call sites?"
35. **Supertrait bounds** — "Trait `Exportable: Display + Debug` with default `fn export` — show impl for `Port(u16)`; what fails if `Display` is missing?"
36. **UFCS fix** — "Type implements `A` and `B`, both define `name()` — show ambiguous call error and UFCS fix with `A::name(&x)`."

