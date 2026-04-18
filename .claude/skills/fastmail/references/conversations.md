# fastmail-cli — Reading & Conversations

Commands: `list emails`, `get`, `thread`, plus triage (`mark-read`, `move`, `spam`).

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

Default returns the full JMAP email (headers, plain+HTML bodies, attachment metadata, `bodyValues`). `--compact` flattens body to plain text and summarizes attachments.

## Full thread

```bash
fastmail-cli thread EMAIL_ID [--compact | --fields CSV]
```

Any email ID in the thread works. Returns messages chronologically. Prefer `--compact` unless you specifically need HTML or `bodyValues` — one `thread` call replaces N `get` calls.

## Typical flow

```bash
fastmail-cli list emails --compact           # triage
fastmail-cli thread EMAIL_ID --compact       # pull context
fastmail-cli mark-read EMAIL_ID
fastmail-cli reply EMAIL_ID --body "Got it."
fastmail-cli move EMAIL_ID --to "Archive"
```

## Mark read / move / spam

```bash
fastmail-cli mark-read EMAIL_ID [--unread]
fastmail-cli move EMAIL_ID --to "Work/Projects"
fastmail-cli spam EMAIL_ID [-y]     # -y skips confirmation
```
