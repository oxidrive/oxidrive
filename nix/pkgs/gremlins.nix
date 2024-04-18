{ buildGoModule, fetchFromGitHub, lib, ... }:
let
  owner = "go-gremlins";
  repo = "gremlins";
  url = "github.com/${owner}/${repo}";
in
buildGoModule rec {
  pname = repo;
  version = "0.5.0";

  src = fetchFromGitHub {
    inherit owner repo;
    rev = "v${version}";
    sha256 = "sha256-QEraB3QzKtU55OmGWsS0iJTXmuNyXKteDewRnRtJO3I=";
  };

  vendorHash = "sha256-N9uN/5Yl9YuLQFKKD/n61Wv9+f/tQIlRixTsCIiU2tQ=";

  ldflags = [
    "-s -w -X ${url}/cmd.version=${version}"
  ];

  meta = with lib; {
    description = "A mutation testing tool for Go. ";
    homepage = "https://${url}";
    license = licenses.asl20;
    maintainers = with maintainers; [ k3rn31 ];
  };
}
