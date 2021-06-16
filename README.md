<p align="center">
    <a href="https://github.com/orhun/git-cliff">
        <img src="https://user-images.githubusercontent.com/24392180/121790699-8808dc80-cbea-11eb-8ab6-2fb6b08b66d8.png" width="300"></a>
    <br>
</p>

## Installation

## Usage

```
git-cliff [FLAGS] [OPTIONS] [RANGE]
```

**Flags:**

```
-v, --verbose       Increases the logging verbosity
-l, --latest        Processes the commits starting from the latest tag
-u, --unreleased    Processes the commits that do not belong to a tag
-h, --help          Prints help information
-V, --version       Prints version information
```

**Options:**

```
-c, --config <PATH>        Sets the configuration file [env: CONFIG=]  [default: cliff.toml]
-w, --workdir <PATH>       Sets the working directory [env: WORKDIR=]
-r, --repository <PATH>    Sets the repository to parse commits from [env: REPOSITORY=]
-p, --changelog <PATH>     Prepends entries to the given changelog file [env: CHANGELOG=]
-t, --tag <TAG>            Sets the tag for the latest version [env: TAG=]
-s, --strip <PART>         Strips the given parts from the changelog [possible values: header, footer, all]
```

**Args:**

```
<RANGE>    Sets the commit range to process
```

### Docker

The easiest way of running **git-cliff** (in the git root directory) is to use the [available tags](https://hub.docker.com/repository/docker/orhunp/git-cliff/tags) from [Docker Hub](https://hub.docker.com/repository/docker/orhunp/git-cliff):

```sh
docker run -t -v "$(pwd)":/app/ orhunp/git-cliff:latest
```

Or you can use the image from the [GitHub Package Registry](https://github.com/orhun/git-cliff/packages/841947):

```sh
docker run -t -v "$(pwd)":/app/ docker.pkg.github.com/orhun/git-cliff/git-cliff:latest
```

Also, you can build the image yourself using `docker build -t git-cliff .` command.

## Examples

## License

GNU General Public License ([v3.0](https://www.gnu.org/licenses/gpl.txt))

## Copyright

Copyright © 2021, [git-cliff contributors](mailto:git-cliff@protonmail.com)
