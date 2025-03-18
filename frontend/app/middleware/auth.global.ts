export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path.startsWith('/panel')) {
    try {
      const { data, error } = await useAuth('/account/', {
        redirect: 'error',
      })
      if (error.value || !data.value) {
        return '/login'
      }
    } catch {
      return '/login'
    }
  }
})
