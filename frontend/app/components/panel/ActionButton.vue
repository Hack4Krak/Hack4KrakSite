<script setup lang="ts">
interface Props {
  to?: string | Record<string, unknown>
  href?: string
  target?: string
  download?: boolean | string
  type?: 'button' | 'submit' | 'reset'
  icon?: string
  disabled?: boolean
  tone?: 'primary' | 'neutral' | 'danger' | 'success'
  filled?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  type: 'button',
  tone: 'primary',
})

const color = computed(() => props.tone === 'danger' ? 'error' : props.tone === 'success' ? 'success' : props.tone === 'neutral' ? 'neutral' : 'primary')
const actionClass = computed(() => [
  'inline-flex w-fit items-center justify-center gap-2 border-2 px-4 py-2 text-sm font-bold transition-colors disabled:cursor-not-allowed disabled:opacity-40',
  props.filled
    ? props.tone === 'success'
      ? 'border-success bg-success text-default hover:opacity-90'
      : 'border-primary bg-primary text-default hover:bg-secondary'
    : props.tone === 'danger'
      ? 'border-error/70 text-error hover:border-error hover:bg-error hover:text-default disabled:border-surface-muted disabled:text-muted'
      : props.tone === 'neutral'
        ? 'border-surface-muted text-default hover:border-primary hover:text-primary'
        : 'border-primary/70 text-primary hover:bg-primary hover:text-default',
  props.disabled ? 'cursor-not-allowed' : 'cursor-pointer',
])
</script>

<template>
  <UButton
    :to="to"
    :href="href"
    :target="target"
    :download="download"
    :type="type"
    :disabled="disabled"
    :color="color"
    variant="ghost"
    :icon="icon"
    :class="actionClass"
    :ui="{ leadingIcon: 'size-4' }"
  >
    <slot />
  </UButton>
</template>
