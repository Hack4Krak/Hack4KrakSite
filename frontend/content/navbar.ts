// https://ui.nuxt.com/components/navigation-menu#usage

import type { NavigationMenuItem } from '#ui/components/NavigationMenu.vue'
import { LANDING_SOCIALS } from './landing/socials'

export const NAVBAR_ITEMS: NavigationMenuItem[] = [
  [
    {
      label: 'O nas',
      ariaLabel: 'Przejdź do informacji o Hack4Krak',
      to: '/about_us',
    },
    {
      label: 'FAQ',
      ariaLabel: 'Przejdź do najczęściej zadawanych pytań',
      to: '/docs/faq',
    },
    {
      label: 'Edycja 2026',
      ariaLabel: 'Zadania, ranking i regulamin edycji 2026',
      children: [
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
      ],
    },
    {
      label: 'Kontakt',
      ariaLabel: 'Przejdź do informacji kontaktowych',
      children: LANDING_SOCIALS,
    },
  ],
]
