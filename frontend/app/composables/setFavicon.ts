export function setFavicon() {
  const colorMode = useColorMode()
  const favicon = computed(() => {
    return colorMode.value === 'dark' ? '/favicon-dark.ico' : '/favicon-light.ico'
  })

  useHead({
    link: [
      {
        rel: 'icon',
        type: 'image/x-icon',
        href: favicon.value,
      },
    ],
  })
}
