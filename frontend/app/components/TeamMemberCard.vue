<script setup lang="ts">
const props = defineProps<{
  name: string
  email: string
  role: string
  github?: string
  linkedin?: string
}>()
</script>

<template>
  <div class="flex flex-col gap-1 border border-primary text-center w-80">
    <div class="flex justify-center items-center font-semibold">
      {{ props.name }}
      <a
        v-if="props.linkedin"
        :href="props.linkedin"
        target="_blank"
        rel="noopener noreferrer"
        class="inline-flex items-center"
      >
        <UIcon v-if="props.linkedin" name="mdi:linkedin" size="16px" class="ml-1.5 transition-transform duration-400 hover:scale-125" />
      </a>
      <a
        v-if="props.github"
        :href="props.github"
        target="_blank"
        rel="noopener noreferrer"
        class="inline-flex items-center"
      >
        <UIcon v-if="props.github" name="mdi:github" size="16px" class="ml-1.5 transition-transform duration-400 hover:scale-125" />
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
