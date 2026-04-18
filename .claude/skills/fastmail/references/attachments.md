# fastmail-cli — Attachments

Reference for `download` — save attachment files or extract text content.

```bash
fastmail-cli download EMAIL_ID [-o DIR] [-f raw|json] [--max-size 1M]
```

- `-f raw` (default): save files to disk.
- `-f json`: extract text via kreuzberg (PDF, DOCX, XLSX, OCR images, 56+ formats). Returns `{content, language}` per attachment — useful when an agent needs the content without saving files.
- `--max-size`: resize images larger than this (e.g. `800K`, `1M`).

## Patterns

```bash
fastmail-cli download abc123 -o ~/Downloads/invoices
fastmail-cli download abc123 -f json | jq '.data[].content'

# Find then download
fastmail-cli search --has-attachment --from invoices@vendor.com --compact
fastmail-cli download EMAIL_ID -o ~/Documents
```

## Tips

- `get EMAIL_ID --compact` returns attachment names/sizes/types without downloading — preview before pulling bytes.
- `-f json` for programmatic reading; `raw` for files on disk.
- `--max-size` matters most on many-image emails where thumbnails are enough.
