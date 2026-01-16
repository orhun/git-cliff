# Contributing

Thank you for considering contributing to [git-cliff](https://github.com/orhun/git-cliff)!

When contributing, please first discuss the change you wish to make via [issue](https://github.com/orhun/git-cliff/issues),
[email](mailto:orhunparmaksiz@gmail.com), or any other method with the owners of this repository before making a change.

Note that we have a [Code of Conduct](./CODE_OF_CONDUCT.md), please follow it in all your interactions with the project.

---

## Required Tooling

- Install the nightly toolchain (required for `rustfmt`):

```sh
rustup toolchain install nightly
```

- Optional: set up editor/IDE integration to use **nightly** `rustfmt` for this repository.

---

## Setup

1. Fork this repository and create your branch from `main`.

2. Clone your forked repository.

```sh
git clone https://github.com/{username}/git-cliff && cd git-cliff
# OR
git clone git@github.com:{username}/git-cliff && cd git-cliff
```

3. Fetch tags (required for tests):

```sh
git fetch --tags https://github.com/orhun/git-cliff
```

4. Install [Rust](https://www.rust-lang.org/) `1.85.1` or later and build the project:

```sh
cargo build
```

> [!NOTE]
>
> - The project uses **stable** Rust for builds and tests.
> - Formatting and linting are run with the **nightly toolchain** in CI due to the use of unstable `rustfmt` options.
>   Contributors are expected to run the same checks locally.

---

## Development Workflow

1. Start committing your changes. Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.

2. Add your tests (if you haven't already) or update the existing tests according to the changes. And check if the tests are passed:

```sh
cargo test
```

3. If you changed snapshot tests (i.e. `expect_test`), update snapshots:

```sh
env UPDATE_EXPECT=1 cargo test
```

4. Run CI checks locally - `clippy` (warnings are errors)


```sh
cargo clippy --tests --verbose -- -D warnings
```

5. Run CI checks locally - `clippy` (**optional**, but recommended for pedantic linting)

```sh
cargo clippy --all-targets --verbose -- -W clippy::pedantic
```

> [!NOTE]
>
> - You may allow specific pedantic lints **only with a clear justification**.
> - Running `clippy` with pedantic lints is **optional**, but it can serve as a helpful guideline for new code and implementations, helping maintain consistency and catch potential issues early.

6. Run CI checks locally – `rustfmt`

```sh
cargo +nightly fmt --all -- --check --verbose
```

If formatting fails, please run:

```sh
cargo +nightly fmt --all
```

---

## Create a Pull Request

1. Ensure that you updated the documentation and filled the [Pull Request template](./.github/PULL_REQUEST_TEMPLATE.md) according to the changes you made.

2. Wait for approval from the project owner/maintainer. Discuss the possible changes and update your Pull Request if necessary.

3. You may merge the Pull Request once you have the sign-off of the project owner/maintainer, or if you do not have permission to do that, you may request the project owner/maintainer to merge it in case they haven't done it after a while.

# License

By contributing, you agree that your contributions will be licensed under [The MIT License](./LICENSE-MIT) or [Apache License 2.0](./LICENSE-APACHE).
