export default defineNuxtPlugin((nuxtApp) => {
  const clients = useRuntimeConfig().public.openFetch
  const localFetch = useRequestFetch()

  const headers = import.meta.server ? useRequestHeaders(['cookie']) || {} : {}

  return {
    provide: {
      api: createOpenFetch(clients.api, localFetch),
      auth: createOpenFetch(localOptions => ({
        ...localOptions,
        ...clients.auth,
        retryStatusCodes: [401],
        retry: 1,
        credentials: 'include',
        headers,
        async onResponse(context) {
          if (context.response.status !== 401) {
            return
          }

          try {
            const response = await $fetch.raw('/auth/refresh', {
              method: 'POST',
              baseURL: clients.auth.baseURL,
              headers,
              credentials: 'include',
              onResponseError() {
                nuxtApp.runWithContext(() => {
                  navigateTo('/login')
                })
              },
            })

            if (!response.ok)
              throw new Error('Refresh token request failed')

            const cookies = (response.headers.get('set-cookie') || '').split(',')
            context.options.headers.set('cookie', cookies.map(c => c.split(';')[0]).join(';'))

            if (import.meta.server) {
              nuxtApp.ssrContext?.event.node.res.setHeader('Set-Cookie', cookies)
            }
          } catch {
            nuxtApp.runWithContext(() => navigateTo('login'))
          }
        },
        onResponseError() {
          navigateTo('/login')
        },
      }), localFetch),
    },
  }
})
