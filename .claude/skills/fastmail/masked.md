---
name: fastmail/masked
description: fastmail-cli masked — create / list / enable / disable / delete masked email addresses
---

# fastmail-cli — Masked Email

Fastmail's disposable addresses. Each one forwards to your real inbox and can be turned off or deleted independently.

```bash
fastmail-cli masked list
fastmail-cli masked create [--domain URL] [--description STR] [--prefix STR]
fastmail-cli masked enable|disable|delete ID    # delete prompts unless -y
```

- `--prefix`: custom prefix, ≤64 chars, `[a-z0-9_]`.
- `--domain` / `--description`: metadata only; helps future-you identify the address.

## Patterns

```bash
# Throwaway for a signup
fastmail-cli masked create --description "Newsletter signup" --prefix "news_acme"

# Stop a leaking address
fastmail-cli masked list    # find the ID
fastmail-cli masked disable MASKED_ID

# Clean up by description match
fastmail-cli masked list | jq '.data[] | select(.description | test("old")) | .id'
fastmail-cli masked delete OLD_ID -y
```
