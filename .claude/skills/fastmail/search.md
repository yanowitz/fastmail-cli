---
name: fastmail/search
description: fastmail-cli search — all filter flags, date ranges, and search workflow patterns
---

# fastmail-cli — Search

Search filters are ANDed together. All return JSON with a `data` array of email summaries.

## Syntax

```bash
fastmail-cli search [OPTIONS]
```

| Flag | Short | Description |
|------|-------|-------------|
| `--text` | `-t` | Full-text: from, to, cc, bcc, subject, body |
| `--from` | | Filter by From header |
| `--to` | | Filter by To header |
| `--cc` | | Filter by Cc header |
| `--bcc` | | Filter by Bcc header |
| `--subject` | | Filter by Subject |
| `--body` | | Filter by body text |
| `--mailbox` | `-m` | Restrict to mailbox (default: all) |
| `--after` | | On/after date (ISO 8601: `2024-01-15`) |
| `--before` | | Before date (ISO 8601: `2024-01-15`) |
| `--unread` | | Unread only |
| `--flagged` | | Flagged/starred only |
| `--has-attachment` | | Must have attachments |
| `--min-size` | | Minimum size in bytes |
| `--max-size` | | Maximum size in bytes |
| `--limit` | `-l` | Max results (default: 50) |

## Examples

```bash
# Simple full-text search
fastmail-cli search --text "invoice"

# From a specific sender, unread only
fastmail-cli search --from boss@company.com --unread

# Subject keyword in a specific folder
fastmail-cli search --subject "deployment" --mailbox "Work"

# Date range
fastmail-cli search --after 2024-01-01 --before 2024-02-01

# Emails with attachments over 1MB
fastmail-cli search --has-attachment --min-size 1048576

# Recent flagged emails
fastmail-cli search --flagged --after 2024-01-01 --limit 20

# Find a thread starter to then get full conversation
fastmail-cli search --subject "Project Kickoff" --from alice@example.com
# then: fastmail-cli thread EMAIL_ID
```

## Tips

- `--text` is the catch-all; use specific flags (`--from`, `--subject`) when you know what field to target — it's faster and more precise.
- Chain with `thread EMAIL_ID` to get full conversation context after finding an email.
- IDs from search output can be passed directly to `get`, `reply`, `forward`, `move`, `download`.
