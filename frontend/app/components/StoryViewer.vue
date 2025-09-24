<script setup lang="ts">
const props = defineProps<{
  storyDialogues: { title: string, message: string }[]
  image: string
}>()

const emit = defineEmits(['complete'])

const phraseIndex = ref(0)
const currentMessage = computed(() => props.storyDialogues[phraseIndex.value] ?? { title: '', message: '' })

function nextDialogue() {
  if (phraseIndex.value < props.storyDialogues.length - 1) {
    phraseIndex.value++
  } else {
    emit('complete')
  }
}

function prevDialogue() {
  if (phraseIndex.value > 0) {
    phraseIndex.value--
  }
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'ArrowRight' || event.key === ' ') {
    nextDialogue()
  } else if (event.key === 'ArrowLeft') {
    prevDialogue()
  }
}

function handleClick(event: MouseEvent) {
  if (event.clientX > window.innerWidth / 2) {
    nextDialogue()
  } else {
    prevDialogue()
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <div
    class="min-h-[inherit] rendering-pixelated relative justify-end flex flex-col bg-cover bg-center cursor-pointer"
    :style="{ backgroundImage: `url('${image}')` }"
    @click="handleClick"
  >
    <div>
      <div class="relative w-full mx-auto p-4 bg-black/80 text-white shadow-lg text-center">
        <div class="overflow-y-auto mb-10 ">
          <h2 class="font-extrabold text-2xl sm:text-3xl md:text-4xl mb-3">
            {{ currentMessage.title }}
          </h2>
          <div class="flex justify-center items-center mb-3">
            <div class="w-[12px] h-[12px] bg-yellow-500" />
            <div class="w-1/2 h-[2px] bg-yellow-500" />
            <div class="w-[12px] h-[12px] bg-yellow-500" />
          </div>
          <AnimatedText :text="currentMessage.message" class="text-xl md:text-2xl" />
        </div>
      </div>
    </div>
  </div>
</template>
