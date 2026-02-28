export function useCommandPaletteState() {
  return useState('command-palette-open', () => false)
}

export function useKeyboardShortcuts() {
  const commandPaletteOpen = useCommandPaletteState()

  defineShortcuts({
    'meta_k': () => {
      commandPaletteOpen.value = !commandPaletteOpen.value
    },
    '/': () => {
      commandPaletteOpen.value = true
    },
  })
}
