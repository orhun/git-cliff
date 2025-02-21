---
sidebar_position: 11
---

# Load context

:::tip

This is useful if you want to [print context](/docs/usage/print-context), modify it with an external tool and then "pipe" it back into **git-cliff**.

:::

To load a context from a file and output the generated changelog:

```bash
# create a context
git cliff --context -o context.json

# process context and generate a changelog
git cliff --from-context context.json

# process context loaded from stdin
git cliff --from-context -
```

Free-form metadata can be added to release objects and commit objects in the context using the `extra` field:

```json
{
  "id": "5061081d6272b1da2146fab49d803c193db309d9",
  "message": "commit message",
  "extra": {
    "note": "this is some arbitrary data"
  }
}
```
