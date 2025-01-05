set dotenv-load

default:
    @just --list --unsorted | grep -v '  default'

add-lib name:
    cargo new --lib ./lib/{{ name }} --vcs=none --name oxidrive-{{ name }}

add-app name:
    cargo new --lib ./app/{{ name }} --vcs=none --name oxidrive-{{ name }}

add-bin name:
    cargo new --bin ./bin/{{ name }} --vcs=none --name oxidrive-{{ name }}

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
