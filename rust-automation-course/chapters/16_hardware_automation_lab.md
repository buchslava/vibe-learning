# Chapter 16: Hardware and Automation Lab

## Hook

University automation and industrial glue code still talk **serial**, **GPIO**, and simple **binary protocols**. Rust is a strong fit: predictable latency, no GC pauses mid-loop, and `Result` at every I/O boundary. This chapter ties the book together in a capstone you can run on a laptop (port list) or on a bench with USB–serial.

## Serial ports

The **`serialport`** crate wraps OS APIs cross-platform.

**Cargo only** — `Cargo.toml`:

```toml
[dependencies]
serialport = "4"
```

```rust
use std::io::{Read, Write};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for p in serialport::available_ports()? {
        println!("{}", p.port_name);
    }

    let Some(port_name) = std::env::args().nth(1) else { return Ok(()); };

    let mut port = serialport::new(&port_name, 9600)
        .timeout(Duration::from_millis(200))
        .open()?;

    port.write_all(b"PING\n")?;
    port.flush()?;

    let mut buf = [0u8; 64];
    match port.read(&mut buf) {
        Ok(n) => println!("rx: {:?}", &buf[..n]),
        Err(_) => println!("timeout (no echo)"),
    }
    Ok(())
}
```

Same **`Read`/`Write`** traits as files ([Chapter 15](15_io_processes_bits.md)).

## GPIO and embedded paths (orientation)

| Platform | Typical crate / API |
|----------|---------------------|
| Linux gpiochip | `gpio-cdev` |
| Raspberry Pi | `rppal` |
| MCU bare-metal | `embedded-hal` + chip HAL |

These are platform-specific; serial remains the portable teaching example.

## Robust unattended operation

Automation binaries should:

1. **Never panic** on bad user input or I/O — return `Result` or exit codes.
2. **Log context** — which port, which config path failed.
3. **Retry with backoff** for transient serial timeouts (not infinite spin).
4. **Structured errors** — see [Chapter 7](07_errors_and_testing.md).

```rust
// Playground — retry policy sketch (no real I/O)
fn with_retries<F: Fn() -> Option<u32>>(mut f: F, max: u32) -> Option<u32> {
    for attempt in 0..max {
        if let Some(v) = f() { return Some(v); }
        println!("retry {}", attempt + 1);
    }
    None
}

fn main() {
    let mut tries = 0;
    let read_sensor = || {
        tries += 1;
        if tries < 3 { None } else { Some(42) }
    };
    println!("{:?}", with_retries(read_sensor, 5));
}
```

## Capstone: sensor gateway CLI (outline)

Combine prior chapters into one project:

| Piece | Chapter |
|-------|---------|
| Parse config TOML | 2, 7, 9 |
| Encode/decode frame | 15 |
| Serial read loop | 15, 16 |
| Log lines to file | 15 |
| Optional async supervisor | 12 |
| Tests for parse/encode | 7 |

Suggested layout:

```
sensor_gateway/
  Cargo.toml
  src/main.rs      # CLI entry
  src/protocol.rs  # packet encode/decode
  src/serial_io.rs # port open, retry
  tests/frame.rs   # round-trip bytes
```

## Idiom spotlight

> **Treat the wire format as a contract.** Version your packet layout; reject unknown frame versions with explicit errors instead of parsing garbage.

## Go deeper

- Archive: [CHAPTER_02 §4–5](../archive/CHAPTER_02_AUTOMATION_AND_SYSTEM_PROGRAMMING.md)

## See also

- [Chapter 15: I/O](15_io_processes_bits.md)
- [Chapter 11: Atomics](11_atomics_and_lockfree.md) (metrics counters)
- [Chapter 10: Threads](10_multithreading.md) (background logger)

### Afterparty: AI Lego blocks

1. **Capstone scaffold** — “Generate module tree and function signatures for sensor_gateway; no bodies.”
2. **Serial debug** — “I get timeout on read; give systematic checklist (baud, cable, permissions).”
3. **Retry policy** — “Design exponential backoff for Modbus-style poll errors; Rust pseudocode.”
4. **Log schema** — “Propose JSON log lines for sensor events with timestamp and error codes.”
5. **GPIO next step** — “After serial works on Pi, outline migration to gpio-cdev for one LED.”
6. **Code review** — “I paste capstone main loop; review for panic risks and missing flush.”
