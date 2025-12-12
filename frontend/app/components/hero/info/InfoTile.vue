<script setup lang="ts">
const { content } = defineProps<{
  content: {
    front: string
    back: string
    color: string
    image: string
    alt: string
  }
}>()
</script>

<template>
  <div
    class="lg:aspect-square lg:grid flex flex-col md:flex-row lg:border-2
           place-content-center justify-items-center gap-y-10
           border-b-2 pb-12 pt-4
           lg:text-3xl relative cursor-pointer lg:overflow-hidden
           group"
    :style="{
      borderColor: content.color,
    }"
  >
    <div
      class="lg:absolute w-full h-full transition-transform duration-500 ease-in-out flex
             flex-col gap-4 lg:gap-12 items-center justify-center lg:p-0
             lg:group-hover:translate-y-[-100%]"
    >
      <div class="text-lg lg:text-2xl text-center" :style="{ color: content.color }">
        <MarkdownContent :prose="false" :text="content.front" />
      </div>
      <NuxtImg :src="content.image" :alt="content.alt" height="400" class="h-12 lg:h-32 rendering-pixelated" />
    </div>

    <div
      class="lg:absolute w-full h-full transition-transform duration-500 ease-in-out flex items-center justify-center
             lg:translate-y-[100%]
             lg:group-hover:translate-y-0"
    >
      <div class="w-full">
        <div class="text-[1rem] text-pretty text-left lg:px-12">
          <MarkdownContent :prose="false" :text="content.back" :style="{ color: content.color }" />
        </div>
      </div>
    </div>
  </div>
</template>
