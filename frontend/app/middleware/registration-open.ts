export default defineNuxtRouteMiddleware(async () => {
  const { data: registrationInformation } = await useApi('/event/registration')

  if (!registrationInformation.value?.is_open)
    return '/account/events'
})
