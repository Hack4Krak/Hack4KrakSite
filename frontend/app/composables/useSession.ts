// Current auth implementation is temporary and missing many features.
// See: https://github.com/Hack4Krak/Hack4KrakSite/issues/435

export default function useSession() {
  const { $auth } = useNuxtApp()

  async function logout() {
    await $auth('/auth/logout', {
      method: 'POST',
    })

    await refreshNuxtData()
    await navigateTo('/login')
  }

  return { logout }
}
