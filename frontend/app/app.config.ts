export default defineAppConfig({
  // https://ui3.nuxt.dev/getting-started/theme#design-system
  ui: {
    accordion: {
      slots: {
        root: 'w-full',
        item: 'border-b border-(--ui-border) last:border-b-0',
        header: 'flex',
        trigger: 'group flex-1 flex items-center gap-1.5 font-medium text-sm py-3.5 focus-visible:outline-(--ui-primary) min-w-0',
        content: 'data-[state=open]:animate-[accordion-down_200ms_ease-out] data-[state=closed]:animate-[accordion-up_200ms_ease-out] overflow-hidden focus:outline-none',
        body: 'text-md font-light pb-3.5',
        leadingIcon: 'shrink-0 size-6',
        trailingIcon: 'shrink-0 size-6 text-yellow-500 ms-auto group-data-[state=open]:rotate-180 transition-transform duration-200',
        label: 'text-lg text-start break-words'
      },
    },
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

