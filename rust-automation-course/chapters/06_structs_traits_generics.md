# Chapter 6: Structs, Traits, and Generics

## Hook

Java: classes bundle data + inheritance. Python: duck typing (“if it quacks…”). Rust: **structs** for data, `**impl`** for methods, **traits** for shared behaviour, **enums** for closed alternatives — **composition over inheritance**, checked at compile time.

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

If you are coming from **Java** or **Python**, a **trait** is the piece that may feel most unfamiliar — and most useful.

**What it is.** A trait names a **capability** or **contract**: a set of methods any type can opt into. `Reading`, `Alarm`, and a third struct do not need a common base class; each implements `Summary` (or not) in its own `impl` block.

**Aim.** Let unrelated types share behaviour **without inheritance**. You write `fn print_summary(item: &impl Summary)` once; anything that implements `Summary` can be passed in — like a Java interface parameter or a Python function that “just calls `.summarize()`”, except Rust checks that at **compile time**.

**Motivation.** Rust deliberately has no subclassing. Traits replace “extend a base class to get polymorphism” with **composition**: small data types (`struct` / `enum`) plus explicit `impl Trait for Type`. When the protocol grows, you add a method to the trait and the compiler lists every type that must be updated — no silent runtime `AttributeError`, no forgotten override in a distant subclass.

**Why traits are a win**


| Benefit               | Java / Python pain                                  | Rust trait answer                                                           |
| --------------------- | --------------------------------------------------- | --------------------------------------------------------------------------- |
| Checked contracts     | interface optional; duck typing fails at runtime    | missing method = **compile error**                                          |
| No hierarchy tax      | deep trees, fragile `super` chains                  | flat types + `impl` blocks                                                  |
| Cross-crate reuse     | can’t add methods to `String` / third-party classes | implement **your** trait for **your** wrapper (orphan rule applies)         |
| Performance           | interface dispatch / dynamic checks                 | **static dispatch** by default (`impl Trait` → monomorphization, no vtable) |
| Optional polymorphism | always reference types / ABCs                       | `dyn Trait` only when you need mixed-type collections                       |


You will use traits everywhere: `Display`, `Clone`, `Iterator`, custom `Measurable` / `Summary` in automation code. Start with `**impl Trait for MyType`** and `**fn f(x: &impl Trait)**`; reach for `dyn Trait` later when types truly differ at runtime (see below).

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

[Chapter 5](05_types_enums_pattern_matching.md) gave you `enum` + exhaustive `match`. This section is the missing piece: **how structs and traits attach to enums** — Rust’s substitute for a class hierarchy or a Python `Union` of unrelated types.

The idiomatic shape for automation and protocol code:

1. `**enum`** — closed set of states or message kinds (compiler tracks every variant).
2. `**struct**` — per-variant payload when a variant carries real data.
3. `**trait**` — shared behaviour across *different* types (`Reading`, `Alarm`, …), or a uniform API over one enum.
4. `**impl*`* — inherent methods on the enum *and/or* trait implementations that `**match` inside**.

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

- `**matches!(value, pattern)`** — standard-library **macro** ([Chapter 13: Metaprogramming](13_metaprogramming.md)) that expands to a `match` returning `true` or `false`. It is syntax sugar for “does this value fit this pattern?” without binding variables you do not need.
- `**SensorReading::Skipped { .. }`** — matches the `Skipped` struct variant and **ignores** the `reason` field (`..` = “other fields don’t matter for this test”). Same pattern token as in [Chapter 5](05_types_enums_pattern_matching.md).
- `**!`** — negates the result: `Temp` and `Press` → `true`; `Skipped { .. }` → `false`.

Equivalent without the macro:

```rust
// Playground
fn is_valid_longhand(r: &SensorReading) -> bool {
    match r {
        SensorReading::Skipped { .. } => false,
        _ => true,
    }
}
```

