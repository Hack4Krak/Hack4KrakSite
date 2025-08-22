// https://ui.nuxt.com/components/navigation-menu#usage

import type { NavigationMenuItem } from '#ui/components/NavigationMenu.vue'
import { LANDING_SOCIALS } from '~~/content/landing-socials'

export const NAVBAR_ITEMS: NavigationMenuItem[] = [
  [
    {
      label: 'Zadania',
      ariaLabel: 'Przejdź do zadań',
      to: '/tasks',
    },
    {
      label: 'Ranking',
      to: '/leaderboard',
      ariaLabel: 'Przejdź do rankingu',
      prefetch: false,
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
      label: 'O nas',
      ariaLabel: 'Przejdź do informacji o Hack4Krak',
      to: '/about_us',
    },
    {
      label: 'Kontakt',
      ariaLabel: 'Przejdź do informacji kontaktowych',
      children: LANDING_SOCIALS,
    },
  ],
]
