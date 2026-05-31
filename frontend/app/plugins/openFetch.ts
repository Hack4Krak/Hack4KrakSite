import type { FetchContext, FetchOptions } from 'ofetch'

type NuxtAppWithContext = Pick<ReturnType<typeof useNuxtApp>, 'runWithContext'>
type ResponseErrorContext = FetchContext & { response: NonNullable<FetchContext['response']> }

async function handleResponseError(
  nuxtApp: NuxtAppWithContext,
  context: ResponseErrorContext,
  localOptions: FetchOptions,
) {
  const hooks = localOptions.onResponseError
  if (Array.isArray(hooks)) {
    await Promise.all(hooks.map(hook => hook(context)))
    return
  } else if (typeof hooks === 'function') {
    await hooks(context)
    return
  }

  if (Object.hasOwn(localOptions, 'onResponseError')) {
    return
  }

  const data = context.response._data
  context.error = data.error

  const description = `${context.response.status}: ${data.message ?? 'Nieznany błąd'}`
  nuxtApp.runWithContext(() => useToast().add({
    title: 'Błąd zapytania',
    description,
    color: 'error',
  }))
}

export default defineNuxtPlugin((nuxtApp) => {
  const clients = useRuntimeConfig().public.openFetch
  const localFetch = useRequestFetch()

  const headers = import.meta.server ? useRequestHeaders(['cookie']) || {} : {}

  return {
    provide: {
      api: createOpenFetch((localOptions = {}) => ({
        ...clients.api,
        headers,
        ...localOptions,
        async onResponseError(context) {
          const data = context.response._data
          if (!data.error) {
            return
          }

          await handleResponseError(nuxtApp, context, localOptions)
        },
      }), localFetch as typeof $fetch),
      auth: createOpenFetch((localOptions = {}) => ({
        ...clients.auth,
        retryStatusCodes: [401],
        retry: 1,
        credentials: 'include',
        headers,
        ...localOptions,
        async onResponseError(context) {
          const data = context.response._data
          const status = context.response.status
          if (!data.error) {
            return
          }

          if (data.error !== 'Unauthorized' || status !== 401) {
            await handleResponseError(nuxtApp, context, localOptions)
            return
          }

          if (context.options.retry === 0 || context.options.retry === false) {
            return
          }

          const response = await $fetch.raw('/auth/refresh', {
            method: 'POST',
            baseURL: clients.auth.baseURL,
            headers,
            credentials: 'include',
            ignoreResponseError: true,
          })

          if (!response.ok) {
            context.options.retry = 0
            if (context.options.redirect !== 'error') {
              nuxtApp.runWithContext(() => navigateTo('/login'))
            }
            return
          }

          if (import.meta.server) {
            const setCookie = response.headers.get('set-cookie')
            if (setCookie) {
              const cookies = setCookie.split(',')
              context.options.headers.set('cookie', cookies.map(c => c.split(';')[0]).join(';'))
              nuxtApp.ssrContext?.event.node.res.setHeader('Set-Cookie', cookies)
            }
          }
        },
      }), localFetch as typeof $fetch),
    },
  }
})
