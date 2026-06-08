# Chapter 19: I/O, Processes, and Bits

## Hook

I/O APIs differ by language (**Python** `open()`/`subprocess`, **Java** `Files`/`ProcessBuilder`, …). Rust unifies files, sockets, serial ports, and pipes behind **`Read`** and **`Write`**. It uses **`Command`** for subprocesses with explicit error types. CLIs, services, and protocol code all live here.

## Scope — a brief tour

Practical capstone for files, processes, and binary frames — not kernel or embedded-HAL depth.

| This chapter covers | Deferred to See also / Afterparty |
|---------------------|-----------------------------------|
| `Read` / `Write`, buffering, `io::Result` | Zero-copy I/O, `io_uring`, mmap |
| File and line processing (sync) | Full logging/observability stacks |
| `Command`, pipes, shell vs direct exec | Job orchestration (Kubernetes, systemd) |
| Bit packing, endianness, CRC sketch | Full Modbus/TCP stacks, `nom` parsers |
| Serial orientation (`serialport`) | GPIO/MCU bare-metal |
| Sync vs async I/O choice | Tokio tuning ([Chapter 16](16_async_tokio.md)) |

## Why `Read` and `Write` — the aim

Application code follows the same patterns: **pull bytes in**, **push bytes out**, **handle errors**, **don't panic on bad input**.

| Medium | Rust surface | Same trait? |
|--------|--------------|-------------|
| File | `File` | `Read` / `Write` |
| `stdin` / `stdout` | `std::io::stdin()` / `stdout()` | yes |
| In-memory test double | `Cursor<&[u8]>`, `Vec<u8>` | yes |
| TCP socket (sync) | `std::net::TcpStream` | yes |
| Serial port | `serialport` open handle | yes |
| Async TCP | `tokio::net::TcpStream` | `.await` variants ([Chapter 16](16_async_tokio.md)) |

**Aim:** write **one** `fn process<R: Read, W: Write>(...)` and reuse it for files, pipes, and test buffers. Java `InputStream`/`OutputStream` and Python file objects play the same role. Rust encodes errors in **`io::Result`** instead of exceptions.

## `Read` and `Write` essentials

| Method | Contract |
|--------|----------|
| `read(&mut buf)` | Up to `buf.len()` bytes; **`Ok(0)`** = EOF |
| `read_exact(&mut buf)` | Fills `buf` or returns `UnexpectedEof` |
| `write(&mut buf)` | May write **partial** slice |
| `write_all(&mut buf)` | Loops until all bytes sent or error |

Prefer **`write_all`** and **`read_exact`** when the protocol has fixed-size fields. Use **`read`** in loops when length is unknown until EOF.

## Examples: elementary → hard

Work through in order. After each snippet: **run it**, then read **what happened**.

### Level 1 — Elementary: generic echo

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

**What happened:** prints **`sensor:42`** — `Cursor` implements `Read` over a byte slice; `Vec` implements `Write`. Same `echo` works for files or sockets without changing the loop.

### Level 2 — Elementary: line-oriented config

```rust
// Playground
use std::io::{BufRead, BufReader, Cursor};

fn parse_kv_lines(data: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let reader = BufReader::new(Cursor::new(data.as_bytes()));
    for line in reader.lines() {
        let Ok(line) = line else { continue };
        let Some((k, v)) = line.split_once('=') else { continue };
        out.push((k.trim().into(), v.trim().into()));
    }
    out
}

fn main() {
    let cfg = "poll_ms=100\nport=502\n# comment\n";
    for (k, v) in parse_kv_lines(cfg) {
        println!("{k} -> {v}");
    }
}
```

**What happened:** prints **`poll_ms -> 100`** and **`port -> 502`** — `BufReader` reduces syscalls for real files; `# comment` line skipped (no `=`). In production, return `io::Result` and map errors ([Chapter 8](08_errors_and_testing.md)) instead of `expect` on lines.

### Level 3 — Intermediate: frame struct + CRC

Wrap bytes in a type once the layout stabilizes:

