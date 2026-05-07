export default defineNuxtRouteMiddleware(async () => {
  const { isRegistered } = useEventRegistration()

  if (!isRegistered)
    return '/panel/event/register'
})
