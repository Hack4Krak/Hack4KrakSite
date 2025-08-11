export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path.startsWith('/panel') || to.path === '/account/submit_personal_info') {
    try {
      const { data, error } = await useAuth('/account/', {
        redirect: 'error',
      })
      if (error.value || !data.value) {
        return '/login'
      }
      if (to.path.startsWith('/panel')) {
        if (data.value.has_personal_information === false) {
          return '/account/submit_personal_info'
        }
      }
    } catch {
      return '/login'
    }
  }
})