```rust
// Playground
#[derive(Debug, PartialEq)]
struct Frame {
    id: u8,
    value: u16,
    valid: bool,
    alarm: bool,
}

impl Frame {
    fn encode(&self) -> [u8; 5] {
        let vb = self.value.to_be_bytes();
        let mut status = 0u8;
        if self.valid { status |= 1; }
        if self.alarm { status |= 2; }
        let body = [self.id, vb[0], vb[1], status];
        let crc = body.iter().fold(0u8, |a, &b| a ^ b);
        let mut out = [0u8; 5];
        out[..4].copy_from_slice(&body);
        out[4] = crc;
        out
    }

    fn decode(buf: &[u8; 5]) -> Option<Self> {
        let body = &buf[..4];
        if body.iter().fold(0u8, |a, &b| a ^ b) != buf[4] {
            return None;
        }
        let value = u16::from_be_bytes([body[1], body[2]]);
        Some(Self {
            id: body[0],
            value,
            valid: body[3] & 1 != 0,
            alarm: body[3] & 2 != 0,
        })
    }
}

fn main() {
    let f = Frame { id: 0x01, value: 1025, valid: true, alarm: false };
    let bytes = f.encode();
    println!("{:02x?}", bytes);
    println!("{:?}", Frame::decode(&bytes));
}
```

**What happened:** prints hex frame and **`Some(Frame { ... })`** — XOR CRC is toy-grade; real Modbus uses CRC-16. Pattern: **`encode`/`decode` on struct**, not loose indices.

### Level 4 — Intermediate: subprocess output

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

**What happened:** `output()` waits for the child, captures stdout/stderr into `Vec<u8>`, returns exit status. `from_utf8_lossy` handles non-UTF8 tool output without panicking.

**Safer than shell:** pass arguments with `.arg()` — no injection surface. Use `sh -c` only when you need `|`, globs, or shell variables.

### Level 5 — Hard: pipeline sketch (Cargo only)

Wire **program A → program B** with pipes (conceptual):

```rust
// Cargo only — Unix-oriented; needs std::process::{Command, Stdio}
// use std::process::{Command, Stdio};
//
// fn main() -> std::io::Result<()> {
//     let mut first = Command::new("cat")
//         .arg("input.txt")
//         .stdout(Stdio::piped())
//         .spawn()?;
//     let first_out = first.stdout.take().expect("piped");
//     let status = Command::new("grep")
//         .arg("ERROR")
//         .stdin(first_out)
//         .status()?;
//     first.wait()?;
//     println!("pipeline status: {status}");
//     Ok(())
// }
```

**What happened (conceptually):** first child's **stdout** becomes second child's **stdin**. Always `wait()` children to avoid zombies. For complex graphs, consider `duct` crate — std is verbose but explicit.

### Level 6 — Hard: sync file transform (Cargo only)

```rust
// Cargo only
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn process_file(input: &str, output: &str) -> std::io::Result<()> {
    let reader = BufReader::new(File::open(input)?);
    let mut out = File::create(output)?;
    for line in reader.lines() {
        let line = line?;
        writeln!(out, "{}", line.trim().to_uppercase())?;
    }
    out.flush()?;
    Ok(())
}
```

**What happened:** `?` propagates `io::Error` from open, read, or write — `main` can use `fn main() -> io::Result<()>`. **`flush`** before exit ensures buffered bytes hit disk (important for logs and PLC command files).

## File I/O patterns

| Pattern | When |
|---------|------|
| `File::open` + `BufReader` | Line or chunk input |
| `read_to_string` | Small text configs only |
| `BufWriter` + `write_all` | Many small writes |
| `main() -> io::Result<()>` | CLI tools — map to exit code at boundary |

**Boundary discipline** ([Chapter 8](08_errors_and_testing.md)): use internal `fn load_config() -> Result<Config, Error>`. In `main`, print `eprintln!("{e}")` and exit `1`. Never `unwrap` on user-supplied paths in unattended services.

## Bitwise and binary protocols

Register and status words on the wire are **bit fields**, not separate bool variables:

| Technique | Use |
|-----------|-----|
| `value \|= 1 << n` | Set bit *n* |
| `value & (1 << n) != 0` | Test bit *n* |
| `to_be_bytes` / `from_be_bytes` | Modbus-style **big-endian** registers |
| `to_le_bytes` / `from_le_bytes` | Intel layouts, many CAN/USB payloads |
| `struct` + `encode`/`decode` | Stable frame layout |

Use **unsigned** types (`u8`, `u16`, `u32`) for shifts — signed shifts are a footgun.

## Sync vs async I/O

| Situation | Prefer |
|-----------|--------|
| CLI tool, batch file transform | **sync** `std::fs`, `BufReader` |
| One serial poll loop, blocking read with timeout | **sync** + dedicated thread ([Chapter 14](14_multithreading.md)) |
| Many TCP connections, one process | **async** Tokio ([Chapter 16](16_async_tokio.md)) |
| Blocking `read` inside `async fn` | **Bad** — stalls executor; `tokio::fs` or `spawn_blocking` |

Same **traits** mentally. Async adds `.await` and a runtime. [Chapter 16](16_async_tokio.md) covers production protocol code.