Use `matches!` for readable guards and filters; use full `match` when each arm returns different data. More on declarative macros: [Chapter 13 — `macro_rules!](13_metaprogramming.md#macro_rules)`.

### Two `impl` blocks on one type


| Block                                       | Role                                                       | Java / Python analogy    |
| ------------------------------------------- | ---------------------------------------------------------- | ------------------------ |
| `impl SensorReading { ... }`                | **Inherent** methods — always available on `SensorReading` | methods on the class     |
| `impl Measurable for SensorReading { ... }` | **Trait** impl — call only where `Measurable` is required  | interface implementation |


Both on the same `enum` is normal. Put variant-specific logic in inherent methods; put cross-type contracts on traits.

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

Unlike a single `struct`, an `enum` trait body almost always `**match self`** (or `match &self`) — one arm per variant. That mirrors [Chapter 5](05_types_enums_pattern_matching.md) exhaustiveness: add a variant, and the compiler lists every `impl` and `match` you must update.

**Default trait methods** still work — override only where a variant differs:

```rust
// Playground
trait HasCode {
    fn code(&self) -> u8;
    fn label(&self) -> String {
        format!("code {}", self.code())
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
            other => HasCode::label(other), // default for OverTemp
        }
    }
}

fn main() {
    println!("{}", Fault::CommLost.label());
    println!("{}", Fault::OverTemp(0x0A).label()); // default: "code 10"
}
```

#### Calling the trait’s default method from your override

Yes — `**HasCode::label(other)**` deliberately calls the **default body written on the trait**, not your override. That is how you reuse the trait’s fallback for some cases (here, `OverTemp`) while customising others (`CommLost`).


| Call                     | What runs                                                                                             |
| ------------------------ | ----------------------------------------------------------------------------------------------------- |
| `fault.label()`          | **Dynamic dispatch on the impl** — always the overridden `label` in `impl HasCode for Fault`          |
| `HasCode::label(&fault)` | **Default trait body** — `format!("code {}", self.code())`, which still uses your overridden `code()` |


**Not Java overloading.** Rust has **no** method overloading (same name, different parameter lists). There is only one `label(&self) -> String` here. What looks like “pick another version” is **explicit qualified call syntax**: `Trait::method(receiver)` — sometimes called UFCS (universal function call syntax).

**Closer Java analogy:** a **default interface method**, not overloads:

```java
// Java — conceptual parallel
interface HasCode {
    int code();
    default String label() { return "code " + code(); }
}
class Fault implements HasCode {
    public String label() {
        if (isCommLost()) return "communication lost";
        return HasCode.super.label(); // call default, not recurse into self.label()
    }
}
```

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

Use `HasCode::label(other)` when you want the **trait default**; use `other.label()` only when you intend the **full override path** (including for `CommLost`).

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


So: `**impl HasCode for Fault`** and `**impl Display for Fault**` are two separate blocks; `**T: HasCode + Display**` means “T implements both.”

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

`SetSpeedLog` is the **struct side** of the same automation model: the enum is the wire/command shape (`Stop`, `SetSpeed(1500)`); the struct is the persisted audit row (`at` + `rpm`). `Command::to_log` bridges them — only `SetSpeed` produces a `SetSpeedLog`; `Stop` returns `None`. Both types share `#[derive(...)]` because you typically want the same tooling (`Debug`, `Clone`, equality) on commands and the records they generate.

`PartialEq`/`Eq` on enums requires every payload type to be comparable. A variant holding `f64` forces you to drop `Eq` or model floats differently — see edge cases below.

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

**Wrong — assume `dyn Trait` works with every trait:**

```rust
// Playground — does not compile
trait Measurable {
    fn value(&self) -> f64;
    fn read() -> f64; // no `self` — not object-safe
}

// let items: Vec<Box<dyn Measurable>> = ...;
// ERROR: trait `Measurable` is not dyn compatible
```

Only **object-safe** traits (`&self` methods, no generic methods on the trait itself) can become `dyn Trait`. Enums with trait impls usually use **static dispatch** (`impl Trait` / generics) instead.

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

> **Model closed protocol/state sets as `enum`, not strings.** Put heavy data in named **struct** variants; implement **traits** with exhaustive `match`.
>
> **Add a variant → compiler updates your checklist.** That is Rust replacing inheritance: no forgotten `else` when the PLC adds fault code 0x07.
>
> **Prefer `&self` methods on enums** unless consuming the value is intentional — avoids partial-move footguns when composing helpers.

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

**Generics and `impl Trait`** pick the concrete type at **compile time** (monomorphization — one machine-code copy per type). A **`dyn Trait`** value is **runtime polymorphism**: one variable, many concrete types, resolved through a **vtable** (virtual method table). Java `Interface` references and Python “anything with `.greet()`” at runtime are closer to this model.

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

`Box` in the `Vec` **homogenizes** size: every element is one pointer pair. Without `Box`, Rust cannot lay out a vector of different struct sizes in one array.

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

Borrowed `&dyn AlarmSink` avoids heap allocation when callers already own `LogSink` / `MetricsSink` on the stack. A **`Vec<Box<dyn AlarmSink>>`** is the next step when sinks are constructed dynamically and stored for the process lifetime.

**Factory returning erased type:**

```rust
// Playground
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

`-> impl Greeter` cannot replace this if the two arms return **different** concrete types — `impl Trait` in return position hides a **single** concrete type per function body. For “either `En` or `Fr`”, use `Box<dyn Greeter>` or an `enum`.

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

**Fix for `-> Self`:** return a boxed trait object or associated type with a bound, or use static dispatch (`impl Clone` on generic `T` instead of `dyn Clone`-style patterns). Standard library `Clone` is not dyn-compatible for this reason — you clone concrete types or use `Box<dyn Display>` etc. for other traits.

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
fn pick_in_place(use_fr: bool) {
    let en = En;
    let fr = Fr;
    let g: &dyn Greeter = if use_fr { &fr } else { &en };
    println!("{}", g.greet());
}
```

Returning `&dyn Greeter` from a function that creates locals **does not compile** (dangling):

```rust
// Playground — does not compile
fn broken(use_fr: bool) -> &dyn Greeter {
    let en = En;
    let fr = Fr;
    if use_fr { &fr } else { &en }
    // ERROR: returns a reference to data owned by the current function
}
```

Return `Box<dyn Greeter>` when the callee allocates/chooses the value.

**Auto traits on trait objects:** `Box<dyn AlarmSink + Send + Sync>` when the handler crosses thread boundaries; omit when single-threaded.

| Trap | Symptom | Idiom |
|------|---------|-------|
| Unsized `dyn` | “size cannot be known at compile time” | `&`, `Box`, `Arc`, never bare `dyn` value |
| Non-object-safe trait | “trait `X` is not dyn compatible” | fix trait API, or use `enum` / generics |
| `-> impl Trait` factory | arms return different types | `Box<dyn Trait>` or `enum` |
| Closed protocol | stringly plug-in names | `enum` beats `dyn` for speed + exhaustiveness |
| Dangling `&dyn` | borrow checker on return | `Box<dyn Trait>` or borrow from caller’s value |

## Idiom spotlight

> **Default: `impl Trait` / generics (static dispatch).** Use **`dyn Trait`** for runtime plug-in lists, factories that erase type, or FFI-style extension points — and prefer **`enum`** when the variant set is closed and owned by your crate.
>
> **`&dyn Trait`** when callers own the concrete values; **`Box<dyn Trait>`** when you need an owning homogeneous collection or return type erasure.

## Go deeper

- [Records / structs](https://hightechmind.io/rust/) — example 062
- Archive: [CHAPTER_01 §4](../archive/CHAPTER_01_RUST_BASICS.md)

## See also

- [Chapter 5: Enums and pattern matching](05_types_enums_pattern_matching.md) — `match`, exhaustiveness, `Option`/`Result`
- [Chapter 3: Iterators](03_iterators.md)
- [Chapter 8: Collections](08_collections_iterators.md)
- [Chapter 12: Async traits](12_async_tokio.md) (advanced)

### Afterparty: AI Lego blocks

Copy a prompt into your AI tutor. Insist on **compiler-accurate** answers — quote error text, show fixed code, and say *why*.

#### Structs and inherent `impl`

1. **Sensor struct** — “Port a Java `Sensor` class (fields + constructor + `scaled()`) to Rust `struct` + `impl`; no getters unless needed.”
2. **new vs default** — “When is `Sensor::new` idiomatic vs `Default` + field update? One automation example each.”
3. **Method receiver** — “Same logic three ways: `fn f(self)`, `fn f(&self)`, `fn f(&mut self)` on a struct; I predict what calls compile.”

#### Traits for Java / Python refugees

4. **Interface port** — “Convert Java interface `Measurable` + two classes to trait + two structs + `impl Measurable for` each.”
5. **Duck typing** — “Python function accepts anything with `.read()`; express as trait bound on a generic `fn load<T: ReadSource>(...)`.”
6. **OOP myth** — “Explain in 100 words why Rust has no inheritance; map Java `extends`/`implements` to struct/trait/enum.”
7. **Trait vs interface table** — “Fill gaps: default methods, static dispatch, orphan rule, `dyn` — compare Java interface vs Rust trait.”

#### Enums, structs, and traits together

8. **Enum trait impl** — “Add variant `Fault` to `Status`; show every compile error until trait `impl` and inherent methods match.”
9. **Struct vs enum layout** — “Modbus/OpcUa device registry: justify `struct Device { kind: Enum }` vs `enum Device { Modbus(...), OpcUa(...) }`; sketch types.”
10. **SensorReading port** — “Python `Union[TempReading, PressReading, Skipped]` → Rust enum + struct payloads + `Measurable` trait; show `match` in impl.”
11. **Two impl blocks** — “Same type: add inherent `is_valid()` and trait `Summary`; explain which call sites see which methods.”
12. **Partial move fix** — “Given `enum Packet { Raw(String), Empty }`, reproduce ‘partially moved value’ and rewrite with `&self` or one `match`.”
13. **matches! drill** — “Rewrite `!matches!(self, Skipped { .. })` as longhand `match`; then back to `matches!`; link to macro_rules conceptually.”

#### Default trait methods and UFCS

14. **Default override** — “Trait `HasCode` with default `label()`; override for one enum variant only; use `HasCode::label(self)` for the rest.”
15. **Not overloading** — “Explain why `HasCode::label(other)` is not Java overloading; compare to `interface.super.method()`.”
16. **Recursion trap** — “Show infinite recursion when override calls `self.label()` instead of `HasCode::label(self)`; fix it.”
17. **One trait per impl** — “Show `impl HasCode, Display for T` compile error; split into two blocks; add `fn show<T: HasCode + Display>(x: T)`.”

#### `#[derive]` and shared models

18. **Command + log row** — “`enum Command` + `struct SetSpeedLog` + `to_log()`; list which derives each type needs and why.”
19. **Eq on floats** — “Add `Analog(f64)` variant; show `#[derive(Eq)]` failure; fix with integer fixed-point or `PartialEq` only.”
20. **Derive vs manual** — “When would you hand-write `impl Debug` instead of `#[derive(Debug)]` on an automation enum?”

#### Generics and trait bounds

21. **Generic bounds** — “Fix compiler error: `T` needs `Display + Clone`; minimal bound set on `fn duplicate_and_print<T>(x: T)`.”
22. **largest pitfalls** — “Why does `largest` need non-empty slice? Add `Option` return or document panic; compare to Java generics erasure story.”
23. **where clause** — “Rewrite `fn f<T: A + B + C>(x: T)` with a `where` block; same signature, longer trait list.”
24. **Monomorphization** — “Explain in 60 words what the compiler generates for `show(42)` and `show("text")` with `fn show<T: Display>(x: T)`.”

#### `impl Trait` vs `dyn Trait` vs `enum`

25. **dyn vs impl quiz** — “Four scenarios (plug-in Vec, single helper fn, closed protocol, factory return) — pick `impl`, `dyn`, or `enum` each time.”
26. **AlarmSink registry** — “Implement `&[&dyn AlarmSink]` for log + metrics; then refactor to `enum Sink` if set is closed — compare trade-offs.”
27. **Factory return** — “`greeter_for(lang) -> Box<dyn Greeter>` vs `-> impl Greeter` — show why `impl` fails when arms return `En` and `Fr`.”
28. **Driver three ways** — “Same `poll()` behaviour with `&impl Driver`, `&[&dyn Driver]`, and `enum Device`; benchmark story without running code.”
29. **Box vs borrow** — “When is `&dyn Trait` enough vs `Box<dyn Trait>` required? Vec of mixed types + dangling return examples.”

#### Object safety and `dyn` traps

30. **Object safety audit** — “Mark each trait dyn-safe or not: no-`self` method, generic method, `-> Self`, `trait Foo: Sized`.”
31. **Reader fix** — “Trait with `fn read() -> f64` fails as `dyn`; redesign for `&dyn Reader` or use enum.”
32. **Clone not dyn** — “Why no `Box<dyn Clone>` pattern for heterogenous clone list; suggest enum or generic `T: Clone` instead.”
33. **Send + Sync** — “Alarm handler shared across threads: write type as `Arc<dyn AlarmSink + Send + Sync>`; what breaks if handler holds `Rc`?”
34. **Unsized trap** — “Show three snippets that fail: `let g: dyn Greeter = En`, `Vec<dyn Greeter>`, returning `&dyn` from locals; fix each.”

#### Orphan rule and cross-crate patterns

35. **Orphan rule** — “Why `impl Display for Vec<u8>` fails; fix with newtype `struct Frame(pub Vec<u8>)` + `impl Display for Frame`.”
36. **External trait** — “Wrap third-party struct; implement your trait on the wrapper; call from automation main.”

#### Capstone drills

37. **Checklist drill** — “Match 8 Chapter 6 compiler errors to snippets (non-exhaustive match, partial move, orphan, not dyn compatible, unsized, moved self, Eq+f64, multi-trait impl).”
38. **PLC message model** — “Design full model: enum frames, struct payloads, two traits, one `dyn` registry for sinks, derive list — no code over 80 lines.”
39. **Java hierarchy kill** — “Given Java abstract `Device` + `ModbusDevice` + `OpcUaDevice`, produce Rust enum+struct+trait layout; no `dyn` unless I ask for plug-ins.”
40. **Refactor story** — “Start with `Vec<Box<dyn Driver>>`; protocol closes to two devices; refactor to `enum`; list what the compiler now catches.”

