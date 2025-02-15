<script setup lang="ts">
import { computed, ref } from 'vue'

const props = defineProps<{
  storyDialogues: { title: string, message: string }[]
  image: string
}>()

const emit = defineEmits(['complete'])

const phraseIndex = ref(0)
const displayedPhrase = ref('')
const currentMessage = computed(() => props.storyDialogues[phraseIndex.value] ?? { title: '', message: '' })

let interval: NodeJS.Timeout | undefined

function typeMessage(message: string) {
  displayedPhrase.value = message[0] ?? ' '
  let index = 1
  interval = setInterval(() => {
    if (index < message.length) {
      displayedPhrase.value += message[index]
      index++
    } else {
      clearInterval(interval)
    }
  }, 20)
}

function nextDialogue() {
  if (phraseIndex.value < props.storyDialogues.length - 1) {
    phraseIndex.value++
    clearInterval(interval)
  } else {
    emit('complete')
  }
}

function prevDialogue() {
  if (phraseIndex.value > 0) {
    phraseIndex.value--
    clearInterval(interval)
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

watch(currentMessage, (newCharacter) => {
  typeMessage(newCharacter.message)
})

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
  typeMessage(currentMessage.value.message)
})
</script>

<template>
  <div
    class="rendering-pixelated relative max-w-full max-h-screen justify-end flex flex-col bg-cover bg-center cursor-pointer"
    :style="{ backgroundImage: `url('${props.image}')` }"
    @click="handleClick"
  >
    <div>
      <div class="relative w-full mx-auto p-4 bg-black/80 text-white shadow-lg text-center">
        <div class="overflow-y-auto mb-10 ">
          <MDC :value="currentMessage.title" class="font-extrabold text-2xl sm:text-3xl md:text-4xl mb-3" />
          <div class="flex justify-center items-center mb-3">
            <div class="w-[12px] h-[12px] bg-yellow-500" />
            <div class="w-1/2 h-[2px] bg-yellow-500" />
            <div class="w-[12px] h-[12px] bg-yellow-500" />
          </div>
          <MDC :value="displayedPhrase" class="text-xl md:text-2xl font-semibold" />
        </div>
      </div>
    </div>
  </div>
</template>
