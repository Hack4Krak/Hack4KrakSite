<script setup lang="ts">
const items = [
  'https://picsum.photos/640/320?random=1',
  'https://picsum.photos/640/320?random=2',
  'https://picsum.photos/640/320?random=3',
  'https://picsum.photos/640/320?random=4',
  'https://picsum.photos/640/320?random=5',
  'https://picsum.photos/640/320?random=6',
]
</script>

<template>
  <UContainer>
    <UCard class="mt-10">
      <template #header>
        <div class="flex justify-between">
          <h1 class="text-2xl font-bold">
            Welcome to Nuxt UI Starter
          </h1>
          <ColorScheme>
            <USelect
              v-model="$colorMode.preference"
              :items="['system', 'light', 'dark']"
            />
          </ColorScheme>
        </div>
      </template>
      <UCarousel
        v-slot="{ item }"
        dots
        loop
        :autoplay="{ delay: 2000 }"
        :items="items"
        class="w-full max-w-xs mb-6"
      >
        <img
          :src="item"
          width="640"
          height="320"
          class="rounded-lg"
        >
      </UCarousel>
      <template #footer>
        <UButton
          icon="i-lucide-book-open"
          to="https://ui3.nuxt.dev"
          target="_blank"
        >
          Open Nuxt UI v3 Documentation
        </UButton>
      </template>
    </UCard>
  </UContainer>
</template>
