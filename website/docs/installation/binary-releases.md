---
sidebar_position: 3
---
# Binary releases

See the available binaries for different operating systems/architectures from the [releases page](https://github.com/orhun/git-cliff/releases).

Release tarballs are signed with the following PGP key: [1D2D410A741137EBC544826F4A92FA17B6619297](https://keyserver.ubuntu.com/pks/lookup?search=0x4A92FA17B6619297&op=vindex)

### Linux

1. Download the latest binary from [releases](https://github.com/orhun/git-cliff/releases) section and pick between [glibc](https://en.wikipedia.org/wiki/Glibc) or [musl-libc](https://musl.libc.org/) binary.

2. To download the package compiled with `glibc`:

```bash
# version="1.0.0"
wget "https://github.com/orhun/git-cliff/releases/download/v${version}/git-cliff-${version}-x86_64-unknown-linux-gnu.tar.gz"
```

2. To download the package compiled with `musl-libc`:

```bash
# version="1.0.0"
wget "https://github.com/orhun/git-cliff/releases/download/v${version}/git-cliff-${version}-x86_64-unknown-linux-musl.tar.gz"
```

3. Extract the files:

```bash
tar -xvzf git-cliff-*.tar.gz
```

4. Enter the folder:

```bash
cd "git-cliff-${version}"
```

5. Run the binary:

```bash
./git-cliff
```

6. Move binary to `/usr/local/bin/` for running it from the terminal using `git-cliff` command.
