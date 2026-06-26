#!/usr/bin/env python3
"""Build a single Kindle-friendly PDF from rust-core markdown sources."""

from __future__ import annotations

import base64
import re
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
KINDLE = Path(__file__).resolve().parent
DIST = ROOT / "dist"
BUILD = KINDLE / "build"
OUTPUT = DIST / "Rust-Core-Kindle.pdf"
COVER_SVG = KINDLE / "cover-art.svg"
COVER_JPG = KINDLE / "cover-page.jpg"
COVER_LOGO = KINDLE / "rust-logo-icons8.png"
# 6×9 in @ 300 DPI — matches print trim; JPEG keeps Android viewers happy.
COVER_PX_W = 1800
COVER_PX_H = 2700
# Bump when rasterization logic changes (invalidates cached cover-page.jpg).
COVER_RENDER_ID = "cairosvg-v3"

PARTS: list[tuple[str, list[Path]]] = [
    (
        "Language Foundation",
        [
            ROOT / "chapters/preface.md",
            ROOT / "chapters/01_paradigm_shift.md",
            ROOT / "chapters/02_types.md",
            ROOT / "chapters/03_functions.md",
            ROOT / "chapters/04_iterators.md",
            ROOT / "chapters/05_lifetimes.md",
            ROOT / "chapters/06_types_enums_pattern_matching.md",
            ROOT / "chapters/07_structs_traits_generics.md",
            ROOT / "chapters/08_errors_and_testing.md",
        ],
    ),
    (
        "Crates, Memory, and Data",
        [
            ROOT / "chapters/09_modules_paths_crates.md",
            ROOT / "chapters/10_smart_pointers_interior_mutability.md",
            ROOT / "chapters/11_collections.md",
            ROOT / "chapters/12_closures.md",
            ROOT / "chapters/13_standard_traits.md",
        ],
    ),
    (
        "Concurrency",
        [
            ROOT / "chapters/14_multithreading.md",
            ROOT / "chapters/15_atomics_and_lockfree.md",
            ROOT / "chapters/16_async_tokio.md",
        ],
    ),
    (
        "Metaprogramming and Unsafe",
        [
            ROOT / "chapters/17_metaprogramming.md",
            ROOT / "chapters/18_unsafe_and_internals.md",
        ],
    ),
    (
        "Standard Library I/O",
        [ROOT / "chapters/19_io_processes_bits.md"],
    ),
    (
        "Production Standards",
        [ROOT / "chapters/20_production_standards.md"],
    ),
]

APPENDICES: list[Path] = [
    ROOT / "appendix/PLAYGROUND_GUIDE.md",
    ROOT / "appendix/JAVA_PYTHON_RUST_MAP.md",
    ROOT / "appendix/AI_PROMPT_INDEX.md",
]

MERMAID_RE = re.compile(r"```mermaid\n.*?```", re.DOTALL)
CHAPTER_H1_RE = re.compile(r"^# Chapter \d+:\s*", re.MULTILINE)
INTERNAL_LINK_RE = re.compile(
    r"\[([^\]]+)\]\((?:\.\./)?(?:chapters/|appendix/)?[\w./-]+\.md(?:#[\w-]+)?\)"
)
RELATIVE_LINK_RE = re.compile(r"\[([^\]]+)\]\(\.\./[^)]+\)")
HTML_COMMENT_RE = re.compile(r"<!--.*?-->", re.DOTALL)
TABLE_ROW_RE = re.compile(r"^\|.+\|\s*$", re.MULTILINE)

# ~48 cols fits 6×9 page with line-number gutter (footnotesize monospace).
CODE_WRAP_WIDTH = 48

UNICODE_REPLACEMENTS = {
    "\u2192": "->",
    "\u2190": "<-",
    "\u2194": "<->",
    "\u2014": "---",
    "\U0001F980": "(crab)",
}


def normalize_unicode(text: str) -> str:
    for src, dst in UNICODE_REPLACEMENTS.items():
        text = text.replace(src, dst)
    return text


def escape_latex_in_table_cell(cell: str) -> str:
    """Escape bare & and % in table cells; leave `code` spans to pandoc."""
    parts = re.split(r"(`[^`]*`)", cell)
    out: list[str] = []
    for i, part in enumerate(parts):
        if i % 2 == 1:
            out.append(part)
        else:
            out.append(part.replace("&", "\\&").replace("%", "\\%"))
    return "".join(out)


