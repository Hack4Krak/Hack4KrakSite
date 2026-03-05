<script setup lang="ts">
const text = ref('hello world! this is a typing anim test')
const speed = ref(40)

function restart() {
  const current = text.value
  text.value = ''
  nextTick(() => {
    text.value = current
  })
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-900">
    <div class="text-center">
      <AnimatedText :text="text" :speed="speed" class="text-white text-3xl font-mono" />

      <div class="mt-8 flex flex-col gap-3 items-center">
        <input v-model="text" type="text" class="bg-gray-800 text-white border border-cyan-400 rounded px-3 py-2 w-80 font-mono">

        <label class="text-white text-sm flex items-center gap-2">
          speed in ms:
          <input v-model.number="speed" type="number" min="5" max="500" class="bg-gray-800 text-white border border-cyan-400 rounded px-2 py-1 w-20 font-mono">
        </label>

        <button class="bg-cyan-400 text-gray-900 px-6 py-2 rounded font-mono" @click="restart">
          restart
        </button>
      </div>
    </div>
  </div>
</template>
