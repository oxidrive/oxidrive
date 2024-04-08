{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{ flake-parts, rust-overlay, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ];

      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];

      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          rustPkgs = pkgs.appendOverlays [ (import rust-overlay) ];
          rust = rustPkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        {
          formatter = pkgs.nixpkgs-fmt;

          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              air
              dioxus-cli
              go
              just
              rust
              tailwindcss
            ];

            shellHook = ''
              alias run="just web/watch"
            '';
          };
        };
    };
}
