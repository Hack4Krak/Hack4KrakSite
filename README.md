# ![Hack4Krak Repository Cover](/.github/assets/banner.png)

![Issues](https://img.shields.io/github/issues-raw/Hack4Krak/Hack4KrakSite?color=f0750f&label=issues&style=for-the-badge)
![Pull Requests](https://img.shields.io/github/issues-pr-raw/Hack4Krak/Hack4KrakSite?color=f0750f&label=PRs&style=for-the-badge)
![Contributors](https://img.shields.io/github/contributors/Hack4Krak/Hack4KrakSite?color=f0750f&label=contributors&style=for-the-badge)
![Lines](https://img.shields.io/endpoint?url=https://ghloc.vercel.app/api/Hack4Krak/Hack4KrakSite/badge?style=flat&logoColor=white&color=f0750f&style=for-the-badge)
![Last Commit](https://img.shields.io/github/last-commit/Hack4Krak/Hack4KrakSite?color=f0750f&label=last%20commit&style=for-the-badge)

Welcome to the official Hack4Krak website repository!

Hack4Krak is the largest Capture The Flag (CTF) competition for high schools in Poland, bringing together the brightest young minds in cybersecurity. This event challenges participants with exciting puzzles and security challenges, fostering learning and collaboration in the world of ethical hacking.

## Requirements
- Node v22
- Pnpm
- Rust
- Docker

## Development

### 1. Download all required dependencies
```shell
pnpm install
```

### 2. Configure `.env` file

### 3. Start development servers

```shell
# Database
docker compose up

# Run each task separately
pnpm frontend:dev
pnpm backend:dev

# Or all at the same time
pnpm dev
```
## Full Test Suite

Because some backend tests are expensive and require docker (e.g. email integration tests), they are not run locally by default. To run the full test suite, use the following command:

```shell
pnpm backend:test:full
```

### Security ⚠️

Running tests with `--features=full-test-suite` may require you to run them in privileged mode. This is because the tests are run in a Docker container and require access to the host's network. If running the tests in privileged mode is not an option, we recommend using alternatives like podman instead of Docker to host test containers.


## Stats

![Repobeats analytics image](https://repobeats.axiom.co/api/embed/b2a0612285a5cfef1231975dc94e601dc5f0b983.svg "Repobeats analytics image")
