export default defineNuxtRouteMiddleware(async () => {
  const { error } = await useApi('/event/status', {
    onResponseError: undefined,
  })

  if (!error?.value)
    return

  if (error.value.statusCode === 403) {
    return showError({
      statusCode: 403,
      data: {
        response: error.value.data,
      },
    })
  }

  return showError(error.value)
})
