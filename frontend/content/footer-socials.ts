import { LANDING_SOCIALS } from '~~/content/landing-socials'

export const FOOTER_SOCIALS = [
  ...LANDING_SOCIALS,
  {
    label: 'Regulamin',
    to: '/docs/rules',
    icon: null,
    target: undefined,
  },
  {
    label: 'Polityka Prywatności',
    to: '/docs/privacy-policy',
    icon: null,
  },
]
