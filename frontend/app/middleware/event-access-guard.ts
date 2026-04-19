export default defineNuxtRouteMiddleware(async () => {
  const { error } = await useApi('/event/status', {
    onResponseError: undefined,
  })

  if (!error?.value)
    return

  if (error.value.status === 403) {
    const responseData = (error.value.data ?? {}) as Record<string, any>

    return showError({
      status: 403,
      message: responseData.message,
      data: responseData,
    })
  }

  return showError(error.value)
})
