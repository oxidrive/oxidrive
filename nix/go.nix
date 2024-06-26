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
          golangci-lint
        ];
      };
    };
}
