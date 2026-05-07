<script setup lang="ts">
const props = defineProps<{
  name: string
  email: string
  role: string
  highlight?: boolean
  github?: string
  linkedin?: string
}>()
</script>

<template>
  <div
    class="flex flex-col gap-1 transition duration-200 border-2 text-center min-w-60 w-80"
    :class="{ 'border-muted hover:border-primary': !props.highlight, 'border-primary bg-primary/5 hover:bg-primary/10': props.highlight }"
  >
    <div class="flex justify-center items-center">
      {{ props.name }}
      <a
        v-if="props.linkedin"
        :href="props.linkedin"
        target="_blank"
        rel="noopener noreferrer"
        class="inline-flex items-center"
      >
        <UIcon
          v-if="props.linkedin"
          name="mdi:linkedin"
          size="16px"
          class="ml-1.5 transition-transform duration-400 hover:scale-125"
        />
      </a>
      <a
        v-if="props.github"
        :href="props.github"
        target="_blank"
        rel="noopener noreferrer"
        class="inline-flex items-center"
      >
        <UIcon
          v-if="props.github"
          name="mdi:github"
          size="16px"
          class="ml-1.5 transition-transform duration-400 hover:scale-125"
        />
      </a>
    </div>
    <div class="text-muted">
      {{ props.email }}
    </div>
    <div class="mt-3">
      {{ props.role }}
    </div>
  </div>
</template>
