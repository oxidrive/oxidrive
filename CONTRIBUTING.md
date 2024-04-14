# Contribution Guide

## Required Tools

Oxidrive is written in the [Go] and [Rust] programming languages, so you'll need their respective toolchains installed. You can find the required versions in the [go.mod](server/go.mod) file for the backend service, and in the [rust-toolchain.toml](web/rust-toolchain.toml) for the frontend application.

As for ancillary tools, a complete and up-to-date list can be found [here](flake.nix#L25).

A notable entry is [Just], our standard command runner. You'll see it used a lot in this guide to compile the application, run formatting tools, and other common development tasks.

If you use [Nix], everything should be automatically configured for you by simply activating the development shell with `nix develop`!

## Supporting services

Oxidrive only depends on its database, [PostgreSQL]. You can install it and run it locally using your system's package manager, but a simple way to get started is using [Docker] and spinning up a containerized version of Postgres.

Assuming you have Docker running and [Docker Compose] installed, running `docker compose up -d` should start Postgres in the background, listening on port `5432` and preconfigured with the standard triple of `user: oxidrive, password: oxidrive, database: oxidrive`.

## Start locally

With Postgres up and running, you are ready to start Oxidrive on your machine. In two separate shells, run the following commands:

```bash
# start the backend server
just server/watch
```
```bash
# start the frontend app
just web/watch
```

The server should be listening on http://127.0.0.1:4000, while the web application should be available on http://127.0.0.1:8080.

## Pre-commit
Each commit is run against a list of checks defined using [pre-commit](https://pre-commit.com/). Before contribuiting to this project, be sure to install them.

### Nix
If you use Nix for everything you are ready to go, [git-hooks.nix](https://github.com/cachix/git-hooks.nix/tree/master) and [flake.parts](https://flake.parts/) take care of everything.

### Not nix
Pre-commit directives are loaded from the `.pre-commit-config.json`, install them using `pre-commit install`.

## End-to-End Tests

Oxidrive inclues a suite of UI tests that verify some of the core UX flows from the end-user's pespective. The test suite is implemented with [Playwright] and is located in the [e2e](e2e) folder. It requiers [NodeJS] 20 and related NPM CLI installed.

To set Playwright up the first time, run `just e2e/setup`. This will install the required NPM packages and download the browsers that will execute the tests.

> [!WARNING]
> Nix users cannot use the regular Playwright-managed browsers. The project's [flake.nix](flake.nix) should install the correct Nix package, but [at the moment](https://github.com/NixOS/nixpkgs/pull/298944) only Chromium is provided.
> Run `just e2e/chromium` instead of the regular command, or select only `chromium` if running `just e2e/ui`. `Mobile Chromium` is also supported.

Running `just e2e/test` from the root of the repository will:
- start a [Docker Compose](e2e/docker-compose.yml) stack with a release build of Oxidrive and a Postgres database
- run the test suite against [all supported browsers](e2e/playwright.config.ts#L37) (both desktop and mobile).

A nice GUI is also provided by running `just e2e/ui`, which also allows inspecting the application as tests run.

> [!NOTE]
> When run locally Playwright will attempt to reuse an existing running instance of Oxidrive to run tests against. This is good if you're working on tests, because it avoids rebuilding the app image for nothing. If you changed the source code, however, you need to trigger a rebuild to see the changes in action. This can be achieved by running `just e2e/rebuild && just e2e/test`.

[Go]: https://go.dev
[Rust]: https://rust-lang.org
[Nix]: https://nixos.org
[PostgreSQL]: https://postgresql.org
[Docker]: https://docker.com
[Docker Compose]: https://docs.docker.com/compose/
[Just]: https://github.com/casey/just
[Playwright]: https://playwright.dev/
[NodeJS]: https://nodejs.org
