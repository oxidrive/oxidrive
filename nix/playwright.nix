{ lib, ... }:
{
  perSystem = { config, pkgs, ... }:
    {
      devshells.default = {
        env = [{ name = "NIX_PLAYWRIGHT_BROWSERS_PATH"; value = pkgs.playwright-driver.browsers; }];

        packages = with pkgs; [
          nodejs_20
        ];
      };

      pre-commit.settings.hooks = {
        biome.enable = true;
      };
    };
}
