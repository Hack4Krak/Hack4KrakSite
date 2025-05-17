<script setup lang="ts">
const slug = useRoute('docs-slug').params.slug
const { data: page } = await useAsyncData(`docs-${slug}`, () => queryCollection('content').path(`/pages/${slug}`).first())

definePageMeta({
  layout: 'static-page',
})

if (!page.value?.title) {
  showError({
    statusCode: 404,
    message: 'Strona nie została znalezione',
  })
}

useSeoMeta({
  title: page.value?.title,
  description: page.value?.description,
})
</script>

<template>
  <h1 class="lg:-translate-y-1/3 font-bold text-center p-2 stroked-text-3 text-3xl lg:text-6xl">
    {{ page && page.title }}
  </h1>
  <div class="mt-5 prose prose-invert pb-8">
    <ContentRenderer v-if="page" :value="page" />
  </div>
</template>
