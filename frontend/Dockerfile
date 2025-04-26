FROM imbios/bun-node:latest AS base
WORKDIR /app

FROM base as install
COPY package.json bun.lock .
COPY frontend/package.json frontend/
COPY backend/package.json backend/

RUN bun install --frozen-lockfile --filter hack4krak-frontend

FROM base as prerelease

COPY --from=install /app/node_modules node_modules
COPY frontend/ .

ENV NODE_ENV=production
RUN bun run build

FROM base AS release
COPY --from=prerelease /app/node_modules node_modules
COPY --from=prerelease /app/.output .output
COPY --from=prerelease /app/package.json .

EXPOSE 3000
CMD ["node", ".output/server/index.mjs"]