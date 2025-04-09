const backendAddress = process.env.BACKEND_ADDRESS || 'http://localhost:8080'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  // Nuxt-specific configuration

  modules: [
    '@nuxt/ui',
    '@nuxt/eslint',
    '@nuxt/image',
    '@nuxt/test-utils/module',
    '@nuxtjs/seo',
    '@nuxtjs/mdc',
    '@formkit/auto-animate/nuxt',
    'nuxt-open-fetch-x',
    'dayjs-nuxt',
    '@compodium/nuxt',
  ],
  experimental: {
    componentIslands: true,
    typedPages: true,
    sharedPrerenderData: true,
  },
  typescript: { typeCheck: true },
  devtools: { enabled: true },
  css: ['~/assets/css/main.css'],
  compatibilityDate: '2025-04-09',
  future: {
    compatibilityVersion: 4,
  },

  // App configuration

  routeRules: {
    '/tasks/description/**': { swr: true },
    '/faq': { prerender: true },
    '/rules': { prerender: true },
    '/': { prerender: true },
  },
  app: {
    head: {
      link: [
        { rel: 'icon', href: 'favicon.ico', sizes: '32x32' },
        { rel: 'icon', href: 'favicon.svg', sizes: 'any', type: 'image/svg+xml' },
      ],
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1',
      meta: [
        { name: 'theme-color', content: '#ffb900' },
        { name: 'lang', content: 'pl' },
      ],
    },
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

  // Module configuration

  // https://eslint.nuxt.com/
  eslint: {
    config: {
      standalone: false,
    },
  },
  // https://nuxt-open-fetch.vercel.app/setup/configuration (we are using patched fork - nuxt-open-fetch-x)
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
  // https://ui.nuxt.com/getting-started/fonts
  fonts: {
    experimental: {
      processCSSVariables: true,
    },
  },
  // https://ui.nuxt.com/getting-started/color-mode/nuxt
  colorMode: {
    preference: 'dark',
    storageKey: 'nuxt-color-mode-forced',
  },
  // https://nuxt.com/modules/mdc
  mdc: {
    remarkPlugins: {
      'remark-math': {},
    },
    rehypePlugins: {
      'rehype-mathjax': {},
    },
  },
  // https://nuxt.com/modules/dayjs
  dayjs: {
    locales: ['en', 'pl'],
    plugins: ['duration', 'timezone', 'isBetween'],
    defaultLocale: 'pl',
    defaultTimezone: 'Poland/Warsaw',
  },
  // https://nuxtseo.com/docs/schema-org/getting-started/introduction
  schemaOrg: {
    enabled: false,
  },
  // https://nuxtseo.com/docs/site-config/guides/setting-site-config
  site: {
    // Use NUXT_SITE_NAME to override
    url: 'https://hack4krak.pl',
    name: 'Hack4Krak CTF',
    description: 'Krakowski CTF dla wszystkich uczniów szkół średnich! przetestuj się i zawalcz o ciekawe nagrody biorąc udział w wydarzeniu! Nie zwlekaj i zgłoś swoją drużynę już dziś!',
    defaultLocale: 'pl',
  },
})
