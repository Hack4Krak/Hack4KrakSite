<script setup lang="ts">
const props = defineProps<{
  question: string
}>()

const { proxy } = useScriptUmamiAnalytics()
const openedQuestion = ref<string>()

const items = computed(() => [{
  label: props.question,
  value: props.question,
}])

watch(openedQuestion, (value) => {
  if (value === props.question) {
    proxy.track('faq_open', {
      question: props.question,
    })
  }
})
</script>

<template>
  <UAccordion
    v-model="openedQuestion"
    trailing-icon="pixelarticons:chevron-down"
    :ui="{
      label: 'text-lg',
      trailingIcon: 'text-primary size-6',
      trigger: 'cursor-pointer',
      body: 'cursor-text text-md font-light',
    }"
    :items="items"
    :unmount-on-hide="false"
  >
    <template #body>
      <slot />
    </template>
  </UAccordion>
</template>
