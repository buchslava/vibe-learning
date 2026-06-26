# Kindle / KDP PDF build

Produces a single **6×9 in** PDF tuned for Amazon Kindle and KDP print replica.

**Build flow:** `cover-only.pdf` (simple one-page JPEG) + Pandoc body PDF → merged with **pypdf**. Page 1 = cover, page 2 = TOC. No `eso-pic`, no `\newgeometry`, no blank leading page.

- Preface + chapters 1–20 (all six parts)
- Appendices: Playground Guide, Java/Python/Rust map, AI Prompt Index
- Table of contents with chapter numbers (auto-numbered; manual `Chapter N:` prefixes stripped from titles)
- Syntax-highlighted Rust code (no line numbers — wrapped lines stay unambiguous)
- **Hard-wrapped** code lines (~48 cols) so nothing overflows the 6×9 page
- Pipe tables sanitized for LaTeX (`&` in cells, empty header → **Aspect** for equal column widths)
- Mermaid diagrams replaced with a short print-edition note

## Cover

| File | Role |
|------|------|
| `cover-art.svg` | Cover artwork — edit this, then rebuild |
| `cover-page.jpg` | Raster cover (from SVG + inlined logo) |
| `cover-only.tex` | Standalone 6×9 sheet → `build/cover-only.pdf` |
| `rust-logo-icons8.png` | Rust logo ([Icons8](https://icons8.com)) |

**Python:** `pip install Pillow cairosvg pypdf`  
**Optional:** `rsvg-convert` (librsvg), Ghostscript (`gs`) for PDF 1.4 sanitize pass

Cover logo by [Icons8](https://icons8.com). This book is not affiliated with or endorsed by the Rust project.

## Requirements

- [Pandoc](https://pandoc.org/) 3.x
- XeLaTeX (MacTeX, TeX Live, etc.)

## Build

```bash
python3 kindle/build.py
```

Output: **`dist/Rust-Core-Kindle.pdf`**

Intermediate files in `kindle/build/`: `Rust-Core-Kindle-body.pdf`, `cover-only.pdf`.

**GitHub preview:** The cover page is a plain image PDF and should preview better than TikZ/`eso-pic` overlays. If GitHub still fails on the full book (size/complexity), download the file — it is not corrupt.

## Kindle upload tips

- **Send to Kindle** (email or app): upload the PDF directly; Kindle scales 6×9 reasonably on e-ink.
- **KDP**: use this PDF as *print replica* or convert to EPUB with Kindle Create if you want reflowable text.
- Internal repo links are flattened to plain text in the PDF; external URLs remain clickable.

## Regenerate after edits

Re-run `build.py` whenever chapter markdown changes. The merged source is written to `kindle/build/Rust-Core-Kindle.md` for debugging.
