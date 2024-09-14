# Contributing

Thank you for considering contributing to [git-cliff](https://github.com/orhun/git-cliff)!

When contributing, please first discuss the change you wish to make via [issue](https://github.com/orhun/git-cliff/issues),
[email](mailto:orhunparmaksiz@gmail.com), or any other method with the owners of this repository before making a change.

Note that we have a [Code of Conduct](./CODE_OF_CONDUCT.md), please follow it in all your interactions with the project.

## Setup

1. Fork this repository and create your branch from `main`.

2. Clone your forked repository.

```sh
git clone https://github.com/{username}/git-cliff && cd git-cliff
```

To ensure the successful execution of the tests, it is essential to fetch the tags as follows:

```sh
git fetch --tags https://github.com/orhun/git-cliff
```

3. Make sure that you have [Rust](https://www.rust-lang.org/) `1.64.0` or later installed and build the project.

```sh
cargo build
```

4. Start committing your changes. Follow the [conventional commit specification](https://www.conventionalcommits.org/) while doing so.

5. Add your tests (if you haven't already) or update the existing tests according to the changes. And check if the tests are passed.

```sh
cargo test
```

6. If needed, update the snapshot tests (i.e. tests using `expect_test`):

```sh
env UPDATE_EXPECT=1 cargo test
```

7. Make sure [rustfmt](https://github.com/rust-lang/rustfmt) and [clippy](https://github.com/rust-lang/rust-clippy) don't complain about your changes.

We use the `nightly` channel for `rustfmt` so please set the appropriate settings for your editor/IDE for that.

## Create a Pull Request

1. Ensure that you updated the documentation and filled the [Pull Request template](./.github/PULL_REQUEST_TEMPLATE.md) according to the changes you made.

2. Wait for approval from the project owner/maintainer. Discuss the possible changes and update your Pull Request if necessary.

3. You may merge the Pull Request once you have the sign-off of the project owner/maintainer, or if you do not have permission to do that, you may request the project owner/maintainer to merge it in case they haven't done it after a while.

# License

By contributing, you agree that your contributions will be licensed under [The MIT License](./LICENSE-MIT) or [Apache License 2.0](./LICENSE-APACHE).
