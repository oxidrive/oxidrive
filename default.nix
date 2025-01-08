{ pkgs
, lib
, rustPlatform
}:

let
  manifest = lib.importTOML ./Cargo.toml;
  meta = manifest.workspace.package;

  web = pkgs.callPackage ./nix/web.nix { };
in
rustPlatform.buildRustPackage {
  pname = "oxidrive";
  version = meta.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;

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
