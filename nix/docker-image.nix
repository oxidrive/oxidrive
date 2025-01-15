{ pkgs ? import <nixpkgs> { }
, oxidrive ? pkgs.callPackage ../. { }
}:


pkgs.dockerTools.streamLayeredImage {
  name = "oxidrive";
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

    Entrypoint = [ "${oxidrive}/bin/oxidrive" ];
    Cmd = [ "server" ];
  };
}
