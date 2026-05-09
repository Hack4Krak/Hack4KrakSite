export default defineNuxtRouteMiddleware(async () => {
  const { data, error } = await useApi('/event/status', {
    onResponseError: undefined,
  })

  if (data.value?.is_live)
    return

  if (error.value)
    return showError(error.value)

  return showError({
    statusCode: 403,
    message: 'Panel CTF będzie dostępny podczas wydarzenia.',
    data: data.value,
  })
})
