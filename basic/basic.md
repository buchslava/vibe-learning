
# Preface: Why Rust? Ownership, Safety, and Modern Coding

Rust is a modern programming language that’s taken the developer world by storm. Its unique approach to memory safety, fearless concurrency, and zero-cost abstractions make it both powerful and approachable. Unlike many languages, Rust enforces strict rules about who owns data and when it can be accessed, preventing entire classes of bugs at compile time.

---

Rust’s ownership model is a compile-time guarantee that eliminates entire classes of memory bugs, such as use-after-free, double free, and data races. When a value is moved (like passing a `String` to a function), the original variable becomes invalid, and any attempt to use it will result in a compile-time error. This ensures that there is always a single, clear owner for each piece of data, and the lifetime of that data is tightly controlled.

Borrowing (via references) allows temporary access to data without transferring ownership, but Rust enforces strict rules: you can have either one mutable reference or any number of immutable references at a time, never both. This prevents data races and makes concurrent code much safer by default.

In practice, this means you can write low-level, high-performance code with confidence, knowing that the compiler will catch common mistakes before your program ever runs. The result is software that’s both fast and reliable, with safety guarantees that are usually only possible in managed languages.

---

**Running the Examples:**  
You can try all the code samples in this book instantly using the official Rust Playground: [https://play.rust-lang.org](https://play.rust-lang.org).  
Just copy the code, paste it into the editor, and hit “Run” to see the output. No installation required!

---

## Example 1: Ownership in Action

Let’s look at a simple but powerful concept: ownership. In Rust, when you give something away, you no longer have it. This is enforced by the compiler, making your code safer and easier to reason about.

**Scenario:**  
Bob gives Mary a single dollar. It’s now Mary’s — Bob no longer owns it. John can’t take it from Bob anymore because Bob doesn’t have it.

This is how Rust models ownership: when a value is moved, the original owner loses access.

```rust
fn give_to_mary(money: String) -> String {
    println!("Mary received {money}");
    money
}

fn main() {
    let dollar = String::from("Dollar from Bob");
    let dollar = give_to_mary(dollar);
    println!("{dollar}"); // Mary has it now
}
```

- `dollar` is created and owned by `main`.
- When we call `give_to_mary(dollar)`, ownership of the dollar moves to the function.
- Inside `give_to_mary`, Mary receives the dollar, and the function returns it (Mary could keep it, or pass it on).
- Back in `main`, we assign the returned value to `dollar` again — now, `main` owns it once more.
- If you tried to use `dollar` after giving it away (without getting it back), Rust would give you a compile-time error.

---

Look at `dollar` variable double assignment. In Rust, this pattern is called variable shadowing. Let’s break it down:

The first `let dollar = ...` creates a new String and binds it to the variable `dollar`.
The second `let dollar = ...` reuses the same variable name. This is called shadowing—the new `dollar` shadows (replaces) the previous one in the current scope.

Why is this idiomatic in Rust?
* Ownership transfer: In Rust, when you pass dollar to give_to_mary, ownership of the String moves into the function. You can’t use the original dollar after this point.
* Shadowing allows reuse: By shadowing, you can conveniently reuse the same variable name for the returned value, which is now owned by you again.

Analogy
Think of it as handing someone a dollar bill (ownership moves), and then they give it back to you. You now have a new dollar bill (possibly the same one), and you can keep calling it dollar.

Gotcha
If you tried to use the original dollar after passing it to give_to_mary without shadowing, you’d get a compile error because you no longer own it.

Example before represents happy flow. Let's look at the example contain an error.

```rust
fn give_to_mary(money: String) {
    println!("Mary received {money}");
}

fn main() {
    let dollar = String::from("Dollar from Bob");
    give_to_mary(dollar);
    // Now, John tries to use the same dollar
    println!("{dollar}"); // Error: value borrowed here after move
}
```

```
error[E0382]: borrow of moved value: `dollar`
 --> src/main.rs:9:16
  |
6 |     let dollar = String::from("Dollar from Bob");
  |         ------ move occurs because `dollar` has type `String`, which does not implement the `Copy` trait
7 |     give_to_mary(dollar);
  |                  ------ value moved here
8 |     // Now, John tries to use the same dollar
9 |     println!("{dollar}"); // Error: value borrowed here after move
  |                ^^^^^^ value borrowed here after move
  |
note: consider changing this parameter type in function `give_to_mary` to borrow instead if owning the value isn't necessary
 --> src/main.rs:1:24
  |
1 | fn give_to_mary(money: String) {
  |    ------------        ^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
7 |     give_to_mary(dollar.clone());
  |                        ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `playground` (bin "playground") due to 1 previous error
```





---


Everyone in the family can look at the same photo, but no one can draw over it while others are looking.


```rust
fn main() {
    let photo = String::from("📷 family photo");
    look(&photo);
    look(&photo);
}

fn look(photo: &String) {
    println!("Someone is looking at {photo}");
}
```