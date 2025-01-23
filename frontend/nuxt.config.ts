// https://nuxt.com/docs/api/configuration/nuxt-config

const baseUrl = process.env.BASE_URL || 'http://localhost:3000'

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
    '@nuxt/image',
  ],

  site: {
    url: baseUrl,
    name: 'Hack4Krak CTF',
    description: 'Krakowski CTF dla uczniów szkół średnich! Zgłoś swoją drużynę, weź udział w wydarzeniu i wygraj atrakcyjne nagrody!',
    defaultLocale: 'pl',
  },

  runtimeConfig: {
    public: {
      baseUrl,
    },
  },

  app: {
    head: {
      link: [{ rel: 'icon', type: 'image/x-icon', href: '/favicon-light.ico' }],
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1',
    },
  },

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
  fonts: {
    experimental: {
      processCSSVariables: true,
    },
  },
})
