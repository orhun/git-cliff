---
sidebar_position: 2
---

# GitLab Integration 🦊

:::warning

This is still an experimental feature, please [report bugs](https://github.com/orhun/git-cliff/issues/new/choose).

:::

:::note

If you have built from source, enable the `gitlab` feature flag for the integration to work.

:::

For projects hosted on GitLab, you can use **git-cliff** to add the following to your changelog:

- GitLab usernames
- Contributors list (all contributors / first time)
- Pull request links (associated with the commits)

## Setting up the remote

As default, remote upstream URL is automatically retrieved from the Git repository.

If that doesn't work or if you want to set a custom remote, there are a couple of ways of doing it:

- Use the [remote option](/docs/configuration/remote) in the configuration file:

```toml
[remote.gitlab]
owner = "orhun"
repo = "git-cliff"
token = "***"
```

- Use the `--gitlab-repo` argument (takes values in `OWNER/REPO` format, e.g. "orhun/git-cliff")

- Use the `GITLAB_REPO` environment variable (same format as `--gitlab-repo`)

## Authentication

:::tip

[GitLab REST API](https://docs.gitlab.com/ee/api/rest/) is being used to retrieve data from GitLab and it has [rate limiting](https://docs.gitlab.com/ee/security/rate_limits.html) rules.

You can follow [this guide](https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html) for creating an access token.

:::

To set an access token, you can use the [configuration file](/docs/configuration/remote) (not recommended), `--gitlab-token` argument or `GITLAB_TOKEN` environment variable.

For example:

```bash
GITLAB_TOKEN="***" git cliff --gitlab-repo "orhun/git-cliff"
```

:::tip

You can use the `GITLAB_API_URL` environment variable want to override the API URL. This is useful if you are using your own GitLab instance.

When your project on your own GitLab has one or many subgroups (e.g my.gitlab.com/myGroup/mySubgroup/myProject) you don't need to URL encode the owner in the configuration. (i.e. simply use "myGroup/mySubgroup")

:::

## Templating

:::tip

See the [templating documentation](/docs/category/templating) for general information about how the template engine works.

:::

### Remote

You can use the following [context](/docs/templating/context) for adding the remote to the changelog:

```json
{
  "gitlab": {
    "owner": "orhun",
    "repo": "git-cliff"
  }
}
```

For example:

```jinja2
https://gitlab.com/{{ remote.gitlab.owner }}/{{ remote.gitlab.repo }}/-/tags/{{ version }}
```

:::tip

If you are using GitLab CI, you can use [`CI_PROJECT_URL`](https://docs.gitlab.com/ee/ci/variables/predefined_variables.html) environment variable instead:

```jinja2
{{ get_env(name="CI_PROJECT_URL") }}/-/tags/{{ version }}
```

:::

### Commit authors

For each commit, GitLab related values are added as a nested object (named `gitlab`) to the [template context](/docs/templating/context):

```json
{
  "id": "8edec7fd50f703811d55f14a3c5f0fd02b43d9e7",
  "message": "refactor(config): remove unnecessary newline from configs\n",
  "group": "🚜 Refactor",

  "...": "<strip>",

  "remote": {
    "username": "orhun",
    "pr_title": "some things have changed",
    "pr_number": 420,
    "pr_labels": ["rust"],
    "is_first_time": false
  }
}
```

This can be used in the template as follows:

```
{% for commit in commits %}
  * {{ commit.message | split(pat="\n") | first | trim }}\
    {% if commit.remote.username %} by @{{ commit.remote.username }}{%- endif %}\
    {% if commit.remote.pr_number %} in #{{ commit.remote.pr_number }}{%- endif %}
{%- endfor -%}
```

The will result in:

```md
- feat(commit): add merge_commit flag to the context by @orhun in #389
- feat(args): set `CHANGELOG.md` as default missing value for output option by @sh-cho in #354
```

### Contributors

For each release, following contributors data is added to the [template context](/docs/templating/context) as a nested object:

```json
{
  "version": "v1.4.0",
  "commits": [],
  "commit_id": "0af9eb24888d1a8c9b2887fbe5427985582a0f26",
  "timestamp": 0,
  "previous": null,
  "gitlab": {
    "contributors": [
      {
        "username": "orhun",
        "pr_title": "some things have changed",
        "pr_number": 420,
        "pr_labels": ["rust"],
        "is_first_time": true
      },
      {
        "username": "cliffjumper",
        "pr_title": "I love jumping",
        "pr_number": 999,
        "pr_labels": ["rust"],
        "is_first_time": true
      }
    ]
  }
}
```

This can be used in the template as follows:

```
{% for contributor in gitlab.contributors | filter(attribute="is_first_time", value=true) %}
  * @{{ contributor.username }} made their first contribution in #{{ contributor.pr_number }}
{%- endfor -%}
```

The will result in:

```md
- @orhun made their first contribution in #420
- @cliffjumper made their first contribution in #999
```
