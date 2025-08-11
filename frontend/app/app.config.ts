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
    input: {
      slots: { root: 'w-full' },
    },
    select: {
      slots: { base: 'w-full' },
    },
  },
  autoForm: {
    submit: {
      props: { label: 'Wyślij', class: 'w-full flex justify-center' },
    },
  },
})
