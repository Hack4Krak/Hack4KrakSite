export type LandingPartnerVariant = 'featured' | 'standard' | 'placeholder'

export interface LandingPartner {
  name: string
  url: string
  tagline: string
  variant: LandingPartnerVariant
  logo?: string
  logoAlt?: string
}

export const PARTNERS_CONTACT_MAIL = 'partnerzy@hack4krak.pl'

export const LANDING_PARTNERS: LandingPartner[] = [
  {
    name: 'CyberFolks',
    url: 'https://cyberfolks.pl',
    tagline: 'Sponsor Główny',
    logo: '/img/partners/cyberfolks.webp',
    logoAlt: 'Logo CyberFolks',
    variant: 'featured',
  },
  {
    name: 'Arkanet',
    url: 'https://arkanet.pl',
    tagline: 'Sponsor',
    logo: '/img/partners/arkanet.webp',
    logoAlt: 'Logo Arkanet',
    variant: 'featured',
  },
  {
    name: 'Zerya',
    url: 'https://zerya.dev',
    tagline: 'Partner technologiczny',
    logo: '/img/partners/zerya.webp',
    logoAlt: 'Logo Zerya',
    variant: 'standard',
  },
  {
    name: 'UKEN',
    url: 'https://www.uken.krakow.pl/',
    tagline: 'Partner wydarzenia',
    logo: '/img/partners/uken.webp',
    logoAlt: 'Logo UKEN',
    variant: 'standard',
  },
  {
    name: 'CERT Polska',
    url: 'https://cert.pl/',
    tagline: 'Patronat merytoryczny',
    logo: '/img/partners/cert.webp',
    logoAlt: 'Logo CERT Polska',
    variant: 'standard',
  },
  {
    name: '?',
    url: 'mailto:kontakt@hack4krak.pl',
    tagline: 'Zostań partnerem',
    variant: 'placeholder',
  },
]

export const LANDING_FEATURED_PARTNERS = LANDING_PARTNERS.filter(partner => partner.variant === 'featured')
export const LANDING_SUPPORTING_PARTNERS = LANDING_PARTNERS.filter(partner => partner.variant !== 'featured')
