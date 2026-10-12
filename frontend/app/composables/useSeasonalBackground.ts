export default function useSeasonalBackground() {
  return useState('seasonal-background', () => {
    const today = new Date().toISOString().slice(5, 10)

    // TODO: Add winter and spring once their artwork is ready
    if (today >= '12-22')
      return '/img/backgrounds/summer.webp'
    else if (today >= '09-23')
      return '/img/backgrounds/autumn.webp'
    else
      return '/img/backgrounds/summer.webp'
  })
}
