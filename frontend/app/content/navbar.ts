// https://ui.nuxt.com/components/navigation-menu#usage

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
      children: [
        {
          label: 'Discord',
          icon: 'ic:baseline-discord',
          to: 'https://discord.gg/ASPqckzEd8',
          target: '_blank',
        },
        {
          label: 'Instagram',
          icon: 'lineicons:instagram-original',
          to: 'https://www.instagram.com/hack4krak/',
          target: '_blank',
        },
        {
          label: 'LinkedIn',
          icon: 'lineicons:linkedin-original',
          to: 'https://pl.linkedin.com/company/hack4krak',
          target: '_blank',
        },
        {
          label: 'Email',
          icon: 'uil:envelope',
          to: 'hack4krak@gmail.com',
          target: '_blank',
        },
      ],
    },
  ],
]
