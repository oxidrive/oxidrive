{ lib, ... }:
{
  perSystem = { config, pkgs, ... }:
    {
      devshells.default = {
        env = [
          { name = "NIX_PLAYWRIGHT_BROWSERS_PATH"; value = pkgs.playwright-driver.browsers; }
          { name = "BIOME_BINARY"; value = "${pkgs.biome}/bin/biome"; }
        ];

        packages = with pkgs; [
          nodejs_20
        ];
      };
    };
}
