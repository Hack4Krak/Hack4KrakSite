# Backend Readme

## Environment Variables

- `DATABASE_URL`: The URL of the database to connect to. Example: `postgres://user:password@localhost:5432/database`
- `BACKEND_ADDRESS`: The address to bind the backend server to. Example: `127.0.0.1:8080`
- `OPENAPI_JSON_FRONTEND_PATH`: The path to write openapi json in frontend, relative to backend/ directory. Example: `../frontend/openapi/api/openapi.json`


## Running Migrator CLI

> All of those commands should be run from the `backend/migration` directory
> You have to set the `DATABASE_URL` environment variable before.

- Generate a new migration file

    ```sh
    cargo run -- generate MIGRATION_NAME
    ```
  
- Apply all pending migrations

    ```sh
    cargo run
    ```
  
    ```sh
    cargo run -- up
    ```
  
- Apply first 10 pending migrations

    ```sh
    cargo run -- up -n 10
    ```
  
- Rollback last applied migrations

    ```sh
    cargo run -- down
    ```

- Rollback last 10 applied migrations

    ```sh
    cargo run -- down -n 10
    ```

- Drop all tables from the database, then reapply all migrations

    ```sh
    cargo run -- fresh
    ```

- Rollback all applied migrations, then reapply all migrations

    ```sh
    cargo run -- refresh
    ```

- Rollback all applied migrations

    ```sh
    cargo run -- reset
    ```

- Check the status of all migrations

    ```sh
    cargo run -- status
    ```


## Generating SeaORM Entities

- Requires to install the `sea-orm-cli` tool

  ```sh
  cargo install sea-orm-cli
  ```

> All following commands should be run from the `backend/entity/src` directory.
> You have to set the `DATABASE_URL` environment variable before.
 
- Generate entities from the database

    ```sh
    sea-orm-cli generate entity
    ```
