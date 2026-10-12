import { DISCORD, INSTAGRAM, LINKEDIN, TIKTOK, X } from './landing/socials'
import { CONTACT_MAIL, CONTACT_PHONE, SOURCE_CODE_URL } from './organization'

export interface FooterLink {
  label: string
  to: string
  icon?: string
  target?: string
}

export const FOOTER_FOUNDATION_NOTE = 'Fundację założył zespół Hack4Krak, żeby przygotowywać wydarzenia na większą skalę.'

export const FOOTER_SOCIALS: FooterLink[] = [DISCORD, INSTAGRAM, TIKTOK, X, LINKEDIN]
  .map(({ label, pixelIcon, to, target }) => ({ label, icon: pixelIcon, to, target }))

export const FOOTER_CONTACTS: FooterLink[] = [
  { label: CONTACT_MAIL, to: `mailto:${CONTACT_MAIL}` },
  { label: CONTACT_PHONE, to: `tel:${CONTACT_PHONE.replaceAll(' ', '')}` },
]

export const FOOTER_LINKS: FooterLink[] = [
  { label: 'Kod źródłowy', to: SOURCE_CODE_URL, target: '_blank' },
  { label: 'Polityka prywatności', to: '/docs/privacy-policy' },
  { label: 'Regulamin', to: '/docs/rules' },
]
