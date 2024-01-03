---
sidebar_position: 8
---

# Docker

### Images

Docker builds are [automated](https://github.com/orhun/git-cliff/tree/main/.github/workflows/docker.yml) and images are available in the following registries:

- [Docker Hub](https://hub.docker.com/r/orhunp/git-cliff)
- [GitHub Container Registry](https://github.com/orhun/git-cliff/pkgs/container/git-cliff%2Fgit-cliff)

### Usage

The easiest way of running **git-cliff** (in the git root directory with [configuration file](/docs/configuration) present) is to use the available tags from [Docker Hub](https://hub.docker.com/r/orhunp/git-cliff):

```bash
docker run -t -v "$(pwd)":/app/ "orhunp/git-cliff:${TAG:-latest}"
```

Or you can use the image from the [GitHub Package Registry](https://github.com/orhun/git-cliff/pkgs/container/git-cliff%2Fgit-cliff):

```bash
docker run -t -v "$(pwd)":/app/ "ghcr.io/orhun/git-cliff/git-cliff:${TAG:-latest}"
```

### Building

Custom Docker images can be built from the [Dockerfile](https://github.com/orhun/git-cliff/blob/main/Dockerfile):

```bash
DOCKER_BUILDKIT=1 docker build -t git-cliff .
```
