export interface AboutUsPhoto {
  src: string
  alt: string
  // CSS aspect ratio of the frame, defaults to 3/2
  ratio?: string
}

export interface AboutUsStat {
  value: string
  label: string
}

export interface AboutUsEdition {
  number: number
  date: string
  title: string
  paragraphs: string[]
  stats: AboutUsStat[]
  photos: AboutUsPhoto[]
  // Shows the flags of the V4 countries instead of photos
  flags?: boolean
  planned?: boolean
}

const PORTRAIT = '4/5'

const photoFrom = (folder: string) => (name: string, alt: string, ratio?: string): AboutUsPhoto => ({ src: `/img/${folder}/${name}.webp`, alt, ratio })
const edition2025February = photoFrom('edition-2025-02')
const edition2025May = photoFrom('edition-2025-05')
const edition2026 = photoFrom('edition-2026')

export const aboutUsContent = {
  meta: {
    title: 'O nas',
    description: 'Hack4Krak zaczął się jako szkolny konkurs w XXXI LO w Krakowie. 4. edycja w marcu 2027 obejmuje uczniów z Polski, Czech, Słowacji i Węgier.',
  },
  hero: {
    subtitle: 'O nas',
    title: 'Jak powstał Hack4Krak',
    description: 'Jesteśmy grupą młodych ludzi z Krakowa, których połączyło cyberbezpieczeństwo. Zaczęliśmy od konkursu dla własnej szkoły, a zadania i platformę do dziś piszemy sami.',
    // Photos that do not appear again in the editions below
    photos: [
      edition2026('hall-crowd', 'Uczestnicy Hack4Krak 2026 w auli UKEN'),
      edition2025February('team-scoreboard', 'Drużyna przy laptopie podczas pierwszej edycji'),
      edition2026('hacking-headphones', 'Drużyna w słuchawkach rozwiązuje zadania'),
      edition2025May('team-laptops', 'Uczniowie szkół podstawowych przy laptopach'),
      edition2026('handshake', 'Gratulacje po zawodach'),
      edition2026('smile', 'Uśmiechnięta uczestniczka przy stanowisku'),
      edition2025February('classroom-row', 'Rząd drużyn w sali XXXI LO'),
    ] satisfies AboutUsPhoto[],
  },
  editions: [
    {
      number: 1,
      date: 'Luty 2025',
      title: 'Pierwszy CTF w XXXI LO',
      paragraphs: [
        'Hack4Krak zaczął się od prostego pomysłu. Kilkoro uczniów XXXI Liceum Ogólnokształcącego w Krakowie chciało zorganizować konkurs z cyberbezpieczeństwa dla własnej szkoły, więc napisaliśmy zadania i platformę od zera.',
        'Konkurs działał w formule CTF (Capture The Flag). W każdym zadaniu ukryta jest flaga, czyli tajny tekst. Żeby ją zdobyć, trzeba złamać szyfr, przeanalizować plik albo znaleźć lukę w aplikacji. Wzięło w nim udział ponad 60 uczniów, a my przetestowaliśmy naszą platformę na żywo. Jej kod jest otwarty do dziś.',
      ],
      stats: [
        { value: '60+', label: 'uczestników' },
        { value: '14', label: 'drużyn' },
        { value: '1', label: 'szkoła' },
      ],
      photos: [
        edition2025February('team-map-screen', 'Drużyny rozwiązują zadania na mapie platformy CTF w sali XXXI LO'),
        edition2025February('platform-countdown', 'Ekran platformy CTF z odliczaniem do końca zawodów', PORTRAIT),
        edition2025February('girls-team-smiles', 'Uśmiechnięta drużyna podczas zawodów'),
        edition2025February('excited-duo', 'Radość po rozwiązaniu zadania'),
        edition2025February('winners-row', 'Laureaci z dyplomami i torbami z nagrodami'),
      ],
    },
    {
      number: 2,
      date: 'Maj 2025',
      title: 'CTF dla szkół podstawowych',
      paragraphs: [
        'Po pierwszej edycji chcieliśmy się przekonać, czy CTF zadziała także u młodszych. Zaprosiliśmy uczniów krakowskich szkół podstawowych do XXXI LO i przygotowaliśmy nowe zadania, dopasowane do ich poziomu.',
        'Przyszło ponad 120 osób. Dla wielu był to pierwszy kontakt z cyberbezpieczeństwem, a zadania uczyły przez zabawę i zachęcały do dalszej nauki.',
      ],
      stats: [
        { value: '120+', label: 'uczestników' },
        { value: '15', label: 'zadań' },
        { value: '1', label: 'dzień zawodów' },
      ],
      photos: [
        edition2025May('group-crowd', 'Wszyscy uczestnicy na wspólnym zdjęciu na korytarzu'),
        edition2025May('volunteer-badge', 'Wolontariuszka z identyfikatorem Hack4Krak', PORTRAIT),
        edition2025May('welcome-sponsors-screen', 'Powitanie uczestników na tle ekranu ze sponsorami wydarzenia'),
        edition2025May('scoreboard-neon', 'Ranking na żywo pod neonem „I ❤ 31 LO”'),
        edition2025May('winners-sofa', 'Nagrodzeni uczestnicy z dyplomami'),
      ],
    },
    {
      number: 3,
      date: 'Maj 2026',
      title: 'Pierwsza duża edycja na UKEN',
      paragraphs: [
        'Pierwszy raz wyszliśmy poza szkołę. Na Uniwersytecie Komisji Edukacji Narodowej przez dwa dni rywalizowało ponad 120 uczniów szkół średnich, w drużynach do pięciu osób.',
        'Oprócz zawodów były prelekcje, w tym wystąpienie CERT Polska, a na koniec nagrody dla najlepszych drużyn. Udział był bezpłatny dzięki 15 partnerom i patronom.',
      ],
      stats: [
        { value: '120+', label: 'uczestników' },
        { value: '29', label: 'drużyn' },
        { value: '30+', label: 'godzin zawodów' },
      ],
      // The full hall and the people on stage first, so the scale of the event shows before the close-ups
      photos: [
        edition2026('hall-full', 'Pełna sala podczas Hack4Krak 2026'),
        edition2026('opening-partners', 'Przemówienie na otwarciu na tle logotypów partnerów', PORTRAIT),
        edition2026('hacking-desks', 'Drużyny rozwiązują zadania w sali komputerowej UKEN'),
        edition2026('cert-hall', 'Wykład CERT Polska w pełnej auli'),
        edition2026('podium-1', 'Zwycięska drużyna Hack4Krak 2026'),
        edition2026('laughing', 'Uczestnicy śmieją się przy stanowisku'),
        edition2026('two-smiles', 'Uśmiechnięte uczestniczki'),
      ],
    },
    {
      number: 4,
      date: 'Marzec 2027',
      title: 'Edycja dla czterech krajów',
      paragraphs: [
        'W 2027 roku podnosimy poprzeczkę jeszcze wyżej. 20–21 marca chcemy zorganizować w Krakowie stacjonarny CTF dla około 200 uczniów szkół średnich z całej Grupy Wyszehradzkiej: Polski, Czech, Słowacji i Węgier.',
        'Aby przeprowadzić wydarzenie takiej skali, założyliśmy Fundację Zerya, która jest oficjalnym organizatorem Hack4Krak, a w przyszłości także innych wydarzeń.',
      ],
      stats: [
        { value: '~200', label: 'uczestników' },
        { value: '4', label: 'kraje' },
        { value: '2', label: 'dni na UKEN' },
      ],
      photos: [],
      flags: true,
      planned: true,
    },
  ] satisfies AboutUsEdition[],
}
