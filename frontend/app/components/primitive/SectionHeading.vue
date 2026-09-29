<script setup lang="ts">
const props = withDefaults(defineProps<{
  subtitle?: string
  title?: string
  // Part of the title set on a tilted marker, e.g. "dla uczniów" in "Od uczniów, dla uczniów"
  highlight?: string
  align?: 'center' | 'left'
  // `lg` is the bigger title used next to content in two-column sections
  size?: 'md' | 'lg'
}>(), {
  align: 'center',
  size: 'md',
})

const parts = computed(() => {
  const start = props.highlight ? props.title?.indexOf(props.highlight) ?? -1 : -1
  if (!props.title || start < 0)
    return null
  return {
    before: props.title.slice(0, start),
    marked: props.highlight!,
    after: props.title.slice(start + props.highlight!.length),
  }
})
</script>

<template>
  <div :class="align === 'center' ? 'mx-auto max-w-3xl text-center' : 'max-w-3xl'">
    <p v-if="subtitle" class="mb-3 text-xs font-bold tracking-[0.25em] text-muted uppercase">
      {{ subtitle }}
    </p>
    <h2
      v-if="title"
      class="font-pixelify text-balance text-default"
      :class="size === 'lg' ? 'text-4xl leading-tight lg:text-6xl' : 'text-3xl lg:text-5xl'"
    >
      <template v-if="parts">
        {{ parts.before }}<span class="marker">{{ parts.marked }}</span>{{ parts.after }}
      </template>
      <template v-else>
        {{ title }}
      </template>
    </h2>
  </div>
</template>

<style scoped>
.marker {
  display: inline-block;
  rotate: -2deg;
  margin-top: 0.1em;
  background: var(--ui-primary);
  padding-inline: 0.2em;
  color: var(--color-ink);
  box-shadow: 0.08em 0.08em 0 var(--color-ink);
}
</style>
