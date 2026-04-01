# Dependency Update Review

Compared against `master` on branch `chore/update-deps`.

Scope:
- Reviewed direct dependency and devDependency changes in `package.json` and `frontend/package.json`.
- Did not review every transitive lockfile change individually; only important downstream effects are called out.
- `backend/package.json` has no direct dependency changes.

## Biggest review points

1. `eslint` `9.39.0 -> 10.0.3` is the highest-risk tooling change. It is a breaking major and can affect custom rules, plugins, and Node support.
2. `@nuxt/test-utils` `3.20.1 -> 4.0.0` is another breaking major. It changes test initialization timing and expects Vitest 4.
3. `@playwright/test` / `playwright-core` `1.56.1 -> 1.58.2` removes some older locator patterns and the `devtools` launch option.
4. `@nuxt/content` moves from a PR preview build to the official `3.12.0`, and `@nuxtjs/mdc` moves `0.18.2 -> 0.20.2`. Markdown rendering, highlighting, and SSR content paths should be re-checked.
5. Root `zod` resolution was removed (`4.0.5`), while frontend `zod` moved to `4.3.6`. That is probably healthier, but it is worth re-running typecheck to confirm there are no version conflicts.

## Runtime dependency updates

### `@nuxt/content`
- Version: `https://pkg.pr.new/@nuxt/content@3482 -> 3.12.0`
- The old version was a PR preview build, so the exact change-by-change delta is not fully reconstructable from standard release notes.
- The move to the official `3.12.0` release should reduce risk from depending on an unpublished preview snapshot.
- Recent `3.12.0` highlights include markdown pipeline fixes, highlighting/shiki improvements, and better Cloudflare/native sqlite support.
- Repo impact: re-test markdown pages, code highlighting, and any content-backed SSR flows.
- Source: https://github.com/nuxt/content/releases/tag/v3.12.0

### `@nuxt/hints`
- Version: `1.0.0-alpha.6 -> 1.0.0-alpha.10`
- Mostly dev-experience updates in the alpha line.
- Recent releases made hints more configurable and fixed some integration issues.
- Low production risk; verify only if custom hints config is used.
- Source: https://github.com/nuxt/hints/releases/tag/v1.0.0-alpha.10

### `@nuxt/ui`
- Version: `4.1.0 -> 4.5.1`
- Adds newer Theme support, neutral color improvements, toast dedupe behavior, and general component fixes.
- Looks like a safe minor-series bump, but theme token changes can affect visual behavior if the app overrides Nuxt UI theme values.
- Repo impact: sanity-check shared components and any global theme overrides.
- Source: https://github.com/nuxt/ui/releases/tag/v4.5.1

### `@nuxtjs/mdc`
- Version: `0.18.2 -> 0.20.2`
- `0.20.x` includes feature work plus security/bug fixes, including preventing unsafe script tag rendering.
- The package also evolved around component/custom element handling and module setup ordering.
- Repo impact: this app imports MDC runtime helpers in `frontend/app/composables/useMarkdownParser.ts` and uses MDC types in tests, so markdown parsing/rendering should be verified carefully.
- Source: https://github.com/nuxt-content/mdc/releases/tag/v0.20.2

### `@nuxtjs/seo`
- Version: `3.2.2 -> 3.4.0`
- Adds better `definePageMeta()` integration for sitemap/robots flows and improves static/zero-runtime sitemap support.
- Includes i18n/stability work and some dependency cleanup around the Nuxt SEO module stack.
- Repo impact: review generated sitemap/robots behavior if this project depends on automatic SEO outputs.
- Source: https://github.com/harlan-zw/nuxt-seo/releases/tag/v3.4.0

### `@vueuse/nuxt`
- Version: `14.0.0 -> 14.2.1`
- Mostly incremental fixes, composable additions, and TS improvements.
- No major migration concerns surfaced for this range.
- Low-risk upgrade unless there are auto-import naming collisions.
- Source: https://github.com/vueuse/vueuse/tree/main/packages/nuxt

