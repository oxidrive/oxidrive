{ pkgs
, lib
, rustPlatform
}:

let
  manifest = lib.importTOML ./Cargo.toml;
  meta = manifest.workspace.package;

  web = pkgs.callPackage ./nix/web.nix { };

  f = import ./nix/filters.nix pkgs;

  src = lib.cleanSourceWith {
    filter = f.hasSuffices [ ".toml" "Cargo.lock" ".rs" ".pest" ".sql" ];
    src = pkgs.nix-gitignore.gitignoreSource [ ] (lib.cleanSource ./.);
  };
in
rustPlatform.buildRustPackage {
  pname = "oxidrive";
  version = meta.version;
  inherit src;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  cargoBuildFlags = "--bin oxidrive";

  useNextest = true;

  configurePhase = ''
    runHook preConfigure

    cp -r ${web} web/build

    runHook postConfigure
  '';

  nativeBuildInputs = with pkgs; [
    clang
    mold
  ];

  meta = {
    mainProgram = "oxidrive";
    homepage = meta.documentation;
    license = lib.licenses.agpl3Plus;
    maintainers = meta.authors;
  };
}
