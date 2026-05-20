# Chapter 7: Errors and Testing

## Hook

Java throws checked/unchecked exceptions. Python raises exceptions. Rust splits **recoverable** (`Result`) from **unrecoverable** (`panic!`). Automation code that runs unattended should treat errors as data, not surprises.

## `Result` and `?`

```rust
// Playground
use std::fs::File;
use std::io::{self, Read};

fn read_first_line(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf.lines().next().unwrap_or("").to_string())
}

fn main() {
    match read_first_line("Cargo.toml") {
        Ok(s) => println!("{}", s),
        Err(e) => println!("error: {}", e),
    }
}
```

**Cargo only** for `File` — on playground, simulate:

```rust
// Playground
fn parse_config(s: &str) -> Result<u32, String> {
    s.trim().parse::<u32>().map_err(|e| e.to_string())
}

fn main() {
    println!("{:?}", parse_config("8080"));
    println!("{:?}", parse_config("oops"));
}
```

## Custom errors (lightweight)

Combine enums or use `thiserror` / `anyhow` crates in real projects. Conceptually:

```rust
// Playground
#[derive(Debug)]
enum AppError {
    Parse(String),
    OutOfRange(u32),
}

fn set_port(s: &str) -> Result<u16, AppError> {
    let p: u16 = s.parse().map_err(|_| AppError::Parse(s.into()))?;
    if p < 1024 { Err(AppError::OutOfRange(p as u32)) } else { Ok(p) }
}

fn main() {
    println!("{:?}", set_port("8080"));
}
```

## `panic!` vs recover

| Use | When |
|-----|------|
| `Result` | Expected failure (missing file, bad input) |
| `panic!` | Logic bug, prototype, `main` after message |

Libraries should not panic on bad user input.

## Testing

**Cargo only:**

```rust
// src/lib.rs or same file with #[cfg(test)]
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(add(2, 2), 4);
    }
}
```

```bash
cargo test
```

Doc tests in `///` examples; integration tests in `tests/`.

## Idiom spotlight

> **`main() -> Result<(), E>`** for CLIs — bubble errors to one place, print context with `anyhow` or map to exit codes.

## Go deeper

- [Result railway](https://hightechmind.io/rust/)
- [Unit test patterns](https://hightechmind.io/rust/) — 744+
- Archive: [CHAPTER_02 §5](../archive/CHAPTER_02_AUTOMATION_AND_SYSTEM_PROGRAMMING.md)

## See also

- [Chapter 5: Result enum](05_types_enums_pattern_matching.md)
- [Chapter 15: I/O errors](15_io_processes_bits.md)

### Afterparty: AI Lego blocks

1. **? chain** — “Refactor nested match on Results to `?` style; explain each change.”
2. **Error enum design** — “Design `AppError` for CLI that reads config + talks serial; variants + `From` impls sketch.”
3. **panic audit** — “Mark which of 10 `unwrap()` calls should stay vs become `Result`.”
4. **Test generation** — “Write table-driven tests for `parse_port` including edge ports.”
5. **Java exceptions** — “Map checked Exception flow to Rust `Result` for file-not-found scenario.”
6. **anyhow vs thiserror** — “When would I pick each for automation binary vs library crate?”
