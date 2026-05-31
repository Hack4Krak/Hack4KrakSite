export default defineNuxtRouteMiddleware(async () => {
  const { status } = await useAuth('/event/participate', {
    onResponseError: undefined,
  })

  if (status.value !== 'success')
    return '/panel/event/register'
})
