# `backend`

Technical documentation for Hack4Krak backend.

## 🛫 Requirements

- Newest stable version of [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/) and [Docker Compose](https://docs.docker.com/compose/) (for running development
  PostgreSQL)
- `sea-orm-cli` tool (for generating database entities) - `cargo install sea-orm-cli`

## 🛖 Architecture

To learn more about code architecture and our crates refer to [ARCHITECTURE.md](ARCHITECTURE.md) file.

## 🏗️ Running

1. First, make sure all environment variables are configured properly.
   For basic development purposes you can just copy [`.env.example`](../.env.example) as `.env`

2. Initialize git submodules (more information in [Submodules](#-submodules) section)
    ```shell
    git submodule update --init --recursive
    ```

3. Then, you need to set up the database.
    ```shell
    docker compose up
    ```

4. And after that you can finally start the backend in two different ways:
    ```shell
    # From our project root directory
    bun backend:dev
    ```

    ```shell
    # From backend directory
    bun dev
    # Or if you don't want to use Node package manger
    cargo run
    ```

## 📜 Environment Variables

- `BACKEND_ADDRESS`: The address to bind the backend server to. Example: `127.0.0.1:8080`
- `RUST_LOG`: The log level for
  crates. [Docs](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html). Example:
  `hack4krak_backend=trace,actix-web=info`
- `TASKS_BASE_PATH`: Path to `tasks` directory of `TasksTemplate` repository

And more defined are defined in [env.rs](src/utils/env.rs)...

## 🚤 Submodules

This repository parses the [TasksTemplate](https://github.com/Hack4Krak/TasksTemplate) repository to display tasks in a
data-driven way.
For development purposes, the template repository is included as
a [Git submodule](https://git-scm.com/book/en/v2/Git-Tools-Submodules).

```shell
# Initialize / update your local submodules
git submodule update --init --recursive
```

```shell
# Sync submodule with remote
git submodule update --remote
```

## 🧪 Testing

```shell
# Run normal tests
bun run test
```

```shell
# Run all tests, even expensive ones that spin up docker containers
bun test:full
```

> [!TIP]
> Running full test suite may require you to run them in privileged mode.
> This is because the tests are run in a Docker container and require access to the host's network.
> If running the tests in privileged mode is not an option, we recommend using alternatives
> like [Podman](https://podman.io/) instead of Docker

## 🌸 Other commands

```shell
# Run linter and formatter
bun lint:fix
```

```shell
# Generate openapi specification from code
bun openapi-cli -- write
```

## 👣 Managing database

We use [SeaORM](https://www.sea-ql.org/SeaORM) as our ORM. You can use it's CLI to generate entities, manage migrations
and more.

- Generate entities from the database
  ```sh
  # You should run this command from `backend/` directory
  sea-orm-cli generate entity --with-serde both -o src/entities/ --model-extra-derives hack4krak_macros::DeriveUpdatableModel
  ```

> [!NOTE]  
> All the following commands should be run from the `backend/migration/` directory
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
