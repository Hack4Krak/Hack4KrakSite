<script setup lang="ts">
const props = withDefaults(defineProps<{
  // A number with optional text around it, e.g. "300+"
  value: string
  duration?: number
}>(), {
  duration: 900,
})

const parsed = computed(() => {
  const match = props.value.match(/^(\D*)(\d+)(\D*)$/)
  return match ? { prefix: match[1]!, target: Number(match[2]), suffix: match[3]! } : null
})

// Starts at the final number, so the server render and the no-JavaScript page show the real value
const current = ref(parsed.value?.target ?? 0)
const root = useTemplateRef('root')
const reducedMotion = usePreferredReducedMotion()

let frame: number | undefined

function play() {
  const target = parsed.value?.target
  if (target === undefined || frame !== undefined || reducedMotion.value === 'reduce')
    return

  const start = performance.now()
  const tick = (now: number) => {
    const progress = Math.min((now - start) / props.duration, 1)
    current.value = Math.round(target * (1 - (1 - progress) ** 3))
    frame = progress < 1 ? requestAnimationFrame(tick) : undefined
  }
  frame = requestAnimationFrame(tick)
}

const { stop } = useIntersectionObserver(root, ([entry]) => {
  if (!entry?.isIntersecting)
    return
  play()
  stop()
}, { threshold: 0.6 })

onBeforeUnmount(() => frame !== undefined && cancelAnimationFrame(frame))
</script>

<template>
  <!-- The final value stays in the layout invisibly, so the width does not jump while counting -->
  <span ref="root" class="inline-grid tabular-nums">
    <span class="invisible col-start-1 row-start-1" aria-hidden="true">{{ value }}</span>
    <span v-if="parsed" class="col-start-1 row-start-1" data-allow-mismatch="text">{{ parsed.prefix }}{{ current }}{{ parsed.suffix }}</span>
    <span v-else class="col-start-1 row-start-1">{{ value }}</span>
  </span>
</template>
