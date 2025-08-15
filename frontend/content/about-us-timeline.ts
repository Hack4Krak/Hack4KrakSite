export interface EventCardProps {
  title: string
  subtitle: string
  img: string
  participants: number
  color?: boolean
}

export const aboutUsTimeline: EventCardProps[] = [
  {
    title: 'Hack4Krak CTF dla uczniów 31 LO w Krakowie',
    subtitle: 'Luty 2025',
    participants: 67,
    img: '/img/events/Event1.webp',
  },
  {
    title: 'Hack4Krak CTF dla uczniów szkół podstawowych',
    subtitle: 'Maj 2025',
    participants: 123,
    img: '/img/events/Event2.webp',
  },
  {
    title: 'Przygotowania do największej edycji Hack4Krak CTF',
    subtitle: 'Obecnie',
    participants: 250,
    img: '/img/events/Event3.webp',
    color: true,
  },
]
