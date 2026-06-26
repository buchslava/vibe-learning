# Kindle / KDP PDF build

Produces a single **6×9 in** PDF tuned for Amazon Kindle and KDP print replica. The PDF title page shows **Draft edition** (see `metadata.yaml`).

- Preface + chapters 1–20 (all six parts)
- Appendices: Playground Guide, Java/Python/Rust map, AI Prompt Index
- Table of contents with chapter numbers (auto-numbered; manual `Chapter N:` prefixes stripped from titles)
- Syntax-highlighted Rust code (no line numbers — wrapped lines stay unambiguous)
- **Hard-wrapped** code lines (~48 cols) so nothing overflows the 6×9 page
- Pipe tables sanitized for LaTeX (`&` in cells, empty header → **Aspect** for equal column widths)
- Mermaid diagrams replaced with a short print-edition note

## Requirements

- [Pandoc](https://pandoc.org/) 3.x
- pdfLaTeX (MacTeX, TeX Live, etc.)
- [Ghostscript](https://ghostscript.com/) (`gs`) — rewrites the PDF to **1.3** for in-browser Android readers

## Build

```bash
python3 kindle/build.py
```

Output: **`dist/Rust-Core-Kindle.pdf`**

**Android browser PDF readers** (Chrome, in-app viewers, etc.) are much pickier than native PDF apps. The build uses **pdfLaTeX + Type1 fonts** and a Ghostscript pass for compatibility. If preview still fails, tap **Download** and open the saved file in Google PDF Viewer or Adobe.

## Kindle upload tips

- **Send to Kindle** (email or app): upload the PDF directly; Kindle scales 6×9 reasonably on e-ink.
- **KDP**: use this PDF as *print replica* or convert to EPUB with Kindle Create if you want reflowable text.
- Internal repo links are flattened to plain text in the PDF; external URLs remain clickable.

## Regenerate after edits

Re-run `build.py` whenever chapter markdown changes. The merged source is written to `kindle/build/Rust-Core-Kindle.md` for debugging.
