FROM imbios/bun-node:1.3.7-current-alpine AS base
RUN apk add --no-cache git
WORKDIR /app

ARG BACKEND_ADDRESS
ENV BACKEND_ADDRESS=$BACKEND_ADDRESS

FROM base AS prerelease
COPY package.json bun.lock ./
COPY frontend/package.json frontend/
COPY backend/package.json backend/
COPY bunfig.toml .

RUN bun install --frozen-lockfile --filter hack4krak-frontend

COPY frontend/ .

# For now the easiest way to get git info into the image is to copy the .git directory
COPY .git .git/
RUN export GIT_COMMIT=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown") && \
    export GIT_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown") && \
    echo "GIT_COMMIT=$GIT_COMMIT" >> .env && \
    echo "GIT_BRANCH=$GIT_BRANCH" >> .env

ENV NODE_ENV=production
RUN bun run build

FROM base AS release
COPY --from=prerelease /app/node_modules node_modules
COPY --from=prerelease /app/.output .output
COPY --from=prerelease /app/package.json .

EXPOSE 3000
CMD ["node", ".output/server/index.mjs"]