{
  pkgs,
  lib,
  rustPlatform,
  toolchain,
}:

let
  manifest = lib.importTOML ./Cargo.toml;
  meta = manifest.workspace.package;

  web = pkgs.callPackage ./nix/web-ui.nix { };

  f = import ./nix/filters.nix pkgs;

  src = lib.cleanSourceWith {
    filter = f.hasSuffices [
      ".toml"
      "Cargo.lock"
      ".rs"
      ".pest"
      ".sql"
      ".cedar"
      ".cedarschema"
    ];
    src = pkgs.nix-gitignore.gitignoreSource [ ] (lib.cleanSource ./.);
  };
in
rustPlatform.buildRustPackage {
  pname = "oxidrive";
  version = meta.version;
  inherit src;

  cargoLock = {
    lockFile = ./Cargo.lock;

    outputHashes = {
      "vite-rs-0.1.0" = "sha256-rTK/81Ras273rAwXW9bVw3BOuWbS/zK7eeqIjWXs/7M=";
    };
  };

  cargoBuildFlags = "--bin oxidrive";

  useNextest = true;

  configurePhase = ''
    runHook preConfigure

    cp -r ${web}/build app/ui/build

    runHook postConfigure
  '';

  nativeBuildInputs = with pkgs; [
    clang
    mold
    nodejs_20
    toolchain
  ];

  meta = {
    mainProgram = "oxidrive";
    homepage = meta.documentation;
    license = lib.licenses.agpl3Plus;
    maintainers = meta.authors;
  };
}
