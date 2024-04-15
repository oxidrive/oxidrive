{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    pre-commit-hooks-nix.url = "github:cachix/pre-commit-hooks.nix";
    devshell.url = "github:numtide/devshell";
  };

  outputs = inputs@{ flake-parts, rust-overlay, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.pre-commit-hooks-nix.flakeModule
        inputs.devshell.flakeModule

        ./nix/go.nix
        ./nix/playwright.nix
        ./nix/rust.nix
        ./nix/shell.nix
      ];

      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];

      perSystem = { system, ... }: {
        rust = {
          toolchain.file = ./web/rust-toolchain.toml;
          overlay = import rust-overlay;
        };
      };
    };
}
