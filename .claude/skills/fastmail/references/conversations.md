# fastmail — Reading & Conversations

Commands: `list emails`, `get`, `thread`, plus triage (`mark-read`, `move`, `spam`).

## Listing

```bash
fastmail list emails [-m MAILBOX] [-l LIMIT] [--compact | --fields CSV]
fastmail list mailboxes
```

Default: INBOX, 50 messages. Returns `{mailbox, emails: [...]}`.

## Single email

```bash
fastmail get EMAIL_ID [--compact | --fields CSV | --headers]
```

Default returns the full JMAP email (headers, plain+HTML bodies, attachment metadata, `bodyValues`). `--compact` flattens body to plain text and summarizes attachments.

`fastmail get EMAIL_ID --headers` returns only `{id, headers: [{name, value}, ...]}` inside the normal JSON envelope, preserving header order and duplicate names for checking DKIM-Signature, Authentication-Results, ARC-* and Received. It conflicts with `--compact` and `--fields`; `--fields id,headers` is also supported.

## Full thread

```bash
fastmail thread EMAIL_ID [--compact | --fields CSV]
```

Any email ID in the thread works. Returns messages chronologically. Prefer `--compact` unless you specifically need HTML or `bodyValues` — one `thread` call replaces N `get` calls.

## Typical flow

```bash
fastmail list emails --compact           # triage
fastmail thread EMAIL_ID --compact       # pull context
fastmail mark-read EMAIL_ID
fastmail reply EMAIL_ID --body "Got it."
fastmail move EMAIL_ID --to "Archive"
```

## Mark read / move / spam

```bash
fastmail mark-read EMAIL_ID [--unread]
fastmail move EMAIL_ID --to "Work/Projects"
fastmail spam EMAIL_ID [-y]     # -y skips confirmation
```
