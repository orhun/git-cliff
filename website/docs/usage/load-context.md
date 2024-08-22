---
sidebar_position: 11
---

# Load context

To load a context from a file and output the generated changelog:

```bash
# process context loaded from stdin
git cliff --from-context -

# process context loaded from a file
git cliff --from-context context.json
```

This is useful if you want to [print context](/docs/usage/print-context), modify it with an external tool and then "pipe" it back into git cliff. Free-form metadata can be added to release objects and commit objects in the context using the `extra` field.
