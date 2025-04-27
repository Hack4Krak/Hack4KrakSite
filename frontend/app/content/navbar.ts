// https://ui.nuxt.com/components/navigation-menu#usage

import { LANDING_SOCIALS } from '~/content/landing-socials'

export const NAVBAR_ITEMS = [
  [
    {
      label: 'Ranking',
      to: '/leaderboard',
    },
    {
      label: 'Regulamin',
      to: '/rules',
    },
    {
      label: 'FAQ',
      to: '/faq',
    },
    {
      label: 'Kontakt',
      to: 'https://discord.gg/ASPqckzEd8',
      children: LANDING_SOCIALS,
    },
  ],
]
