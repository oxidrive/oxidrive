set dotenv-load := true

default:
    @just --list

# === BUILD === #

[group('build')]
build *args:
    cargo build --release {{ args }}

[group('build')]
build-debug *args:
    cargo build {{ args }}

[group('build')]
[group('node')]
build-ui *args:
    npm run build --workspace app/ui -- {{ args }}

[group('build')]
[group('openapi')]
openapi-generate: openapi-generate-schema openapi-generate-types

[group('build')]
[group('openapi')]
openapi-generate-schema:
    cargo run --bin generate-openapi

[group('build')]
[group('openapi')]
openapi-generate-types *args:
    npx openapi-typescript openapi.json -o app/ui/src/lib/openapi.ts --root-types {{ args }}

# === RUN === #

[group('run')]
run *args: _npm_install
    cargo run --bin oxidrive {{ args }} server

[group('run')]
run-dev *args: _npm_install
    cargo run --bin oxidrive --features vite-dev-server {{ args }} server

[group('run')]
watch *args: _npm_install
    bacon server -- {{ args }}

[group('run')]
watch-dev *args: _npm_install
    bacon dev-server -- {{ args }}

# === CHECK === #

[group('check')]
check: check-rust check-node check-cedar check-just

[group('check')]
[group('rust')]
check-rust:
    cargo check --all-targets --all-features

[group('check')]
[group('node')]
check-node:
    npm run check --workspace app/ui

[group('cedar')]
[group('check')]
check-cedar:
    #!/usr/bin/env sh
       schema=$(mktemp XXXXX.cedarschema --tmpdir)
       find . -name "*.cedarschema" -exec sh -c "cat {} >> $schema" \;

       for file in $(find . -name "*.cedar"); do
         cedar validate -s "$schema" -p "$file" --deny-warnings
       done

[group('check')]
[group('just')]
check-just:
    #!/usr/bin/env sh
       for file in $(find . -name "Justfile"); do
         just --unstable --fmt --check --justfile "$file"
       done

# === FORMAT === #

alias fmt := format

[group('format')]
format: format-rust format-node format-cedar format-just

[group('format')]
[group('rust')]
format-rust *args:
    cargo fmt --all {{ args }}

[group('format')]
[group('node')]
format-node: _npm_install
    npm run format --workspace app/ui

[group('cedar')]
[group('format')]
format-cedar mode="write":
    for file in $(find . -name "*.cedar"); do \
      cedar format -p "$file" --{{ mode }}; \
    done

[group('format')]
[group('just')]
format-just:
    #!/usr/bin/env sh
       for file in $(find . -name "Justfile"); do
         just --unstable --fmt --justfile "$file"
       done

# === LINT === #

[group('lint')]
lint: lint-rust lint-node

[group('lint')]
[group('rust')]
lint-rust:
    cargo clippy --workspace --all-targets --all-features

[group('lint')]
[group('node')]
lint-node *args: _npm_install
    npm run lint --workspace app/ui -- {{ args }}

[group('lint')]
lint-fix: lint-fix-rust lint-fix-node

[group('lint')]
[group('rust')]
lint-fix-rust:
    cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged

[group('lint')]
[group('node')]
lint-fix-node: _npm_install
    npm run lint:fix --workspace app/ui

# === TEST === #

[group('test')]
test: test-rust test-node

[group('test')]
test-full: test-rust-full test-node

[group('rust')]
[group('test')]
test-rust *args:
    cargo nextest run --all-targets --all-features {{ args }}

[group('rust')]
[group('test')]
test-rust-full *args:
    cargo nextest run --all-targets --all-features --profile=full {{ args }}

[group('node')]
[group('test')]
test-node *args: _npm_install
    npm run test:unit --workspace app/ui -- --run {{ args }}

# === SCAFFOLD === #

[group('scaffold')]
add-lib name:
    cargo new --lib ./lib/{{ name }} --vcs=none --name oxidrive-{{ name }}

[group('scaffold')]
add-app name:
    cargo new --lib ./app/{{ name }} --vcs=none --name oxidrive-{{ name }}

[group('scaffold')]
add-bin name:
    cargo new --bin ./bin/{{ name }} --vcs=none --name oxidrive-{{ name }}

[group('scaffold')]
add-tool name:
    cargo new --bin ./tools/{{ name }} --vcs=none --name {{ name }}

# === MISC == #

[group('db')]
db-reset:
    docker compose down -v && docker compose up -d && just migrations/run

[group('cli')]
oxidrive *args:
    cargo run -p oxidrive -- {{ args }}

_npm_install:
    @[ -d "node_modules" ] || npm ci
