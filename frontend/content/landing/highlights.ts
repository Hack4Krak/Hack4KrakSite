export interface LandingHighlight {
  accent: string
  title: string
  description: string
  color: string
  image: string
  imageAlt: string
}

export const LANDING_HIGHLIGHTS: LandingHighlight[] = [
  {
    accent: 'FREE',
    title: 'Całkowicie bezpłatny udział',
    description:
      'Wystarczy zabrać zgody oraz własny laptop. My zapewniamy wyżywienie, prelekcje, świetną atmosferę i nagrody dla najlepszych drużyn.',
    color: '#fc4d3d',
    image: '/img/character/knight.png',
    imageAlt: 'Pixelowy rycerz',
  },
  {
    accent: '0',
    title: 'Wymaganego doświadczenia',
    description:
      'Nie musisz mieć wcześniejszego doświadczenia. Zadania są zróżnicowane poziomem trudności i kategoriami - od webu po OSINT.',
    color: '#ffdf26',
    image: '/img/character/princess.png',
    imageAlt: 'Pixelowa księżniczka',
  },
  {
    accent: '5',
    title: 'Osób maksymalnie w drużynie',
    description:
      'Startują uczniowie szkół ponadpodstawowych. Możesz działać solo albo zebrać drużynę do 5 osób i podzielić się zadaniami.',
    color: '#dab2ff',
    image: '/img/character/king.png',
    imageAlt: 'Pixelowy król',
  },
]

export const LANDING_FORMAT_CARD = {
  accent: 'CTF',
  eyebrow: 'Format',
  title: 'Jak wygląda rywalizacja?',
  description:
    'Gracie w formule Jeopardy CTF, więc sami wybieracie zadania i tempo pracy. Każda poprawnie oddana flaga daje punkty i przesuwa was wyżej w rankingu.',
  color: '#00c950',
  image: '/img/character/tower.png',
  imageAlt: 'Pixelowa wieża',
} as const
