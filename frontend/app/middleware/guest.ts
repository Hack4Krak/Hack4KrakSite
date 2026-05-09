export default defineNuxtRouteMiddleware(async () => {
  try {
    const user = await useAuth('/account/', {
      redirect: 'error',
    })
    if (!user.error.value && user.data.value) {
      return '/account'
    }
  } catch (error) {
    console.error(error)
  }
})
