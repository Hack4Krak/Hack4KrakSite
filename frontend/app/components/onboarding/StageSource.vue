<script setup lang="ts">
const REFERRAL_OPTIONS = [
  { value: 'Instagram', label: 'Instagram', icon: 'mdi:instagram' },
  { value: 'Linkedin', label: 'LinkedIn', icon: 'mdi:linkedin' },
  { value: 'Facebook', label: 'Facebook', icon: 'mdi:facebook' },
  { value: 'Discord', label: 'Discord', icon: 'ic:baseline-discord' },
  { value: 'Friend', label: 'Od znajomego', icon: 'lucide:users' },
  { value: 'School', label: 'W szkole', icon: 'lucide:graduation-cap' },
  { value: 'Internet', label: 'W internecie', icon: 'lucide:globe' },
  { value: 'Other', label: 'Inne', icon: 'lucide:more-horizontal' },
] as const

const sources = defineModel<string[]>('sources', { required: true })

function toggle(value: string) {
  const current = sources.value ?? []
  const idx = current.indexOf(value)
  sources.value = idx === -1 ? [...current, value] : current.filter((_, i) => i !== idx)
}

function isSelected(value: string) {
  return sources.value?.includes(value) ?? false
}
</script>

<template>
  <section class="space-y-5">
    <div>
      <h2 class="text-lg font-semibold sm:text-xl">
        Jak nas znalazłeś?
      </h2>
      <p class="text-sm text-muted">
        Możesz wybrać kilka opcji.
      </p>
    </div>

    <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
      <button
        v-for="option in REFERRAL_OPTIONS"
        :key="option.value"
        type="button"
        class="aspect-square flex flex-col items-center justify-center gap-2 p-3 border-2 transition duration-150 cursor-pointer bg-elevated/60"
        :class="isSelected(option.value)
          ? 'border-primary bg-primary/10 text-default'
          : 'border-transparent text-muted hover:border-default hover:text-default'"
        @click="toggle(option.value)"
      >
        <UIcon
          :name="option.icon"
          class="w-7 h-7 transition-colors"
          :class="isSelected(option.value) ? 'text-primary' : ''"
        />
        <span class="text-xs font-medium text-center leading-tight">
          {{ option.label }}
        </span>
      </button>
    </div>
  </section>
</template>
