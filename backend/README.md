# Hack4Krak Backend

Technical documentation for [Hack4Krak](https://github.com/Hack4Krak)'s backend written in Rust.

## Environment Variables

- `DATABASE_URL`: The URL of the database to connect to. Example: `postgres://user:password@localhost:5432/database`
- `BACKEND_ADDRESS`: The address to bind the backend server to. Example: `127.0.0.1:8080`
- `OPENAPI_JSON_FRONTEND_PATH`: The path to write openapi json in frontend, relative to backend/ directory. Example: `../frontend/openapi/api/openapi.json`
- `RUST_LOG`: The log level for crates. [Docs](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html). Example: `hack4krak_backend=trace,actix-web=info`
- `TASKS_BASE_PATH`: Path to `tasks` directory of `TasksTemplate` repository

And more defined are defined in [env.rs](src/utils/env.rs)...

## Tasks Template

This repository parses the [TasksTemplate](https://github.com/Hack4Krak/TasksTemplate) repository to display tasks in a data-driven way.

You can customize tasks for your CTF by setting the appropriate environment variable.
For development purposes, the template repository is included as a Git submodule.

```shell
# Initialize / update your local submodule
git submodule update --init --recursive
```

```shell
# Sync submodule with remote
git submodule update --remote
```

## Running database migrator

> All of those commands should be run from the `backend/migration` directory
> You have to set the `DATABASE_URL` environment variable before.

- Generate a new migration file

    ```sh
    cargo run -- generate MIGRATION_NAME
    ```

- Rollback last applied migration

    ```sh
    cargo run -- down
    ```

- Drop all tables from the database, then reapply all migrations

    ```sh
    cargo run -- fresh
    ```

## Generating SeaORM Entities

- Requires to install the `sea-orm-cli` tool

  ```sh
  cargo install sea-orm-cli
  ```

> All following commands should be run from the `backend/` directory.
> You have to set the `DATABASE_URL` environment variable before.
 
- Generate entities from the database

    ```sh
      sea-orm-cli generate entity --with-serde both -o src/entities/ --model-extra-derives hack4krak_backend_macros::DeriveUpdatableModel
    ```
