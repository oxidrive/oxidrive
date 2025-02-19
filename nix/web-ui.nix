{ pkgs
, buildNpmPackage
, lib
}:

let
  manifest = lib.importTOML ../Cargo.toml;
  meta = manifest.workspace.package;

  excluded = [
    "docs/"
    "e2e/"
    "biome.json"
    "Cargo.toml"
    "*.rs"
    "playwright.config.ts"
  ];

  src = pkgs.nix-gitignore.gitignoreSource excluded (lib.cleanSource ../.);
in
buildNpmPackage {
  pname = "oxidrive-ui-deps";
  version = meta.version;
  inherit src;

  nodejs = pkgs.nodejs_20;

  npmDepsHash = "sha256-M79Ek1lIFhYQlxQaofBTOBHBexP6Bq2RBEKzoJweMWs=";
  npmFlags = "--workspace app/ui";

  installPhase = ''
    runHook preInstall

    mkdir -p $out

    cp -r app/ui/build $out/build

    runHook postInstall
  '';

  meta = {
    homepage = meta.documentation;
    license = lib.licenses.agpl3Plus;
    maintainers = meta.authors;
  };
}
