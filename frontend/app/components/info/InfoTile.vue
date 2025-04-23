<script setup lang="ts">
const { content } = defineProps<{
  content: {
    front: string
    back: string
    color: string
    image: string
  }
}>()

const isFlipped = ref(false)
const colorString = computed(() => `var(--color-${content.color})`)
</script>

<template>
  <div
    class="lg:aspect-square lg:grid flex flex-col md:flex-row lg:border-2
           place-content-center justify-items-center gap-y-10
           border-b-2 pb-12 pt-4
           lg:text-3xl relative cursor-pointer lg:overflow-hidden"
    :style="{
      borderColor: colorString,
    }"
    @mouseover="() => isFlipped = true"
    @mouseleave="() => isFlipped = false"
  >
    <div
      class="lg:absolute w-full h-full transition-transform duration-500 ease-in-out flex
             flex-col gap-4 lg:gap-12 items-center justify-center lg:p-0"
      :class="{ 'lg:translate-y-[-100%]': isFlipped }"
    >
      <h1 class="text-lg lg:text-2xl text-center" :style="{ color: colorString }">
        <MDC :value="content.front" />
      </h1>
      <NuxtImg :src="content.image" height="400" class="h-12 lg:h-32 rendering-pixelated" />
    </div>

    <div
      class="lg:absolute w-full h-full transition-transform duration-500 ease-in-out flex items-center justify-center"
      :class="{ 'lg:translate-y-0': isFlipped, 'lg:translate-y-[100%]': !isFlipped }"
    >
      <div class="w-full">
        <p class="text-sm text-left lg:px-12" :style="{ color: colorString }">
          <MDC :value="content.back" />
        </p>
      </div>
    </div>
  </div>
</template>
