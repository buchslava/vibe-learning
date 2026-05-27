#!/usr/bin/env python3
"""Extract and verify Rust examples from rust-core/chapters markdown."""

from __future__ import annotations

import re
import shutil
import subprocess
import sys
import tempfile
from dataclasses import dataclass, field
from pathlib import Path

CHAPTERS_DIR = Path(__file__).resolve().parent.parent / "chapters"

SKIP_SUBSTRINGS = (
    "does not compile",
    "uncomment one failing",
    "uncomment one wrong",
    "uncomment to see the error",
    "uncomment `drop`",
    "macro ok; main fails",
    "macro parses; expanded code fails",
    "does not compile when used",
    "conceptual boilerplate",
    "anti-pattern",
    "awkward; prefer",
    "with thiserror apperror above",
    "composite sketch",
    "signatures only; not runnable",
    "cargo only",
    "src/lib.rs or",
    "src/main.rs",
    "src/config.rs",
    "lib.rs pattern",
    "tests/parse_config.rs",
    "cli pattern (conceptual)",
    "stack overflow at runtime",
)

# Compile-only: expected to build but must not be executed.
RUN_SKIP_SUBSTRINGS = (
    "panics at runtime",
    "panics on",
    "runs, panics",
    "stdin blocks until enter",
    "unsound",
    "do not ship",
    "do not write",
    "illustration only",
    "do not do this",
    "conceptual bug",
    "may silently fail",
    "logic bug",
    "outline",
    "stdin",
    "read_line",
)

CARGO_DEPS: dict[str, str] = {
    "tokio": '{ version = "1", features = ["full"] }',
    "thiserror": '"2"',
    "anyhow": '"1"',
    "serde": '{ version = "1", features = ["derive"] }',
    "clap": '{ version = "4", features = ["derive"] }',
}


@dataclass
class Example:
    chapter: str
    index: int
    marker: str
    code: str
    toml: str | None = None
    skipped: bool = False
    skip_reason: str = ""
    compile_only: bool = False


def first_comment_line(code: str) -> str:
    for line in code.splitlines():
        s = line.strip()
        if s.startswith("//"):
            return s
    return ""


def should_skip(marker: str, code: str) -> str | None:
    low = marker.lower()
    for sub in SKIP_SUBSTRINGS:
        if sub in low:
            return sub
    if not marker.startswith("// Playground") and not marker.startswith("// Cargo"):
        return "no playground/cargo marker"
    # Entirely commented-out teaching fragments
    lines = [ln for ln in code.splitlines() if ln.strip() and not ln.strip().startswith("//")]
    if not lines:
        return "comment-only block"
    if "conceptual" in low and "needs libc" in low:
        return "needs libc"
    if "conceptual" in low and "needs compile-time env" in low:
        return "needs env"
    return None


def run_skip_reason(marker: str) -> str | None:
    low = marker.lower()
    for sub in RUN_SKIP_SUBSTRINGS:
        if sub in low:
            return sub
    if "outline" in low:
        return "outline sketch"
    return None


def parse_chapters() -> list[Example]:
    examples: list[Example] = []
    md_files = sorted(CHAPTERS_DIR.glob("*.md"))
    md_files = [p for p in md_files if p.name != "preface.md"]

    for md_path in md_files:
        text = md_path.read_text(encoding="utf-8")
        chapter = md_path.name
        blocks = re.findall(r"```rust\n(.*?)```", text, re.DOTALL)
        pending_toml: str | None = None

        # Also capture toml blocks for dependency hints
        all_parts = re.split(r"```(rust|toml)\n", text)
        # Re-walk with paired toml
        idx = 0
        last_toml: str | None = None
        for m in re.finditer(r"```(rust|toml)\n(.*?)```", text, re.DOTALL):
            lang, body = m.group(1), m.group(2)
            if lang == "toml":
                last_toml = body.strip()
                continue
            marker = first_comment_line(body)
            if not marker and not body.strip().startswith("#!"):
                # file fragment without playground marker — skip
                idx += 1
                continue
            ex = Example(
                chapter=chapter,
                index=idx,
                marker=marker,
                code=body,
                toml=last_toml if marker.startswith("// Cargo") else None,
            )
            reason = should_skip(marker, body)
            if reason:
                ex.skipped = True
                ex.skip_reason = reason
            rs = run_skip_reason(marker)
            if rs:
                ex.compile_only = True
            if "cargo project" in marker.lower() or "cargo only" in marker.lower():
                if ex.toml is None and last_toml:
                    ex.toml = last_toml
            examples.append(ex)
            idx += 1
            if not marker.startswith("// Cargo"):
                last_toml = None

    return examples


def detect_deps(code: str, toml: str | None) -> dict[str, str]:
    deps: dict[str, str] = {}
    blob = code + (toml or "")
    for name, spec in CARGO_DEPS.items():
        if name in blob or f"{name} =" in blob:
            deps[name] = spec
    if toml:
        for line in toml.splitlines():
            m = re.match(r"^\s*(\w+)\s*=", line)
            if m and m.group(1) not in ("package", "edition"):
                k = m.group(1)
                if k not in deps and k in CARGO_DEPS:
                    deps[k] = CARGO_DEPS[k]
    return deps


