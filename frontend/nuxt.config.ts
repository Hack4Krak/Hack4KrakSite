// https://nuxt.com/docs/api/configuration/nuxt-config

const baseUrl = process.env.BASE_URL || 'http://localhost:3000'
const backendAddress = process.env.BACKEND_ADDRESS || 'http://localhost:8080'

export default defineNuxtConfig({
  modules: [
    '@nuxt/ui',
    '@nuxt/eslint',
    '@nuxt/test-utils/module',
    '@nuxtjs/seo',
    '@nuxt/scripts',
    '@formkit/auto-animate/nuxt',
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
      link: [{ rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }],
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1',
      meta: [
        { name: 'theme-color', content: '#ffb900' },
      ],
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
    disableNuxtPlugin: true,
    clients: {
      api: {
        baseURL: backendAddress,
        schema: '../openapi/api/openapi.json',
      },
      auth: {
        baseURL: backendAddress,
        schema: '../openapi/api/openapi.json',
      },
    },
  },
  fonts: {
    experimental: {
      processCSSVariables: true,
    },
  },
  colorMode: {
    preference: 'dark',
    storageKey: 'nuxt-color-mode-forced',
  },
  components: [
    {
      path: '~/components/',
      pathPrefix: true,
    },
    {
      path: '~/components/primitive/',
      pathPrefix: false,
    },
  ],
})
