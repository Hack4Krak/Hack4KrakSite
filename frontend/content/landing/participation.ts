import LANDING_CONTENT from './page'

export interface LandingParticipationStep {
  title: string
  description: string
  color: string
  icon: string
  highlight?: boolean
}

export const LANDING_PARTICIPATION_STEPS: LandingParticipationStep[] = [
  {
    title: 'Zarejestruj się',
    description:
      'Załóż konto na hack4krak.pl i uzupełnij profil. Rejestracja jest darmowa i zajmuje mniej niż minutę.',
    color: 'var(--ui-color-primary-500)',
    icon: 'pixelarticons:user',
  },
  {
    title: 'Stwórz drużynę',
    description:
      'Zbierz zespół liczący maksymalnie 5 uczniów szkół średnich. Możesz też dołączyć do istniejącej drużyny.',
    color: '#dab2ff',
    icon: 'pixelarticons:users',
  },
  {
    title: 'Przyjedź do Krakowa',
    description: `${LANDING_CONTENT.event.dateDisplay} przyjedź na UKEN. Zabierz sprzęt potrzebny do pracy, chęć do nauki i swoją drużynę - o resztę zadbamy na miejscu.`,
    color: '#ffdf26',
    icon: 'pixelarticons:map-pin',
    highlight: true,
  },
  {
    title: 'Rozwiązuj i wygrywaj',
    description:
      'Przez 30 godzin rozwiązuj zadania z różnych obszarów cyberbezpieczeństwa, wspinaj się w rankingu, słuchaj prelekcji, jedz dobre jedzenie i poznawaj innych uczestników!',
    color: '#d08700',
    icon: 'pixelarticons:flag',
  },
]
