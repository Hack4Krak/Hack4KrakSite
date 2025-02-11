<script setup lang="ts">
import { computed, ref } from 'vue'

const props = defineProps<{
  storyDialogues: { title: string, message: string }[]
  image: string
}>()

const emit = defineEmits(['complete'])

const characterIndex = ref(0)

const currentCharacter = computed(() => props.storyDialogues[characterIndex.value] ?? { title: '', message: '' })

function nextDialogue() {
  if (characterIndex.value < props.storyDialogues.length - 1) {
    characterIndex.value++
  } else {
    emit('complete')
  }
}
</script>

<template>
  <div
    class="rendering-pixelated relative max-w-full max-h-screen justify-end flex flex-col bg-cover bg-center cursor-pointer"
    :style="{ backgroundImage: `url('${props.image}')` }"
    @click="nextDialogue"
  >
    <div>
      <div class="relative w-full mx-auto p-4 bg-black/80 text-white shadow-lg text-center">
        <div class="overflow-y-auto mb-15 sm:text-2xl md:text-3xl">
          <p class="font-bold mb-5">
            {{ currentCharacter.title || 'Unknown' }}
          </p>
          <div class="flex justify-center">
            <div class="w-1/2 h-px bg-yellow-500 mb-10" />
          </div>
          <p>
            {{ currentCharacter.message }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
