---
sidebar_position: 1
---

# Crates.io

**git-cliff** can be installed from [crates.io](https://crates.io/crates/git-cliff):

```bash
cargo install git-cliff
```

If you want to install the latest git version:

```bash
cargo install --git https://github.com/orhun/git-cliff
```

The minimum supported Rust version is `1.85.1`.

Also, **git-cliff** has the following feature flags which can be enabled via `--features` argument:

- `update-informer`: inform about the new releases of **git-cliff**
- `github`: enables the [GitHub integration](/docs/integration/github)
- `gitlab`: enables the [GitLab integration](/docs/integration/gitlab)
- `bitbucket`: enables the [Bitbucket integration](/docs/integration/bitbucket)

All these features are enabled as default.

To install without these features:

```bash
cargo install git-cliff --no-default-features
```

e.g. disable GitHub integration but enable the new version notifier:

```bash
cargo install git-cliff --no-default-features --features update-informer
```
