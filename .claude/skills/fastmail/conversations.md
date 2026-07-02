---
name: fastmail/conversations
description: fastmail-cli list, get, thread — reading emails, conversations, mark-read, and triage
---

# fastmail-cli — Conversations & Email Reading

## Listing Emails

```bash
fastmail-cli list emails [-m MAILBOX] [-l LIMIT]
```

- Default mailbox: `INBOX`, default limit: `50`
- Returns email summaries (id, subject, from, date, flags)

```bash
# List a different folder
fastmail-cli list emails --mailbox "Sent"
fastmail-cli list emails --mailbox "Archive" --limit 100

# See all folders first
fastmail-cli list mailboxes
```

## Reading a Single Email

```bash
fastmail-cli get EMAIL_ID
```

Returns full email: headers, body (plain + HTML), attachment metadata.

## Reading a Full Thread/Conversation

```bash
fastmail-cli thread EMAIL_ID
```

- Provide **any** email ID in the thread — returns all messages in chronological order.
- Ideal for understanding full context before replying.

## Typical Read Workflow

```bash
# 1. List inbox
fastmail-cli list emails

# 2. Get a specific email by ID
fastmail-cli get abc123

# 3. Get full thread for context
fastmail-cli thread abc123

# 4. Mark as read when done
fastmail-cli mark-read abc123

# 5. Reply or move
fastmail-cli reply abc123 --body "Got it, thanks."
fastmail-cli move abc123 --to "Archive"
```

## Mark Read / Unread

```bash
fastmail-cli mark-read EMAIL_ID          # mark as read
fastmail-cli mark-read EMAIL_ID --unread # mark as unread
```

## Triage

```bash
# Move to folder
fastmail-cli move EMAIL_ID --to "Work/Projects"

# Mark as spam (prompts confirmation)
fastmail-cli spam EMAIL_ID

# Skip confirmation
fastmail-cli spam EMAIL_ID -y
```

## Tips

- Use `search` to find emails, then `thread` to get full context — this is the most useful combo for agents.
- `thread` is cheaper than running multiple `get` calls for each message in a conversation.
- IDs are stable — safe to store and reference later.
