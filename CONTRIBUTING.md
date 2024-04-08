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

The server should be listening on http://0.0.0.0:4000, while the web application should be available on http://0.0.0.0:8080.

[Go]: https://go.dev
[Rust]: https://rust-lang.org
[Nix]: https://nixos.org
[Just]: https://github.com/casey/just
