---
sidebar_position: 13
---

# Submodules

If `recurse_submodules` is set to true in the `[git]` config section you can include submodule commits into your changelog:

```
{% for submodule_path, commits in submodule_commits %}
    ### {{ submodule_path | upper_first }}
    {% for group, commits in commits | group_by(attribute="group") %}
        #### {{ group | upper_first }}
        {% for commit in commits %}
            - {{ commit.message | upper_first }}\
        {% endfor %}
    {% endfor %}
{% endfor %}\n
``` 

If a release does not contain any submodule updates, `submodule_commits` is just an empty map.