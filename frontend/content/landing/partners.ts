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
    name: 'Ambasada Społeczności',
    url: 'https://ambasadaspolecznosci.org.pl/',
    tagline: 'Partner organizacyjny',
    logo: '/img/partners/ambasada-spolecznosci.webp',
    logoAlt: 'Logo Ambasada Społeczności',
    variant: 'standard',
  },
  {
    name: 'CERT Polska',
    url: 'https://cert.pl/',
    tagline: 'Patronat merytoryczny',
    logo: '/img/partners/cert.webp',
    logoAlt: 'Logo CERT (Nask)',
    variant: 'standard',
  },
  {
    name: '31 Liceum Ogólnokształcące w Krakowie',
    url: 'https://lo31.krakow.pl/',
    tagline: 'Partner',
    logo: '/img/partners/31lo.webp',
    logoAlt: 'Logo 31 LO',
    variant: 'standard',
  },
  {
    name: 'Netwrix',
    url: 'https://netwrix.com/',
    tagline: 'Partner',
    logo: '/img/partners/netwrix.webp',
    logoAlt: 'Logo Netwrix',
    variant: 'standard',
  },
  {
    name: 'Krakowski Park Technologiczny',
    url: 'https://www.kpt.krakow.pl/',
    tagline: 'Partner',
    logo: '/img/partners/kpt.webp',
    logoAlt: 'Logo Krakowski Park Technologiczny',
    variant: 'standard',
  },
  {
    name: 'Teach for Poland',
    url: 'https://teachforpoland.org/',
    tagline: 'Patronat medialny',
    logo: '/img/partners/teachforpoland.webp',
    logoAlt: 'Teach for Poland',
    variant: 'standard',
  },
  {
    name: 'FABLAB Małopolska',
    url: 'https://www.fablabmalopolska.pl/',
    tagline: 'Patronat medialny',
    logo: '/img/partners/fablab.webp',
    logoAlt: 'FABLAB Małopolska',
    variant: 'standard',
  },
  {
    name: 'Serwer Discord "Egzaminy zawodowe"',
    url: 'https://discord.com/invite/egzaminy-zawodowe-it-matury-studia-723560181996191914',
    tagline: 'Patronat medialny',
    logo: '/img/partners/egzaminy-zawodowe.png',
    logoAlt: 'Serwer Discord "Egzaminy zawodowe"',
    variant: 'standard',
  },
  {
    name: 'Serwer Discord "Programowanie"',
    url: 'https://programowanie.org',
    tagline: 'Patronat medialny',
    logo: '/img/partners/programowanie.webp',
    logoAlt: 'Serwer Discord "Programowanie"',
    variant: 'standard',
  },
  {
    name: 'INFOSEC',
    url: 'https://infosec.info.pl',
    tagline: 'Patronat medialny',
    logo: '/img/partners/infosec.webp',
    logoAlt: 'Logo INFOSEC',
    variant: 'standard',
  },
  {
    name: 'CyberDot',
    url: 'https://cyberdot.pl/',
    tagline: 'Patronat medialny',
    logo: '/img/partners/cyberdot.webp',
    logoAlt: 'CyberDot',
    variant: 'standard',
  },
]

export const LANDING_FEATURED_PARTNERS = LANDING_PARTNERS.filter(partner => partner.variant === 'featured')
export const LANDING_SUPPORTING_PARTNERS = LANDING_PARTNERS.filter(partner => partner.variant !== 'featured')
