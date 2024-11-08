{
  description = "git-cliff";

  inputs = {
    nixpkgs.url =
      "github:nixos/nixpkgs/nixos-unstable"; # We want to use packages from the binary cache
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ inputs.rust-overlay.overlays.rust-overlay ];
        };
        base-git-cliff = { buildType }: pkgs.rustPlatform.buildRustPackage {
          name = "git-cliff";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          checkType = "debug";
          inherit buildType;
          checkFlags = [
            "--skip=command"
            "--skip=repo"
          ];
          meta = with pkgs.lib; {
            description = "A highly customizable Changelog Generator that follows Conventional Commit specifications";
            homepage = "https://git-cliff.org/";
            license = [ licenses.mit licenses.asl20 ];
          };
        };
      in rec {
        packages = rec {
          git-cliff = base-git-cliff { buildType = "release"; };
          git-cliff-debug = base-git-cliff { buildType = "debug"; };
          default = git-cliff;
        };

        devShell = pkgs.mkShell {
          CARGO_INSTALL_ROOT = "${toString ./.}/.cargo";

          buildInputs = with pkgs; [
            pkg-config
            openssl
            cargo-binutils
            cargo-watch
            lld
            (rust-bin.fromRustupToolchain {
              channel = "stable";
              components =
                [ "rust-analyzer" "rust-src" "rustfmt" "rustc" "cargo" "clippy" ];
            })
          ];
        };

        checks.check = packages.git-cliff-debug;
      });
}
