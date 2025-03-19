<p align="center">
    <a href="https://git-cliff.org">
        <img src="https://raw.githubusercontent.com/orhun/git-cliff/main/website/static/img/git-cliff-logo.png" width="300"></a><!-- </a> being on the same line as the <img> tag is intentional! -->
    <br>
    <a href="https://github.com/orhun/git-cliff/releases">
        <img src="https://img.shields.io/github/v/release/orhun/git-cliff?style=flat&labelColor=1C2C2E&color=C96329&logo=GitHub&logoColor=white"></a>
    <a href="https://crates.io/crates/git-cliff/">
        <img src="https://img.shields.io/crates/v/git-cliff?style=flat&labelColor=1C2C2E&color=C96329&logo=Rust&logoColor=white"></a>
    <a href="https://codecov.io/gh/orhun/git-cliff">
        <img src="https://img.shields.io/codecov/c/gh/orhun/git-cliff?style=flat&labelColor=1C2C2E&color=C96329&logo=Codecov&logoColor=white"></a>
    <br>
    <a href="https://github.com/orhun/git-cliff/actions?query=workflow%3A%22Continuous+Integration%22">
        <img src="https://img.shields.io/github/actions/workflow/status/orhun/git-cliff/ci.yml?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=GitHub%20Actions&logoColor=BEC5C9"></a>
    <a href="https://github.com/orhun/git-cliff/actions?query=workflow%3A%22Continuous+Deployment%22">
        <img src="https://img.shields.io/github/actions/workflow/status/orhun/git-cliff/cd.yml?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=GitHub%20Actions&logoColor=BEC5C9&label=deploy"></a>
    <a href="https://hub.docker.com/r/orhunp/git-cliff">
        <img src="https://img.shields.io/github/actions/workflow/status/orhun/git-cliff/docker.yml?style=flat&labelColor=1C2C2E&color=BEC5C9&label=docker&logo=Docker&logoColor=BEC5C9"></a>
    <a href="https://docs.rs/git-cliff-core/">
        <img src="https://img.shields.io/docsrs/git-cliff-core?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=Rust&logoColor=BEC5C9"></a>
    <br>
</p>

<h4 align="center">
  <a href="https://git-cliff.org/docs">Documentation</a> |
  <a href="https://git-cliff.org">Website</a>
</h4>

**git-cliff** can generate [changelog](https://en.wikipedia.org/wiki/Changelog) files from the [Git](https://git-scm.com/) history by utilizing [conventional commits](https://git-cliff.org/docs/configuration/git#conventional_commits) as well as regex-powered [custom parsers](https://git-cliff.org/docs/configuration/git#commit_parsers). The [changelog template](https://git-cliff.org/docs/category/templating) can be customized with a [configuration file](https://git-cliff.org/docs/configuration) to match the desired format.

![animation](https://raw.githubusercontent.com/orhun/git-cliff/main/website/static/img/git-cliff-anim.gif)

## Documentation

Learn how to use **git-cliff** from the [official documentation](https://git-cliff.org/docs).

- [Installation](https://git-cliff.org/docs/installation/)
- [Usage](https://git-cliff.org/docs/usage/examples)
- [Configuration](https://git-cliff.org/docs/configuration)
- [Templating](https://git-cliff.org/docs/category/templating)

You can also check out the blog posts written by the community:

- [An introduction to git-cliff for release management](https://substack.evancarroll.com/p/git-cliff-for-automated-release-management): Learn how to automate your software releases
- [Git-cliff and monorepos](https://substack.evancarroll.com/p/git-cliff-and-monorepos): An introduction to the monorepo capabilities of git-cliff
- [git-cliff: The Smart Way to Handle Changelogs](https://medium.com/@toniomasotti/git-cliff-96449950db48)

## In The Media

- [Turning Git commits into changelog with git-cliff](https://www.youtube.com/watch?v=RWh8qbiLRts) - RustLab 2023 (Talk)
- [An Interview with Orhun of git-cliff](https://console.substack.com/p/console-141) - Console #141 (Newsletter)
- [KaiCode Open Source Festival 2024](https://www.kaicode.org/2024.html) (Second place winner)

## Editor Support

- [git-cliff.el](https://github.com/liuyinz/git-cliff.el) - Generate, update and release changelog in Emacs

## Similar/Related Projects

- [git-journal](https://github.com/saschagrunert/git-journal) - The Git Commit Message and Changelog Generation Framework
- [clog-cli](https://github.com/clog-tool/clog-cli) - Generate beautiful changelogs from your Git commit history
- [relnotes](https://crates.io/crates/relnotes) - A tool to automatically generate release notes for your project.
- [cocogitto](https://github.com/oknozor/cocogitto) - A set of CLI tools for the conventional commit and semver specifications.
- [cliff-jumper](https://github.com/favware/cliff-jumper) - A NodeJS CLI tool that combines git-cliff and
  [conventional-recommended-bump](https://github.com/conventional-changelog/conventional-changelog/tree/master/packages/conventional-recommended-bump)
  to semantically bump a NodeJS package and generate a git-cliff powered changelog.
- [release-plz](https://github.com/MarcoIeni/release-plz) - Release Rust packages from CI.
- [git-changelog-command-line](https://github.com/tomasbjerre/git-changelog-command-line) - Generate changelog and determine next version with conventional commits.
- [git-changelog](https://github.com/pawamoy/git-changelog): Automatic Changelog generator using Jinja2 templates.

## Contributors

Thanks goes to these wonderful people ✨

<a href="https://github.com/orhun/git-cliff/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=orhun/git-cliff" />
</a>

Made with [contrib.rocks](https://contrib.rocks).

## Socials

<a href="https://discord.gg/W3mAwMDWH4">
    <img src="https://discord.com/api/guilds/1093977388892819553/embed.png?style=banner2"></a> <!-- </a> being on the same line as the <img> tag is intentional! -->
<br>
<a href="https://matrix.to/#/#git-cliff:matrix.org">
    <img src="https://img.shields.io/matrix/git-cliff:matrix.org?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=matrix&logoColor=BEC5C9&label=join%20matrix"></a>
<a href="https://discord.gg/W3mAwMDWH4">
    <img src="https://img.shields.io/discord/1093977388892819553?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=discord&logoColor=BEC5C9&label=join%20discord"></a>
<a href="https://twitter.com/git_cliff">
    <img src="https://img.shields.io/twitter/follow/git_cliff?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=twitter&logoColor=BEC5C9"></a>
<a href="https://fosstodon.org/@git_cliff">
    <img src="https://img.shields.io/mastodon/follow/111545487385097711?domain=https%3A%2F%2Ffosstodon.org&style=flat&labelColor=1C2C2E&color=BEC5C9&logo=mastodon&logoColor=BEC5C9"></a>

## License

Licensed under either of [Apache License Version 2.0](./LICENSE-APACHE) or [The MIT License](./LICENSE-MIT) at your option.

## Copyright

Copyright © 2021-2025, [git-cliff contributors](mailto:git-cliff@protonmail.com)
