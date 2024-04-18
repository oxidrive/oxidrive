{ lib
, fetchFromGitHub
, buildGoModule
}:

buildGoModule rec {
  pname = "gotestdox";
  version = "0.2.2";

  src = fetchFromGitHub {
    owner = "bitfield";
    repo = "gotestdox";
    rev = "refs/tags/v${version}";
    hash = "sha256-AZDXMwADOjcaMiofMWoHp+eSnD3a8iFtwpWDKl9Ess8=";
  };

  vendorHash = "sha256-kDSZ4RZTHDFmu7ernYRjg0PV7eBB2lH8q5wW3kTExDs=";

  doCheck = false;

  ldflags = [
    "-s"
    "-w"
    "-X github.com/bitfield/gotestdox/cmd.version=${version}"
  ];

  subPackages = [ "./cmd/gotestdox" ];

  meta = with lib; {
    homepage = "https://github.com/bitfield/gotestdox";
    changelog = "https://github.com/bitfield/gotestdox/releases/tag/v${version}";
    description = "A tool for formatting Go test results as readable documentation";
    platforms = platforms.linux ++ platforms.darwin;
    license = licenses.mit;
  };
}
