# syntax=docker/dockerfile:1

# Keep in sync with /.node-version
# renovate: datasource=node-version depName=node
ARG NODE_VERSION=24.19.0

FROM node:${NODE_VERSION}-trixie-slim AS build
WORKDIR /app

ARG BACKEND_ADDRESS
ENV BACKEND_ADDRESS=$BACKEND_ADDRESS
ENV NODE_ENV=production

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates git \
 && rm -rf /var/lib/apt/lists/*

RUN corepack enable

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml .node-version ./
COPY frontend/package.json frontend/
COPY backend/package.json backend/
RUN --mount=type=cache,id=pnpm,target=/root/.local/share/pnpm/store \
    pnpm install --frozen-lockfile --filter hack4krak-frontend --ignore-scripts \
 && pnpm rebuild --filter hack4krak-frontend

COPY frontend/ frontend/

COPY .git .git/
RUN echo "GIT_COMMIT=$(git rev-parse --short HEAD)" >> frontend/.env \
 && echo "GIT_BRANCH=$(git rev-parse --abbrev-ref HEAD)" >> frontend/.env

RUN pnpm -F hack4krak-frontend --config.verifyDepsBeforeRun=false exec nuxt prepare
RUN pnpm -F hack4krak-frontend --config.verifyDepsBeforeRun=false run build

FROM debian:trixie-slim AS release
WORKDIR /app
ENV NODE_ENV=production

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

COPY --from=build /usr/local/bin/node /usr/local/bin/node
COPY --from=build /app/frontend/.output ./.output

USER nobody
EXPOSE 3000
CMD ["node", ".output/server/index.mjs"]
