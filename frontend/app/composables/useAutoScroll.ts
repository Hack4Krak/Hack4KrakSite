import { useIntervalFn } from '@vueuse/core'

export function useAutoScroll(containerRef: Ref<HTMLElement | null>, options: {
  speed?: number
  pauseMs?: number
} = {}) {
  const speed = options.speed ?? 1
  const pauseMs = options.pauseMs ?? 2000

  let direction: 'down' | 'up' = 'down'
  let pauseUntil = 0

  const { pause, resume } = useIntervalFn(() => {
    const el = containerRef.value
    if (!el)
      return

    const maxScroll = el.scrollHeight - el.clientHeight
    if (maxScroll <= 0)
      return

    const now = Date.now()
    if (now < pauseUntil)
      return

    if (direction === 'down') {
      el.scrollTop += speed
      if (el.scrollTop >= maxScroll) {
        el.scrollTop = maxScroll
        direction = 'up'
        pauseUntil = now + pauseMs
      }
    }
    else {
      el.scrollTop -= speed
      if (el.scrollTop <= 0) {
        el.scrollTop = 0
        direction = 'down'
        pauseUntil = now + pauseMs
      }
    }
  }, 30)

  return { pause, resume }
}
