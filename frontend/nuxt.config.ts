// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: [
    '@nuxt/ui',
    '@nuxt/eslint',
    '@nuxt/test-utils/module',
    '@nuxtjs/seo',
    '@nuxt/scripts',
    '@formkit/auto-animate/nuxt',
    '@nuxt/content',
    'nuxt-open-fetch',
  ],
  devtools: { enabled: true },
  css: ['~/assets/css/main.css'],

  future: {
    compatibilityVersion: 4,
  },

  compatibilityDate: '2024-11-27',

  eslint: {
    config: {
      standalone: false,
    },
  },

  openFetch: {
    clients: {
      api: {
        baseURL: process.env.BACKEND_ADDRESS || 'http://localhost:8080/',
        schema: '../openapi/api/openapi.json',
      },
    },
  },
})
