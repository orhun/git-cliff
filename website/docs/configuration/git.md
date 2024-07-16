# `git`

This section contains the parsing and git related configuration options.

```toml
[git]
conventional_commits = true
filter_unconventional = true
split_commits = false
commit_parsers = [
    { message = "^feat", group = "Features"},
    { message = "^fix", group = "Bug Fixes"},
    { message = "^doc", group = "Documentation"},
    { message = "^perf", group = "Performance"},
    { message = "^refactor", group = "Refactor"},
    { message = "^style", group = "Styling"},
    { message = "^test", group = "Testing"},
]
protect_breaking_commits = false
filter_commits = false
tag_pattern = "v[0-9].*"

skip_tags = "v0.1.0-beta.1"
ignore_tags = ""
topo_order = false
sort_commits = "oldest"
link_parsers = [
    { pattern = "#(\\d+)", href = "https://github.com/orhun/git-cliff/issues/$1"},
    { pattern = "RFC(\\d+)", text = "ietf-rfc$1", href = "https://datatracker.ietf.org/doc/html/rfc$1"},
]
limit_commits = 42
```

### conventional_commits

If set to `true`, commits are parsed according to the [Conventional Commits specifications](https://www.conventionalcommits.org).

> The Conventional Commits specification is a lightweight convention on top of commit messages. It provides an easy set of rules for creating an explicit commit history; which makes it easier to write automated tools on top of. This convention dovetails with SemVer, by describing the features, fixes, and breaking changes made in commit messages.

> The commit message should be structured as follows:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

e.g. `feat(parser): add ability to parse arrays`

### filter_unconventional

If set to `true`, commits that are not conventional are excluded. This option can be used to generate changelogs with conventional and unconventional commits mixed together. For example:

```toml
conventional_commits = true
filter_unconventional = false
commit_parsers = [
  { message = ".*", group = "Other", default_scope = "other"},
]
```

With the configuration above, conventional commits are parsed as usual and unconventional commits will be also included in the changelog as "Other".

To completely exclude unconventional commits from the changelog:

```toml
# default behaviour
conventional_commits = true
filter_unconventional = true
```

To include any type of commit in the changelog without parsing:

```toml
conventional_commits = false
filter_unconventional = false
```

### split_commits

> This flag violates "conventional commits". It should remain off by default if conventional commits is to be respected.

If set to `true`, each line of a commit is processed individually, as if it were its own commit message. This may cause
a commit to appear multiple times in a changelog, once for each match.

```toml
conventional_commits = true
filter_unconventional = true
split_commits = true
commit_parsers = [
    { message = "^feat", group = "Features"},
]
```

With the configuration above, lines are parsed as conventional commits and unconventional lines are omitted.

If `filter_unconventional = false`, every line will be processed as an unconventional commit, resulting in each line of
a commit being treated as a changelog entry.

### commit_preprocessors

An array of commit preprocessors for manipulating the commit messages before parsing/grouping them. These regex-based preprocessors can be used for removing or selecting certain parts of the commit message/body to be used in the following processes.

Examples:

- `{ pattern = "foo", replace = "bar"}`
  - Replace text.
- `{ pattern = 'Merged PR #[0-9]: (.*)', replace = "$1"}`
  - Remove prefix.
- `{ pattern = "  +", replace = " "}`
  - Replace multiple spaces with a single space.
- `{ pattern = "\\(#([0-9]+)\\)", replace = "([#${1}](https://github.com/orhun/git-cliff/issues/${1}))"}`
  - Replace the issue number with the link.
- `{ pattern = "https://github.com/[^ ]/issues/([0-9]+)", replace = "[Issue #${1}]"}`
  - Replace the issue link with the number.
- `{ pattern = "Merge pull request #([0-9]+) from [^ ]+", replace = "PR # [${1}](https://github.com/orhun/git-cliff/pull/${1}):"}`
  - Hyperlink PR references from merge commits.
- `{ pattern = "https://github.com/orhun/git-cliff/commit/([a-f0-9]{7})[a-f0-9]*", replace = "commit # [${1}](${0})"}`
  - Hyperlink commit links, with short commit hash as description.
- `{ pattern = "([ \\n])(([a-f0-9]{7})[a-f0-9]*)", replace = "${1}commit # [${3}](https://github.com/orhun/git-cliff/commit/${2})"}`
  - Hyperlink bare commit hashes like "abcd1234" in commit logs, with short commit hash as description.

Custom OS commands can also be used for modifying the commit messages:

- `{ pattern = "foo", replace_command = "pandoc -t commonmark"}`

This is useful when you want to filter/encode messages using external commands. In the example above, [pandoc](https://pandoc.org/) is used to convert each commit message that matches the given `pattern` to the [CommonMark](https://commonmark.org/) format.

A more fun example would be reversing each commit message:

- `{ pattern = '.*', replace_command = 'rev | xargs echo "reversed: $@"' }`

`$COMMIT_SHA` environment variable is set during execution of the command so you can do fancier things like reading the commit itself:

- `{ pattern = '.*', replace_command = 'git show -s --format=%B $COMMIT_SHA' }`

### commit_parsers

An array of commit parsers for determining the commit groups by using regex.

Examples:

- `{ message = "^feat", group = "Features" }`
  - Group the commit as "Features" if the commit message (description) starts with "feat".
- `{ body = ".*security", group = "Security" }`
  - Group the commit as "Security" if the commit body contains "security".
    <!-- Conventional commits parser is out of sync with spec, parsing only separator ":", not ": "; see: -->
    <!-- https://github.com/conventional-commits/parser/issues/47 -->
- `{ footer = "^changelog: ?ignore", skip = true }`
  - Skip processing the commit if the commit footer contains "changelog: ignore".
- `{ message = '^fix\((.*)\)', group = 'Fix (${1})' }`
  - Use the matched scope value from the commit message in the group name.
- `{ message = ".*deprecated", body = ".*deprecated", group = "Deprecation" }`
  - Group the commit as "Deprecation" if the commit body and message contains "deprecated".
- `{ message = "^revert", skip = true }`
  - Skip processing the commit if the commit message (description) starts with "revert".
- `{ message = "^doc", group = "Documentation", default_scope = "other" },`
  - If the commit starts with "doc", group the commit as "Documentation" and set the default scope to "other". (e.g. `docs: xyz` will be processed as `docs(other): xyz`)
- `{ message = "(www)", scope = "Application" }`
  - If the commit contains "(www)", override the scope with "Application". Scoping order is: scope specification, conventional commit's scope and default scope.
- `{ sha = "f6f2472bdf0bbb5f9fcaf2d72c1fa9f98f772bb2", skip = true }`
  - Skip a specific commit by using its SHA1.
- `{ sha = "f6f2472bdf0bbb5f9fcaf2d72c1fa9f98f772bb2", group = "Stuff" }`
  - Set the group of the commit by using its SHA1.
- `{ field = "author.name", pattern = "John Doe", group = "John's stuff" }`
  - If the author's name attribute of the commit matches the pattern "John Doe" (as a regex), override the scope with "John' stuff". Supported commit attributes are:
    - `id`
    - `message`
    - `body`
    - `author.name`
    - `author.email`
    - `committer.email`
    - `committer.name`

### protect_breaking_commits

If set to `true`, any breaking changes will be protected against being skipped
due to any commit parser.

### filter_commits

If set to `true`, commits that are not matched by [`commit_parsers`](#commit_parsers) are filtered out.

### tag_pattern

A regular expression for matching the git tags.

This value can be also overridden with using the `--tag-pattern` argument.

### skip_tags

A regex for skip processing the matched tags.

### ignore_tags

A regex for ignore processing the matched tags.

This value can be also overridden with using the `--ignore-tags` argument.

While `skip_tags` drop commits from the changelog, `ignore_tags` include ignored commits into the next tag.

* Note that if a commit has multiple tags, any matched tag will result in all associated tags being ignored, including those not explicitly matched by the regex. This is because git-cliff processes tags at the commit level rather than individually.
For more details, you can view the discussion [here](https://github.com/orhun/git-cliff/discussions/707)

### topo_order

If set to `true`, tags are processed in topological order instead of chronological.

This can also be achieved by using the `--topo-order` command line flag.

### sort_commits

Sort the commits inside sections by specified order.

Possible values:

- `oldest`
- `newest`

This can also be achieved by specifying the `--sort` command line argument.

### link_parsers

An array of link parsers for extracting external references, and turning them into URLs, using regex.

Examples:

- `{ pattern = "#(\\d+)", href = "https://github.com/orhun/git-cliff/issues/$1"}`
  - Extract all GitHub issues and PRs and generate URLs linking to them. The link text will be the matching pattern.
- `{ pattern = "RFC(\\d+)", text = "ietf-rfc$1", href = "https://datatracker.ietf.org/doc/html/rfc$1"}`,
  - Extract mentions of IETF RFCs and generate URLs linking to them. It also rewrites the text as "ietf-rfc...".

These extracted links can be used in the [template](/docs/templating/context) with `commits.links` variable.

### limit_commits

`limit_commits` is an **optional** positive integer number that limits the number of included commits in the generated changelog.

`limit_commits` is not part of the default configuration.
