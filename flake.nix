{
  description = "Tag-based private cloud storage";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ { flake-parts, rust-overlay, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } rec {
      imports = [
      ];

      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];

      perSystem = { pkgs, ... }:
        let
          rustPkgs = pkgs.appendOverlays [ (import rust-overlay) ];
          toolchain = rustPkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        {
          formatter = pkgs.nixpkgs-fmt;

          packages = rec {
            default = oxidrive;
            oxidrive = pkgs.callPackage ./. { };
          };

          devShells.default = pkgs.mkShell {
            BIOME_BINARY = "${pkgs.biome}/bin/biome";

            packages = with pkgs; [
              bacon
              bruno-cli
              cargo-machete
              cargo-mutants
              cargo-nextest
              clang
              just
              mold
              sqlx-cli
              toolchain
              nodejs_20
              lefthook
              lychee
              typos
            ];
          };
        };
    };
}
