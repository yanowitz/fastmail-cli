---
name: fastmail/compose
description: fastmail-cli send / reply / forward / draft — flags, identities, compose patterns
---

# fastmail-cli — Compose

## Identities

`--from` on send/reply/forward takes an identity email. Must match one returned by `list identities` — arbitrary addresses won't work.

```bash
fastmail-cli list identities
```

## Send / Reply / Forward

```bash
fastmail-cli send --to ADDR --subject S --body B [--cc] [--bcc] [--from] [--draft]
fastmail-cli reply EMAIL_ID --body B [--all] [--cc] [--bcc] [--from] [--draft]
fastmail-cli forward EMAIL_ID --to ADDR [--body] [--cc] [--bcc] [--from] [--draft]
```

- Addresses are comma-separated.
- `--draft` saves to Drafts instead of sending.
- `reply --all` is reply-all. Threading headers (`In-Reply-To`, `References`) are set automatically.
- `forward --body` prepends a note above the forwarded content.
- Body is plain text. For HTML, compose in Fastmail web and use `--draft` to stage.

## Patterns

```bash
# Reply from a specific alias
fastmail-cli reply abc123 --body "On it." --from work@mydomain.com

# Reply-all + BCC archive
fastmail-cli reply abc123 --body "Thanks all." --all --bcc archive@mydomain.com

# Draft for review
fastmail-cli send --to x@y.com --subject S --body "..." --draft

# Forward with note
fastmail-cli forward abc123 --to manager@co.com --body "FYI"

# Quick reply to first hit
fastmail-cli reply $(fastmail-cli search --from boss@co.com --unread --fields id | jq -r '.data[0].id') \
  --body "Done."
```
