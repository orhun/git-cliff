---
sidebar_position: 5
---

# Nix

If you are using Nix, **git-cliff** can be installed from the [official latest stable channel](https://search.nixos.org/packages?channel=24.05&show=git-cliff&from=0&size=50&sort=relevance&type=packages&query=git-cliff).

## Using nix-shell

To temporarily install git-cliff in a shell environment, run:

```bash
nix-shell -p git-cliff
```

## Using nix-env

To install git-cliff permanently, use:

```bash
nix-env -iA nixpkgs.git-cliff
```

:::warning

Using nix-env permanently modifies a local profile of installed packages.
This must be updated and maintained by the user in the same way as with a traditional package manager,
foregoing many of the benefits that make Nix uniquely powerful.
Using nix-shell or a NixOS configuration is recommended instead.

:::

## The New CLI

If you're using the new experimental CLI, you can use any of the following:

### `nix run`

```bash
nix run nixpkgs#git-cliff
```

### `nix shell`

To open a new shell with git-cliff available, use:

```bash
nix shell nixpkgs#git-cliff
```

## Using the Unstable Channel

While new releases of **git-cliff** typically become available in the stable channel relatively quickly, you can use the unstable channel if you want to access the latest features and updates sooner.

To add and update the unstable channel, run:

```bash
nix-channel --add https://nixos.org/channels/nixpkgs-unstable
nix-channel --update nixpkgs
```

After updating to the unstable channel, you can use any of the previous installation commands to install **git-cliff**.
