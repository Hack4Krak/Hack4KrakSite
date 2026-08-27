export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path.startsWith('/panel') || to.path.startsWith('/account')) {
    try {
      const { data, error } = await useAuth('/account/', {
        redirect: 'error',
      })
      if (error.value || !data.value) {
        return await navigateTo({ name: 'login', query: { callback: to.fullPath } })
      }
      if (to.path.startsWith('/panel')) {
        if (data.value.has_completed_onboarding === false) {
          return await navigateTo({ path: '/account/onboarding', query: { callback: to.fullPath } })
        }
      }
      if (to.path === '/account/onboarding') {
        if (data.value.has_completed_onboarding === true) {
          const callback = to.query.callback?.toString()
          if (callback?.startsWith('/')) {
            return callback
          }
          return '/panel'
        }
      }
    } catch {
      return await navigateTo({ name: 'login', query: { callback: to.fullPath } })
    }
  }
})
