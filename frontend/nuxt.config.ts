import dotenv from 'dotenv'
import { defineOrganization } from 'nuxt-schema-org/schema'
import { shikiLangNames, shikiTheme } from './app/utils/shiki'
import { getNodeTransforms } from './app/utils/vite-node-transforms'

dotenv.config({ path: '../.env' })

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
    'nuxt-echarts',
    'nuxt-open-fetch',
    'nuxt-qrcode',
    '@norbiros/nuxt-auto-form',
    'dayjs-nuxt',
    '@compodium/nuxt',
    '@vueuse/nuxt',
    '@nuxt/hints',
  ],
  echarts: {
    charts: ['LineChart'],
    components: [
      'TooltipComponent',
      'LegendComponent',
      'GridComponent',
    ],
  },
  experimental: {
    componentIslands: true,
    typedPages: true,
    sharedPrerenderData: true,
  },
  typescript: { typeCheck: true },
  devtools: { enabled: true },
  css: ['~/assets/css/main.css'],
  compatibilityDate: '2025-07-16',

  vite: {
    vue: {
      template: {
        compilerOptions: {
          nodeTransforms: getNodeTransforms(),
        },
      },
    },
  },

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
    '/panel/**': { appLayout: 'panel' },
    '/account/**': { appLayout: 'centered' },
    '/tasks/description/**': { swr: true },
    // For now, we have to manually list all docs to prerender them
    // due to some issues with Nitro crawling dynamic routes
    '/docs/**': { appLayout: 'static-page', prerender: true },
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
    openAPITS: {
      rootTypes: true,
    },
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
    identity: defineOrganization({
      name: 'Hack4Krak',
      description: 'Inicjatywa młodych liderów cyberbezpieczeństwa, których połączyła pasja do technologii, wyzwań i ciągłego rozwoju. Tworzymy wydarzenia, które uczą, inspirują i integrują młodzież zainteresowaną światem IT i bezpieczeństwa cyfrowego.',
      url: 'https://hack4krak.pl',
      logo: '/img/logo.png',

      email: 'kontakt@hack4krak.pl',
      foundingDate: '2024-10-14',
      numberOfEmployees: {
        '@type': 'QuantitativeValue',
        'minValue': 10,
        'maxValue': 25,
      },

      sameAs: [
        'https://www.instagram.com/hack4krak/',
        'https://www.linkedin.com/company/hack4krak/',
        'https://www.facebook.com/profile.php?id=61573226541589',
        'https://github.com/Hack4Krak',
      ],
    }),
  },
  linkChecker: {
    runOnBuild: false,
  },
  robots: {
    disallow: ['/admin', '/panel', '/account'],
  },
  // https://nuxtseo.com/docs/site-config/guides/setting-site-config
  site: {
    // Use NUXT_SITE_NAME to override
    url: 'https://hack4krak.pl',
    name: 'Hack4Krak CTF',
    description: 'Hack4Krak to największy w Polsce CTF dla uczniów szkół średnich! Sprawdź swoje umiejętności w cyberbezpieczeństwie, zgłoś swoją drużynę i rywalizuj o nagrody!',
    defaultLocale: 'pl',
  },
  // https://content.nuxt.com/docs/getting-started
  content: {
    experimental: { sqliteConnector: 'native' },
    build: {
      markdown: {
        highlight: {
          theme: shikiTheme,
          langs: shikiLangNames,
        },
      },
    },
  },
  // https://ui.nuxt.com/docs/getting-started/installation/nuxt#options
  ui: {
    experimental: {
      componentDetection: true,
    },
  },
})
