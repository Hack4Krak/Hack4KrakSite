export default defineNuxtRouteMiddleware(async (to) => {
  try {
    const user = await useAuth('/account/', {
      redirect: 'error',
    })
    if (!user.error.value && user.data.value) {
      return (to.query.callback as string) || '/panel'
    }
  } catch (error) {
    console.error(error)
  }
})
