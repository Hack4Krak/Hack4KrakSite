export default defineAppConfig({
  // https://ui3.nuxt.dev/getting-started/theme#design-system
  ui: {
    colors: {
      primary: 'amber',
      neutral: 'zinc',
      warning: 'orange',
    },
    theme: {
      radius: 0.25,
    },
    button: {
      slots: {
        base: 'cursor-pointer',
      },
    },
  },
})
