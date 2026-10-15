# fastmail — Masked Email

Fastmail's disposable addresses. Each one forwards to your real inbox and can be turned off or deleted independently.

```bash
fastmail masked list
fastmail masked create [--domain URL] [--description STR] [--prefix STR]
fastmail masked enable|disable|delete ID    # delete prompts unless -y
```

- `--prefix`: custom prefix, ≤64 chars, `[a-z0-9_]`.
- `--domain` / `--description`: metadata only; helps future-you identify the address.

## Patterns

```bash
# Throwaway for a signup
fastmail masked create --description "Newsletter signup" --prefix "news_acme"

# Stop a leaking address
fastmail masked list    # find the ID
fastmail masked disable MASKED_ID

# Clean up by description match
fastmail masked list | jq '.data[] | select(.description | test("old")) | .id'
fastmail masked delete OLD_ID -y
```
