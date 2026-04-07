const apiProxyPrefix = /^\/api-proxy/

export default defineEventHandler(async (event) => {
  if (!import.meta.dev) {
    throw createError({ status: 404 })
  }

  const backendAddress = process.env.BACKEND_ADDRESS || 'http://localhost:8080'
  const normalizedBackendAddress = backendAddress.includes('://') ? backendAddress : `http://${backendAddress}`
  const targetPath = event.path.replace(apiProxyPrefix, '') || '/'

  return proxyRequest(event, `${normalizedBackendAddress}${targetPath}`, {
    cookieDomainRewrite: { '*': '' },
  })
})