def fix_empty_table_headers(text: str) -> str:
    """Pandoc shrinks columns whose header cell is empty — causes overlap."""
    lines = text.splitlines(keepends=True)
    out: list[str] = []
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.rstrip("\n")
        if (
            i + 1 < len(lines)
            and re.match(r"^\|\s*\|", stripped)
            and re.match(r"^\|[\s\-:|]+\|\s*$", lines[i + 1].strip())
        ):
            newline = "\n" if line.endswith("\n") else ""
            line = re.sub(r"^\|\s*\|", "| Aspect |", stripped, count=1) + newline
        out.append(line)
        i += 1
    return "".join(out)


def normalize_table_separators(text: str) -> str:
    """Replace ragged --- separators with uniform | --- | --- | ... |."""
    lines = text.splitlines(keepends=True)
    out: list[str] = []
    for i, line in enumerate(lines):
        if (
            i > 0
            and re.match(r"^\|[\s\-:|]+\|\s*$", line.strip())
            and "|" in lines[i - 1]
            and not re.match(r"^\|[\s\-:|]+\|\s*$", lines[i - 1].strip())
        ):
            ncols = len([p for p in lines[i - 1].strip().strip("|").split("|")])
            out.append("| " + " | ".join(["---"] * ncols) + " |\n")
        else:
            out.append(line)
    return "".join(out)


def split_table_header_code_rows(text: str) -> str:
    """Move (`fn f(...)`) from header cells into a row below the separator."""
    lines = text.splitlines(keepends=True)
    out: list[str] = []
    i = 0
    while i < len(lines):
        line = lines[i]
        if (
            i + 1 < len(lines)
            and "|" in line
            and re.match(r"^\|[\s\-:|]+\|\s*$", lines[i + 1].strip())
            and not re.match(r"^\|[\s\-:|]+\|\s*$", line.strip())
        ):
            parts = [p.strip() for p in line.strip().strip("|").split("|")]
            cleaned: list[str] = []
            extras: list[str] = []
            split_any = False
            for part in parts:
                m = re.match(r"^(.*?)\s+\(`([^`]+)`\)\s*$", part)
                if m:
                    cleaned.append(m.group(1).strip())
                    extras.append(f"`{m.group(2)}`")
                    split_any = True
                else:
                    cleaned.append(part)
                    extras.append("")
            if split_any:
                out.append("| " + " | ".join(cleaned) + " |\n")
                out.append("| " + " | ".join(["---"] * len(cleaned)) + " |\n")
                extra_cells = [c if c else " " for c in extras]
                out.append("| " + " | ".join(extra_cells) + " |\n")
                i += 2
                continue
        out.append(line)
        i += 1
    return "".join(out)


def fix_table_rows(text: str) -> str:
    """Escape LaTeX-special chars inside pipe-table cells."""

    def fix_row(match: re.Match[str]) -> str:
        line = match.group(0)
        if re.match(r"^\|[\s\-:|]+\|\s*$", line):
            return line
        parts = line.split("|")
        if len(parts) < 3:
            return line
        fixed: list[str] = []
        for i, part in enumerate(parts):
            if i == 0 or i == len(parts) - 1:
                fixed.append(part)
            else:
                fixed.append(escape_latex_in_table_cell(part))
        return "|".join(fixed)

    return TABLE_ROW_RE.sub(fix_row, text)


def _wrap_segment(text: str, width: int, prefix: str, cont_prefix: str) -> list[str]:
    """Greedy word-wrap with fixed first-line and continuation prefixes."""
    if not text:
        return []
    if len(prefix + text) <= width:
        return [prefix + text]
    lines: list[str] = []
    remaining = text
    current_prefix = prefix
    while remaining:
        budget = width - len(current_prefix)
        if budget < 8:
            current_prefix = cont_prefix
            budget = width - len(cont_prefix)
        if len(remaining) <= budget:
            lines.append(current_prefix + remaining)
            break
        # Prefer breaking at space; else hard-break for long tokens
        chunk = remaining[:budget]
        break_at = chunk.rfind(" ")
        if break_at <= 0:
            break_at = budget
        lines.append(current_prefix + remaining[:break_at].rstrip())
        remaining = remaining[break_at:].lstrip()
        current_prefix = cont_prefix
    return lines


