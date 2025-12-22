export async function useNavbarUser() {
  const refreshToken = useCookie('refresh_token').value
  if (refreshToken) {
    const payload = JSON.parse(atob(refreshToken.split('.')[1] ?? ''))
    const email = payload?.email

    if (email) {
      const { data } = await useApi('/users/username-by-email', {
        method: 'POST',
        body: { email },
      })
      return data?.value
    }
  }
  return undefined
}
