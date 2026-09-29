import type { AboutUsPhoto } from '../about-us-content'
import { CONTACT_MAIL } from '../organization'
import { DISCORD_URL } from './socials'

export interface LandingStat {
  value: string
  label: string
  image: string
  color: string
}

export interface LandingOffer {
  icon: string
  title: string
  description: string
  color: string
}

// Kept independent of a single edition, so the landing page does not need rewriting every year
export const LANDING_STATS: LandingStat[] = [
  { value: '300+', label: 'uczestników', image: '/img/character/knight.png', color: '#fc4d3d' },
  { value: '30+', label: 'wolontariuszy', image: '/img/character/princess.png', color: '#ffdf26' },
  { value: '15', label: 'partnerów i patronów', image: '/img/character/king.png', color: '#dab2ff' },
  { value: '3', label: 'edycje', image: '/img/character/tower.png', color: '#00c950' },
]

const photo = (edition: string, name: string, alt: string): AboutUsPhoto => ({ src: `/img/${edition}/${name}.webp`, alt })

// Mixed across editions, and each row repeats crowd, team, close-up, team, so no stretch is only faces or only crowds.
// The component splits the list into two rows at the midpoint
export const LANDING_COMMUNITY_PHOTOS: AboutUsPhoto[] = [
  photo('edition-2026', 'hall-opening', 'Otwarcie Hack4Krak w auli UKEN'),
  photo('edition-2025-02', 'team-huddle', 'Drużyna naradza się nad zadaniem'),
  photo('edition-2026', 'two-smiles', 'Uśmiechnięte uczestniczki'),
  photo('edition-2025-05', 'team-laptops', 'Uczniowie szkół podstawowych przy laptopach'),
  photo('edition-2026', 'cert-talk', 'Prelekcja CERT Polska'),
  photo('edition-2025-05', 'group-crowd', 'Wspólne zdjęcie uczestników'),
  photo('edition-2025-02', 'girls-team-smiles', 'Uśmiechnięta drużyna uczestniczek'),
  photo('edition-2026', 'handshake', 'Organizator gratuluje uczestnikowi po zawodach'),
  photo('edition-2025-05', 'prize-handover', 'Wręczenie nagród'),

  photo('edition-2026', 'hacking-team', 'Drużyna przy laptopach rozwiązuje zadania'),
  photo('edition-2025-02', 'winners-diplomas', 'Zwycięzcy z dyplomami'),
  photo('edition-2025-05', 'organizer-helping', 'Organizator pomaga uczestnikom'),
  photo('edition-2026', 'laughing', 'Uczestnicy śmieją się przy stanowisku'),
  photo('edition-2025-05', 'welcome-sponsors-screen', 'Powitanie uczestników na tle ekranu ze sponsorami'),
  photo('edition-2025-05', 'classroom-full', 'Pełna sala drużyn przy laptopach'),
  photo('edition-2025-02', 'team-scoreboard', 'Drużyna patrzy na ranking'),
  photo('edition-2026', 'thinking-trio', 'Trójka uczestników zastanawia się nad zadaniem na korytarzu'),
  photo('edition-2026', 'podium-1', 'Zwycięska drużyna'),
]

// What a partner gets, described by outcome. Prices stay in the offer PDF sent by email
export const LANDING_PARTNER_OFFERS: LandingOffer[] = [
  {
    icon: 'pixelarticons:flag',
    title: 'Własne zadanie w konkursie',
    color: '#fc4d3d',
    description: 'Przygotujemy z Wami zadanie CTF oparte na Waszej technologii. Uczestnicy poznają firmę, rozwiązując realny problem.',
  },
  {
    icon: 'pixelarticons:users',
    title: 'Poznajcie przyszłych specjalistów',
    color: '#ffdf26',
    description: 'Dwa dni z uczniami, którzy sami zgłosili się, żeby rozwiązywać zadania z cyberbezpieczeństwa. Stoisko, prelekcja i rozmowy w przerwach.',
  },
  {
    icon: 'pixelarticons:heart',
    title: 'Widoczność wśród uczestników',
    color: '#dab2ff',
    description: 'Logo na stronie, ekranach i materiałach, wpisy w naszych mediach i wystąpienie na otwarciu.',
  },
]

export const PARTNER_MAIL_URL = `mailto:${CONTACT_MAIL}?subject=${encodeURIComponent('Partnerstwo Hack4Krak')}`

export const ORGANIZER_URL = DISCORD_URL
