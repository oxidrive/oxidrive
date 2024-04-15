{ lib, ... }:
let
  inherit (lib) mkOption;

  l = lib // builtins;
  t = lib.types;
in
{
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      rustPkgs = pkgs.appendOverlays [ config.rust.overlay ];
      rust = rustPkgs.rust-bin.fromRustupToolchainFile config.rust.toolchain.file;
    in
    {
      options = {
        rust = {
          toolchain = {
            file = mkOption {
              type = t.path;
            };

            package = mkOption {
              type = t.package;
              description = "Toolchain package containing the correct versions of rustc, cargo, etc...";
            };
          };

          overlay = mkOption {
            type = t.functionTo (t.functionTo t.attrs);
          };
        };
      };

      config = {
        rust.toolchain.package = rust;

        devshells.default = {
          packages = with pkgs; [
            dioxus-cli
            rust
          ];
        };

        pre-commit.settings.hooks =
          let
            rustCheck = {
              enable = true;
              package = rust;
              packageOverrides = {
                rustc = rust;
                cargo = rust;
                rustfmt = rust;
                clippy = rust;
              };
            };
          in
          {
            cargo-check = rustCheck;
            clippy = rustCheck;
            rustfmt = rustCheck;
          };
      };
    };
}
