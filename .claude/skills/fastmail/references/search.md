# fastmail-cli — Search

Reference for `fastmail-cli search`: filters, projection flags, triage/enrichment patterns.

Filters are ANDed. Output is `{success, data: [emails...]}`.

## Filters

| Flag | Notes |
|---|---|
| `--text`/`-t` | Full-text across from/to/cc/bcc/subject/body — catch-all |
| `--from`, `--to`, `--cc`, `--bcc`, `--subject`, `--body` | Field-targeted; faster and more precise than `--text` |
| `--mailbox`/`-m` | Restrict to mailbox (default: all). Use `list mailboxes` first if unsure |
| `--after`, `--before` | ISO 8601 (`2024-01-15`) |
| `--unread`, `--flagged`, `--has-attachment` | Booleans |
| `--min-size`, `--max-size` | Bytes |
| `--limit`/`-l` | Default 50 |

## Token economy — pick a projection

Default search at `-l 50` returns ~57 KB (~14K tokens). Pick one of:

- **`--compact`** — curated summary. Drops JMAP internals, adds `unread`/`flagged` bools. Default for read-and-reply workflows. ~3× smaller.
- **`--fields id,from,subject,receivedAt[,size]`** — JMAP passthrough. Default for bulk/purge workflows where preview isn't needed. Pushes `properties` to JMAP, so saves bandwidth too. Can be 2–3× smaller than `--compact`.
- **(neither)** — full output. Debugging or when you need raw `keywords`/`mailboxIds`.

Mutually exclusive. `--fields` only accepts JMAP property names (camelCase) — unknown names error with the valid list. `id` is always returned.

## Examples

```bash
fastmail-cli search --text "invoice"
fastmail-cli search --from boss@company.com --unread --compact
fastmail-cli search --subject "deployment" --mailbox "Work" --compact
fastmail-cli search --after 2024-01-01 --before 2024-02-01 -l 100 --compact
fastmail-cli search --has-attachment --min-size 1048576 --compact

# Purge workflow: minimal fields, larger limit
fastmail-cli search --from newsletter@ --before 2024-01-01 -l 200 \
  --fields id,from,subject,receivedAt,size

# Read workflow: compact, then pull full thread
fastmail-cli search --subject "Project Kickoff" --from alice@ --compact
fastmail-cli thread EMAIL_ID --compact
```

## Tips

- Field-targeted flags beat `--text` on both speed and relevance.
- IDs from search output feed `get`, `reply`, `forward`, `move`, `download`, `thread` directly.
- For history enrichment (`--from X -l 20`), always use `--compact` — 3× savings with no loss of useful info.
