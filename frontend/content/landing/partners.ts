import { CONTACT_MAIL } from '../organization'

export type LandingPartnerVariant = 'featured' | 'standard' | 'placeholder'

export interface LandingPartner {
  name: string
  url: string
  tagline: string
  variant: LandingPartnerVariant
  logo?: string
  logoAlt?: string
  // Width divided by height of the logo file, used to give every logo a similar visual weight
  logoAspect?: number
}

export const PARTNERS_CONTACT_MAIL = CONTACT_MAIL

export const LANDING_PARTNERS: LandingPartner[] = [
  {
    name: 'CyberFolks',
    url: 'https://cyberfolks.pl',
    tagline: 'Sponsor Główny',
    logo: '/img/partners/cyberfolks.webp',
    logoAspect: 6.0,
    logoAlt: 'Logo CyberFolks',
    variant: 'featured',
  },
  {
    name: 'Arkanet',
    url: 'https://arkanet.pl',
    tagline: 'Sponsor',
    logo: '/img/partners/arkanet.webp',
    logoAspect: 4.09,
    logoAlt: 'Logo Arkanet',
    variant: 'featured',
  },
  {
    name: 'Zerya',
    url: 'https://zerya.dev',
    tagline: 'Partner technologiczny',
    logo: '/img/partners/zerya.webp',
    logoAspect: 2.0,
    logoAlt: 'Logo Zerya',
    variant: 'standard',
  },
  {
    name: 'UKEN',
    url: 'https://www.uken.krakow.pl/',
    tagline: 'Partner wydarzenia',
    logo: '/img/partners/uken.webp',
    logoAspect: 2.34,
    logoAlt: 'Logo UKEN',
    variant: 'standard',
  },
  {
    name: 'Netwrix',
    url: 'https://netwrix.com/',
    tagline: 'Partner',
    logo: '/img/partners/netwrix.webp',
    logoAspect: 5.0,
    logoAlt: 'Logo Netwrix',
    variant: 'standard',
  },
  {
    name: 'Krakowski Park Technologiczny',
    url: 'https://www.kpt.krakow.pl/',
    tagline: 'Partner',
    logo: '/img/partners/kpt.webp',
    logoAspect: 4.51,
    logoAlt: 'Logo Krakowski Park Technologiczny',
    variant: 'standard',
  },
  {
    name: 'Ambasada Społeczności',
    url: 'https://ambasadaspolecznosci.org.pl/',
    tagline: 'Partner organizacyjny',
    logo: '/img/partners/ambasada-spolecznosci.webp',
    logoAspect: 5.2,
    logoAlt: 'Logo Ambasada Społeczności',
    variant: 'standard',
  },
  {
    name: '31 Liceum Ogólnokształcące w Krakowie',
    url: 'https://lo31.krakow.pl/',
    tagline: 'Partner',
    logo: '/img/partners/31lo.webp',
    logoAspect: 1.01,
    logoAlt: 'Logo 31 LO',
    variant: 'standard',
  },
  {
    name: 'Teach for Poland',
    url: 'https://teachforpoland.org/',
    tagline: 'Patronat medialny',
    logo: '/img/partners/teachforpoland.webp',
    logoAspect: 2.4,
    logoAlt: 'Teach for Poland',
    variant: 'standard',
  },
  {
    name: 'FABLAB Małopolska',
    url: 'https://www.fablabmalopolska.pl/',
    tagline: 'Patronat medialny',
    logo: '/img/partners/fablab.webp',
    logoAspect: 0.9,
    logoAlt: 'FABLAB Małopolska',
    variant: 'standard',
  },
  {
    name: 'Serwer Discord "Egzaminy zawodowe"',
    url: 'https://discord.com/invite/egzaminy-zawodowe-it-matury-studia-723560181996191914',
    tagline: 'Patronat medialny',
    logo: '/img/partners/egzaminy-zawodowe.png',
    logoAspect: 1.15,
    logoAlt: 'Serwer Discord "Egzaminy zawodowe"',
    variant: 'standard',
  },
  {
    name: 'Serwer Discord "Programowanie"',
    url: 'https://programowanie.org',
    tagline: 'Patronat medialny',
    logo: '/img/partners/programowanie.webp',
    logoAspect: 1.75,
    logoAlt: 'Serwer Discord "Programowanie"',
    variant: 'standard',
  },
  {
    name: 'INFOSEC',
    url: 'https://infosec.info.pl',
    tagline: 'Patronat medialny',
    logo: '/img/partners/infosec.webp',
    logoAspect: 1.04,
    logoAlt: 'Logo INFOSEC',
    variant: 'standard',
  },
  {
    name: 'CyberDot',
    url: 'https://cyberdot.pl/',
    tagline: 'Patronat medialny',
    logo: '/img/partners/cyberdot.webp',
    logoAspect: 4.24,
    logoAlt: 'CyberDot',
    variant: 'standard',
  },
  {
    name: 'CERT Polska',
    url: 'https://cert.pl/',
    tagline: 'Patronat merytoryczny',
    logo: '/img/partners/cert.webp',
    logoAspect: 3.85,
    logoAlt: 'Logo CERT (Nask)',
    variant: 'standard',
  },
]

export const LANDING_FEATURED_PARTNERS = LANDING_PARTNERS.filter(partner => partner.variant === 'featured')
export const LANDING_SUPPORTING_PARTNERS = LANDING_PARTNERS.filter(partner => partner.variant !== 'featured')

export interface LandingPartnerGroup {
  label: string
  partners: LandingPartner[]
}

const byTagline = (prefix: string) => LANDING_PARTNERS.filter(partner => partner.tagline.startsWith(prefix))

export const LANDING_PARTNER_GROUPS: LandingPartnerGroup[] = [
  { label: 'Sponsorzy', partners: byTagline('Sponsor') },
  { label: 'Partnerzy', partners: byTagline('Partner') },
  { label: 'Patronat', partners: byTagline('Patronat') },
]

// The best-known names, shown on the landing page. The full list lives on the about page
const TRUSTED_PARTNER_NAMES = ['CyberFolks', 'Arkanet', 'Zerya', 'CERT Polska', 'Krakowski Park Technologiczny', 'UKEN']

export const LANDING_TRUSTED_PARTNERS = TRUSTED_PARTNER_NAMES.map(name => LANDING_PARTNERS.find(partner => partner.name === name)!)
