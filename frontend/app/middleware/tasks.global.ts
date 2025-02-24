export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path.startsWith('/tasks')) {
    const toast = useToast()
    let description = 'Nieznany błąd'

    try {
      const response = await useApi('/event/status', {
        key: 'event-status',
        redirect: 'error',
      })

      const error = response.error.value
      if (error) {
        switch (error.data.code) {
          case 403:
            description = 'Nie możesz otworzyć tej strony przed rozpocząciem wydarzenia'
            break
          case 410:
            description = 'Nie możesz otworzyć tej strony po zakończeniu wydarzenia'
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
