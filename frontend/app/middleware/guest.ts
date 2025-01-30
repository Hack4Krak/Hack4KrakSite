export default defineNuxtRouteMiddleware(async () => {
  try {
    const user = await useAuth('/user/')
    if (!user.error.value && user.data.value) {
      return '/panel'
    }
  } catch {
    return true
  }

  return true
})
