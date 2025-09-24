const backendAddress = process.env.BACKEND_ADDRESS || 'http://localhost:8080'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  // Nuxt-specific configuration

  modules: [
    '@nuxt/ui',
    '@nuxt/eslint',
    '@nuxt/image',
    '@nuxt/content',
    '@nuxt/test-utils/module',
    '@nuxtjs/seo',
    '@nuxtjs/mdc',
    '@formkit/auto-animate/nuxt',
    'nuxt-open-fetch',
    'nuxt-qrcode',
    '@norbiros/nuxt-auto-form',
    'dayjs-nuxt',
    '@compodium/nuxt',
    '@vueuse/nuxt',
  ],
  experimental: {
    componentIslands: true,
    typedPages: true,
    sharedPrerenderData: true,
  },
  typescript: { typeCheck: true },
  devtools: { enabled: true },
  css: ['~/assets/css/main.css'],
  compatibilityDate: '2025-07-16',
  imports: {
    presets: [
      {
        from: 'zod',
        imports: [
          { as: 'z', name: '*' },
          {
            name: 'infer',
            as: 'zInfer',
            type: true,
          },
        ],
      },
    ],
  },

  // App configuration

  runtimeConfig: {
    public: {
      gitCommit: process.env.GIT_COMMIT,
      gitBranch: process.env.GIT_BRANCH,
    },
  },

  routeRules: {
    '/tasks/description/**': { swr: true },
    // For now, we have to manually list all docs to prerender them
    // due to some issues with Nitro crawling dynamic routes
    '/docs/**': { prerender: true },
    '/docs/faq': { prerender: true },
    '/docs/rules': { prerender: true },
    '/docs/privacy-policy': { prerender: true },
    '/': { prerender: true },
    '/about_us': { prerender: true },
    '/faq': { redirect: '/docs/faq' },
    '/rules': { redirect: '/docs/rules' },
  },

  app: {
    head: {
      link: [
        { rel: 'icon', href: '/favicon-light.ico', sizes: '48x48' },
      ],
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1',
      htmlAttrs: {
        class: 'dark',
        lang: 'pl',
      },
      meta: [
        { name: 'theme-color', content: '#ffb900' },
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
  // https://nuxt-open-fetch.norbiros.dev/setup/configuration
  openFetch: {
    disableNuxtPlugin: true,
    clients: {
      api: {
        baseURL: backendAddress,
        schema: 'openapi/api/openapi.json',
      },
      auth: {
        baseURL: backendAddress,
        schema: 'openapi/api/openapi.json',
      },
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
      'rehype-katex': {
        options: {
          output: 'mathml',
        },
      },
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
  linkChecker: {
    runOnBuild: false,
  },
  robots: {
    disallow: ['/admin'],
  },
  // https://nuxtseo.com/docs/site-config/guides/setting-site-config
  site: {
    // Use NUXT_SITE_NAME to override
    url: 'https://hack4krak.pl',
    name: 'Hack4Krak CTF',
    description: 'Krakowski CTF dla wszystkich uczniów szkół średnich. Sprawdź swoje umiejętności i zawalcz o ciekawe nagrody biorąc udział w wydarzeniu! Nie zwlekaj i zgłoś swoją drużynę już dziś!',
    defaultLocale: 'pl',
  },
})
