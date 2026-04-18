# fastmail-cli — Reading & Conversations

Reference for `list emails`, `get`, `thread`, and triage (`mark-read`, `move`, `spam`). `thread --compact` is the biggest single token-economy win.

## Listing

```bash
fastmail-cli list emails [-m MAILBOX] [-l LIMIT] [--compact | --fields CSV]
fastmail-cli list mailboxes
```

Default: INBOX, 50 messages. Returns `{mailbox, emails: [...]}`.

## Single email

```bash
fastmail-cli get EMAIL_ID [--compact | --fields CSV]
```

Default returns the full JMAP email (headers, plain+HTML bodies, attachment metadata, `bodyValues`). `--compact` flattens to plain text (HTML stripped if no text part), summarizes attachments, drops internals. Typically 2× smaller on text, much more on HTML-heavy email.

## Full thread

```bash
fastmail-cli thread EMAIL_ID [--compact | --fields CSV]
```

Any email ID in the thread works. Returns messages chronologically.

**`thread --compact` is the single biggest token-economy win.** A 5-message HTML thread on this account: **79 KB → 5 KB (16×)**. Always use it for history enrichment or reply context unless you specifically need HTML or `bodyValues`.

## Typical workflow

```bash
# Triage inbox (compact = essentials only)
fastmail-cli list emails --compact

# Pull full conversation for context
fastmail-cli thread EMAIL_ID --compact

# Act
fastmail-cli mark-read EMAIL_ID
fastmail-cli reply EMAIL_ID --body "Got it."
fastmail-cli move EMAIL_ID --to "Archive"
```

## Mark read / spam / move

```bash
fastmail-cli mark-read EMAIL_ID [--unread]
fastmail-cli move EMAIL_ID --to "Work/Projects"
fastmail-cli spam EMAIL_ID [-y]     # -y skips confirmation
```

## Tips

- `search` → `thread --compact` → `reply`: the canonical agent flow.
- Single `thread` call beats N `get` calls for a conversation.
- IDs are stable; safe to store and reuse later.