### `better-sqlite3`
- Version: `new dependency added at ^12.6.2`
- Main practical impact is native module/binary handling rather than app-level API changes.
- Useful if `@nuxt/content` or related flows need native sqlite support.
- Repo impact: CI, Docker, and local install environments may need a clean native rebuild if Node/Bun changes.
- Source: https://github.com/WiseLibs/better-sqlite3/releases/tag/v12.6.2

### `nuxt`
- Version: `4.2.0 -> 4.3.1`
- This looks like a normal minor/patch upgrade with framework fixes rather than a disruptive migration.
- Recent fixes in this range touch routing, HMR/dev-server behavior, error handling, and head/unhead transforms.
- Repo impact: re-test local dev startup, SSR pages, and any head/meta-heavy routes.
- Source: https://github.com/nuxt/nuxt/releases/tag/v4.3.1

### `zod`
- Version: `4.0.17 -> 4.3.6`
- Still within Zod 4, so this is not a major migration.
- Expect bug fixes and smaller feature/stability improvements rather than breaking schema API changes.
- Important repo note: root `resolutions.zod` was removed (`4.0.5`), so the dependency graph is no longer force-pinned to that older version.
- Repo impact: re-run typecheck and check that there is no duplicate-version weirdness in the lockfile or Nuxt modules.
- Source: https://github.com/colinhacks/zod/releases/tag/v4.3.6

## Tooling and test dependency updates

### `@antfu/eslint-config`
- Version: `7.2.0 -> 7.7.0`
- Mostly iterative lint-config updates, optional plugin additions, and bug fixes.
- Includes newer editor/plugin support and newer peer compatibility.
- Low migration risk on its own, but it should be considered together with the ESLint 10 bump.
- Source: https://github.com/antfu/eslint-config/releases/tag/v7.7.0

### `@nuxt/devtools`
- Version: `3.0.1 -> 3.2.2`
- Includes DX fixes and prompt/storage watcher improvements.
- Prompt handling changed internally, so CLI/devtools workflows are a bit different under the hood.
- Low app risk; mostly relevant to local developer experience.
- Source: https://github.com/nuxt/devtools/releases/tag/v3.2.2

### `@nuxt/eslint`
- Version: `1.10.0 -> 1.15.2`
- Mainly stability work in the generated ESLint config flow.
- One relevant fix in this range prevents the checker from running before config generation completes.
- Low migration risk; likely a net improvement for CI/local lint reliability.
- Source: https://github.com/nuxt/eslint/releases/tag/v1.15.2

### `@nuxt/test-utils`
- Version: `3.20.1 -> 4.0.0`
- This is a breaking major.
- It expects Vitest 4 and changes Nuxt test initialization timing from setup-file style behavior to `beforeAll`-style initialization.
- Common pitfall: top-level composable usage in tests may need to move into hooks or test bodies.
- Repo impact: review `frontend/vitest.config.ts`, `frontend/tests/pages/tasks/index.spec.ts`, and any tests that assume Nuxt is ready at module evaluation time.
- Source: https://github.com/nuxt/test-utils/releases/tag/v4.0.0

### `@playwright/test` and `playwright-core`
- Version: `1.56.1 -> 1.58.2`
- Includes new tooling/UI improvements, but also removes some older selector conveniences such as `_react`, `_vue`, and `:light`.
- The old `devtools` launch option was removed in favor of Chromium args.
- Repo impact: verify custom Playwright helpers/config do not depend on removed selector patterns or launch options.
- Source: https://github.com/microsoft/playwright/releases/tag/v1.58.2

### `eslint`
- Version: `9.39.0 -> 10.0.3`
- This is a breaking major and the most important tooling change in this update set.
- ESLint 10 removes more deprecated APIs, tightens rule/test behavior, and continues the flat-config-first direction.
- It also raises compatibility expectations for plugins and Node versions.
- Repo impact: run lint end-to-end and watch for plugin compatibility issues involving `@antfu/eslint-config`, `@nuxt/eslint`, and `eslint-plugin-format`.
- Source: https://github.com/eslint/eslint/releases/tag/v10.0.3

