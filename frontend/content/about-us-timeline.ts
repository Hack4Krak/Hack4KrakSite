export interface EventCardProps {
  title: string
  subtitle: string
  img: string
  imgCrop?: string
  participants: number
  description: string
  addColorAccent?: boolean
}

export const aboutUsTimeline: EventCardProps[] = [
  {
    title: 'Hack4Krak CTF dla uczniów 31 LO w Krakowie',
    subtitle: 'Luty 2025',
    description: ''
      + '**67 uczniów** rozwiązywało zadania z zakresu cyberbezpieczeństwa, rozwijając przy tym swoje umiejętności '
      + 'techniczne, logiczne myślenie oraz pracę zespołową.\n\n'
      + 'Wydarzenie pozwoliło również na przetestowanie i potwierdzenie działania naszej autorskiej platformy.',
    participants: 67,
    img: '/img/events/event1.webp',
  },
  {
    title: 'Hack4Krak CTF dla uczniów szkół podstawowych',
    subtitle: 'Maj 2025',
    description: '**123 uczniów** szkół podstawowych z Krakowa wzięło udział w drugiej edycji Hack4Krak CTF. \n\n'
      + 'Dla wielu było to pierwsze zetknięcie z tematyką cyberbezpieczeństwa. Zadania, dostosowane do ich poziomu, '
      + 'rozwijały kluczowe kompetencje przyszłości i uczyły przez zabawę, inspirując do dalszej nauki.',
    participants: 123,
    img: '/img/events/event2.webp',
  },
  {
    title: 'Przygotowania do największej edycji Hack4Krak CTF',
    subtitle: 'Marzec 2026',
    participants: 250,
    description: 'Trzecia edycja **Hack4Krak CTF** będzie stacjonarnym wydarzeniem dla uczniów szkół średnich. \n\n'
      + 'Planujemy przyciągnąć setki uczestników, oferując angażujące zadania i rozwijając kluczowe umiejętności. '
      + 'Wydarzenie ma podnieść standard konkursów cyberbezpieczeństwa dla młodzieży w całej Polsce.',
    img: '/img/our-team.webp',
    imgCrop: '900_100_3500_1700',
    addColorAccent: true,
  },
]
