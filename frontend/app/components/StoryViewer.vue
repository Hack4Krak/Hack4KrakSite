<script setup lang="ts">
import { computed, ref } from 'vue'

const props = defineProps<{
  storyDialogues: { title: string, message: string }[]
  image: string
}>()

const emit = defineEmits(['complete'])

const characterIndex = ref(0)
const displayedMessage = ref('')
const currentCharacter = computed(() => props.storyDialogues[characterIndex.value] ?? { title: '', message: '' })

let interval: NodeJS.Timeout

function typeMessage(message: string) {
  displayedMessage.value = message[0] ?? ' '
  let index = 1
  interval = setInterval(() => {
    if (index < message.length) {
      displayedMessage.value += message[index]
      index++
    } else {
      clearInterval(interval)
    }
  }, 20)
}

function nextDialogue() {
  if (characterIndex.value < props.storyDialogues.length - 1) {
    characterIndex.value++
    clearInterval(interval)
  } else {
    emit('complete')
  }
}

function prevDialogue() {
  if (characterIndex.value > 0) {
    characterIndex.value--
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

watch(currentCharacter, (newCharacter) => {
  typeMessage(newCharacter.message)
})

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
  typeMessage(currentCharacter.value.message)
})
</script>

<template>
  <div
    class="rendering-pixelated relative max-w-full max-h-screen justify-end flex flex-col bg-cover bg-center cursor-pointer"
    :style="{ backgroundImage: `url('${props.image}')` }"
    @click="nextDialogue"
  >
    <div>
      <div class="relative w-full mx-auto p-4 bg-black/80 text-white shadow-lg text-center">
        <div class="overflow-y-auto mb-10 ">
          <MDC :value="currentCharacter.title" class="font-extrabold text-2xl sm:text-3xl md:text-4xl mb-3" />
          <div class="flex justify-center items-center mb-3">
            <div class="w-[12px] h-[12px] bg-[#F6B216]" />
            <div class="w-1/2 h-[2px] bg-[#F6B216]" />
            <div class="w-[12px] h-[12px] bg-[#F6B216]" />
          </div>
          <MDC :value="displayedMessage" class="text-xl md:text-2xl font-semibold" />
        </div>
      </div>
    </div>
  </div>
</template>
