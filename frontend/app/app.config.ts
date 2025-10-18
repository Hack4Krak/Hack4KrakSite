export default defineAppConfig({
  // https://ui.nuxt.com/getting-started/theme#design-system
  ui: {
    button: {
      slots: {
        base: 'cursor-pointer',
      },
    },
    badge: {
      slots: {
        base: 'font-normal',
      },
      defaultVariants: {
        size: 'md',
        color: 'primary',
        variant: 'outline',
      },
      variants: {
        size: {
          md: {
            base: 'text-md px-4 py-2',
          },
        },
      },
      compoundVariants: {
        color: 'primary',
        variant: 'outline',
        class: 'text-default ring-2 ring-primary',
      },
    },
    input: {
      slots: { root: 'w-full' },
      variants: {
        variant: {
          outline: 'ring-default',
        },
      },
    },
    select: {
      slots: { base: 'w-full' },
      variants: {
        variant: {
          outline: 'ring-default',
        },
      },
    },
  },
  autoForm: {
    submit: {
      props: { label: 'Wyślij', class: 'w-full flex justify-center' },
    },
  },
})
