import { LazyPanelModalConfirmDelete } from '#components'

export function useConfirmModal(modalData: any) {
  const overlay = useOverlay()
  const isOpen = ref(false)

  const modal = overlay.create(LazyPanelModalConfirmDelete, { props: modalData })

  async function open() {
    isOpen.value = true
    await modal.open()
  }

  function close() {
    isOpen.value = false
    modal.close()
  }

  return { open, close }
}
