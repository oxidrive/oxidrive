{ ... }:

{
  perSystem = { pkgs, ... }: {
    pre-commit = {
      inherit pkgs;

      check.enable = true;
      settings = {
        hooks = {
          # Go


          # Rust
          cargo-check.enable = true;
          clippy.enable = true;
          rustfmt.enable = true;
        };
      };
    };
  };
}
