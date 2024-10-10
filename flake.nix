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
      in {
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
      });
}
