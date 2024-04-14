{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    pre-commit-hooks-nix.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs = inputs@{ flake-parts, rust-overlay, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.pre-commit-hooks-nix.flakeModule
      ];

      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];

      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          rustPkgs = pkgs.appendOverlays [ (import rust-overlay) ];
          rust = rustPkgs.rust-bin.fromRustupToolchainFile ./web/rust-toolchain.toml;
          goPkgs = with pkgs; [
            go
            gotools
            go-migrate
          ];
        in
        {
          pre-commit = {
            check.enable = true;
            pkgs = pkgs;
            settings = {
              hooks = {
                gofmt.enable = true;
                govet.enable = true;
                golangci-lint = {
                  enable = true;
                  name = "golangci-lint";
                  description = "Lint my golang code";
                  files = "\.go$";
                  entry =
                    let
                      script = pkgs.writeShellScript "precommit-golangci" ''
                        set -e
                        export PATH=$PATH:${pkgs.go}/bin
                        ${pkgs.golangci-lint}/bin/golangci-lint run --new-from-rev HEAD --fix
                      '';
                    in
                    builtins.toString script;
                  require_serial = true;
                  pass_filenames = false;
                };
                goimports = {
                  enable = true;
                  name = "goimports";
                  description = "Format my golang code";
                  files = "\.go$";
                  entry =
                    let
                      script = pkgs.writeShellScript "precommit-goimports" ''
                        set -e
                        failed=false
                        for file in "$@"; do
                            # redirect stderr so that violations and summaries are properly interleaved.
                            if ! ${pkgs.gotools}/bin/goimports -l -d "$file" 2>&1
                            then
                                failed=true
                            fi
                        done
                        if [[ $failed == "true" ]]; then
                            exit 1
                        fi
                      '';
                    in
                    builtins.toString script;
                };
              };
            };
          };

          formatter = pkgs.nixpkgs-fmt;

          devShells.default = pkgs.mkShell {
            PLAYWRIGHT_BROWSERS_PATH = pkgs.playwright-driver.browsers;
            shellHook = ''
              ${config.pre-commit.installationScript}
              echo 1>&2 "Oxidrive development shell, pre-commits are enabled by default and can be run using nix flake check!"
            '';

            packages = with pkgs; [
              act
              air
              dioxus-cli
              just
              nodejs_20
              rust

              # Server
              goPkgs
            ] ++ config.pre-commit.settings.enabledPackages;
          };
        };
    };
}
