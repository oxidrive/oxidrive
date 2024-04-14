# Development
1. Spin up the PostgreSQL container in the `docker-compose.yml` in the root of the project.
2. Copy the `.env.sample`
2. `just build` and `just run`.
3. Enjoy the coding session, and thank you for the contribution!

## PostgreSQL
The Postgres db can be inspected using psql

```sh
just psql
```

If you want to craete a new migration you need [migrate](https://github.com/golang-migrate/migrate), the Nix flake already provides a shell that contains it

```sh
just create_migration my_shiny_migration
```

Once you have your migration, you only have to edit the SQL files created under the `migrations` folder.

If you don't use Nix ([you should give it a shot](https://nixos.org/learn/)), install migrate with the package manager you normally use, but keep in mind that we are using the v4.

Migrations are embedded in the binary and run every time the server is started.
