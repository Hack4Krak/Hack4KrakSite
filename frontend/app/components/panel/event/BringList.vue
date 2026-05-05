<script setup lang="ts">
interface BringItem {
  icon: string
  title: string
  description: string
  required?: boolean
}

defineProps<{
  items: BringItem[]
  title?: string
}>()
</script>

<template>
  <article class="panel-card">
    <header class="panel-card-header border-b-2 border-surface-muted bg-surface-muted/20">
      <h2 class="panel-card-title">
        {{ title ?? 'Co zabrać ze sobą' }}
      </h2>
    </header>

    <div class="panel-card-body grid md:grid-cols-2 gap-x-6 gap-y-4">
      <article
        v-for="item in items"
        :key="item.title"
        class="grid grid-cols-[auto_1fr] gap-4 items-start"
      >
        <div
          class="size-10 border-2 border-surface-muted bg-surface-muted/20 flex items-center justify-center shrink-0"
          :class="item.required !== false ? 'border-primary text-primary' : 'text-muted'"
        >
          <UIcon
            :name="item.icon"
            class="size-5"
          />
        </div>

        <div>
          <p class="font-bold leading-tight">
            {{ item.title }}
          </p>
          <p class="text-sm text-muted mt-1 leading-relaxed">
            {{ item.description }}
          </p>
          <span
            v-if="item.required === false"
            class="inline-block text-xs uppercase tracking-wider text-muted border border-surface-muted px-2 py-0.5 mt-3"
          >
            opcjonalne
          </span>
          <span
            v-else
            class="inline-block text-xs uppercase tracking-wider text-primary border border-primary px-2 py-0.5 mt-3"
          >
            wymagane
          </span>
        </div>
      </article>
    </div>
  </article>
</template>
