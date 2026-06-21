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

Page 1 is a custom **6×9 cover** (no duplicate Pandoc title page):

| File | Role |
|------|------|
| `cover-page.tex` | TikZ cover inserted via `--include-before-body` |
| `cover-art.svg` | Reference artwork (web preview, KDP thumbnail mockups) |
| `rust-logo-icons8.svg` | Rust logo source ([Icons8](https://icons8.com)) |
| `rust-logo-icons8.png` | Raster used by XeLaTeX on the cover |
| `rust-logo.svg` | Official Rust vector ([rust-lang/rust-artwork](https://github.com/rust-lang/rust-artwork), CC-BY) |

Cover logo by [Icons8](https://icons8.com). Official Rust artwork in `rust-logo.svg` is CC-BY (Rust Foundation); not used on the cover. This book is not affiliated with or endorsed by the Rust project.

Design: full-bleed dark ground, high-contrast title hierarchy, official logo on a neutral plate on the right. Edit layout in `cover-page.tex`; colors in `header.tex` (`cover*` definitions).

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
