export type V4CountryCode = 'pl' | 'cz' | 'sk' | 'hu'

export const V4_COUNTRIES: { code: V4CountryCode, name: string }[] = [
  { code: 'pl', name: 'Polska' },
  { code: 'cz', name: 'Czechy' },
  { code: 'sk', name: 'Słowacja' },
  { code: 'hu', name: 'Węgry' },
]

// Flags are drawn on a 24×16 grid, so every shape stays aligned to whole pixels. Later rects paint over earlier ones
export type FlagRect = [x: number, y: number, width: number, height: number, fill: string]

export const FLAG_WIDTH = 24
export const FLAG_HEIGHT = 16

const WHITE = '#fafafa'

function stripes(colors: string[]): FlagRect[] {
  const height = FLAG_HEIGHT / colors.length
  return colors.map((fill, index) => [0, index * height, FLAG_WIDTH, height, fill])
}

// Stepped triangle from the hoist, reaching the middle of the flag
function czechTriangle(): FlagRect[] {
  return Array.from({ length: FLAG_HEIGHT }, (_, row) => {
    const width = Math.round(12 * (1 - Math.abs(row + 0.5 - 8) / 8))
    return [0, row, width, 1, '#11457e'] as FlagRect
  })
}

function slovakCoatOfArms(): FlagRect[] {
  const red = '#ee1c25'
  const blue = '#0b4ea2'
  return [
    [4, 3, 8, 8, WHITE],
    [5, 11, 6, 1, WHITE],
    [6, 12, 4, 1, WHITE],
    [5, 4, 6, 7, red],
    [6, 11, 4, 1, red],
    [7, 5, 2, 5, WHITE],
    [6, 6, 4, 1, WHITE],
    [5, 8, 6, 1, WHITE],
    [5, 10, 6, 1, blue],
    [6, 11, 4, 1, blue],
  ]
}

export const V4_FLAGS: Record<V4CountryCode, FlagRect[]> = {
  pl: stripes([WHITE, '#dc143c']),
  cz: [...stripes([WHITE, '#d7141a']), ...czechTriangle()],
  sk: [...stripes([WHITE, '#0b4ea2', '#ee1c25']), ...slovakCoatOfArms()],
  hu: stripes(['#ce2939', WHITE, '#477050']),
}
