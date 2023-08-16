# Build from source

### Prerequisites

- [Rust](https://www.rust-lang.org/) (nightly)
  - Minimum supported Rust version is `1.68.2`.
- [zlib](https://zlib.net/) (Linux)

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
