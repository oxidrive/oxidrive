{ pkgs
, buildNpmPackage
, lib
}:

let
  manifest = lib.importTOML ../Cargo.toml;
  meta = manifest.workspace.package;
in
buildNpmPackage {
  pname = "oxidrive-web";
  version = meta.version;
  src = lib.cleanSource ../web;

  nativeBuildInputs = with pkgs; [
    nodejs_20
  ];

  npmDepsHash = "sha256-8/zkME4+C3+CfCcFYOJLkEm9q8GKcQPIrxHoOV2GdFw=";

  installPhase = ''
    runHook preInstall

    cp -r build $out

    runHook postInstall
  '';

  meta = {
    homepage = meta.documentation;
    license = lib.licenses.agpl3Plus;
    maintainers = meta.authors;
  };
}
