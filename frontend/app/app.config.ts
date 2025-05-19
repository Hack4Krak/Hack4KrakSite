export default defineAppConfig({
  // https://ui.nuxt.com/getting-started/theme#design-system
  ui: {
    colors: {
      primary: 'amber',
      neutral: 'zinc',
      warning: 'orange',
    },
    button: {
      slots: {
        base: 'cursor-pointer',
      },
    },
  },
})