## Practical I/O scenarios

| Task | Approach |
|------|----------|
| Read CSV/TOML config | `BufReader` + parse, or `serde` + file ([Chapter 17](17_metaprogramming.md)) |
| Run `systemctl` / vendor CLI | `Command` direct args; capture stdout; check `status` |
| Modbus-style register | `Frame` struct, BE `u16`, CRC per spec |
| Serial sensor / PLC | `serialport` crate — same `Read`/`Write` |
| Vendor C driver only | Safe Rust wrapper ([Chapter 18](18_unsafe_and_internals.md)) |

**Serial orientation (Cargo only):**

```toml
# Cargo.toml
# [dependencies]
# serialport = "4"
```

```rust
// Cargo only — list ports; optional open when arg provided
// use std::io::{Read, Write};
// use std::time::Duration;
//
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     for p in serialport::available_ports()? {
//         println!("{}", p.port_name);
//     }
//     // serialport::new("/dev/ttyUSB0", 9600).timeout(...).open()?
//     // port.write_all(b"PING\n")?; port.read(&mut buf)?;
//     Ok(())
// }
```

Poll loops: set **read timeout**, log the port on error, and **retry with backoff** (Afterparty P345). Do not spin tight on failure.

## CLI utility — paths, env, and time

One tool thread: resolve config path from env → check file size → read key=value lines.

### Path and env

```rust
// Cargo only — conceptual main fragment
// use std::path::{Path, PathBuf};
// use std::env;
//
// fn config_path() -> PathBuf {
//     env::var("APP_CONFIG")
//         .map(PathBuf::from)
//         .unwrap_or_else(|_| {
//             Path::new(&env::var("HOME").unwrap_or_else(|_| ".".into()))
//                 .join(".config")
//                 .join("gateway.toml")
//         })
// }
```

`Path` / `PathBuf` join segments without manual slash handling. Accept `impl AsRef<Path>` in helpers ([Chapter 13](13_standard_traits.md)).

### File metadata before read

```rust
// Cargo only
// use std::fs;
//
// fn read_if_small(path: &Path) -> std::io::Result<String> {
//     let meta = fs::metadata(path)?;
//     if meta.len() > 1_000_000 {
//         return Err(std::io::Error::new(
//             std::io::ErrorKind::InvalidData,
//             "config too large",
//         ));
//     }
//     fs::read_to_string(path)
// }
```

Check `metadata().len()` before allocating a huge buffer — cheap guard for untrusted paths.

### Stdin prompt alongside args

```rust
// Playground — stdin blocks until Enter (compile-only in CI)
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let arg_port = std::env::args().nth(1);
    let port = if let Some(p) = arg_port {
        p
    } else {
        print!("port: ");
        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        buf.trim().to_string()
    };
    println!("using port {}", port);
    Ok(())
}
```

Args for scripts; stdin for interactive runs — same binary, two entry paths.

### Timeout with `Duration`

```rust
// Playground
use std::time::Duration;

fn retry_delay(attempt: u32) -> Duration {
    let secs: u64 = std::env::var("RETRY_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2);
    Duration::from_secs(secs.saturating_mul(attempt as u64))
}

fn main() {
    println!("attempt 3 sleep {:?}", retry_delay(3));
}
```

`Duration` pairs with thread `sleep` and serial timeouts — env var gives ops a knob without recompile.

### CLI I/O edge cases

**Wrong — assume path is UTF-8 string on all OSes:** use `Path` end to end; convert to `str` only when displaying.

**Line endings:** `read_line` keeps `\n`; call `.trim()` before parsing numbers.

## PTY (note)

Interactive CLI tools sometimes need a **pseudo-terminal** (line discipline, echo). Not in `std`; crates like `pty-process` or `expectrl` fill the gap. Start with `Command`; add PTY when the tool checks `isatty`.

## Edge cases and compiler traps

| Trap | Symptom | Idiom |
|------|---------|-------|
| `read` once assuming full buffer | short read, corrupt parse | loop `read` or `read_exact` |
| `write` without `write_all` | partial send on socket | always `write_all` for protocols |
| Wrong endianness | register value off by 256× | document BE vs LE per spec |
| `unwrap` on `lines()` | panic on bad UTF-8 | `?` + error type |
| `shell -c` with user input | command injection | `.arg()` list |
| Forgot `flush` | lost tail bytes on crash | `flush` before exit |
| Blocking serial in async task | stalled Tokio worker | dedicated thread or async crate |

## Idiom spotlight

