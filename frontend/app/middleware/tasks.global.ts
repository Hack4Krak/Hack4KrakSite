export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path.startsWith('/tasks')) {
    const toast = useToast()
    try {
      const response = await useApi('/tasks/list', {
        key: 'tasks',
        redirect: 'error',
      })
      if (response.error.value) {
        toast.add({ title: 'Błąd', description: 'Nie możesz otworzyć strony z zadaniami przed rozpoczęciem wydarzenia', color: 'error' })
        return '/'
      }
    } catch {
      toast.add({ title: 'Błąd', description: 'Nie możesz otworzyć strony z zadaniami przed rozpoczęciem wydarzenia', color: 'error' })
      return '/'
    }
  }
})
