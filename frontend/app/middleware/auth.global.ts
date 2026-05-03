export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path.startsWith('/panel') || to.path === '/account/onboarding') {
    try {
      const { data, error } = await useAuth('/account/', {
        redirect: 'error',
      })
      if (error.value || !data.value) {
        return '/login'
      }
      if (to.path.startsWith('/panel')) {
        if (data.value.has_completed_onboarding === false) {
          return '/account/onboarding'
        }
      }
      if (to.path === '/account/onboarding') {
        if (data.value.has_completed_onboarding === true) {
          return '/panel'
        }
      }
    } catch {
      return '/login'
    }
  }
})
