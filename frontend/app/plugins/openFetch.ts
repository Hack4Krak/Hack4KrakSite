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

          const response = await $fetch.raw('/auth/refresh', {
            method: 'POST',
            baseURL: clients.auth.baseURL,
            headers,
            credentials: 'include',
            onResponseError() {
              nuxtApp.runWithContext(() => {
                navigateTo('login')
              })
            },
          })

          const cookies = (response.headers.get('set-cookie') || '').split(',')
          context.options.headers.set('cookie', cookies.map(c => c.split(';')[0]).join(';'))

          if (import.meta.server) {
            const cookie = (response.headers.get('set-cookie') || '').split(',')
            nuxtApp.ssrContext?.event.node.res.setHeader('Set-Cookie', cookie)
          }
        },
      }), localFetch),
    },
  }
})
