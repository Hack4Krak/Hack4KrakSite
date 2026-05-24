export default defineNuxtRouteMiddleware(async () => {
  const { data, error } = await useApi('/event/status', {
    onResponseError: undefined,
  })

  if (data.value?.is_live || data.value?.is_after_event)
    return

  if (error.value)
    return showError(error.value)

  return showError({
    status: 403,
    message: 'Panel CTF będzie dostępny po rozpoczęciu wydarzenia.',
    data: {
      error: 'AccessBeforeEventStart',
      ...data.value,
    },
  })
})
