export default defineEventHandler(async (event) => {
  if (!import.meta.dev) {
    throw createError({ status: 404 })
  }

  const backendAddress = process.env.BACKEND_ADDRESS || 'http://localhost:8080'
  const targetPath = event.path.replace(/^\/api-proxy/, '') || '/'

  return proxyRequest(event, `${backendAddress}${targetPath}`, {
    cookieDomainRewrite: { '*': '' },
  })
})
