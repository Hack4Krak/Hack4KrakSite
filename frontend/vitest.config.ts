import { defineVitestConfig } from '@nuxt/test-utils/config'
import { configDefaults } from 'vitest/config'

export default defineVitestConfig({
  test: {
    environment: 'nuxt',
    exclude: [...configDefaults.exclude, 'tests/e2e/**/*'],
    setupFiles: ['../tests/setup.ts'],
    environmentOptions: {
      nuxt: {
        overrides: {
          // @ts-expect-error Vitest doesn't recognize config options from modules
          ogImage: { enabled: false },
        },
      },
    },
  },
})
