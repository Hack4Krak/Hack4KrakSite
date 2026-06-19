export default defineNuxtRouteMiddleware(async (to) => {
  try {
    const user = await useAuth('/account/', {
      redirect: 'error',
    })
    if (!user.error.value && user.data.value) {
      const callback = to.query.callback?.toString()
      return callback?.startsWith('/') ? callback : '/account'
    }
  } catch (error) {
    console.error(error)
  }
})
