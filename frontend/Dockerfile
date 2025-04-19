# use the official Bun image
# see all versions at https://hub.docker.com/r/oven/bun/tags
FROM node:22-slim AS base

# install dependencies into temp directory
# this will cache them and speed up future builds
FROM oven/bun:1.2-alpine AS install
WORKDIR /usr/src/app

# install with --production (exclude devDependencies)
RUN mkdir -p /temp/prod
COPY package.json bun.lock /temp/prod/
COPY frontend/package.json /temp/prod/frontend/
COPY backend/package.json /temp/prod/backend/
COPY patches/ /temp/prod/patches

WORKDIR /temp/prod
RUN bun install --frozen-lockfile --filter hack4krak-frontend

# copy node_modules from temp directory
# then copy all (non-ignored) project files into the image
FROM base AS prerelease
WORKDIR /usr/src/app

COPY --from=install /temp/prod/node_modules node_modules
COPY frontend/ .

ENV NODE_ENV=production
RUN npx nuxt prepare
RUN npx nuxt build

## copy production dependencies and source code into final image
FROM base AS release
COPY --from=prerelease /usr/src/app/node_modules node_modules
COPY --from=prerelease /usr/src/app/.output .output
COPY --from=prerelease /usr/src/app/package.json .

# run the app
EXPOSE 3000/tcp
ENTRYPOINT [ "node", ".output/server/index.mjs" ]