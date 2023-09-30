<p align="center">
    <a href="https://git-cliff.org">
        <img src="https://user-images.githubusercontent.com/24392180/121790699-8808dc80-cbea-11eb-8ab6-2fb6b08b66d8.png" width="300"></a>
    <br>
    <a href="https://github.com/orhun/git-cliff/releases">
        <img src="https://img.shields.io/github/v/release/orhun/git-cliff?style=flat&labelColor=1C2C2E&color=C96329&logo=GitHub&logoColor=white">
    </a>
    <a href="https://crates.io/crates/git-cliff/">
        <img src="https://img.shields.io/crates/v/git-cliff?style=flat&labelColor=1C2C2E&color=C96329&logo=Rust&logoColor=white">
    </a>
    <a href="https://codecov.io/gh/orhun/git-cliff">
        <img src="https://img.shields.io/codecov/c/gh/orhun/git-cliff?style=flat&labelColor=1C2C2E&color=C96329&logo=Codecov&logoColor=white">
    </a>
    <br>
    <a href="https://github.com/orhun/git-cliff/actions?query=workflow%3A%22Continuous+Integration%22">
        <img src="https://img.shields.io/github/actions/workflow/status/orhun/git-cliff/ci.yml?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=GitHub%20Actions&logoColor=BEC5C9">
    </a>
    <a href="https://github.com/orhun/git-cliff/actions?query=workflow%3A%22Continuous+Deployment%22">
        <img src="https://img.shields.io/github/actions/workflow/status/orhun/git-cliff/cd.yml?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=GitHub%20Actions&logoColor=BEC5C9&label=deploy">
    </a>
    <a href="https://hub.docker.com/r/orhunp/git-cliff">
        <img src="https://img.shields.io/github/actions/workflow/status/orhun/git-cliff/docker.yml?style=flat&labelColor=1C2C2E&color=BEC5C9&label=docker&logo=Docker&logoColor=BEC5C9">
    </a>
    <a href="https://docs.rs/git-cliff-core/">
        <img src="https://img.shields.io/docsrs/git-cliff-core?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=Rust&logoColor=BEC5C9">
    </a>
    <br>
    <a href="https://matrix.to/#/#git-cliff:matrix.org">
        <img src="https://img.shields.io/matrix/git-cliff:matrix.org?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=matrix&logoColor=BEC5C9&label=join%20matrix">
    </a>
    <a href="https://discord.gg/W3mAwMDWH4">
        <img src="https://img.shields.io/discord/1093977388892819553?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=discord&logoColor=BEC5C9&label=join%20discord">
    </a>
</p>

<h4 align="center">
  <a href="https://git-cliff.org/docs">Documentation</a> |
  <a href="https://git-cliff.org">Website</a>
</h4>

**git-cliff** can generate [changelog](https://en.wikipedia.org/wiki/Changelog) files from the [Git](https://git-scm.com/) history by utilizing [conventional commits](https://git-cliff.org/docs/configuration#conventional_commits) as well as regex-powered [custom parsers](https://git-cliff.org/docs/configuration#commit_parsers). The [changelog template](https://git-cliff.org/docs/category/templating) can be customized with a [configuration file](https://git-cliff.org/docs/configuration) to match the desired format.

![preview](https://user-images.githubusercontent.com/24392180/128637997-5713ba25-d8f3-40c7-8ba8-ea7f333ead88.png)

## Documentation

Learn how to use **git-cliff** from the [documentation](https://git-cliff.org/docs).

- [Installation](https://git-cliff.org/docs/installation/)
- [Usage](https://git-cliff.org/docs/usage/examples)
- [Configuration](https://git-cliff.org/docs/configuration)
- [Templating](https://git-cliff.org/docs/category/templating)

## In The Media

- **git-cliff** was featured in [Console #141 - The Open Source Newsletter](https://console.substack.com/p/console-141)

## Similar/Related Projects

- [git-journal](https://github.com/saschagrunert/git-journal) - The Git Commit Message and Changelog Generation Framework
- [clog-cli](https://github.com/clog-tool/clog-cli) - Generate beautiful changelogs from your Git commit history
- [relnotes](https://crates.io/crates/relnotes) - A tool to automatically generate release notes for your project.
- [cocogitto](https://github.com/oknozor/cocogitto) - A set of CLI tools for the conventional commit and semver specifications.
- [cliff-jumper](https://github.com/favware/cliff-jumper) - A NodeJS CLI tool that combines git-cliff and
  [conventional-recommended-bump](https://github.com/conventional-changelog/conventional-changelog/tree/master/packages/conventional-recommended-bump)
  to semantically bump a NodeJS package and generate a git-cliff powered changelog.
- [release-plz](https://github.com/MarcoIeni/release-plz) - Release Rust packages from CI.

## Contributors

Thanks goes to these wonderful people ✨

<a href="https://github.com/orhun/git-cliff/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=orhun/git-cliff" />
</a>

Made with [contrib.rocks](https://contrib.rocks).

## License

Licensed under either of [Apache License Version 2.0](./LICENSE-APACHE) or [The MIT License](./LICENSE-MIT) at your option.

## Copyright

Copyright © 2021-2023, [git-cliff contributors](mailto:git-cliff@protonmail.com)
