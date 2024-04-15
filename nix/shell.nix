{ lib, ... }:
{
  perSystem = { config, pkgs, ... }:
    let
      mkJustCmd = command: { help, category }: { inherit help category; name = "just ${command}"; package = pkgs.just; };
    in
    {
      formatter = pkgs.nixpkgs-fmt;

      devshells.default = {
        name = "Oxidrive";

        commands = [
          (mkJustCmd "server/watch" { help = "Start the API server locally (with autoreload)"; category = "Server"; })
          (mkJustCmd "server/test" { help = "Run the API server unit tests"; category = "Server"; })
          (mkJustCmd "server/migration-reate <name>" { help = "Create a new SQL migration"; category = "Server"; })

          (mkJustCmd "web/watch" { help = "Start the web frontend locally (with autoreload)"; category = "Web"; })
          (mkJustCmd "web/watch-app" { help = "Start only the web frontend locally without the stylesheets (with autoreload)"; category = "Web"; })
          (mkJustCmd "web/watch-styles" { help = "Recompile only the stylesheets (with autoreload)"; category = "Web"; })
          (mkJustCmd "web/test" { help = "Run the web frontend unit tests"; category = "Web"; })

          (mkJustCmd "e2e/setup" { help = "Prepare the local environment for running E2E tests. Only required once"; category = "E2E"; })
          (mkJustCmd "e2e/test" {
            help = ''
              Run the E2E test suite. If it's the first time or the source code was recently changed, run just e2e/rebuild before (e.g. just e2e/rebuild && just e2e/test).
              Doesn't work on Nix yet due to missing browsers
            '';
            category = "E2E";
          })
          (mkJustCmd "e2e/ui" { help = "Run the Playwright UI to interactively run tests"; category = "E2E"; })
          (mkJustCmd "e2e/chromium" { help = "Like just e2e/test, but only runs chromium-based tests"; category = "E2E"; })

          { command = "pre-commit run --all-files"; name = "format"; help = "Reformat everything (Go, Rust, TypeScript, Nix...) in one go"; }
          (mkJustCmd "act" { help = "Run the GitHub Actions workflows locally. Requires a running Docker engine and authenticated gh CLI"; category = "CI"; })
        ];

        packages = with pkgs; [
          act
          gh
        ];

        devshell.startup.pre-commit-hooks.text = config.pre-commit.installationScript;
      };

      pre-commit = {
        check.enable = false;
        settings.hooks = {
          nixpkgs-fmt.enable = true;
        };
      };
    };
}
