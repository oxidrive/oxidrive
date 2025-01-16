{ pkgs
, buildNpmPackage
, lib
}:

let
  manifest = lib.importTOML ../Cargo.toml;
  meta = manifest.workspace.package;

  excluded = [
    "e2e/"
    "biome.json"
    "Justfile"
    "Cargo.toml"
    "*.rs"
    "playwright.config.ts"
  ];

  src = pkgs.nix-gitignore.gitignoreSource excluded (lib.cleanSource ../web);
in
buildNpmPackage {
  pname = "oxidrive-web";
  version = meta.version;
  inherit src;

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
