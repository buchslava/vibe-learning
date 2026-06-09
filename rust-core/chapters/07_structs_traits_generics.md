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


You will use traits everywhere: `Display`, `Clone`, `Iterator`, custom `Measurable` / `Summary` in automation code. Start with `impl Trait for MyType` and `fn f(x: &impl Trait)`; reach for `dyn Trait` later when types truly differ at runtime (see below).

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

#### Calling the trait’s default method from your override

Yes — `other.default_label()` deliberately calls the **default body written on the trait**, not your override. That is how you reuse the trait’s fallback for some cases (here, `OverTemp`) while customising others (`CommLost`).

**Footgun:** `HasCode::label(other)` or `other.label()` inside `impl HasCode for Fault` still dispatches to **this override** and can recurse forever. Extract the default into a separate trait method (here `default_label`) or inline `format!("code {}", other.code())`.


| Call                     | What runs                                                                                             |
| ------------------------ | ----------------------------------------------------------------------------------------------------- |
| `fault.label()`          | **Dynamic dispatch on the impl** — always the overridden `label` in `impl HasCode for Fault`          |
| `fault.default_label()` | **Default trait body** — `format!("code {}", self.code())`, which still uses your overridden `code()` |


**Not Java overloading.** Rust has **no** method overloading (same name, different parameter lists). There is only one `label(&self) -> String` here. What looks like “pick another version” is **explicit qualified call syntax**: `Trait::method(receiver)`. This is sometimes called UFCS (universal function call syntax).

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

Use `other.default_label()` (or duplicate the default logic) when you want the **trait default**; use `other.label()` only when you intend the **full override path** (including for `CommLost`).

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

`SetSpeedLog` is the **struct side** of the same automation model. The enum is the wire/command shape (`Stop`, `SetSpeed(1500)`); the struct is the persisted audit row (`at` + `rpm`). `Command::to_log` bridges them — only `SetSpeed` produces a `SetSpeedLog`; `Stop` returns `None`.

Both types share `#[derive(...)]` because you typically want the same tooling (`Debug`, `Clone`, equality) on commands and the records they generate. What `#[derive]` is (compile-time codegen, not annotations/decorators), ecosystem vs custom derives — [Chapter 17: Derive attributes](17_metaprogramming.md#derive-attributes).

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

Only **object-safe** traits can become `dyn Trait` — `&self` methods, no generic methods on the trait itself. Enums with trait impls usually use **static dispatch** (`impl Trait` / generics) instead.

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

`-> impl Greeter` cannot replace this if the two arms return **different** concrete types. `impl Trait` in return position hides a **single** concrete type per function body. For “either `En` or `Fr`”, use `Box<dyn Greeter>` or an `enum`.

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

In async services, the same split applies with `async fn` traits — see [Chapter 16 — async trait boundaries](16_async_tokio.md#async-trait-boundaries-for-testing).

## Strategy enum — closed dispatch without a top-level vtable

When you have a **fixed set** of backends, an internal **`enum`** often beats `Box<dyn Trait>` at the runner layer. Each variant holds its parser/state; one `match` dispatches:

```rust
// Playground
enum SourceKind {
    Modbus { unit_id: u8 },
    OpcUa { endpoint: String },
}

impl SourceKind {
    fn default_port(&self) -> u16 {
        match self {
            SourceKind::Modbus { .. } => 502,
            SourceKind::OpcUa { .. } => 4840,
        }
    }

    fn label(&self) -> &str {
        match self {
            SourceKind::Modbus { .. } => "modbus",
            SourceKind::OpcUa { .. } => "opcua",
        }
    }
}

fn run_source(src: &SourceKind) {
    println!("{} on port {}", src.label(), src.default_port());
}

fn main() {
    run_source(&SourceKind::Modbus { unit_id: 1 });
    run_source(&SourceKind::OpcUa {
        endpoint: "opc://plc".into(),
    });
}
```

Use **`enum` + `match`** when variants are known at compile time in your crate. Use **`dyn Trait`** when plug-ins arrive at runtime from outside. Many production runners combine both: enum at the top, `impl Stream` or trait objects **inside** a variant when needed.

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

## Extension traits in your crate

Extension traits add methods without wrapping every value in a newtype. Typical targets: **`Option<T>`** for required-field checks, **`&str`** for normalizers ([Chapter 13](13_standard_traits.md#extension-traits-on-str)), **`Stream`** adapters in async pipelines ([Chapter 16](16_async_tokio.md)).

```rust
// Playground
trait RequiredField<T> {
    fn required(self, name: &str) -> Result<T, String>;
}

impl<T> RequiredField<T> for Option<T> {
    fn required(self, name: &str) -> Result<T, String> {
        self.ok_or_else(|| format!("missing field '{name}'"))
    }
}

fn parse_id(raw: Option<&str>) -> Result<u32, String> {
    let s = raw.required("device_id")?;
    s.parse().map_err(|e| e.to_string())
}

fn main() {
    println!("{:?}", parse_id(Some("42")));
    println!("{:?}", parse_id(None));
}
```

Implement extension traits in the **same module** as the error type they produce — keeps `?` chains readable in hand-written parsers.

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

### Associated types and object safety

Traits with `type Item` or `type Err` can still be object-safe when methods don't return bare `Self`. `Clone` returns `Self` — that is why `dyn Clone` is awkward. `Iterator` is object-safe, but boxing iterators is rare; prefer `impl Iterator` or generics.

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

