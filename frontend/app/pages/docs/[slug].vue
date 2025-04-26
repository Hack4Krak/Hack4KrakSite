<script setup lang="ts">
const slug = useRoute('docs-slug').params.slug

const { data: page } = await useAsyncData(`docs-${slug}`, () => queryCollection('content').path(`/${slug}`).first())

if (!page.value) {
  showError({
    statusCode: 404,
    message: 'Zadanie nie zostało znalezione',
  })
}

useSeoMeta({
  title: page.value?.title,
})
</script>

<template>
  <div class="w-full mt-5 md:pl-[25svw] pl-[15svw] prose prose-invert">
    <ContentRenderer v-if="page" :value="page" class="md:w-[50svw] w-[70svw]" />
  </div>
</template>
