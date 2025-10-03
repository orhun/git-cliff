---
sidebar_position: 5
---

# Azure DevOps Integration 🪟

:::note

If you have built from source, enable the `azure_devops` feature flag for the integration to work.

:::

For projects hosted on Azure DevOps, you can use **git-cliff** to add the following to your changelog:

- Azure DevOps usernames
- Contributors list (all contributors / first time)
- Pull request links (associated with the commits)

## Setting up the remote

As default, remote upstream URL is automatically retrieved from the Git repository.

If that doesn't work or if you want to set a custom remote, there are a couple of ways of doing it:

- Use the [remote option](/docs/configuration/remote) in the configuration file:

```toml
[remote.azure_devops]
owner = "organization/project"
repo = "repository-name"
token = "***"
```

:::warning

Note that for Azure DevOps, the `owner` field should be in the format `organization/project` (e.g., "myorg/myproject"), not just the organization name.

:::

- Use the `--azure-devops-repo` argument (takes values in `OWNER/REPO` format where OWNER is "organization/project")

- Use the `AZURE_DEVOPS_REPO` environment variable (same format as `--azure-devops-repo`)

## Authentication

[Azure DevOps REST API](https://learn.microsoft.com/en-us/rest/api/azure/devops/) is being used to retrieve data from Azure DevOps and it has [rate limiting](https://learn.microsoft.com/en-us/azure/devops/integrate/concepts/rate-limits) rules.

:::tip

You can follow [this guide](https://learn.microsoft.com/en-us/azure/devops/organizations/accounts/use-personal-access-tokens-to-authenticate) for creating a personal access token (PAT).

The token needs **Code (Read)** permissions to access repository information.

:::

To set an access token, you can use the [configuration file](/docs/configuration/remote) (not recommended), `--azure-devops-token` argument or `AZURE_DEVOPS_TOKEN` environment variable.

For example:

```bash
AZURE_DEVOPS_TOKEN="***" git cliff --azure-devops-repo "myorg/myproject/myrepo"
```

:::tip

You can use the `AZURE_DEVOPS_API_URL` environment variable if you want to override the API URL. This is useful if you are using Azure DevOps Server (on-premises).

:::

:::info

If you are getting invalid peer certificate errors, you can use the `--use-native-tls` flag to load certificates from the platform's native certificate store.

It is also possible to configure this in the configuration file, see the [remote configuration](/docs/configuration/remote#native_tls) for more information.

:::

## Templating

:::tip

See the [templating documentation](/docs/category/templating) for general information about how the template engine works.

:::

### Remote

You can use the following [context](/docs/templating/context) for adding the remote to the changelog:

```json
{
  "azure_devops": {
    "owner": "myorg/myproject",
    "repo": "myrepo"
  }
}
```

For example:

```jinja2
https://dev.azure.com/{{ remote.azure_devops.owner }}/_git/{{ remote.azure_devops.repo }}/branchCompare?baseVersion=GT{{ previous.version }}&targetVersion=GT{{ version }}
```

### Commit authors

For each commit, Azure DevOps related values are added as a nested object (named `remote`) to the [template context](/docs/templating/context):

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
    "pr_labels": ["enhancement"],
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

This will result in:

```md
- feat(commit): add merge_commit flag to the context by @orhun in #389
- feat(args): set `CHANGELOG.md` as default missing value for output option by @sh-cho in #354
```

### Contributors

For each release, following contributors data is added to the [template context](/docs/templating/context) as a nested object:

:::warning

Note that for contributors, the template variable is `azureDevops` (camelCase), not `azure_devops`. This is different from the remote configuration which uses `remote.azure_devops`.

:::

```json
{
  "version": "v1.4.0",
  "commits": [],
  "commit_id": "0af9eb24888d1a8c9b2887fbe5427985582a0f26",
  "timestamp": 0,
  "previous": null,
  "azureDevops": {
    "contributors": [
      {
        "username": "orhun",
        "pr_title": "some things have changed",
        "pr_number": 420,
        "pr_labels": ["enhancement"],
        "is_first_time": true
      },
      {
        "username": "cliffjumper",
        "pr_title": "I love jumping",
        "pr_number": 999,
        "pr_labels": ["feature"],
        "is_first_time": true
      }
    ]
  }
}
```

This can be used in the template as follows:

```
{% for contributor in azureDevops.contributors | filter(attribute="is_first_time", value=true) %}
  * @{{ contributor.username }} made their first contribution in #{{ contributor.pr_number }}
{%- endfor -%}
```

This will result in:

```md
- @orhun made their first contribution in #420
- @cliffjumper made their first contribution in #999
```

## Azure DevOps Changelog

If you would like to create a changelog that integrates with Azure DevOps, you can use the [`azure-devops-keepachangelog.toml`](https://github.com/orhun/git-cliff/tree/main/examples/azure-devops-keepachangelog.toml) example which follows the [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format with Azure DevOps integration.

This will generate a changelog such as:

```md
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-01-15

### Added
- feat: add new feature by @orhun in #123
- feat: support Azure DevOps integration by @contributor in #456

### Fixed
- fix: resolve bug in parser by @orhun in #789
- test: add test coverage for edge cases by @someone in #101

### Changed
- refactor: improve performance by @cliffjumper in #202

[1.0.0]: https://dev.azure.com/myorg/myproject/_git/myrepo/branchCompare?baseVersion=GTv0.9.0&targetVersion=GTv1.0.0

<!-- generated by git-cliff -->
```
