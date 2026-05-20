# Chapter 2: Automation and System Programming

**Course:** Rust as a pre-part of Computer Systems  
**Audience:** University students (Automation area), familiar with Python  
**Prerequisite:** Chapter 1 (Rust Basics)  
**Estimated duration:** ~10 hours (≈70% lectures, 30% practice)

---

## Table of Contents

1. [File Input/Output and Streams](#1-file-inputoutput-and-streams)
2. [Bitwise Operations and Binary Data](#2-bitwise-operations-and-binary-data)
3. [Processes, Shell, and PTY](#3-processes-shell-and-pty)
4. [Working with Hardware (Basics)](#4-working-with-hardware-basics)
5. [Error Handling and Robustness](#5-error-handling-and-robustness)

---

## 1. File Input/Output and Streams

**Duration:** ~2 h (lecture ~1.5 h, practice ~0.5 h)

### 1.1 Why Files and Streams Matter in Automation

In automation you often need to:

- Read configuration or calibration data from files.
- Log sensor data or events to disk.
- Exchange data with other programs via stdin/stdout (streams).

Rust treats I/O through **traits**: `Read` and `Write`. Files, sockets, and stdin/stdout all implement these traits, so the same patterns work everywhere.

### 1.2 Reading and Writing Files

The standard library provides:

- **`std::fs::File`** – open a file for reading or writing.
- **`std::io::Read`** – `read()`, `read_to_string()`, etc.
- **`std::io::Write`** – `write_all()`, `flush()`.

**Opening modes:**

- `File::open(path)` – read-only.
- `File::create(path)` – write; overwrites if exists.
- `OpenOptions` – for append, create-if-missing, etc.

### 1.3 Streams: stdin, stdout, stderr

- **`std::io::stdin()`** – standard input (implements `Read`).
- **`std::io::stdout()`** – standard output (implements `Write`).
- **`std::io::stderr()`** – standard error.

Many CLI tools read from stdin and write to stdout so they can be composed with pipes (e.g. `cat file | your_program`).

### 1.4 Buffering

For efficiency, use **buffered** I/O when doing many small reads or writes:

- **`std::io::BufReader::new(reader)`** – buffered input.
- **`std::io::BufWriter::new(writer)`** – buffered output.

### 1.5 Practical Example: File I/O and Streams

**Goal:** Read a text file line by line, process each line (e.g. trim and uppercase), and write the result to another file; then show reading from stdin when no file is given.

**Create a new binary or add to an existing project. `Cargo.toml`:** no extra dependencies.

**File: `src/main.rs`** (or `examples/file_io.rs` and run with `cargo run --example file_io`):

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn process_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let f = File::open(input_path)?;
    let reader = BufReader::new(f);
    let mut out = File::create(output_path)?;

    for line in reader.lines() {
        let line = line?;
        let processed = line.trim().to_uppercase();
        writeln!(out, "{}", processed)?;
    }
    out.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 {
        process_file(&args[1], &args[2])?;
        println!("Processed {} -> {}", args[1], args[2]);
    } else {
        println!("Usage: {} <input_file> <output_file>", args.get(0).unwrap_or(&"program".into()));
        println!("Or pipe input: echo 'hello' | {} (read from stdin not shown here)", args.get(0).unwrap_or(&"program".into()));
    }
    Ok(())
}
```

**How it works:**

- `File::open` and `File::create` return `io::Result<File>`; `?` propagates errors.
- `BufReader::new(f)` wraps the file for efficient line-by-line reading.
- `reader.lines()` yields `Result<String, io::Error>`; `line?` unwraps or returns an error.
- `writeln!(out, "{}", processed)` writes to the output file; `flush()` ensures data is written.

**Run:**

```bash
echo "  hello world  " > input.txt
echo "  automation  " >> input.txt
cargo run -- input.txt output.txt
```

**Expected output (stdout):**

```
Processed input.txt -> output.txt
```

**Output file `output.txt` contents:**

```
  HELLO WORLD  
  AUTOMATION  
```

**Input:** Two CLI arguments (input and output paths); or no args for usage message.  
**Output:** A processed file (trimmed and uppercased lines) and a confirmation line on stdout. This demonstrates file streams and buffered I/O.

---

## 2. Bitwise Operations and Binary Data

**Duration:** ~2 h (lecture ~1.5 h, practice ~0.5 h)

### 2.1 Why Bits Matter in Automation

Automation and embedded systems often use:

- **Registers** and **protocols** where each bit has a meaning (flags, masks).
- **Binary protocols** (e.g. Modbus, custom frames) where you pack/unpack bytes.
- **Hardware interfaces** where you set or read individual bits (GPIO, status words).

Rust has the same bitwise operators as C: `&`, `|`, `^`, `!`, `<<`, `>>`.

### 2.2 Bitwise Operators

| Operator | Meaning        | Example (binary)        |
|----------|----------------|-------------------------|
| `&`      | AND            | `0b1100 & 0b1010 == 0b1000` |
| `\|`     | OR             | `0b1100 \| 0b1010 == 0b1110` |
| `^`      | XOR            | `0b1100 ^ 0b1010 == 0b0110` |
| `!`      | NOT (invert)   | `!0u8` = 255            |
| `<<`     | Left shift     | `1 << 3` = 8            |
| `>>`     | Right shift    | `8 >> 2` = 2            |

Use **unsigned** types (`u8`, `u16`, `u32`) for bit manipulation to avoid sign-extension surprises with `>>`.

### 2.3 Working with Bytes

- **`u8`** – one byte; use slices `&[u8]` for binary buffers.
- **Endianness:** when building or parsing multi-byte values, use **`to_le_bytes()`** / **`from_le_bytes()`** (little-endian) or **`to_be_bytes()`** / **`from_be_bytes()`** (big-endian).

### 2.4 Practical Example: Bitwise and Binary Packing

**Goal:** Encode a simple “sensor packet”: 1 byte ID, 2 bytes value (big-endian), 1 byte status (bit 0 = valid, bit 1 = alarm). Then decode it back and print.

**File: `src/main.rs`** (or `examples/bitwise.rs`):

```rust
fn encode_packet(id: u8, value: u16, valid: bool, alarm: bool) -> [u8; 4] {
    let value_bytes = value.to_be_bytes();
    let mut status: u8 = 0;
    if valid {
        status |= 1;  // bit 0
    }
    if alarm {
        status |= 2;  // bit 1
    }
    [id, value_bytes[0], value_bytes[1], status]
}

fn decode_packet(buf: &[u8; 4]) -> (u8, u16, bool, bool) {
    let id = buf[0];
    let value = u16::from_be_bytes([buf[1], buf[2]]);
    let valid = (buf[3] & 1) != 0;
    let alarm = (buf[3] & 2) != 0;
    (id, value, valid, alarm)
}

fn main() {
    let packet = encode_packet(0x01, 1025, true, false);
    println!("Encoded bytes: {:02x?}", packet);

    let (id, value, valid, alarm) = decode_packet(&packet);
    println!("Decoded: id={}, value={}, valid={}, alarm={}", id, value, valid, alarm);
}
```

**How it works:**

- `value.to_be_bytes()` gives a big-endian 2-byte array; we put it in bytes 1–2.
- Status byte: bit 0 = valid, bit 1 = alarm; we set them with `|= 1` and `|= 2`.
- Decoding: `buf[3] & 1` and `buf[3] & 2` extract the two flags; `u16::from_be_bytes` reconstructs the value.

**Run:** `cargo run`

**Expected output:**

```
Encoded bytes: [01, 04, 01, 01]
Decoded: id=1, value=1025, valid=true, alarm=false
```

**Input:** None (fixed values).  
**Output:** Encoded 4-byte packet and decoded fields—illustrating bitwise ops and binary layout for automation-style protocols.

---

## 3. Processes, Shell, and PTY

**Duration:** ~2 h (lecture ~1.5 h, practice ~0.5 h)

### 3.1 Running External Commands

In automation you often need to:

- Run system tools (e.g. `ip`, `systemctl`, custom scripts).
- Capture their output for parsing.
- Chain commands or drive interactive tools.

Rust’s **`std::process::Command`** lets you spawn processes, set environment, redirect stdin/stdout/stderr, and wait for exit status—similar to Python’s `subprocess`.

### 3.2 Command API (Brief)

- **`Command::new("program").arg("arg").output()`** – run and collect stdout/stderr (blocking).
- **`Command::new("program").arg("arg").status()`** – run and get exit code.
- **`.stdin(Stdio::piped())`** / **`.stdout(Stdio::piped())`** – connect pipes for I/O.

### 3.3 Shell vs Direct Execution

- **`Command::new("sh").arg("-c").arg("echo hello")`** – runs the string in a shell (like Python’s `shell=True`). Use when you need shell features (e.g. `|`, `$VAR`).
- **`Command::new("echo").arg("hello")`** – runs `echo` directly, no shell. Safer and more portable when you don’t need shell syntax.

### 3.4 PTY (Pseudo-Terminal) in Brief

A **PTY** makes the program think it is talking to a real terminal. Useful for:

- Driving interactive CLI tools (e.g. expect-style automation).
- Getting “cooked” terminal behaviour (line buffering, echo).

Rust does not include PTY in the standard library. Common crates: **`pty-process`**, **`expectrl`**, or platform-specific APIs. For a first course, showing **`Command`** plus a short note on PTY is enough; full PTY examples depend on the chosen crate and OS.

### 3.5 Practical Example: Shell and Process Output

**Goal:** Run a shell command (e.g. `echo` or `uname`), capture stdout/stderr and exit code, and print them—demonstrating process spawning and output handling.

**File: `src/main.rs`** (or `examples/shell_process.rs`):

```rust
use std::process::Command;
use std::io::Result;

fn main() -> Result<()> {
    // Run a command directly (no shell)
    let output = Command::new("echo")
        .arg("Hello from Rust process")
        .output()?;

    println!("Exit status: {}", output.status);
    println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Run via shell (e.g. list files)
    let out2 = Command::new("sh")
        .arg("-c")
        .arg("echo 'Shell says:' && whoami")
        .output()?;
    println!("Shell output:\n{}", String::from_utf8_lossy(&out2.stdout));

    Ok(())
}
```

**How it works:**

- `Command::new("echo").arg(...).output()` runs `echo`, waits for it, and returns stdout/stderr and status.
- `String::from_utf8_lossy` turns bytes into a string (replacing invalid UTF-8 with a replacement character).
- The second block runs `sh -c "..."` so we can use `&&` and shell behaviour.

**Run:** `cargo run`

**Expected output (example on a Unix system):**

```
Exit status: exit status: 0
Stdout: Hello from Rust process

Shell output:
Shell says:
your_username
```

**Input:** None.  
**Output:** Exit status, stdout (and stderr if any), and the result of the shell command—showing how to integrate with the shell and external programs in automation scripts.

---

## 4. Working with Hardware (Basics)

**Duration:** ~2.5 h (lecture ~1.5 h, practice ~1 h)

### 4.1 What “Hardware” Means Here

In automation, “hardware” often means:

- **Serial ports** (UART) – sensors, actuators, PLCs, legacy devices.
- **GPIO** – digital inputs/outputs on embedded Linux (e.g. Raspberry Pi, BeagleBone).
- **Buses** (I2C, SPI) – chips on the board; usually via kernel drivers and device nodes.

We focus on **serial port** as a universal, cross-platform example. The same program can run on a PC with a USB–serial adapter or on an embedded device with `/dev/ttyS0` or `/dev/ttyUSB0`.

### 4.2 Serial Port in Rust

The **`serialport`** crate provides:

- List available ports.
- Open a port with baud rate, data bits, stop bits, parity.
- Read/write bytes (blocking or with timeouts).

You add to `Cargo.toml`:

```toml
[dependencies]
serialport = "4"
```

### 4.3 Practical Example: Enumerate Ports and Optional Loopback

**Goal:** Enumerate serial ports (and, if the user passes a port name, open it and send/receive a few bytes). The example **compiles and runs** on any system; without a real device it only lists ports (or shows “no ports”). With a loopback adapter or two connected ports, you can see read/write.

**File: `Cargo.toml`** (excerpt):

```toml
[dependencies]
serialport = "4"
```

**File: `src/main.rs`** (or `examples/serial_basics.rs`):

```rust
use serialport::SerialPortType;
use std::io::{Read, Write};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ports = serialport::available_ports()?;
    println!("Available serial ports:");
    for p in &ports {
        match &p.port_type {
            SerialPortType::UsbPort(info) => {
                println!("  {} (USB: {:?})", p.port_name, info);
            }
            SerialPortType::PciPort => println!("  {} (PCI)", p.port_name),
            SerialPortType::BluetoothPort => println!("  {} (Bluetooth)", p.port_name),
            _ => println!("  {}", p.port_name),
        }
    }
    if ports.is_empty() {
        println!("  (none)");
    }

    // If user provides a port, open and do a minimal write/read
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        let port_name = &args[1];
        let mut port = serialport::new(port_name, 9600)
            .timeout(Duration::from_millis(100))
            .open()?;
        let msg = b"Hello hardware!\n";
        port.write_all(msg)?;
        port.flush()?;
        println!("Wrote {} bytes to {}", msg.len(), port_name);
        let mut buf = [0u8; 64];
        match port.read(&mut buf) {
            Ok(n) => println!("Read {} bytes: {:?}", n, &buf[..n]),
            Err(_) => println!("No data read (timeout or no device echoing)"),
        }
    } else {
        println!("Usage: {} [port_name]  (e.g. /dev/ttyUSB0 or COM3)", args.get(0).unwrap_or(&"program".into()));
    }
    Ok(())
}
```

**How it works:**

- `serialport::available_ports()` returns a list of port names and types (USB, PCI, etc.).
- `serialport::new(name, baud).timeout(...).open()` opens the port with a 100 ms read timeout.
- `write_all` and `read` use the same `Read`/`Write` traits as files; the code is generic over the medium.

**Run:**

```bash
cargo run
cargo run -- /dev/ttyUSB0   # Linux example
cargo run -- COM3           # Windows example
```

**Expected output (no port argument):**

```
Available serial ports:
  /dev/ttyUSB0 (USB: ...)
  (or "(none)" if no ports)
Usage: target/debug/your_binary [port_name]  ...
```

**With a port argument:** “Wrote 14 bytes…” and either “Read N bytes: …” if the device echoes, or “No data read” on timeout.  
**Input:** Optional CLI argument = serial port name.  
**Output:** List of ports and, when a port is given, confirmation of write and optional read—demonstrating hardware I/O with a compilable, runnable example.

### 4.4 GPIO and Other Hardware (Short Note)

- **Linux GPIO:** Use **`gpio-cdev`** with `/dev/gpiochipN` (modern ABI). Requires appropriate permissions and hardware.
- **Raspberry Pi:** **`rppal`** gives GPIO, I2C, SPI, UART on that platform.
- **Embedded (no OS):** The **`embedded-hal`** ecosystem and HAL crates per chip (e.g. `stm32f4xx-hal`) are the next step; they are beyond “basics” but good to mention as the path from “serial on Linux” to “bare-metal automation.”

---

## 5. Error Handling and Robustness

**Duration:** ~1.5 h (lecture ~1 h, practice ~0.5 h)

### 5.1 Why It Matters in Automation

Automation code often runs unattended. Crashes or silent failures are unacceptable. Rust encourages:

- **Explicit error types** – no uncaught exceptions.
- **`Result<T, E>`** – either a value or an error; the caller must handle both.
- **`?`** – propagate errors upward without boilerplate.

### 5.2 Result and Option

- **`Result<T, E>`** – `Ok(value)` or `Err(e)`; used for fallible operations (I/O, parsing).
- **`Option<T>`** – `Some(x)` or `None`; used for “maybe missing” (e.g. “first line of file”).

### 5.3 Practical Example: Robust File Read with Error Handling

**Goal:** Read the first line of a file; print it or a clear error (file missing, permission, empty file).

**File: `src/main.rs`** (or `examples/error_handling.rs`):

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn first_line(path: &str) -> Result<String, std::io::Error> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    Ok(line.trim().to_string())
}

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "Cargo.toml".to_string());
    match first_line(&path) {
        Ok(s) if s.is_empty() => println!("File is empty or first line is blank"),
        Ok(s) => println!("First line: {}", s),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

**How it works:**

- `File::open(path)?` and `read_line(...)?` return errors to the caller instead of panicking.
- `match first_line(...)` forces handling of both `Ok` and `Err`; we print a clear message for empty first line and for I/O errors.

**Run:**

```bash
cargo run
cargo run -- /nonexistent
```

**Expected output (with `Cargo.toml`):**

```
First line: [package]
```

**With missing file:**

```
Error: No such file or directory (os error 2)
```

**Input:** Optional path (default `Cargo.toml`).  
**Output:** First line of the file or an explicit error—showing how Rust makes error handling part of the API and encourages robust automation code.

---

## Chapter 2 Summary

| Topic           | Takeaway                                                                 |
|-----------------|--------------------------------------------------------------------------|
| File I/O        | `File`, `BufReader`/`BufWriter`, `Read`/`Write`; same traits for files and streams. |
| Bitwise / binary| `&`, `\|`, `^`, `<<`, `>>`; `u8` and `to_be_bytes`/`from_be_bytes` for protocols. |
| Processes       | `std::process::Command` for spawning, piping, and capturing output; shell via `sh -c`. |
| PTY             | For interactive tools; use a crate (e.g. `pty-process`); concept explained. |
| Hardware        | Serial with `serialport` (list ports, open, read/write); GPIO/embedded mentioned. |
| Error handling  | `Result` and `?` for robust, explicit error propagation in automation.   |

**Total suggested time:** ~10 h (theory ~7 h, practice ~3 h). Together with Chapter 1 (~14 h), the course fits in about **24 hours** with the requested 70% / 30% split.

---

## Practice Suggestions

1. **File I/O:** Write a program that reads a CSV with two columns (name, value), parses each line, and writes a new file with lines `name=value`.
2. **Bitwise:** Implement a function that takes a `u16` and returns (bit 15 as “sign”, bits 0–11 as “magnitude”); pack and unpack.
3. **Process:** Run `ls -la` (or `dir` on Windows) via `Command`, capture stdout, and print only the first 5 lines.
4. **Serial:** If you have a board or USB–serial adapter, send a command string and log the first line of the response to a file.
