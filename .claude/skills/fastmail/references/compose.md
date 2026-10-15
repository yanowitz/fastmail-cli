# fastmail — Compose

Reference for `send`, `reply`, `forward`, drafts, and sender identities.

## Identities

`--from` on send/reply/forward takes an identity email. Must match one returned by `list identities` — arbitrary addresses won't work.

```bash
fastmail list identities
```

## Send / Reply / Forward

```bash
fastmail send --to ADDR --subject S --body B [--cc] [--bcc] [--from] [--reply-to MSG_ID] [--draft]
fastmail reply EMAIL_ID --body B [--all] [--cc] [--bcc] [--from] [--draft]
fastmail forward EMAIL_ID --to ADDR [--body] [--cc] [--bcc] [--from] [--draft]
```

All three also accept: `--html-body <HTML>` / `--html-file <PATH>` and `-a, --attachment <FILE>` (repeatable).

- Addresses are comma-separated.
- `--draft` saves to Drafts instead of sending.
- `reply --all` is reply-all. On `reply`, threading headers (`In-Reply-To`, `References`) are set automatically.
- `forward --body` prepends a note above the forwarded content.
- `--body` is plain text. For HTML, pass `--html-body "<p>…</p>"` or `--html-file message.html` — no need to stage a draft in the web UI.
- `send --reply-to <MSG_ID>` sets `In-Reply-To` and `References`. It accepts a bare or bracketed Message-ID, inherits the parent’s References when found in this account, and falls back to the supplied ID if the parent is absent or lookup fails.

## Patterns

```bash
# Reply from a specific alias
fastmail reply abc123 --body "On it." --from work@mydomain.com

# Reply-all + BCC archive
fastmail reply abc123 --body "Thanks all." --all --bcc archive@mydomain.com

# Draft for review
fastmail send --to x@y.com --subject S --body "..." --draft

# Forward with note
fastmail forward abc123 --to manager@co.com --body "FYI"

# HTML body + attachments
fastmail send --to x@y.com --subject "Report" \
  --html-file report.html -a report.pdf -a chart.png

# Threaded follow-up via a fresh send
fastmail send --to x@y.com --subject "Re: S" --body "Bump." --reply-to "$MSG_ID"

# Quick reply to first hit
fastmail reply $(fastmail search --from boss@co.com --unread --fields id | jq -r '.data[0].id') \
  --body "Done."
```
