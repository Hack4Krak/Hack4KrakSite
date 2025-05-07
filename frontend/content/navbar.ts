// https://ui.nuxt.com/components/navigation-menu#usage

import type { NavigationMenuItem } from '#ui/components/NavigationMenu.vue'
import { LANDING_SOCIALS } from '~~/content/landing-socials'

export const NAVBAR_ITEMS: NavigationMenuItem[] = [
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
      prefetch: false,
    },
    {
      label: 'Zadania',
      to: '/tasks',
    },
    {
      label: 'Regulamin',
      to: '/docs/rules',
    },
    {
      label: 'FAQ',
      to: '/docs/faq',
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
