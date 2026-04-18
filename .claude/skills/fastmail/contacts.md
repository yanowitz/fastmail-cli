---
name: fastmail/contacts
description: fastmail-cli contacts — CardDAV config, list and search contacts
---

# fastmail-cli — Contacts

CardDAV. Needs separate credentials from the JMAP token:

```toml
# ~/.config/fastmail-cli/config.toml
[contacts]
username = "you@fastmail.com"
app_password = "..."
```

Or `FASTMAIL_USERNAME` + `FASTMAIL_APP_PASSWORD`. App passwords: Fastmail Settings → Privacy & Security → App Passwords.

## Commands

```bash
fastmail-cli contacts list
fastmail-cli contacts search QUERY    # matches name, email, or organization
```

## Patterns

```bash
# Find email before composing
fastmail-cli contacts search "Bob Smith" | jq '.data[0].emails[0].value'

# Verify unknown sender
fastmail-cli contacts search "bob@unknown.com"

# All contacts at a company
fastmail-cli contacts search "bigcorp.com"
```

## Notes

- `list` can be large — prefer `search` for targeted lookups.
- Read-only via CLI. Create/edit in Fastmail web or another CardDAV client.
