<script setup lang="ts">
const { content } = defineProps<{
  content: {
    front: string
    back: string
    color: string
    image: string
  }
}>()

// Mobile tap-to-flip state
const flipped = ref(false)
</script>

<template>
  <div
    class="lg:aspect-square lg:grid flex flex-col
           place-content-center justify-items-center
           border-2 lg:text-3xl relative cursor-pointer lg:overflow-hidden
           group select-none"
    :style="{ borderColor: content.color }"
    @click="flipped = !flipped"
  >
    <!-- FRONT face -->
    <div
      class="lg:absolute w-full h-full transition-transform duration-500 ease-in-out flex
             flex-col gap-4 lg:gap-12 items-center justify-center p-6 lg:p-0
             lg:group-hover:translate-y-[-100%]"
      :class="flipped ? 'hidden lg:flex' : 'flex'"
    >
      <div class="text-lg lg:text-2xl text-center" :style="{ color: content.color }">
        <MarkdownContent :prose="false" :text="content.front" />
      </div>
      <NuxtImg :src="content.image" alt="" height="400" class="h-16 lg:h-32 rendering-pixelated" />
      <!-- Mobile hint -->
      <p class="text-[10px] uppercase tracking-widest text-muted lg:hidden">
        Dotknij, aby zobaczyć więcej
      </p>
    </div>

    <!-- BACK face -->
    <div
      class="lg:absolute w-full h-full transition-transform duration-500 ease-in-out flex items-center justify-center
             p-6
             lg:translate-y-[100%] lg:group-hover:translate-y-0"
      :class="flipped ? 'flex' : 'hidden lg:flex'"
    >
      <div class="w-full">
        <div class="text-[1rem] text-pretty text-left lg:px-12">
          <MarkdownContent :prose="false" :text="content.back" :style="{ color: content.color }" />
        </div>
        <!-- Mobile back-to-front hint -->
        <p class="text-[10px] uppercase tracking-widest text-muted mt-4 lg:hidden text-center">
          Dotknij, aby wrócić
        </p>
      </div>
    </div>
  </div>
</template>
