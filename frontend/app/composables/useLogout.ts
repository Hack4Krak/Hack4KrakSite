export function useLogout() {
  const { $auth } = useNuxtApp()

  async function logout() {
    await $auth('/auth/logout', {
      method: 'POST',
      credentials: 'include',
    })
    await refreshNuxtData()
    await navigateTo('/login')
  }

  return { logout }
}