### `eslint-plugin-format`
- Version: `1.0.1 -> 2.0.1`
- Mostly bug fixes and cleanup in the newer major line.
- No large migration issue surfaced in the reviewed notes, but it should still be validated together with ESLint 10.
- Source: https://github.com/antfu/eslint-plugin-format/releases/tag/v2.0.1

### `happy-dom`
- Version: `20.0.0 -> 20.8.3`
- Patch-level fixes and DOM behavior improvements.
- Low risk, but useful because `@nuxt/test-utils` 4 expects a newer `happy-dom` floor anyway.
- Source: https://github.com/capricorn86/happy-dom/releases/tag/v20.8.3

### `typescript`
- Version: `5.9.0 -> 5.9.3`
- Patch release line only.
- Expect stability fixes rather than behavior changes.
- Low migration risk.
- Source: https://github.com/microsoft/TypeScript/releases/tag/v5.9.3

### `vitest`
- Version: `4.0.7 -> 4.0.18`
- Same major line, mostly fixes and stability updates.
- The important repo-level note is that this aligns correctly with `@nuxt/test-utils` 4.
- Low risk by itself; medium importance because it participates in the test-utils major upgrade.
- Source: https://github.com/vitest-dev/vitest/releases/tag/v4.0.18

### `vue-tsc`
- Version: `3.1.3 -> 3.2.5`
- Minor/patch improvements around TS compatibility and Vue SFC typing.
- Low migration risk; verify only through normal typecheck.
- Source: https://github.com/vuejs/language-tools

## Low-risk/data refresh updates

### `@iconify-json/*`
- Updated packages: `hugeicons`, `ic`, `logos`, `lucide`, `octicon`, `pixelarticons`
- These are icon data packages, not framework/runtime libraries.
- In practice these are data refreshes with new/corrected icons, not risky code migrations.
- Actionable impact is usually close to zero unless the app depends on a renamed or removed icon glyph.

## Repo-specific watch list

- `frontend/nuxt.config.ts`: central place where `@nuxt/content`, `@nuxtjs/mdc`, `@nuxtjs/seo`, `@nuxt/test-utils/module`, and `zod` imports are wired.
- `frontend/app/composables/useMarkdownParser.ts`: MDC runtime import surface may be the most sensitive runtime integration here.
- `frontend/tests/composables/useMarkdownParser.test.ts`: good place to catch MDC/content regressions.
- `frontend/vitest.config.ts` and `frontend/tests/pages/tasks/index.spec.ts`: good places to catch `@nuxt/test-utils` 4 migration issues.
- `frontend/playwright.config.ts` and `frontend/tests/e2e/*`: good places to catch Playwright API/selector drift.
- `frontend/app/plugins/zod.ts`: worth checking after the Zod bump and root resolution removal.

## Reply-ready summary

If you need a short response for the dependency update PR, these are the main points:

- Biggest risk items are `eslint` 10 and `@nuxt/test-utils` 4 because both can require follow-up fixes.
- `@nuxt/content` and `@nuxtjs/mdc` should be validated together because this repo has custom markdown/MDC integration.
- `@playwright/test` is not a scary upgrade, but removed selectors/options should be checked.
- `zod` is probably healthier now because the old root override is gone, but typecheck should confirm there is no dependency graph conflict.
- Iconify bumps look like normal data refreshes, not meaningful runtime changes.

## Suggested verification after merge

1. Run `bun run frontend:typecheck`.
2. Run `bun run frontend:test`.
3. Run `bun run frontend:e2e` if the branch is intended to fully validate test tooling changes.
4. Manually verify markdown/content pages and code block rendering.
5. Run `bun run frontend:lint` and look specifically for ESLint 10/plugin compatibility fallout.
