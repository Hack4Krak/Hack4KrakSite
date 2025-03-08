export default defineNuxtPlugin((nuxtApp) => {
  const clients = useRuntimeConfig().public.openFetch
  const localFetch = useRequestFetch()

  const headers = import.meta.server ? useRequestHeaders(['cookie']) || {} : {}

  return {
    provide: {
      api: createOpenFetch(localOptions => ({
        ...clients.api,
        headers,
        async onResponseError(context) {
          const data = context.response._data
          if (!data.error) {
            return
          }

          const description = `${context.response.status}: ${data.message ?? 'Nieznany błąd'}`
          nuxtApp.runWithContext(() => useToast().add({ title: 'Błąd zapytania', description, color: 'error' }))

          context.error = data?.error
        },
        ...localOptions,
      }), localFetch as typeof $fetch),
      auth: createOpenFetch(localOptions => ({
        ...clients.auth,
        retryStatusCodes: [401],
        retry: 1,
        credentials: 'include',
        ignoreResponseError: true,
        headers,
        async onResponse(context) {
          if (context.response?._data.error && context.response?._data.error !== 'Unauthorized') {
            const hooks = context.options.onResponseError
            if (Array.isArray(hooks)) {
              hooks.forEach(hook => hook(context))
            } else if (typeof hooks === 'function') {
              hooks(context)
            }
          }

          if (context.response.status !== 401) {
            return
          }

          const response = await $fetch.raw('/auth/refresh', {
            method: 'POST',
            baseURL: clients.auth.baseURL,
            headers,
            credentials: 'include',
            onResponseError() {
              nuxtApp.runWithContext(() => {
                if (context.options.redirect !== 'error') {
                  navigateTo('/login')
                }
              })
            },
          })

          if (!response.ok) {
            if (context.options.redirect !== 'error') {
              nuxtApp.runWithContext(() => navigateTo('/login'))
            }
            return
          }

          const cookies = (response.headers.get('set-cookie') || '').split(',')
          context.options.headers.set('cookie', cookies.map(c => c.split(';')[0]).join(';'))

          if (import.meta.server) {
            nuxtApp.ssrContext?.event.node.res.setHeader('Set-Cookie', cookies)
          }
        },
        onResponseError(context) {
          const data = context.response._data
          const status = context.response.status
          if (!data.error) {
            return
          }

          // We sometimes get Unauthorized, but we handle it and refresh access token
          if (data.error === 'Unauthorized') {
            return
          }
          nuxtApp.runWithContext(() => useToast().add({
            title: `Błąd ${status}`,
            description: data.message ?? 'Nieznany błąd',
            color: 'error',
          }))

          context.error = data.error
        },
        ...localOptions,
      }), localFetch as typeof $fetch),
    },
  }
})
