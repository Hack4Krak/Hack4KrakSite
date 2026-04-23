import { defineUnlighthouseConfig } from 'unlighthouse/config'

export default defineUnlighthouseConfig({
  site: 'http://localhost:3000',
  scanner: {
    samples: 1,
  },
  ci: {
    budget: {
      'performance': 55,
      'accessibility': 90,
      'best-practices': 98,
      'seo': 98,
    },
  },
})
