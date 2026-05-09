export default defineAppConfig({
  // https://ui.nuxt.com/getting-started/theme#design-system
  ui: {
    button: {
      slots: {
        base: 'cursor-pointer rounded-none',
      },
    },
    formField: {
      slots: {
        label: 'text-xs uppercase tracking-wider text-muted font-bold',
      },
    },
    badge: {
      slots: {
        base: 'rounded-none border-2 font-medium normal-case tracking-wide ring-0',
      },
      defaultVariants: {
        size: 'md',
        color: 'primary',
        variant: 'outline',
      },
      variants: {
        size: {
          sm: {
            base: 'px-2.5 py-0.5 text-xs',
          },
          md: {
            base: 'text-md px-4 py-2',
          },
        },
      },
      compoundVariants: [
        {
          color: 'primary',
          variant: 'outline',
          class: 'border-primary bg-primary/10 text-primary ring-0',
        },
        {
          color: 'success',
          variant: 'outline',
          class: 'border-success bg-success/10 text-success ring-0',
        },
        {
          color: 'neutral',
          variant: 'outline',
          class: 'border-muted bg-muted/10 text-muted ring-0',
        },
      ],
    },
    input: {
      slots: { root: 'w-full', base: 'rounded-none' },
      variants: {
        variant: {
          outline: 'ring-default',
        },
      },
    },
    select: {
      slots: { base: 'w-full rounded-none' },
      variants: {
        variant: {
          outline: 'ring-default',
        },
      },
    },
  },
  // https://nuxt-auto-form.norbiros.dev/customization/config
  autoForm: {
    submit: {
      props: { label: 'Wyślij', class: 'w-full flex justify-center' },
    },
    modal: {
      submitLabel: 'Potwierdź',
      closeLabel: 'Anuluj',
    },
  },
})
