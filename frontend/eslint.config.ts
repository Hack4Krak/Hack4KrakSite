import antfu from '@antfu/eslint-config'
import withNuxt from './.nuxt/eslint.config.mjs'

export default withNuxt(
  antfu({
    stylistic: true,
    vue: true,
    typescript: true,
    formatters: true,
    rules: {
      'node/prefer-global/process': ['off'],
      'style/brace-style': ['warn', '1tbs'],
    },
    ignores: [
      'openapi/api/openapi.json',
    ],
  }),
)
