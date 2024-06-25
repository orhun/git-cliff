---
sidebar_position: 1
---

# Context

Context is the model that holds the required data for a template rendering. The [JSON](https://en.wikipedia.org/wiki/JSON) format is used in the following examples for the representation of a context.

## Conventional Commits

> conventional_commits = **true**

For a [conventional commit](/docs/configuration/git#conventional_commits) like this,

```
<type>[scope]: <description>

[body]

[footer(s)]
```

following context is generated to use for templating:

```json
{
  "version": "v0.1.0-rc.21",
  "message": "The annotated tag message for the release"
  "commits": [
    {
      "id": "e795460c9bb7275294d1fa53a9d73258fb51eb10",
      "group": "<type> (overridden by commit_parsers)",
      "scope": "[scope]",
      "message": "<description>",
      "body": "[body]",
      "footers": [
        {
          "token": "<name of the footer, such as 'Signed-off-by'>",
          "separator": "<the separator between the token and value, such as ':'>",
          "value": "<the value following the separator",
          "breaking": false
        }
      ],
      "breaking_description": "<description>",
      "breaking": false,
      "conventional": true,
      "merge_commit": false,
      "links": [
        { "text": "(set by link_parsers)", "href": "(set by link_parsers)" }
      ],
      "author": {
        "name": "User Name",
        "email": "user.email@example.com",
        "timestamp": 1660330071
      },
      "committer": {
        "name": "User Name",
        "email": "user.email@example.com",
        "timestamp": 1660330071
      }
    }
  ],
  "commit_id": "a440c6eb26404be4877b7e3ad592bfaa5d4eb210 (release commit)",
  "timestamp": 1625169301,
  "repository": "/path/to/repository",
  "previous": {
    "version": "previous release"
  }
}
```

:::info

See the [GitHub integration](/docs/integration/github) for the additional values you can use in the template.

:::

### Footers

A conventional commit's body may end with any number of structured key-value pairs known as [footers](https://www.conventionalcommits.org/en/v1.0.0/#specification). These consist of a string token naming the footer, a separator (which is either `: ` or ` #`), and a value, similar to [the git trailers convention](https://git-scm.com/docs/git-interpret-trailers).

For example:

- `Signed-off-by: User Name <user.email@example.com>`
- `Reviewed-by: User Name <user.email@example.com>`
- `Fixes #1234`
- `BREAKING CHANGE: breaking change description`

When a conventional commit contains footers, the footers are passed to the template in a `footers` array in the commit object. Each footer is represented by an object with the following fields:

- `token`, the name of the footer (preceding the separator character)
- `separator`, the footer's separator string (either `: ` or ` #`)
- `value`, the value following the separator character
- `breaking`, which is `true` if this is a `BREAKING CHANGE:` footer, and `false` otherwise

### Breaking Changes

`breaking` flag is set to `true` when the commit has an exclamation mark after the commit type and scope, e.g.:

```
feat(scope)!: this is a breaking change
```

Or when the `BREAKING CHANGE:` footer is defined:

```
feat: add xyz

BREAKING CHANGE: this is a breaking change
```

`breaking_description` is set to the explanation of the breaking change. This description is expected to be present in the `BREAKING CHANGE` footer. However, if it's not provided, the `message` is expected to describe the breaking change.

If the `BREAKING CHANGE:` footer is present, the footer will also be included in
`commit.footers`.

Breaking changes will be skipped if [`protect_breaking_commits`](/docs/configuration/git#protect_breaking_commits) is set to `true`, even when matched by a skipping [commit_parser](/docs/configuration/git#commit_parsers).

### Committer vs Author

From [Git docs](https://git-scm.com/book/en/v2/Git-Basics-Viewing-the-Commit-History):

> You may be wondering what the difference is between author and committer. The author is the person who originally wrote the work, whereas the committer is the person who last applied the work. So, if you send in a patch to a project and one of the core members applies the patch, both of you get credit — you as the author, and the core member as the committer.

## Non-Conventional Commits

> conventional_commits = **false**

If [`conventional_commits`](/docs/configuration/git#conventional_commits) is set to `false`, then some of the fields are omitted from the context or squashed into the `message` field:

```json
{
  "version": "v0.1.0-rc.21",
  "message": "The annotated tag message for the release"
  "commits": [
    {
      "id": "e795460c9bb7275294d1fa53a9d73258fb51eb10",
      "group": "(overridden by commit_parsers)",
      "scope": "(overridden by commit_parsers)",
      "message": "(full commit message including description, footers, etc.)",
      "conventional": false,
      "merge_commit": false,
      "links": [
        { "text": "(set by link_parsers)", "href": "(set by link_parsers)" }
      ],
      "author": {
        "name": "User Name",
        "email": "user.email@example.com",
        "timestamp": 1660330071
      },
      "committer": {
        "name": "User Name",
        "email": "user.email@example.com",
        "timestamp": 1660330071
      }
    }
  ],
  "commit_id": "a440c6eb26404be4877b7e3ad592bfaa5d4eb210 (release commit)",
  "timestamp": 1625169301,
  "repository": "/path/to/repository",
  "previous": {
    "version": "previous release"
  }
}
```

:::info

See the [GitHub integration](/docs/integration/github) for the additional values you can use in the template.

:::
