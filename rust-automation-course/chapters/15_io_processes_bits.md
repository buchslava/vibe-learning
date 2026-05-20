# Chapter 15: I/O, Processes, and Bits

## Hook

Python’s `open()` and `subprocess` get scripts talking to the world quickly. Java uses `Files` and `ProcessBuilder`. Rust unifies files, sockets, and pipes behind **`Read`** and **`Write`** traits — and uses **`Command`** for subprocesses with explicit error types. Automation lives here.

## Traits: `Read` and `Write`

Anything that can supply or sink bytes implements these traits. Your code can stay generic:

```rust
// Playground — in-memory stand-in (no filesystem)
use std::io::{self, Cursor, Read, Write};

fn echo(mut r: impl Read, mut w: impl Write) -> io::Result<()> {
    let mut buf = [0u8; 64];
    loop {
        let n = r.read(&mut buf)?;
        if n == 0 { break; }
        w.write_all(&buf[..n])?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let input = b"sensor:42\n";
    let mut reader = Cursor::new(&input[..]);
    let mut out = Vec::new();
    echo(&mut reader, &mut out)?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
```

## File I/O (Cargo lab)

**Cargo only:**

```rust
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn process_file(input: &str, output: &str) -> std::io::Result<()> {
    let reader = BufReader::new(File::open(input)?);
    let mut out = File::create(output)?;
    for line in reader.lines() {
        writeln!(out, "{}", line?.trim().to_uppercase())?;
    }
    Ok(())
}
```

Use `BufReader` / `BufWriter` for many small operations. `main() -> io::Result<()>` propagates I/O errors cleanly ([Chapter 7](07_errors_and_testing.md)).

## Subprocesses

| Python | Java | Rust |
|--------|------|------|
| `subprocess.run` | `ProcessBuilder` | `Command::new(...).output()` |
| `shell=True` | — | `sh -c "..."` (use sparingly) |

**Cargo only:**

```rust
use std::process::Command;

fn main() -> std::io::Result<()> {
    let out = Command::new("echo").arg("hello automation").output()?;
    println!("status: {}", out.status);
    println!("stdout: {}", String::from_utf8_lossy(&out.stdout));
    Ok(())
}
```

Prefer direct execution over shell when you do not need pipes in shell syntax.

## Bitwise and binary protocols

Automation registers and frames are built from bytes:

```rust
// Playground
fn encode(id: u8, value: u16, valid: bool, alarm: bool) -> [u8; 4] {
    let vb = value.to_be_bytes();
    let mut status = 0u8;
    if valid { status |= 1; }
    if alarm { status |= 2; }
    [id, vb[0], vb[1], status]
}

fn decode(buf: &[u8; 4]) -> (u8, u16, bool, bool) {
    let value = u16::from_be_bytes([buf[1], buf[2]]);
    (buf[0], value, buf[3] & 1 != 0, buf[3] & 2 != 0)
}

fn main() {
    let p = encode(0x01, 1025, true, false);
    println!("{:02x?}", p);
    println!("{:?}", decode(&p));
}
```

Use **unsigned** types for shifts; pick endianness explicitly (`to_be_bytes` / `from_be_bytes`).

## PTY (note)

Interactive CLI automation sometimes needs a **pseudo-terminal**. Not in `std`; crates like `pty-process` or `expectrl` fill the gap. Start with `Command`; add PTY when line discipline matters.

## Idiom spotlight

> **Parse bytes with types, not magic indices forever.** Once a frame layout stabilizes, wrap it in `struct` + `encode`/`decode` methods.

## Go deeper

- Archive: [CHAPTER_02 §1–3](../archive/CHAPTER_02_AUTOMATION_AND_SYSTEM_PROGRAMMING.md)
- [Csv parser / writer](https://hightechmind.io/rust/) — 958–959

## See also

- [Chapter 7: Errors](07_errors_and_testing.md)
- [Chapter 16: Hardware lab](16_hardware_automation_lab.md)
- [Chapter 12: Async I/O](12_async_tokio.md)

### Afterparty: AI Lego blocks

1. **Trait refactor** — “Refactor file copy loop to generic `copy<R: Read, W: Write>`; discuss error propagation.”
2. **CSV tool** — “Spec for CLI: read two-column CSV, emit `name=value`; I implement with BufRead.”
3. **Packet layout** — “Add CRC byte to 4-byte packet; update encode/decode with XOR — show tests.”
4. **Command safety** — “Review shell=True style command; rewrite without shell when possible.”
5. **Endian trap** — “Quiz: 3 scenarios pick LE vs BE for Modbus-style register.”
6. **Pipeline** — “Design `program A | program B` using only Rust std (two processes, pipe).”
