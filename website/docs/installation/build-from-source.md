---
sidebar_position: 2
---

# Build from source

### Prerequisites

- [Rust](https://www.rust-lang.org/) (nightly)
  - The minimum supported Rust version is `1.75.0`.
- [zlib](https://zlib.net/)
- [libgit2](https://libgit2.org/)

### Instructions

1. Clone the repository.

```bash
git clone https://github.com/orhun/git-cliff
cd git-cliff/
```

2. Build.

```bash
CARGO_TARGET_DIR=target cargo build --release
```

Binary will be located at `target/release/git-cliff`.

Also, see the [available feature flags](/docs/installation/crates-io).

### Shell completions

To generate completions in `target`:

```bash
OUT_DIR=target target/release/git-cliff-completions
```

### Manpage

To generate a manpage in `target`:

```bash
OUT_DIR=target target/release/git-cliff-mangen
```
