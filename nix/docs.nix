{ lib, ... }:
{
  perSystem = { config, pkgs, ... }:
    {
      devshells.default = {
        env = [{ name = "BIOME_BINARY"; value = "${pkgs.biome}/bin/biome"; }];
      };

      pre-commit.settings.hooks = {
        rome.enable = true;
      };
    };
}
