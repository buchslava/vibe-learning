# Chapter 1: Rust Basics

**Course:** Rust as a pre-part of Computer Systems  
**Audience:** University students (Automation area), familiar with Python  
**Estimated duration:** ~14 hours (≈70% lectures, 30% practice)

---

## Table of Contents

1. [Cargo and Installation](#1-cargo-and-installation)
2. [Rust vs Python: Ownership and Borrowing](#2-rust-vs-python-ownership-and-borrowing)
3. [Passing by Value, Reference, and Cloning](#3-passing-by-value-reference-and-cloning)
4. [Functions, Structs, and Rust’s Approach to OOP](#4-functions-structs-and-rusts-approach-to-oop)

---

## 1. Cargo and Installation

**Duration:** ~1.5 h (lecture ~1 h, practice ~0.5 h)

### 1.1 Why Cargo?

In Python you typically use `pip` for packages and run scripts with `python script.py`. Rust uses **Cargo** as the single tool for:

- Creating and building projects
- Managing dependencies (see below)
- Running tests and benchmarks
- Generating documentation

There is no “run a single file” as the default workflow; the unit of work is the **package** (project), which keeps dependencies and build reproducible—important for automation and embedded targets.

**Package management: what and why.**  
Modern software is built by composing **packages** (reusable units of code). A **package manager** is responsible for: (1) defining what your project depends on and in which versions; (2) **resolving** a consistent set of versions for your direct dependencies and all of their dependencies (the *dependency tree*), so that no two packages require incompatible versions of the same library; (3) **fetching** those packages from a **registry** (a central or private catalog) or other sources; (4) **building** and linking them so your project can use them. Good package management gives you **reproducibility**: the same manifest and lockfile produce the same dependency set and build everywhere (developer machines, CI, production). It also keeps **isolation** (per-project dependency sets avoid conflicts) and **version semantics** (e.g. semantic versioning) so that compatible updates are chosen automatically while breaking changes are constrained. In system and automation contexts, predictable, reproducible builds are essential for deployment and maintenance.

**Package management in Cargo (compared to Python):**

- **Declarative dependencies** – You list crates (packages) and version constraints in `Cargo.toml` under `[dependencies]`, similar to `requirements.txt`, but Cargo **resolves** a consistent set of versions for the whole dependency tree. You do not run a separate “install” step; the first `cargo build` fetches and compiles what is needed.
- **Central registry** – Most public crates are published on **crates.io**. You can search and browse [crates.io](https://crates.io) to find any package (by name, keyword, or category); each crate page shows the exact line to add to `Cargo.toml`, documentation, and download stats. Adding a dependency is then just a line in `Cargo.toml` (e.g. `serde = "1.0"`); Cargo downloads from the registry. Git repositories and local paths are also supported for private or unreleased code.
- **Version semantics** – You specify a **version requirement** (e.g. `"1.2.3"`, `"^1.0"` for semver-compatible, or `"=1.2.3"` for exact). Cargo picks the latest compatible version and records it in `Cargo.lock`.
- **Reproducible builds** – The **`Cargo.lock`** file pins exact versions of every dependency (including transitive ones). For applications and automation projects, you commit `Cargo.lock` so that everyone (and CI) gets the same build. For libraries, `Cargo.lock` is often not committed, so dependents can resolve within your declared ranges.
- **No global site-packages** – Each project has its own resolved dependency graph and build artifacts. There is no “virtualenv” step; isolation is per project by default.
- **Dev and build dependencies** – Optional or test-only crates go under `[dev-dependencies]` or `[build-dependencies]`, so they do not bloat the main binary or leak into the public API.

### 1.2 Installing Rust

Install the toolchain with **rustup** (recommended):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then verify:

```bash
rustc --version   # compiler
cargo --version   # build tool
```

- **rustc**: compiler (you rarely call it directly).
- **cargo**: project manager; you use it daily.

### 1.3 Cargo Concepts

| Concept        | Python analogue      | In Rust / Cargo                          |
|----------------|----------------------|------------------------------------------|
| Project        | Repository / folder  | A **package** (with `Cargo.toml`)        |
| Dependencies   | `requirements.txt`  | `[dependencies]` in `Cargo.toml`         |
| Virtual env    | `venv`               | Each project has its own build/artifacts |
| Run script     | `python main.py`     | `cargo run`                              |
| Install package| `pip install`        | Add to `Cargo.toml`, then `cargo build`  |

**Key files:**

- **`Cargo.toml`** – manifest: package name, version, dependencies.
- **`src/main.rs`** – default binary entry (like `if __name__ == "__main__"` in Python).
- **`Cargo.lock`** – locked dependency versions (commit this for applications).

### 1.4 Practical Example: First Cargo Project

**Goal:** Create a small CLI that prints a greeting and the current Cargo environment.

**Steps:**

```bash
cargo new hello_automation --bin
cd hello_automation
```

**File: `Cargo.toml`** (after creation):

```toml
[package]
name = "hello_automation"
version = "0.1.0"
edition = "2021"

[dependencies]
```

**File: `src/main.rs`:**

```rust
fn main() {
    let project_name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    println!("Hello, Automation! This is {} v{}", project_name, version);
    println!("Rust is ready for system programming.");
}
```

**How it works:**

- `cargo new --bin` creates a binary package and a `main()` entry point.
- `env!("CARGO_PKG_NAME")` and `env!("CARGO_PKG_VERSION")` are compile-time constants from `Cargo.toml`.

**Run:**

```bash
cargo run
```

**Expected output:**

```
Hello, Automation! This is hello_automation v0.1.0
Rust is ready for system programming.
```

**Input:** None (no arguments).  
**Output:** Two lines printed to stdout. This confirms Cargo and the toolchain work.


**Environment variables from the OS perspective.**  
From the operating system’s point of view, the **process environment** is a block of key–value strings associated with each process. When the kernel (or the program loader) creates a new process, it passes an **environment block** (e.g. the `envp` argument to `exec` on Unix, or the Process Environment Block on Windows). The new process **inherits** a copy of its parent’s environment unless the parent explicitly replaces it when spawning a child. The **shell** (e.g. bash, zsh) is where variables are usually defined and exported: `export VAR=value` makes `VAR` part of the environment of every program the shell later runs. System-wide or user-wide defaults are set in shell configuration files (e.g. `/etc/environment`, `.profile`) or by the session manager. Common variables include **`PATH`** (search path for executables), **`HOME`** (user’s home directory), **`USER`** / **`LOGNAME`** (current user), **`PWD`** (current working directory), and **`DISPLAY`** (X11 display for GUI apps). A program reads this process environment **at runtime** (e.g. in Rust with **`std::env::var("NAME")`** or **`std::env::vars()`**).

**Where `CARGO_PKG_NAME` and `CARGO_PKG_VERSION` come from.**  
The example above does **not** use the process environment. In Rust, **`env!("NAME")`** is a **compile-time** macro: the compiler substitutes it with a string literal before the program runs. The names **`CARGO_PKG_NAME`** and **`CARGO_PKG_VERSION`** are **not** set by the OS or the shell—they are injected by **Cargo** when it invokes the compiler, and their values are taken from the `[package]` section of your `Cargo.toml`. So the binary gets the package name and version baked in at build time; there is no runtime lookup of an environment variable. Use **`std::env::var("NAME")`** when you need the actual process environment (e.g. `PATH`, `HOME`); use **`env!("NAME")`** only for build-time constants provided by the toolchain (such as Cargo’s `CARGO_PKG_*` variables).

---

## 2. Rust vs Python: Ownership and Borrowing

**Duration:** ~3.5 h (lecture ~2.5 h, practice ~1 h)

### 2.1 Memory: Python vs Rust

**Interpreter vs compiler.**  
An **interpreter** runs the program by reading the source code (or bytecode) step by step and executing it: there is no separate “build” that produces a standalone executable. Python is typically used this way: `python script.py` feeds the source to the interpreter. A **compiler** translates the source code into **machine code** (or another low-level representation) ahead of time; you run the resulting **binary**, not the source. The compiler analyses the whole program at **compile time** and can enforce rules (types, ownership, etc.) before the program ever runs. **Rust is a compiled language**: `rustc` (via Cargo) compiles your code to a native executable, so many checks happen once at build time rather than repeatedly at runtime.

In **Python**, the interpreter manages memory with a garbage collector. You assign variables and do not think about who “owns” the data or when it is freed. For example:

```python
a = [1, 2, 3]
b = a          # b and a refer to the same list
b.append(4)    # a is also changed
```

In **Rust** (a **compiled** language) there is no garbage collector. Every value has a single **owner**. When the owner goes **out of scope**, the value is dropped (freed). The **scope** of a variable is the region of code where that variable is valid: it starts at the point where the variable is introduced (e.g. with `let`) and lasts until the end of the **block** that contains it (the closing `}`). When execution leaves that block—or when the function returns—the variable goes out of scope and Rust automatically runs the **drop** logic to free the value. So “goes out of scope” means “is no longer in the valid region,” and that is when ownership ends and the value is released. This **drop logic is not part of your source code**: you do not write explicit “free” or “release” calls. The **compiler** inserts the necessary cleanup code during the **compilation stage**: when it compiles your program, it analyses each variable’s scope and generates calls to the appropriate drop implementation at every point where a value goes out of scope. So the behaviour is fixed at **compile time**, and the generated binary already contains the right deallocation steps. Because the code is compiled, the compiler also enforces the **ownership** rules at build time—preventing use-after-free and data races without any runtime cost.

**Why no garbage collector is a performance advantage.**  
A **garbage collector** (GC) must periodically scan the heap, decide what is still in use, and reclaim the rest. That work costs CPU time and can cause **pause times** (short freezes), which are undesirable in real-time or low-latency systems (e.g. automation, control loops, embedded). In Rust, memory is freed **at a known time** (when the owner goes out of scope), so there is no background GC thread and no unpredictable pauses. Allocation and deallocation are explicit and predictable, which helps **cache locality** and makes it easier to reason about performance—a big plus for system and automation code.

**Why compiled code is generally much more performant than interpreted code.**  
An interpreter executes the program by **decoding and dispatching** each operation at runtime (e.g. “this is a function call, look up the function, push arguments, jump there”). That loop and the indirection through the interpreter add constant overhead. The program is also represented in a form (e.g. bytecode) that is optimized for the interpreter, not for the CPU. A **compiler** instead translates the program once into **native machine code** that the CPU runs directly. The compiler can apply **aggressive optimizations** (inlining, dead-code elimination, register allocation, etc.) at build time, and the resulting binary has no interpreter in the loop. So compiled code typically runs much faster and with more predictable latency—which matters for performance-sensitive and real-time applications.

**Examples of scope:**

```rust
fn main() {
    let x = 10;              // scope of x starts here
    println!("{}", x);       // OK: x is in scope

    {
        let y = 20;          // scope of y starts here (inner block)
        println!("{} {}", x, y);  // OK: both in scope
    }                        // y goes out of scope here and is dropped

    // println!("{}", y);    // ERROR: y is not in scope
    println!("{}", x);       // OK: x is still in scope
}                            // x goes out of scope here
```

In the inner block `{ ... }`, `y` exists only between its `let` and the closing `}`. After the block, `y` is no longer valid. The variable `x` lives for the whole of `main`, so it is in scope both outside and inside the inner block. When a variable goes out of scope, any value it owns (e.g. a `String`) is dropped at that point.

### 2.2 Ownership Rules (Brief)

1. Each value has exactly one owner.
2. When the owner goes out of scope, the value is dropped.
3. You can **move** or **borrow**; you cannot use a value after it has been moved.

**Move (Rust) vs shared reference (Python):**

```rust
let s1 = String::from("hello");
let s2 = s1;   // s1 is moved into s2; s1 is no longer valid
// println!("{}", s1);  // ERROR: use of moved value
println!("{}", s2);     // OK
```

In Python, `s2 = s1` would make both names point to the same object. In Rust, `s1` is moved into `s2` and cannot be used afterward. This is the main conceptual difference from Python.

**Mental model: move as renaming.**  
A useful trick is to think of a **move** as **renaming** the value: after `let s2 = s1`, the same data now “lives under” the name `s2`; the name `s1` is no longer a valid label for it. You did not copy the string and you did not create a second reference—you just changed which variable owns it. So in your head you can read “s1 is moved into s2” as “the value is renamed from s1 to s2.” That keeps the “one owner” idea clear and avoids the Python habit of imagining two names pointing at one thing.

### 2.3 Borrowing

Instead of moving, you can **borrow**:

- **Immutable borrow** `&T`: many readers, no writers.
- **Mutable borrow** `&mut T`: one writer, no other borrows at the same time.

The compiler checks that:

- You do not use a value after it has been moved.
- You do not have a mutable borrow and another borrow (mutable or immutable) at the same time.
- References never outlive the data they point to.

So “borrow” in Rust is the mechanism that lets you pass data to functions or use it in multiple places without copying or moving, while keeping memory safety.

### 2.4 Practical Example: Ownership and Borrows

**Goal:** Show how ownership and borrowing affect a function that “inspects” vs “modifies” a string, and what happens when you try to use a moved value.

**File: `src/main.rs`** (replace previous `main` or use a separate example binary):

```rust
fn length(s: &String) -> usize {
    s.len()
}

fn append_hello(s: &mut String) {
    s.push_str(", world!");
}

fn main() {
    let mut msg = String::from("Hello");
    println!("Original: {}", msg);

    let len = length(&msg);   // immutable borrow
    println!("Length: {}", len);
    println!("Still valid: {}", msg);  // msg still usable

    append_hello(&mut msg);   // mutable borrow
    println!("After append: {}", msg);
}
```

**How it works:**

- `length(&msg)` borrows `msg` immutably; `msg` stays owned by `main` and can be used later.
- `append_hello(&mut msg)` borrows `msg` mutably; the function can change the string. After the call, `main` can use `msg` again.

**Run:** `cargo run`

**Expected output:**

```
Original: Hello
Length: 5
Still valid: Hello
After append: Hello, world!
```

**Input:** None.  
**Output:** Printed lines showing that borrowing does not take ownership and that mutable borrows allow in-place modification.

**Brief introduction to the `&` operator.**  
In Rust, **`&`** is the **reference** (borrow) operator. It has two roles. (1) **In types**: `&T` means “a reference to a value of type `T`” (read-only); `&mut T` means “a mutable reference to a value of type `T`” (the function may modify it). (2) **In expressions**: writing `&x` creates a reference to `x`—you pass or store “a pointer to `x`” without moving or copying `x`. So `length(&msg)` passes a reference to `msg` into the function; the function’s parameter type is `&String`, and the caller keeps ownership of `msg`. The `&` in the call and the `&` in the type go together: you create a reference with `&value`, and the function receives it as `param: &T`. There is no separate “pointer” syntax; `&` is the way you borrow a value.


**Additional example: passing by value moves ownership.**  
If the function takes `String` (or any non-`Copy` type) **by value**, the argument is **moved** into the function and the caller can no longer use it:

```rust
fn print_string(s: String) {
    println!("Print from function: {}", s);
}

fn main() {
    let msg = String::from("Hello");

    print_string(msg);       // msg is moved into print_string; ownership transferred
    // print_string(msg);    // WRONG! msg was already moved; use-after-move error
}
```

**What happens:** The first `print_string(msg)` **moves** `msg` into the function parameter `s`. Inside `print_string`, `s` owns the string; when the function returns, `s` goes out of scope and the string is dropped. In `main`, `msg` is no longer valid—it is an empty "slot" left after the move. So the second `print_string(msg)` would be a **use-after-move**: the compiler would reject it. If you need to use `msg` again in `main`, either pass by reference (`fn print_string(s: &String)` and `print_string(&msg)`) or clone (`print_string(msg.clone())` and keep `msg`).

---

## 3. Passing by Value, Reference, and Cloning

**Duration:** ~2.5 h (lecture ~1.5 h, practice ~1 h)

### 3.1 By Value (Move)

Function parameters are passed **by value** by default: the value is **moved** into the function (for types that are not `Copy`). The caller no longer owns it.

```rust
fn take_ownership(s: String) {
    println!("I own: {}", s);
}  // s is dropped here

let s = String::from("hello");
take_ownership(s);
// println!("{}", s);  // ERROR: s was moved
```

### 3.2 By Reference (Borrow)

Pass **by reference** when the function should only read or when you want to allow modification without giving up ownership:

- `fn f(x: &T)` – read-only.
- `fn f(x: &mut T)` – can modify `x`.

The caller keeps ownership; the function only borrows.

### 3.3 Cloning

When you need a real **copy** of the data (e.g. to keep one copy in the caller and give one to a function), use `.clone()`:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // s1 and s2 are independent copies
```

Cloning can be expensive (heap allocation, copying bytes). Use references when reading or modifying in place is enough; use cloning when you genuinely need two separate copies.

### 3.4 Copy Types

Some types (e.g. integers, `bool`, `char`) are **Copy**: they are copied by value automatically; no move happens and the caller can still use the value. For example:

```rust
let x = 42;
let y = x;   // x is copied, not moved
println!("{} {}", x, y);  // both valid
```

### 3.5 Practical Example: Value, Reference, and Clone

**Goal:** Compare “pass by value” (move), “pass by reference,” and “clone” with a simple counter type.

**File: `src/main.rs`** (or a dedicated example):

```rust
fn by_value(n: u32) -> u32 {
    n + 1
}

fn by_reference(n: &u32) -> u32 {
    *n + 1
}

fn by_clone(s: String) -> String {
    let mut t = s.clone();
    t.push('!');
    t
}

fn main() {
    let num: u32 = 10;
    let result = by_value(num);
    println!("by_value: {} -> {} (original still usable: {})", num, result, num);

    let result2 = by_reference(&num);
    println!("by_reference: {} -> {}", num, result2);

    let text = String::from("Rust");
    let modified = by_clone(text.clone());
    println!("by_clone: original '{}', modified '{}'", text, modified);
}
```

**How it works:**

- `by_value(num)`: `u32` is Copy, so `num` is copied; `num` remains valid.
- `by_reference(&num)`: we pass a reference; `num` is not moved or copied.
- `by_clone(text.clone())`: we pass a clone into the function; `text` stays in `main`, and the function works on its own copy.

**Run:** `cargo run`

**Expected output:**

```
by_value: 10 -> 11 (original still usable: 10)
by_reference: 10 -> 11
by_clone: original 'Rust', modified 'Rust!'
```

**Input:** None.  
**Output:** Three lines showing that Copy is copied, references avoid move, and cloning gives an independent copy.

---

## 4. Functions, Structs, and Rust’s Approach to OOP

**Duration:** ~3.5 h (lecture ~2.5 h, practice ~1 h)

### 4.1 Functions

Rust functions are similar to Python’s `def`: they take arguments and can return a value. Types are required in signatures:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b   // no semicolon = return value
}
```

- Arguments and return type are explicitly typed.
- Last expression without `;` is the return value; you can also use `return x;`.

### 4.2 No Classical OOP

Rust has **no classes** and no inheritance. Instead it offers:

- **Structs** – data (like `@dataclass` or a class that only holds fields).
- **Implementations** (`impl`) – functions associated with a type (methods).
- **Traits** – shared behaviour (similar to interfaces or abstract base classes); no inheritance of data.

So “Rust’s alternative to OOP” is: **structs + impl + traits**, with composition over inheritance.

### 4.3 Structures

A **struct** groups related data. Example:

```rust
struct SensorReading {
    name: String,
    value: f64,
    unit: String,
}
```

You create instances and access fields:

```rust
let r = SensorReading {
    name: String::from("temperature"),
    value: 23.5,
    unit: String::from("°C"),
};
println!("{}: {} {}", r.name, r.value, r.unit);
```

### 4.4 Methods via `impl`

Methods are functions in an `impl` block for that struct. They take `self`, `&self`, or `&mut self` (like `self` in Python):

```rust
impl SensorReading {
    fn summary(&self) -> String {
        format!("{} = {} {}", self.name, self.value, self.unit)
    }

    fn scale(&mut self, factor: f64) {
        self.value *= factor;
    }
}
```

- `&self`: read-only method.
- `&mut self`: method that can modify the struct.

### 4.5 Practical Example: Struct and Methods (Automation Flavour)

**Goal:** Model a simple “sensor” with a struct and methods; demonstrate construction, read-only summary, and in-place scaling.

**File: `src/main.rs`** (or a separate example):

```rust
struct SensorReading {
    name: String,
    value: f64,
    unit: String,
}

impl SensorReading {
    fn new(name: &str, value: f64, unit: &str) -> Self {
        Self {
            name: name.to_string(),
            value,
            unit: unit.to_string(),
        }
    }

    fn summary(&self) -> String {
        format!("{} = {} {}", self.name, self.value, self.unit)
    }

    fn scale(&mut self, factor: f64) {
        self.value *= factor;
    }
}

fn main() {
    let mut temp = SensorReading::new("Temperature", 25.0, "°C");
    println!("{}", temp.summary());

    temp.scale(0.5);
    println!("After scale(0.5): {}", temp.summary());
}
```

**How it works:**

- `SensorReading::new(...)` is a constructor-like associated function (no `self`).
- `summary(&self)` borrows the struct immutably and returns a formatted string.
- `scale(&mut self, factor)` borrows mutably and multiplies `value` by `factor`.

**Run:** `cargo run`

**Expected output:**

```
Temperature = 25 °C
After scale(0.5): Temperature = 12.5 °C
```

**Input:** None (values are hardcoded).  
**Output:** Two lines showing the initial reading and the reading after scaling—illustrating structs and methods as Rust’s way to group data and behaviour without classes.

---

## Chapter 1 Summary

| Topic                    | Takeaway                                                                 |
|--------------------------|--------------------------------------------------------------------------|
| Cargo                    | One tool for project, deps, build, run; project = package with `Cargo.toml`. |
| Ownership                | Every value has one owner; when owner goes out of scope, value is dropped.  |
| Borrowing                | Use `&T` / `&mut T` to read or modify without taking ownership.         |
| Pass by value / reference| By value = move (for non-Copy); by reference = borrow.                  |
| Cloning                  | Use when you need an independent copy; avoid when a reference is enough. |
| Structs and methods      | Data in structs; behaviour in `impl`; no classes, use traits for shared behaviour. |

**Total suggested time:** ~14 h (theory ~10 h, practice ~4 h). Adjust to fit your 24 h total with Chapter 2.

---

## Practice Suggestions

1. Create a Cargo project that takes one command-line argument (a name) and prints “Hello, &lt;name&gt;!” using `std::env::args()`.
2. Write a function that takes `&[i32]` and returns the sum; call it from `main` with a slice.
3. Define a struct `Actuator { id: u32, state: bool }` with methods `on()`, `off()`, and `toggle(&mut self)`.
