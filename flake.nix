{
  description = "Tag-based private cloud storage";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nixpkgs-stable.url = "github:NixOS/nixpkgs/nixos-25.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      self,
      flake-parts,
      rust-overlay,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } rec {
      imports = [
      ];

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];

      perSystem =
        { pkgs, system, ... }:
        let
          rustPkgs = pkgs.appendOverlays [ (import rust-overlay) ];
          toolchain = rustPkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

          stablePkgs = import inputs.nixpkgs-stable { inherit system; };
        in
        {
          formatter = pkgs.nixfmt-tree;

          packages = rec {
            default = oxidrive;
            oxidrive = pkgs.callPackage ./. { inherit toolchain; };
            oci-image = pkgs.callPackage ./nix/oci-image.nix {
              inherit pkgs oxidrive;

              revision = self.rev or self.dirtyRev or null;
              created = builtins.substring 0 8 self.lastModifiedDate;
            };
          };

          devShells.default = pkgs.mkShell {
            BIOME_BINARY = "${pkgs.biome}/bin/biome";

            packages = with pkgs; [
              bacon
              stablePkgs.bruno-cli
              cosign
              cargo-machete
              cargo-mutants
              cargo-nextest
              cedar
              clang
              just
              mold
              skopeo
              sqlx-cli
              toolchain
              nodejs_22
              lefthook
              lychee
              typos
            ];
          };
        };
    };
}
