FROM oven/bun:alpine AS base
WORKDIR /usr/src/app

RUN apk add --no-cache git

ARG BACKEND_ADDRESS
ENV BACKEND_ADDRESS=$BACKEND_ADDRESS

FROM base as prerelease
COPY package.json bun.lock ./
COPY frontend/package.json frontend/
COPY backend/package.json backend/

RUN bun install --frozen-lockfile --filter hack4krak-frontend

COPY frontend/ .

# THE BIG DOCKER WHALE MUST BE FED — COMMIT HASH REQUIRED FOR FOOTER.
COPY .git ./

RUN bun run build

FROM base AS release
COPY --from=prerelease /app/node_modules node_modules
COPY --from=prerelease /app/.output .output
COPY --from=prerelease /app/package.json .

EXPOSE 3000
CMD ["bun", "run", ".output/server/index.mjs", "--bun"]