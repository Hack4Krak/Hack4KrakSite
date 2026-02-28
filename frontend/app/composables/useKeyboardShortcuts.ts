export function useCommandPaletteState() {
  return useState('command-palette-open', () => false)
}

export function useFlagModalState() {
  return useState('flag-modal-open', () => false)
}

export function useKeyboardShortcuts() {
  const commandPaletteOpen = useCommandPaletteState()
  const flagModalOpen = useFlagModalState()

  defineShortcuts({
    'meta_k': () => {
      commandPaletteOpen.value = !commandPaletteOpen.value
    },
    '/': () => {
      commandPaletteOpen.value = true
    },
    's-f': () => {
      commandPaletteOpen.value = false
      flagModalOpen.value = true
    },
  })
}
