pkgs:

let
  lib = pkgs.lib;
in
{
  hasSuffices =
    exts: name: type:
    let
      base = baseNameOf (toString name);
    in
    type == "directory" || lib.any (ext: lib.hasSuffix ext base) exts;
}
