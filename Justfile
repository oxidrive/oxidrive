set dotenv-load

default:
    @just --list --unsorted | grep -v '  default'

add-lib name:
    cargo new --lib ./lib/{{ name }} --vcs=none --name oxidrive-{{ name }}

add-app name:
    cargo new --lib ./app/{{ name }} --vcs=none --name oxidrive-{{ name }}

add-bin name:
    cargo new --bin ./bin/{{ name }} --vcs=none --name oxidrive-{{ name }}

add-tool name:
    cargo new --bin ./tools/{{ name }} --vcs=none --name {{ name }}

watch *args:
    bacon run-server -- {{ args }}

run *args:
    cargo run -p oxidrive -- server {{ args }}

test *args:
    cargo nextest run {{ args }}

fmt:
    cargo fmt

clippy:
    cargo clippy --fix --workspace --all-targets --all-features --fix --allow-dirty --allow-staged

db-reset:
    docker compose down -v && docker compose up -d && just migrations/run

oxidrive *args:
    cargo run -p oxidrive -- {{ args }}

generate-openapi-schema:
    cargo run --bin generate-openapi

generate-openapi-types *args:
    [ -d ./node_modules ] || npm ci
    npx openapi-typescript openapi.json -o web/src/lib/openapi.ts --root-types {{ args }}

generate-openapi: generate-openapi-schema generate-openapi-types

