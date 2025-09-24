<script setup lang="ts">
import file from '~~/content/faq.json'

definePageMeta({
  layout: 'static-page',
})

useSeoMeta({
  title: 'FAQ',
  description: 'Najczęściej zadawane pytania dotyczące naszego wydarzenia!',
})
</script>

<template>
  <h1 class="lg:-translate-y-1/3 font-bold text-center p-2 stroked-text-3 text-3xl lg:text-6xl">
    FAQ
  </h1>
  <div class="mt-5 prose prose-invert pb-15 max-w-[65ch]">
    <div class="font-light mt-3 text-center text-xl mb-12">
      Czyli najczęściej zadawane pytania:
    </div>
    <UAccordion
      class="cursor-pointer mt-8" type="multiple" trailing-icon="mdi:chevron-down" :ui="{
        label: 'text-lg',
        trailingIcon: 'text-primary size-6',
        trigger: 'cursor-pointer',
        body: 'cursor-text text-md font-light',
      }" :items="file" :unmount-on-hide="false"
    >
      <template #body="{ item }">
        <LazyMarkdownContent :text="item.content" hydrate-never />
      </template>
    </UAccordion>
  </div>
</template>
