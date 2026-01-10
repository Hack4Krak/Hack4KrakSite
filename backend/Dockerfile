FROM lukemathwalker/cargo-chef:latest-rust-1.91.1 AS chef
WORKDIR /app

FROM chef AS planner
COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/macros/Cargo.toml ./macros/
COPY backend/migration/Cargo.toml ./migration/
COPY backend/src/lib.rs backend/src/main.rs ./src/
COPY backend/macros/src/lib.rs ./macros/src/
COPY backend/migration/src/lib.rs ./migration/src/
# Remove openapi-cli from workspace members since it's excluded by .dockerignore
RUN sed -i 's/members = \[.*\]/members = [".", "macros", "migration"]/' Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY backend .
RUN cargo build --release --bin hack4krak-backend

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y libssl3 ca-certificates git

# Download TasksTemplate or other tasks repository
ARG GIT_USER=Hack4Krak
ARG GIT_REPO=TasksTemplate
ARG GIT_ACCESS_TOKEN=token
ARG BRANCH=master

ADD https://$GIT_ACCESS_TOKEN:x-oauth-basic@api.github.com/repos/$GIT_USER/$GIT_REPO/git/refs/heads/$BRANCH version.json
RUN git clone -b $BRANCH https://$GIT_USER:$GIT_ACCESS_TOKEN@github.com/$GIT_USER/$GIT_REPO.git TasksRepo/

ENV TASKS_BASE_PATH=TasksRepo/

COPY --from=builder /app/target/release/hack4krak-backend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/hack4krak-backend"]
