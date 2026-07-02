# fastmail-cli — Search

Filters are ANDed. Output is `{success, data: [emails...]}`. For the `--compact` vs `--fields` distinction, see `SKILL.md`.

**Output size.** A default `search -l 50` is ~57 KB raw. `--compact` is typically 2–3× smaller; `thread --compact` on a long HTML chain is often 10–20× smaller. `--fields` beats `--compact` by another 2–3× for bulk triage where preview text isn't needed.

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
| `--offset` | Skip N matches before returning; pair with `--limit` to page. Default 0. JMAP offsets are position-based, so results can shift if the mailbox changes between calls |

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

# Bulk action: loop ids into another command
for id in $(fastmail-cli search --from newsletter@ --before 2024-01-01 --fields id | jq -r '.data[].id'); do
  fastmail-cli move "$id" --to Archive
done

# Page through a large result set
fastmail-cli search --mailbox Archive -l 100 --offset 100 --fields id,subject

# Read context: narrow, then pull full thread
fastmail-cli search --subject "Project Kickoff" --from alice@ --compact
fastmail-cli thread EMAIL_ID --compact
```

Prefer field-targeted flags over `--text` when you know which field to hit — faster and more precise.
