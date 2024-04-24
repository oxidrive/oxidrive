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
    @just web/fmt
    @just server/fmt
    @just docs/fmt

act *args:
    act -s GITHUB_TOKEN=$(gh auth token) {{ args }}

test e2e="test":
    @just server/test-integration
    @just web/test
    @just e2e/rebuild && just e2e/{{ e2e }}
