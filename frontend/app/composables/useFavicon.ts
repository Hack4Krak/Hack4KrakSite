import { usePreferredDark } from '@vueuse/core'

export function useFavicon() {
  const isDarkMode = usePreferredDark()
  const favicon = computed(() => {
    return isDarkMode.value ? '/favicon.ico' : '/favicon-light.ico'
  })

  watch(favicon, (newFavicon) => {
    useHead({
      link: [
        {
          rel: 'icon',
          type: 'image/x-icon',
          href: newFavicon,
        },
      ],
    })
  }, { immediate: true })
}
