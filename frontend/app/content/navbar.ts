// https://ui.nuxt.com/components/navigation-menu#usage

import { LANDING_SOCIALS } from '~/content/landing-socials'

export const NAVBAR_ITEMS = [
  [
    {
      slot: 'logo' as const,
      to: '/',
    },
  ],
  [
    {
      label: 'Ranking',
      to: '/leaderboard',
    },
    {
      label: 'Zadania',
      to: '/tasks',
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
      children: LANDING_SOCIALS,
    },
  ],
  [
    {
      slot: 'button' as const,
      to: '/login',
    },
  ],
]
