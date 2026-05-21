export function useTaskCountdown(getAvailableSince: () => string | null | undefined) {
  const now = useTimestamp({ interval: 1000 })
  return computed(() => {
    const since = getAvailableSince()
    if (!since)
      return null
    const diff = new Date(since).getTime() - now.value
    if (diff <= 0)
      return null
    return humanizeDifference(diff)
  })
}
