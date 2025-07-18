---
sidebar_position: 6
---

# Multiple repositories

To generate a changelog for multiple git repositories:

```bash
git cliff --repository path1 path2
```

Note that the changelog will be generated using the merged history of the given repositories.

:::tip

You can use the `{{ repository }}` variable in the template to display which release belongs to which repository.

See [context](/docs/templating/context) for more information.

:::
