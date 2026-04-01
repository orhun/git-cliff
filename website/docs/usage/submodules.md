---
sidebar_position: 7
---

# Submodules

If [`recurse_submodules`](/docs/configuration/git#recurse_submodules) is set to `true`, you can include submodule commits into your changelog.

For example:

```toml
[git]
recurse_submodules = true

[changelog]
body = """
{% for submodule_path, commits in submodule_commits %}
    ### {{ submodule_path | upper_first }}
    {% for group, commits in commits | group_by(attribute="group") %}
        #### {{ group | upper_first }}
        {% for commit in commits %}
            - {{ commit.message | upper_first }}\
        {% endfor %}
    {% endfor %}
{% endfor %}\n
"""
```

:::info

If a release does not contain any submodule updates, `submodule_commits` is just an empty map.

:::

:::warning

Nested submodules are not yet supported.

:::
