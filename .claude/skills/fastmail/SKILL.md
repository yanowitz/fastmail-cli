---
name: fastmail
description: Use when the user mentions Fastmail, fastmail-cli, or needs to send, search, read, reply, forward, or triage email from the CLI. Also covers Fastmail contacts (CardDAV), masked email addresses, and email attachments.
---

# fastmail-cli

Rust CLI for Fastmail via JMAP (email) and CardDAV (contacts). Output is JSON: `{"success": bool, "data": ..., "error"?, "message"?}`.

## Setup

```bash
fastmail-cli auth fmu1-YOUR-TOKEN
```

Email: `FASTMAIL_API_TOKEN` env or `[core].api_token` in `~/.config/fastmail-cli/config.toml`.
**Contacts need separate CardDAV credentials** — `FASTMAIL_USERNAME` + `FASTMAIL_APP_PASSWORD` env, or `[contacts]` section in the config. App passwords are at Fastmail Settings → Privacy & Security → App Passwords.
Debug: `RUST_LOG=debug`.

---

## Output projection (`--compact` / `--fields`)

Default JMAP responses are verbose — a default `search -l 50` is ~57 KB. `search`, `list emails`, `get`, and `thread` accept two mutually-exclusive flags that also push `properties` down to JMAP so bandwidth shrinks:

- **`--compact`** — curated shape. Drops `mailboxIds`/`keywords`/always-null fields; derives `unread`/`flagged` bools; on `get`/`thread` flattens body to plain text (HTML stripped when no text part exists) and summarizes attachments. Typically 2–3× smaller; `thread --compact` on a long HTML chain is often 10–20× smaller.
- **`--fields id,from,subject,receivedAt`** — JMAP passthrough. Validated against the JMAP Email property list (camelCase); unknown names error. Use for bulk/triage where a curated shape is still more than needed.
- **Neither** — full output. Use when you need raw `keywords` or `bodyValues`.

`id` is always included.

---

## Commands

```bash
fastmail-cli list emails [-m MAILBOX] [-l LIMIT] [--compact | --fields CSV]    # default INBOX/50
fastmail-cli list mailboxes
fastmail-cli list identities                                                    # aliases for --from

fastmail-cli get EMAIL_ID [--compact | --fields CSV]
fastmail-cli thread EMAIL_ID [--compact | --fields CSV]

fastmail-cli search [FILTERS] [--compact | --fields CSV] [-l LIMIT]

fastmail-cli send --to ADDR --subject S --body B [--cc] [--bcc] [--from IDENT] [--draft]
fastmail-cli reply EMAIL_ID --body B [--all] [--cc] [--bcc] [--from] [--draft]
fastmail-cli forward EMAIL_ID --to ADDR [--body] [--cc] [--bcc] [--from] [--draft]

fastmail-cli move EMAIL_ID --to MAILBOX
fastmail-cli mark-read EMAIL_ID [--unread]
fastmail-cli spam EMAIL_ID [-y]

fastmail-cli download EMAIL_ID [-o DIR] [-f raw|json] [--max-size 1M]

fastmail-cli masked list|create|enable|disable|delete ...
fastmail-cli contacts list|search QUERY
```

**Search filters** (all ANDed): `--text/-t`, `--from`, `--to`, `--cc`, `--bcc`, `--subject`, `--body`, `--mailbox/-m`, `--before`, `--after` (ISO 8601), `--unread`, `--flagged`, `--has-attachment`, `--min-size`, `--max-size`.

---

## Piping between commands

`jq` is only needed to feed a value into the next shell command. For inspection, just read the JSON. When piping, pair `--fields id` with `jq -r`:

```bash
fastmail-cli reply \
  $(fastmail-cli search --from boss@ --unread --fields id | jq -r '.data[0].id') \
  --body "On it."

for id in $(fastmail-cli search --from newsletter@ --before 2024-01-01 --fields id | jq -r '.data[].id'); do
  fastmail-cli move "$id" --to Archive
done
```

---

## References

Read these when the task needs more than the above:

- [`references/search.md`](references/search.md) — filter combinations, projection trade-offs
- [`references/conversations.md`](references/conversations.md) — list/get/thread patterns
- [`references/compose.md`](references/compose.md) — send, reply, forward, drafts, identities
- [`references/attachments.md`](references/attachments.md) — download and extract
- [`references/masked.md`](references/masked.md) — masked email CRUD
- [`references/contacts.md`](references/contacts.md) — CardDAV contacts
