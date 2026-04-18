---
name: fastmail
description: fastmail-cli reference — all commands, flags, token-economy flags (--compact/--fields), and common agent workflows
---

# fastmail-cli

Rust CLI for Fastmail via JMAP (email) and CardDAV (contacts). Output is JSON: `{"success": bool, "data": ..., "error"?, "message"?}`.

## Setup

```bash
fastmail-cli auth fmu1-YOUR-TOKEN
```

`~/.config/fastmail-cli/config.toml` or env (`FASTMAIL_API_TOKEN`, `FASTMAIL_USERNAME`, `FASTMAIL_APP_PASSWORD`). Contacts need `username` + `app_password`. Debug: `RUST_LOG=debug`.

---

## Token economy — READ FIRST for agent workflows

Default JMAP responses are verbose. Observed: `search -l 50` ≈ 57 KB, `thread` on a 5-msg HTML chain ≈ 80 KB. A full-limit search burns ~14K tokens.

Four commands accept output-projection flags: `search`, `list emails`, `get`, `thread`. They also push `properties` down to JMAP, so bandwidth shrinks too.

- **`--compact`** — curated agent shape. Drops `mailboxIds`/`keywords`/always-null fields; adds derived `unread`/`flagged` bools; on `get`/`thread` flattens body to plain text (HTML stripped if no text part), summarizes attachments. **Default choice for agent workflows.**
- **`--fields id,subject,from,receivedAt`** — JMAP property passthrough. Use when you know the exact fields (bulk triage, purge). Can be 2–3× smaller than `--compact`. Valid names are the JMAP Email properties (camelCase); unknown names error.
- **Neither flag** — full JMAP output. Use for debugging or when you need raw `keywords`/`bodyValues`/`blobId`.

`--compact` and `--fields` are mutually exclusive. `id` is always included.

Measured shrinkage on this account:

| Call | Default | `--compact` | `--fields id,subject,from,receivedAt` |
|---|---|---|---|
| `search -l 5` | 5.7 KB | 3.7 KB | 1.3 KB |
| `thread` (5-msg HTML) | 79 KB | 5 KB (**16×**) | 1.2 KB (**66×**) |
| `get` (small text) | 3.0 KB | 1.7 KB | — |

---

## Command Reference

### List

```bash
fastmail-cli list emails [-m MAILBOX] [-l LIMIT] [--compact | --fields CSV]    # default INBOX/50
fastmail-cli list mailboxes
fastmail-cli list identities                                                    # sender aliases for --from
```

### Get & Thread

```bash
fastmail-cli get EMAIL_ID [--compact | --fields CSV]
fastmail-cli thread EMAIL_ID [--compact | --fields CSV]
```

### Search

```bash
fastmail-cli search [FILTERS] [--compact | --fields CSV] [-l LIMIT]
```

Filters: `--text/-t`, `--from`, `--to`, `--cc`, `--bcc`, `--subject`, `--body`, `--mailbox/-m`, `--before`, `--after` (ISO 8601), `--unread`, `--flagged`, `--has-attachment`, `--min-size`, `--max-size`. All ANDed. See `/fastmail/search`.

### Compose

```bash
fastmail-cli send --to ADDR --subject S --body B [--cc] [--bcc] [--from IDENT] [--draft]
fastmail-cli reply EMAIL_ID --body B [--all] [--cc] [--bcc] [--from] [--draft]
fastmail-cli forward EMAIL_ID --to ADDR [--body] [--cc] [--bcc] [--from] [--draft]
```

### Manage

```bash
fastmail-cli move EMAIL_ID --to MAILBOX
fastmail-cli mark-read EMAIL_ID [--unread]
fastmail-cli spam EMAIL_ID [-y]
```

### Attachments

```bash
fastmail-cli download EMAIL_ID [-o DIR] [-f raw|json] [--max-size 1M]
```

### Masked Email / Contacts / Misc

```bash
fastmail-cli masked list|create|enable|disable|delete ...
fastmail-cli contacts list|search QUERY
fastmail-cli completions bash|zsh|fish|powershell
fastmail-cli mcp
```

---

## Common Patterns

```bash
# Triage: find marketing to purge (minimal fields)
fastmail-cli search --from newsletter@ --before 2024-01-01 \
  --fields id,from,subject,receivedAt,size

# Read-and-reply: search, pull thread for context, reply
fastmail-cli search --from boss@co.com --unread --compact
fastmail-cli thread EMAIL_ID --compact
fastmail-cli reply EMAIL_ID --body "On it." --from work@me.com

# Enrich incoming mail with history from a sender
fastmail-cli search --from someone@ -l 20 --compact

# Save draft instead of sending
fastmail-cli send --to x@y.com --subject S --body B --draft
```

---

## Subcommand Skills

- `/fastmail/search` — filter combinations and token-economy workflows
- `/fastmail/compose` — send, reply, forward, drafts, identities
- `/fastmail/conversations` — threading, listing, reading (incl. `thread --compact`)
- `/fastmail/attachments` — download and extract
- `/fastmail/masked` — masked email management
- `/fastmail/contacts` — contact search
