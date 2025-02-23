export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path.startsWith('/tasks')) {
    const toast = useToast()
    let description = 'Nieznany błąd'

    try {
      const response = await useApi('/event/is_event_in_progress', {
        key: 'tasks',
        redirect: 'error',
      })

      const error = response.error.value
      if (error) {
        if (error.data.code === 403) {
          description = 'Nie możesz otworzyć tej strony przed rozpocząciem wydarzenia'
        } else if (error.data.code === 410) {
          description = 'Wydarzenie zostało zakończone'
        }

        toast.add({ title: `Błąd ${error.data.code}`, description: `${description}`, color: 'error' })
        return '/'
      }
    } catch {
      toast.add({ title: 'Błąd', description: `${description}`, color: 'error' })
      return '/'
    }
  }
})
