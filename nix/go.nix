{ ... }:

{
  perSystem = { config, pkgs, ... }:
    let
      gotestdox = pkgs.callPackage ./pkgs/gotestdox.nix { };
      gremlins = pkgs.callPackage ./pkgs/gremlins.nix { };
    in
    {

      devshells.default = {
        packages = with pkgs; [
          air
          go
          go-migrate
          gotestdox
          gotools
          gremlins
        ];
      };

      pre-commit.settings.hooks = {
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
                    if ! ${pkgs.gotools}/bin/goimports -local github.com/oxidrive/oxidrive -l -d "$file" 2>&1
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
}