def wrap_for_binary(code: str) -> str:
    if "fn main(" in code or "#[tokio::main]" in code:
        return code
    if "#[cfg(test)]" in code and "fn main(" not in code:
        return code + "\n\nfn main() {\n    // stub for binary check\n}\n"
    return code + "\n\nfn main() {\n    // auto stub\n}\n"


def is_test_crate(code: str) -> bool:
    return "#[cfg(test)]" in code and "fn main(" not in code


def verify_one(ex: Example, work: Path) -> tuple[bool, str, bool]:
    """Returns (success, message, ran)."""
    proj = work / "crate"
    if proj.exists():
        shutil.rmtree(proj)
    src = proj / "src"
    src.mkdir(parents=True)

    deps = detect_deps(ex.code, ex.toml)
    if ex.toml and "[package]" in ex.toml:
        cargo_toml = ex.toml
        if deps and "[dependencies]" not in cargo_toml:
            cargo_toml += "\n[dependencies]\n"
            for k, v in sorted(deps.items()):
                cargo_toml += f"{k} = {v}\n"
    elif ex.toml and "[dependencies]" in ex.toml:
        cargo_toml = (
            "[package]\nname = \"snippet\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n"
            + ex.toml
        )
        if deps:
            for k, v in sorted(deps.items()):
                if f"{k} =" not in cargo_toml:
                    if "[dependencies]" not in cargo_toml:
                        cargo_toml += "\n[dependencies]\n"
                    elif not cargo_toml.endswith("\n"):
                        cargo_toml += "\n"
                    cargo_toml += f"{k} = {v}\n"
    else:
        cargo_toml = (
            "[package]\nname = \"snippet\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n"
        )
        if deps:
            cargo_toml += "[dependencies]\n"
            for k, v in sorted(deps.items()):
                cargo_toml += f"{k} = {v}\n"

    (proj / "Cargo.toml").write_text(cargo_toml, encoding="utf-8")

    code = ex.code
    if is_test_crate(code):
        (src / "lib.rs").write_text(code, encoding="utf-8")
        (src / "main.rs").write_text("fn main() {}\n", encoding="utf-8")
    elif "fn main(" in code or "#[tokio::main]" in code:
        (src / "main.rs").write_text(code, encoding="utf-8")
    else:
        (src / "lib.rs").write_text(code, encoding="utf-8")
        (src / "main.rs").write_text("fn main() {}\n", encoding="utf-8")

    check = subprocess.run(
        ["cargo", "build", "--quiet"],
        cwd=proj,
        capture_output=True,
        text=True,
    )
    if check.returncode != 0:
        err = (check.stderr or check.stdout or "").strip()
        return False, err[-4000:], False

    if ex.compile_only:
        return True, "compile-only", False

    if is_test_crate(ex.code):
        test = subprocess.run(
            ["cargo", "test", "--quiet"],
            cwd=proj,
            capture_output=True,
            text=True,
        )
        if test.returncode != 0:
            err = (test.stderr or test.stdout or "").strip()
            return False, f"tests failed:\n{err[-3000:]}", False
        return True, "tests ok", True

    if "fn main(" not in ex.code and "#[tokio::main]" not in ex.code:
        return True, "compile-only (no main)", False

    run = subprocess.run(
        ["cargo", "run", "--quiet"],
        cwd=proj,
        capture_output=True,
        text=True,
        timeout=15,
    )
    if run.returncode != 0:
        err = (run.stderr or run.stdout or "").strip()
        return False, f"run failed:\n{err[-3000:]}", False
    return True, "run ok", True


def main() -> int:
    examples = parse_chapters()
    skipped = [e for e in examples if e.skipped]
    runnable = [e for e in examples if not e.skipped]

    print(f"Found {len(examples)} rust blocks in chapters")
    print(f"  Skipped (intentional fail): {len(skipped)}")
    print(f"  To verify: {len(runnable)}")
    print()

    failures: list[tuple[Example, str]] = []
    ok = 0

    with tempfile.TemporaryDirectory(prefix="rust-chapter-check-") as tmp:
        work = Path(tmp)
        for i, ex in enumerate(runnable):
            # fresh dir per example
            for child in work.iterdir():
                if child.is_dir():
                    shutil.rmtree(child)
            try:
                success, msg, _ran = verify_one(ex, work)
            except subprocess.TimeoutExpired:
                success, msg = False, "timeout"
            except Exception as e:
                success, msg = False, str(e)

            label = f"{ex.chapter}#{ex.index}"
            if success:
                ok += 1
                print(f"OK   {label} ({msg})")
            else:
                failures.append((ex, msg))
                print(f"FAIL {label}")
                print(f"     marker: {ex.marker[:80]}")
                print(f"     {msg[:500]}")
                print()

    print()
    print(f"Results: {ok} passed, {len(failures)} failed, {len(skipped)} skipped")
    if failures:
        report = Path(__file__).parent / "verify_failures.txt"
        lines = []
        for ex, msg in failures:
            lines.append(f"=== {ex.chapter} #{ex.index} ===")
            lines.append(ex.marker)
            lines.append(msg)
            lines.append("")
        report.write_text("\n".join(lines), encoding="utf-8")
        print(f"Details written to {report}")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
