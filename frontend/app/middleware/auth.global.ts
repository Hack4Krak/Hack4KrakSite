export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path.startsWith('/panel')) {
    try {
      const user = await useAuth('/user/', {
        redirect: 'error',
      })
      if (user.error.value || !user.data.value) {
        return '/login'
      }
    } catch {
      return '/login'
    }
  }
})
