# fastmail-cli — Attachments

```bash
fastmail-cli download EMAIL_ID [-o DIR] [-f raw|json] [--max-size 1M]
```

- `-f raw` (default): save files to disk.
- `-f json`: extract text via kreuzberg (PDF, DOCX, XLSX, OCR on images, 56+ formats). Returns `{content, language}` per attachment.
- `--max-size`: resize images larger than this (e.g. `800K`, `1M`).

Preview attachments without downloading via `get EMAIL_ID --compact` — returns names, sizes, and content types.

## Patterns

```bash
fastmail-cli download abc123 -o ~/Downloads/invoices
fastmail-cli download abc123 -f json | jq '.data[].content'

# Find then download
fastmail-cli search --has-attachment --from invoices@vendor.com --compact
fastmail-cli download EMAIL_ID -o ~/Documents
```
