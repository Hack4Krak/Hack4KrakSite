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
      ariaLabel: 'Przejdź do rankingu',
      prefetch: false,
    },
    {
      label: 'Zadania',
      ariaLabel: 'Przejdź do zadań',
      to: '/tasks',
    },
    {
      label: 'Regulamin',
      ariaLabel: 'Przejdź do regulaminu',
      to: '/docs/rules',
    },
    {
      label: 'FAQ',
      ariaLabel: 'Przejdź do najczęściej zadawanych pytań',
      to: '/docs/faq',
    },
    {
      label: 'Kontakt',
      ariaLabel: 'Przejdź do informacji kontaktowych',
      children: LANDING_SOCIALS,
    },
  ],
  [
    {
      slot: 'button' as const,
      ariaLabel: 'Przejdź panelu logowania',
      to: '/login',
    },
  ],
]
