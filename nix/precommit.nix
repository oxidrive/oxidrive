{ ... }:

{
  perSystem = { pkgs, ... }: {
    pre-commit = {
      inherit pkgs;

      check.enable = true;
      settings = {
        hooks = {
          cargo-check.enable = true;
          clippy.enable = true;
          rustfmt.enable = true;
        };
      };
    };
  };
}
