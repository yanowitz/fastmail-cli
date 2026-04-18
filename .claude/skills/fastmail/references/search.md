# fastmail-cli — Search

Filters are ANDed. Output is `{success, data: [emails...]}`. For `--compact` / `--fields` trade-offs, see `SKILL.md`.

## Filters

| Flag | Notes |
|---|---|
| `--text`/`-t` | Full-text across from/to/cc/bcc/subject/body — catch-all |
| `--from`, `--to`, `--cc`, `--bcc`, `--subject`, `--body` | Field-targeted; faster and more precise than `--text` |
| `--mailbox`/`-m` | Restrict to mailbox (default: all). `list mailboxes` to discover names |
| `--after`, `--before` | ISO 8601 (`2024-01-15`) |
| `--unread`, `--flagged`, `--has-attachment` | Booleans |
| `--min-size`, `--max-size` | Bytes |
| `--limit`/`-l` | Default 50 |

## Examples

```bash
fastmail-cli search --text "invoice"
fastmail-cli search --from boss@company.com --unread --compact
fastmail-cli search --subject "deployment" --mailbox "Work" --compact
fastmail-cli search --after 2024-01-01 --before 2024-02-01 -l 100 --compact
fastmail-cli search --has-attachment --min-size 1048576 --compact

# Bulk triage: minimal fields, larger limit
fastmail-cli search --from newsletter@ --before 2024-01-01 -l 200 \
  --fields id,from,subject,receivedAt,size

# Read context: narrow, then pull full thread
fastmail-cli search --subject "Project Kickoff" --from alice@ --compact
fastmail-cli thread EMAIL_ID --compact
```

Prefer field-targeted flags over `--text` when you know which field to hit — faster and more precise.