> **Parse bytes with types, not magic indices forever.** Once a frame layout stabilizes, wrap it in `struct` + `encode`/`decode` methods.
>
> **Generic over `Read`/`Write` for testability** — `Cursor` in unit tests, real `File` in integration tests.
>
> **Treat I/O as `Result` at boundaries** — long-running binaries should survive bad paths and timeouts without panic ([Chapter 8](08_errors_and_testing.md)).

## Go deeper

- [Csv parser / writer](https://hightechmind.io/rust/) — 958–959
- [Rust book — I/O](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

## See also

- [Chapter 8: Errors](08_errors_and_testing.md) — `io::Error`, `?`, no panic in production loops
- [Chapter 14: Multithreading](14_multithreading.md) — blocking I/O on worker threads
- [Chapter 16: Async I/O](16_async_tokio.md) — `tokio::fs`, `TcpListener`
- [Chapter 17: Metaprogramming](17_metaprogramming.md) — `include_str!`, config derives
- [Chapter 18: Unsafe](18_unsafe_and_internals.md) — FFI and safe wrappers over I/O handles

### Afterparty

#### Read / Write and buffering

1. **Trait refactor** — “Refactor file copy loop to generic `copy<R: Read, W: Write>`; discuss error propagation.”
2. **read vs read_exact** — “Give 3 protocol shapes; I pick `read` loop vs `read_exact` each time; you verify.”
3. **Cursor test** — “Write unit test for `parse_kv_lines` using `Cursor` — no filesystem.”
4. **BufReader why** — “Explain in 60 words why `BufReader` matters for 10k-line log files.”

#### Files and CLI

5. **CSV tool** — “Spec for CLI: read two-column CSV, emit `name=value`; I implement with BufRead.”
6. **Boundary errors** — “Sketch `main` mapping `io::Error` to exit code 1 with context path — no `unwrap`.”
7. **read_to_string trap** — “When is `read_to_string` wrong for automation configs? Give size threshold rule.”

#### Binary frames and endianness

8. **Packet layout** — “Add CRC byte to 4-byte packet; update encode/decode with XOR — show tests.”
9. **Endian trap** — “Quiz: 3 scenarios pick LE vs BE for Modbus-style register.”
10. **Bit field port** — “Java status int with flags — port to Rust `encode` with `|=` and `&` masks.”
11. **CRC upgrade** — “Replace XOR toy CRC with CRC-16-Modbus — outline steps, no full crate required.”

#### Processes and pipelines

12. **Command safety** — “Review shell=True style command; rewrite without shell when possible.”
13. **Pipeline** — “Design `program A | program B` using only Rust std (two processes, pipe).”
14. **Exit status** — “Child exited 2 — how should gateway log and retry? Table: fatal vs transient.”
15. **Env and cwd** — “Show `Command` with `.env("PORT","502")` and `.current_dir` — when needed?”

#### Sync vs async and serial

16. **Sync vs async pick** — “Three gateway designs: I pick sync thread vs Tokio per scenario; you justify.”
17. **Serial debug** — “I get timeout on read; give systematic checklist (baud, cable, permissions).”
18. **serialport traits** — “Explain how `serialport` maps to `Read`/`Write` — diagram only.”
19. **Blocking in async** — “Show wrong `std::fs::read` inside `async fn`; fix with `tokio::fs` or `spawn_blocking`.”

#### Capstone and operations

20. **Capstone scaffold** — “Generate module tree and function signatures for sensor_gateway; no bodies.”
21. **Retry policy** — “Design exponential backoff for Modbus-style poll errors; Rust pseudocode.”
22. **Log schema** — “Propose JSON log lines for sensor events with timestamp and error codes.”
23. **GPIO next step** — “After serial works on Pi, outline migration to gpio-cdev for one LED.”
24. **Code review** — “I paste capstone main loop; review for panic risks and missing flush.”
25. **Gateway capstone** — “End-to-end: config file → serial poll → JSON log line — module list and error types only.”
26. **Ch16 bridge** — “Same echo server: sketch sync thread version vs Tokio version — tradeoffs table.”

#### Paths, env, and CLI

27. **Path join** — "Build config path from `HOME` + `.config/app.toml` — show `Path::join` vs string concat trap."
28. **Env default** — "`TIMEOUT` env var with default 30 — `var().unwrap_or_else` pattern."
29. **Metadata guard** — "Reject config files over 1MB before `read_to_string` — sketch `metadata().len()` check."
30. **Stdin fallback** — "Port from argv or interactive prompt — one `main` with both paths."
31. **Line parser** — "BufRead lines, skip `#`, parse `key=value` — handle trailing newline."
32. **Capstone CLI** — "End-to-end: env path → metadata check → line parse → print port — function list only."

