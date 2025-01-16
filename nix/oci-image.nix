{ pkgs ? import <nixpkgs> { }
, oxidrive ? pkgs.callPackage ../. { }
, tag ? "latest"
, revision ? null
}:


pkgs.dockerTools.streamLayeredImage {
  name = "oxidrive";

  contents = [ oxidrive pkgs.cacert ];

  config = {
    WorkingDir = "/oxidrive";

    Env = [
      "HOST=::"
      "OXIDRIVE_STORAGE__PROVIDER=fs"
      "OXIDRIVE_STORAGE__ROOT_FOLDER_PATH=/oxidrive/files"
      "DATABASE_URL=sqlite:/oxidrive/oxidrive.db"
    ];

    ExposedPorts = {
      "4000" = { };
    };

    Volumes = {
      "/oxidrive" = { };
    };

    Entrypoint = [ "/bin/oxidrive" ];
    Cmd = [ "server" ];

    Labels = {
      "org.opencontainers.image.title" = "Oxidrive";
      "org.opencontainers.image.description" = "Self-hostable, simple personal storage service";
      "org.opencontainers.image.licenses" = "AGPL-3.0";
      "org.opencontainers.image.revision" = revision;
      "org.opencontainers.image.version" = if tag == "latest" then oxidrive.version else "${oxidrive.version}-${tag}";
      "org.opencontainers.image.source" = "https://github.com/oxidrive/oxidrive";
      "org.opencontainers.image.url" = "https://oxidrive.github.io/oxidrive";
    };
  };
}
