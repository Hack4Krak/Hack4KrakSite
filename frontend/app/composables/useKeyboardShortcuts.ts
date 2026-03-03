export function useCommandPaletteState() {
  return useState('command-palette-open', () => false)
}

export function useFlagModalState() {
  return useState('flag-modal-open', () => false)
}

export function useKeyboardShortcuts() {
  const isCommandPaletteOpen = useCommandPaletteState()
  const isFlagModalOpen = useFlagModalState()

  defineShortcuts({
    'meta_k': () => {
      isCommandPaletteOpen.value = !isCommandPaletteOpen.value
    },
    '/': () => {
      isCommandPaletteOpen.value = true
    },
    's-f': () => {
      isCommandPaletteOpen.value = false
      isFlagModalOpen.value = true
    },
  })
}