def wrap_single_code_line(line: str, width: int = CODE_WRAP_WIDTH) -> list[str]:
    """Wrap one long line inside a fenced code block."""
    stripped = line.rstrip("\n")
    if len(stripped) <= width:
        return [line]

    indent_len = len(stripped) - len(stripped.lstrip(" "))
    prefix = stripped[:indent_len]
    body = stripped[indent_len:]
    cont = prefix + "    "

    # Put trailing // comments on their own wrapped lines (common in teaching examples)
    if " // " in body:
        code, _, comment = body.partition(" // ")
        code_line = prefix + code
        if len(code_line) <= width:
            wrapped = [code_line]
            wrapped.extend(
                _wrap_segment(comment, width, prefix + "// ", prefix + "// ")
            )
            return [w + "\n" for w in wrapped]

    wrapped = _wrap_segment(body, width, prefix, cont)
    return [w + "\n" for w in wrapped]


def wrap_code_blocks(text: str, width: int = CODE_WRAP_WIDTH) -> str:
    """Hard-wrap source lines so code stays inside the 6×9 text block."""
    lines = text.splitlines(keepends=True)
    out: list[str] = []
    in_fence = False
    for line in lines:
        if line.lstrip().startswith("```"):
            in_fence = not in_fence
            out.append(line)
            continue
        if in_fence and line.strip():
            out.extend(wrap_single_code_line(line, width))
        else:
            out.append(line)
    return "".join(out)


def normalize_chapter_heading(text: str, *, path: Path | None = None) -> str:
    """Align PDF chapter numbers with source titles.

    Pandoc ``--number-sections`` prepends ``Chapter N.`` in headers and TOC.
    Source files also use ``# Chapter N: Title`` — strip the manual prefix so
    headers read ``Chapter 1. Paradigm Shift``, not ``Chapter 2. Chapter 1: …``.
    """
    if path and path.name == "preface.md":
        text = re.sub(r"^# Preface\s*$", "# Preface {-}", text, count=1, flags=re.MULTILINE)
    return CHAPTER_H1_RE.sub("# ", text)


def preprocess(text: str, *, path: Path | None = None) -> str:
    text = normalize_unicode(text)
    text = MERMAID_RE.sub(
        "\n\n*[Diagram omitted in print edition — see the web repository.]*\n\n",
        text,
    )
    text = INTERNAL_LINK_RE.sub(r"\1", text)
    text = RELATIVE_LINK_RE.sub(r"\1", text)
    text = HTML_COMMENT_RE.sub("", text)
    text = fix_empty_table_headers(text)
    text = normalize_table_separators(text)
    text = split_table_header_code_rows(text)
    text = fix_table_rows(text)
    text = wrap_code_blocks(text)
    text = normalize_chapter_heading(text, path=path)
    return text.strip() + "\n\n"


def part_break(title: str) -> str:
    return f"""```{{=latex}}
\\cleardoublepage
\\part{{{title}}}
```

"""


def appendix_break() -> str:
    return """```{=latex}
\\appendix
```

# Appendices {-}

"""


def assemble_book() -> Path:
    BUILD.mkdir(parents=True, exist_ok=True)
    book = BUILD / "Rust-Core-Kindle.md"

    chunks: list[str] = []
    first_part = True
    for part_title, files in PARTS:
        if not first_part:
            chunks.append(part_break(part_title))
        first_part = False
        for path in files:
            if not path.exists():
                raise FileNotFoundError(path)
            chunks.append(preprocess(path.read_text(encoding="utf-8"), path=path))

    chunks.append(appendix_break())
    for path in APPENDICES:
        if not path.exists():
            raise FileNotFoundError(path)
        chunks.append(preprocess(path.read_text(encoding="utf-8"), path=path))

    book.write_text("".join(chunks), encoding="utf-8")
    return book


def _cover_source_mtime() -> float:
    stamp = KINDLE / ".cover-render-stamp"
    paths = [COVER_SVG, COVER_LOGO, KINDLE / "cover-page.tex", stamp]
    return max(p.stat().st_mtime for p in paths if p.exists())


def _cover_needs_render() -> bool:
    stamp = KINDLE / ".cover-render-stamp"
    if not COVER_JPG.exists():
        return True
    if not stamp.exists() or stamp.read_text(encoding="utf-8").strip() != COVER_RENDER_ID:
        return True
    return COVER_JPG.stat().st_mtime < _cover_source_mtime()


