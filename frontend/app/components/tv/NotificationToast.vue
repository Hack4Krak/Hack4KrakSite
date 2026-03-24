<script setup lang="ts">
const props = defineProps<{
  type: 'first-solve' | 'all-solved'
  taskName: string
  teamName?: string
  teamColor?: string
}>()

const emit = defineEmits<{
  dismiss: []
}>()

const visible = ref(false)

onMounted(() => {
  requestAnimationFrame(() => {
    visible.value = true
  })
  setTimeout(() => {
    visible.value = false
    setTimeout(emit, 500, 'dismiss')
  }, 5000)
})

const isAllSolved = computed(() => props.type === 'all-solved')
</script>

<template>
  <Transition name="tv-toast">
    <div
      v-if="visible"
      class="fixed bottom-8 left-1/2 -translate-x-1/2 z-50 max-w-2xl w-full px-4"
    >
      <div
        class="flex items-center gap-5 px-8 py-5 border-2" :class="[
          isAllSolved
            ? 'bg-surface-default border-primary'
            : 'bg-surface-default border-surface-muted',
        ]"
      >
        <div
          class="shrink-0 w-14 h-14 flex items-center justify-center border-2" :class="[
            isAllSolved ? 'border-primary' : 'border-surface-muted',
          ]"
        >
          <UIcon
            :name="isAllSolved ? 'pixelarticons:trophy' : 'pixelarticons:flag'"
            class="text-3xl" :class="[
              isAllSolved ? 'text-primary' : 'text-green-500',
            ]"
          />
        </div>

        <div class="flex flex-col gap-1 min-w-0">
          <span class="text-xs uppercase tracking-widest text-text-muted">
            {{ isAllSolved ? 'Wszystkie drużyny ukończyły' : 'Nowe rozwiązanie' }}
          </span>
          <span class="text-xl font-bold text-text-default truncate">
            {{ taskName }}
          </span>
          <div v-if="teamName && !isAllSolved" class="flex items-center gap-2">
            <div
              class="w-2.5 h-2.5 shrink-0"
              :style="{ backgroundColor: teamColor || '#737373' }"
            />
            <span class="text-sm text-text-muted">
              {{ teamName }}
            </span>
          </div>
        </div>

        <div v-if="isAllSolved" class="ml-auto shrink-0">
          <UIcon name="pixelarticons:check-double" class="text-4xl text-primary" />
        </div>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.tv-toast-enter-active,
.tv-toast-leave-active {
  transition: all 0.5s cubic-bezier(0.16, 1, 0.3, 1);
}

.tv-toast-enter-from,
.tv-toast-leave-to {
  opacity: 0;
  transform: translate(-50%, 100%);
}
</style>
