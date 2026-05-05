<script setup lang="ts">
interface ChecklistItem {
  id: string
  title: string
  meta?: string
  done: boolean
  meAction?: { label: string, to?: string, onClick?: () => void }
}

const props = defineProps<{
  items: ChecklistItem[]
  title?: string
}>()

const doneCount = computed(() => props.items.filter(i => i.done).length)
</script>

<template>
  <article class="panel-card">
    <header class="panel-card-header border-b-2 border-surface-muted bg-surface-muted/20">
      <h2 class="panel-card-title">
        {{ title ?? 'Status zgłoszenia' }}
      </h2>
      <span class="text-xs text-muted tabular-nums">
        {{ doneCount }}/{{ items.length }} gotowe
      </span>
    </header>

    <ul class="panel-card-body space-y-2">
      <li
        v-for="item in items"
        :key="item.id"
        class="grid grid-cols-[auto_1fr_auto] gap-3 items-start py-2"
        :class="item.done ? '' : 'opacity-80'"
      >
        <span
          class="size-6 flex items-center justify-center border-2 shrink-0 mt-0.5"
          :class="item.done
            ? 'bg-success border-success text-default'
            : 'border-surface-muted text-muted'"
        >
          <UIcon v-if="item.done" name="pixelarticons:check" class="size-4" />
          <UIcon v-else name="pixelarticons:alert" class="size-4" />
        </span>

        <div class="min-w-0">
          <p class="text-sm leading-snug font-medium">
            {{ item.title }}
          </p>
          <p
            v-if="item.meta"
            class="text-xs mt-1"
            :class="item.done ? 'text-muted/70' : 'text-muted'"
          >
            {{ item.meta }}
          </p>
        </div>

        <component
          :is="item.meAction?.to ? 'NuxtLink' : 'button'"
          v-if="item.meAction"
          :to="item.meAction.to"
          type="button"
          class="text-xs font-bold uppercase tracking-wider text-primary hover:text-secondary cursor-pointer transition-colors self-start whitespace-nowrap"
          @click="item.meAction.onClick?.()"
        >
          {{ item.meAction.label }} →
        </component>
      </li>
    </ul>
  </article>
</template>