def _cover_svg_with_inlined_logo() -> bytes:
    """cairosvg does not resolve external href= PNGs — inline as data URI."""
    if not COVER_LOGO.exists():
        raise FileNotFoundError(COVER_LOGO)
    svg_text = COVER_SVG.read_text(encoding="utf-8")
    logo_uri = "data:image/png;base64," + base64.b64encode(COVER_LOGO.read_bytes()).decode(
        "ascii"
    )
    svg_text = svg_text.replace('href="rust-logo-icons8.png"', f'href="{logo_uri}"')
    svg_text = re.sub(
        r'(<image\b[^>]*)\shref="[^"]*"',
        rf'\1 href="{logo_uri}" xlink:href="{logo_uri}"',
        svg_text,
        count=1,
    )
    return svg_text.encode("utf-8")


def _svg_to_png_cairosvg(svg: Path, png: Path) -> bool:
    try:
        import cairosvg
    except ImportError:
        return False
    cairosvg.svg2png(
        bytestring=_cover_svg_with_inlined_logo(),
        write_to=str(png),
        output_width=COVER_PX_W,
        output_height=COVER_PX_H,
    )
    return png.exists()


def _svg_to_png_rsvg(svg: Path, png: Path) -> bool:
    exe = shutil.which("rsvg-convert")
    if not exe:
        return False
    subprocess.run(
        [exe, "-w", str(COVER_PX_W), "-h", str(COVER_PX_H), "-o", str(png), str(svg)],
        check=True,
    )
    return png.exists()


def _svg_to_png(svg: Path, png: Path) -> None:
    if _svg_to_png_cairosvg(svg, png) or _svg_to_png_rsvg(svg, png):
        return
    raise RuntimeError(
        "Cannot rasterize cover-art.svg. Install one of:\n"
        "  pip install cairosvg Pillow\n"
        "  brew install librsvg   # provides rsvg-convert"
    )


def _png_to_jpeg(png: Path, jpg: Path) -> None:
    try:
        from PIL import Image
    except ImportError as exc:
        raise RuntimeError(
            "Pillow is required to build the mobile-safe cover JPEG. "
            "Install with: pip install Pillow"
        ) from exc
    with Image.open(png) as im:
        rgb = im.convert("RGB")
        if rgb.size != (COVER_PX_W, COVER_PX_H):
            raise RuntimeError(
                f"Cover PNG is {rgb.size}, expected {(COVER_PX_W, COVER_PX_H)} — "
                "check the SVG rasterizer (do not use qlmanage thumbnails)."
            )
        rgb.save(jpg, "JPEG", quality=92, optimize=True, progressive=True)


def render_cover_jpeg() -> Path:
    """Rasterize cover-art.svg to a flat JPEG (no alpha/transparency for mobile PDF viewers)."""
    if not _cover_needs_render():
        return COVER_JPG

    if not COVER_SVG.exists():
        raise FileNotFoundError(COVER_SVG)

    tmp_png = KINDLE / "cover-page.tmp.png"
    _svg_to_png(COVER_SVG, tmp_png)
    _png_to_jpeg(tmp_png, COVER_JPG)
    tmp_png.unlink(missing_ok=True)
    (KINDLE / ".cover-render-stamp").write_text(COVER_RENDER_ID, encoding="utf-8")
    print(f"  → {COVER_JPG} ({COVER_JPG.stat().st_size // 1024} KB, mobile-safe JPEG)")
    return COVER_JPG


def run_pandoc(book: Path) -> None:
    DIST.mkdir(parents=True, exist_ok=True)
    cmd = [
        "pandoc",
        str(book),
        "--from=markdown",
        "--to=pdf",
        "--pdf-engine=xelatex",
        "--listings",
        "--toc",
        "--toc-depth=2",
        "--number-sections",
        "--top-level-division=chapter",
        "--metadata-file",
        str(KINDLE / "metadata.yaml"),
        "--include-in-header",
        str(KINDLE / "header.tex"),
        "--include-before-body",
        str(KINDLE / "cover-page.tex"),
        "--syntax-highlighting=tango",
        "--resource-path",
        f"{ROOT}:{ROOT / 'chapters'}:{ROOT / 'appendix'}",
        "-o",
        str(OUTPUT),
    ]
    print("Running:", " ".join(cmd))
    subprocess.run(cmd, check=True, cwd=ROOT)


def main() -> int:
    print("Assembling markdown…")
    book = assemble_book()
    print(f"  → {book} ({book.stat().st_size // 1024} KB)")
    print("Rendering cover JPEG (mobile-safe page 1)…")
    render_cover_jpeg()
    print("Generating PDF (xelatex — may take a few minutes)…")
    run_pandoc(book)
    size_mb = OUTPUT.stat().st_size / (1024 * 1024)
    print(f"Done: {OUTPUT} ({size_mb:.1f} MB)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
