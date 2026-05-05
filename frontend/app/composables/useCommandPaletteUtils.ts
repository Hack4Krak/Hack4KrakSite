export function useCommandPaletteState() {
  return useState('command-palette-open', () => false)
}

export function useFlagModalState() {
  return useState('flag-modal-open', () => false)
}

export function openFlagSubmitModal() {
  const isFlagModalOpen = useFlagModalState()
  isFlagModalOpen.value = true
}

export function useCommandPaletteShortcuts() {
  const isCommandPaletteOpen = useCommandPaletteState()

  defineShortcuts({
    'meta_k': {
      usingInput: true,
      handler: () => {
        isCommandPaletteOpen.value = !isCommandPaletteOpen.value
      },
    },
    '/': () => {
      isCommandPaletteOpen.value = true
    },
    's-f': () => {
      isCommandPaletteOpen.value = false
      openFlagSubmitModal()
    },
  })
}
