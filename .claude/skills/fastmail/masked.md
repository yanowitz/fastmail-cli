---
name: fastmail/masked
description: fastmail-cli masked — create, list, enable, disable, and delete masked email addresses
---

# fastmail-cli — Masked Email

Masked emails are Fastmail's disposable address feature — each masked address forwards to your real inbox and can be disabled or deleted independently.

## Commands

```bash
fastmail-cli masked list
fastmail-cli masked create [--domain URL] [--description STR] [--prefix STR]
fastmail-cli masked enable ID
fastmail-cli masked disable ID
fastmail-cli masked delete ID [-y]
```

## Create a Masked Email

```bash
# Basic (auto-generated address)
fastmail-cli masked create

# With context metadata
fastmail-cli masked create \
  --domain "https://example.com" \
  --description "Example site signup" \
  --prefix "example_shop"
```

- `--prefix`: custom address prefix, max 64 chars, `a-z`, `0-9`, `_` only
- `--domain`: the site it's for (metadata only, not enforced)
- `--description`: human-readable label

## Manage Existing Masked Addresses

```bash
# See all masked addresses with their IDs and status
fastmail-cli masked list

# Temporarily stop forwarding (keep address, bounce/drop inbound)
fastmail-cli masked disable MASKED_ID

# Re-enable
fastmail-cli masked enable MASKED_ID

# Permanently delete
fastmail-cli masked delete MASKED_ID
fastmail-cli masked delete MASKED_ID -y   # skip confirmation
```

## Typical Patterns

```bash
# Create a throwaway for a signup
fastmail-cli masked create --description "Newsletter signup" --prefix "news_acme"

# Getting spam? Disable immediately
fastmail-cli masked list  # find the ID
fastmail-cli masked disable abc-masked-id

# Clean up old ones
fastmail-cli masked list | jq '.data[] | select(.description | test("old")) | .id'
fastmail-cli masked delete OLD_ID -y
```
