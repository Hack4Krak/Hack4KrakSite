import { CONTACT_MAIL } from '../organization'

export const DISCORD_URL = 'https://discord.gg/ASPqckzEd8'

export interface Social {
  label: string
  icon: string
  // Pixel art version of the icon, for places drawn in the pixel style like the footer
  pixelIcon: string
  to: string
  target: string
}

export const DISCORD: Social = {
  label: 'Discord',
  icon: 'ic:baseline-discord',
  pixelIcon: 'pixelarticons:discord',
  to: DISCORD_URL,
  target: '_blank',
}

export const INSTAGRAM: Social = {
  label: 'Instagram',
  icon: 'mdi:instagram',
  pixelIcon: 'pixelarticons:instagram',
  to: 'https://www.instagram.com/hack4krak/',
  target: '_blank',
}

export const TIKTOK: Social = {
  label: 'TikTok',
  icon: 'ic:baseline-tiktok',
  pixelIcon: 'pixelarticons:tiktok',
  to: 'https://www.tiktok.com/@hack4krakctf',
  target: '_blank',
}

export const X: Social = {
  label: 'X',
  icon: 'hugeicons:new-twitter',
  pixelIcon: 'pixelarticons:x',
  to: 'https://x.com/hack4krak',
  target: '_blank',
}

export const LINKEDIN: Social = {
  label: 'LinkedIn',
  icon: 'mdi:linkedin',
  pixelIcon: 'pixelarticons:linkedin',
  to: 'https://pl.linkedin.com/company/hack4krak',
  target: '_blank',
}

export const EMAIL: Social = {
  label: 'Email',
  icon: 'mdi:envelope',
  pixelIcon: 'pixelarticons:mail',
  to: `mailto:${CONTACT_MAIL}`,
  target: '_blank',
}

export const LANDING_SOCIALS: Social[] = [DISCORD, INSTAGRAM, TIKTOK, X, LINKEDIN, EMAIL]
