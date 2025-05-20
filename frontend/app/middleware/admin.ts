export default defineNuxtRouteMiddleware(async () => {
  try {
    const { data, error } = await useAuth('/admin/', {
      redirect: 'error',
    })
    if (error.value || !data.value) {
      showError({
        statusCode: 403,
        message: 'Ta strona dostępna jest tylko dla administratorów.',
      })
    }
  } catch (error) {
    console.error(error)
  }
})
