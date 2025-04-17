<script setup lang="ts">
const props = withDefaults(defineProps<{
  text: string
  speed?: number
}>(), {
  speed: 20,
})

const displayed = ref('')
let interval: NodeJS.Timeout | undefined

function typeText(text: string, speed: number) {
  clearInterval(interval)
  displayed.value = text[0] ?? ''
  let index = 1
  interval = setInterval(() => {
    if (index < text.length) {
      displayed.value += text[index]
      index++
    } else {
      clearInterval(interval)
    }
  }, speed)
}

watch(() => props.text, (newText) => {
  typeText(newText, props.speed)
})

onMounted(() => {
  typeText(props.text, props.speed)
})

onBeforeUnmount(() => {
  clearInterval(interval)
})
</script>

<template>
  <span>{{ displayed }}</span>
</template>
