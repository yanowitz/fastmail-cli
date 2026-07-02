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

fastmail-cli contacts create --name NAME [--email A,B] [--phone P] [--organization ORG] [--title T] [--notes N]
fastmail-cli contacts update CONTACT_ID [--name] [--email] [--phone] [--organization] [--title] [--notes]
fastmail-cli contacts delete CONTACT_ID [-y]
```

- `create` requires `--name`; `--email`/`--phone` take comma-separated lists.
- `update` **replaces** the existing `--email`/`--phone` lists (not additive); omitted fields are left unchanged. `CONTACT_ID` comes from `list`/`search` output.
- `delete -y` skips the confirmation prompt.

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
- Full CRUD is supported via CardDAV (`create`/`update`/`delete`), so no need to switch to the Fastmail web UI for edits.
