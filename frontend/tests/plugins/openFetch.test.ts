import { useNuxtApp } from 'nuxt/app'
import { createFetch } from 'ofetch'
import { afterEach, describe, expect, it, vi } from 'vitest'

describe('auth openFetch client', () => {
  const originalFetch = globalThis.$fetch

  afterEach(() => {
    globalThis.$fetch = originalFetch
    vi.restoreAllMocks()
  })

  it('refreshes the access token and retries the original request', async () => {
    const requests: string[] = []

    const fetch = vi.fn(async (input: string | URL | Request) => {
      const url = typeof input === 'string' ? input : input instanceof URL ? input.toString() : input.url
      requests.push(new URL(url).pathname)

      if (url.endsWith('/account/') && requests.filter(request => request === '/account/').length === 1) {
        return new Response(JSON.stringify({ error: 'Unauthorized', message: 'Unauthorized' }), {
          status: 401,
          headers: { 'content-type': 'application/json' },
        })
      }

      if (url.endsWith('/auth/refresh')) {
        return new Response(null, {
          status: 200,
          headers: { 'set-cookie': 'access_token=fresh; Path=/; HttpOnly' },
        })
      }

      if (url.endsWith('/account/')) {
        return new Response(JSON.stringify({ username: 'RefreshedUser' }), {
          status: 200,
          headers: { 'content-type': 'application/json' },
        })
      }

      throw new Error(`Unexpected request: ${url}`)
    })
    globalThis.$fetch = createFetch({ fetch }) as typeof globalThis.$fetch

    const { $auth } = useNuxtApp() as unknown as {
      $auth: (url: string, options?: Record<string, unknown>) => Promise<unknown>
    }

    await expect($auth('/account/', { redirect: 'error', onResponseError: undefined })).resolves.toEqual({ username: 'RefreshedUser' })
    expect(requests).toEqual(['/account/', '/auth/refresh', '/account/'])
  })

  it('does not retry the original request when token refresh fails', async () => {
    const requests: string[] = []

    const fetch = vi.fn(async (input: string | URL | Request) => {
      const url = typeof input === 'string' ? input : input instanceof URL ? input.toString() : input.url
      requests.push(new URL(url).pathname)

      if (url.endsWith('/account/')) {
        return new Response(JSON.stringify({ error: 'Unauthorized', message: 'Unauthorized' }), {
          status: 401,
          headers: { 'content-type': 'application/json' },
        })
      }

      if (url.endsWith('/auth/refresh')) {
        return new Response(JSON.stringify({ error: 'Unauthorized', message: 'Unauthorized' }), {
          status: 401,
          headers: { 'content-type': 'application/json' },
        })
      }

      throw new Error(`Unexpected request: ${url}`)
    })
    globalThis.$fetch = createFetch({ fetch }) as typeof globalThis.$fetch

    const { $auth } = useNuxtApp() as unknown as {
      $auth: (url: string, options?: Record<string, unknown>) => Promise<unknown>
    }

    await expect($auth('/account/', { redirect: 'error' })).rejects.toMatchObject({ status: 401 })
    expect(requests).toEqual(['/account/', '/auth/refresh'])
  })
})
