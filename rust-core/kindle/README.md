# Kindle / KDP PDF build

Produces a single **6×9 in** PDF tuned for Amazon Kindle and KDP print replica. **Page 1 is the custom cover** (`cover-page.tex`). Pandoc’s `\maketitle` is disabled in `header.tex` (Pandoc 3.x would otherwise insert a title page before the cover).

- Preface + chapters 1–20 (all six parts)
- Appendices: Playground Guide, Java/Python/Rust map, AI Prompt Index
- Table of contents with chapter numbers (auto-numbered; manual `Chapter N:` prefixes stripped from titles)
- Syntax-highlighted Rust code (no line numbers — wrapped lines stay unambiguous)
- **Hard-wrapped** code lines (~48 cols) so nothing overflows the 6×9 page
- Pipe tables sanitized for LaTeX (`&` in cells, empty header → **Aspect** for equal column widths)
- Mermaid diagrams replaced with a short print-edition note

## Cover

Page 1 is a **flat JPEG cover** pasted full bleed at 6×9 in via `eso-pic` (no `\newgeometry`, which caused a blank first page and clipped the image on mobile).

| File | Role |
|------|------|
| `cover-page.tex` | Inserts `cover-page.jpg` full bleed via `--include-before-body` |
| `cover-page.jpg` | Generated at build time (not hand-edited) |
| `cover-art.svg` | Cover source artwork — edit this, then rebuild |
| `rust-logo-icons8.svg` / `.png` | Rust logo ([Icons8](https://icons8.com)) |
| `rust-logo.svg` | Official Rust vector ([rust-lang/rust-artwork](https://github.com/rust-lang/rust-artwork), CC-BY) |

Cover rasterization needs **Pillow** (`pip install Pillow`) and **cairosvg** (`pip install cairosvg`), or **rsvg-convert** (librsvg). Do not use macOS `qlmanage` — it crops/distorts the cover.

Cover logo by [Icons8](https://icons8.com). This book is not affiliated with or endorsed by the Rust project.

Rebuild after cover edits: `python3 kindle/build.py`

## Requirements

- [Pandoc](https://pandoc.org/) 3.x
- XeLaTeX (MacTeX, TeX Live, etc.)

## Build

```bash
python3 kindle/build.py
```

Output: **`dist/Rust-Core-Kindle.pdf`**

## Kindle upload tips

- **Send to Kindle** (email or app): upload the PDF directly; Kindle scales 6×9 reasonably on e-ink.
- **KDP**: use this PDF as *print replica* or convert to EPUB with Kindle Create if you want reflowable text.
- Internal repo links are flattened to plain text in the PDF; external URLs remain clickable.

## Regenerate after edits

Re-run `build.py` whenever chapter markdown changes. The merged source is written to `kindle/build/Rust-Core-Kindle.md` for debugging.
