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
  <article class="panel-card panel-card-body">
    <p class="panel-section-title mb-5">
      {{ title ?? 'Co zabrać ze sobą' }}
    </p>

    <ul class="grid md:grid-cols-2 gap-x-8 gap-y-5">
      <li
        v-for="item in items"
        :key="item.title"
        class="grid grid-cols-[auto_1fr] gap-4 items-start"
      >
        <span
          class="font-pixelify text-2xl leading-none tabular-nums mt-0.5"
          :class="item.required !== false ? 'text-primary' : 'text-muted/70'"
        >
          <UIcon
            :name="item.icon"
            class="size-6"
          />
        </span>

        <div>
          <p class="font-bold leading-tight flex items-center gap-2 flex-wrap">
            {{ item.title }}
            <span
              v-if="item.required === false"
              class="text-[10px] uppercase tracking-widest text-muted font-bold"
            >
              opcjonalne
            </span>
            <span
              v-else
              class="text-[10px] uppercase tracking-widest text-primary font-bold"
            >
              wymagane
            </span>
          </p>
          <p class="text-sm text-muted mt-1 leading-relaxed">
            {{ item.description }}
          </p>
        </div>
      </li>
    </ul>
  </article>
</template>
