import { EVENT_DOCUMENTS } from './event-documents'

export const EVENT_BRING_ITEMS = [
  {
    icon: 'pixelarticons:device-laptop',
    title: 'Laptop + ładowarka',
    description: 'Mamy gniazdka pod każdym stanowiskiem · przynieś swój sprzęt.',
    required: true,
  },
  {
    icon: 'pixelarticons:contact',
    title: 'Dokument tożsamości',
    description: 'Legitymacja albo dowód · weryfikujemy przy wejściu.',
    required: true,
  },
  {
    icon: 'pixelarticons:script-text',
    title: 'Podpisane zgody',
    description: 'Przynieś dwa podpisane formularze: zgodę rodzica/opiekuna oraz zgodę RODO i wizerunkową.',
    required: true,
    links: EVENT_DOCUMENTS.map(document => ({ href: document.href, label: document.title })),
  },
  {
    icon: 'pixelarticons:moon-star',
    title: 'Rzeczy do wyspania',
    description: 'Zapewniamy materace. Zabierz śpiwór albo koc, poduszkę i rzeczy, w których wygodnie odpoczniesz.',
    required: true,
  },
  {
    icon: 'pixelarticons:download',
    title: 'Twój QR kod',
    description: 'Pokaż przy rejestracji · masz go w tym panelu.',
    required: true,
  },
]
