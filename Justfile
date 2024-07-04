default:
    @echo $'Available commands:\n'
    @just --list --list-heading $'Global:\n' --unsorted | grep -v '  default'
    @echo ""
    @just -f web/Justfile --list --list-heading $'Web:\n'  --unsorted| grep -v '  default'
    @echo ""
    @just -f server/Justfile --list --list-heading $'Server:\n'  --unsorted | grep -v '  default'
    @echo ""
    @just -f e2e/Justfile --list --list-heading $'E2E Tests:\n'  --unsorted | grep -v '  default'

build:
    @just web/build
    @just server/build

fmt:
    just web/fmt
    just server/fmt
    just docs/fmt
    just e2e/fmt

lint:
    @just web/lint
    @just server/lint
    @just docs/lint
    @just e2e/lint

act *args:
    act -s GITHUB_TOKEN=$(gh auth token) {{ args }}

test e2e="test":
    @just server/test
    @just web/test
    @just e2e/{{ e2e }}

release:
    rm -rf release && mkdir -p release/staging
    @just server/openapi
    @just web/build
    @just server/build
    cp ./server/bin/oxidrive release/staging/
    cp -r ./web/build release/staging/assets
    tar -cvzf release/oxidrive.tar.gz -C ./release/staging .
    rm -rf release/staging
