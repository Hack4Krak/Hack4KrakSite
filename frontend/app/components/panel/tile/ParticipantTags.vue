<script setup lang="ts">
import type { SchemaParticipantTag } from '#open-fetch-schemas/api'

const props = defineProps<{
  allTags: SchemaParticipantTag[]
  appliedTags: SchemaParticipantTag[]
}>()

const appliedIds = computed(() => new Set(props.appliedTags.map(t => t.id)))
const progress = computed(() =>
  props.allTags.length ? (props.appliedTags.length / props.allTags.length) * 100 : 0,
)
</script>

<template>
  <PanelCard body>
    <div class="flex items-baseline justify-between gap-4 mb-3">
      <PanelSectionTitle>
        Atrybuty
      </PanelSectionTitle>
      <span class="text-xs text-muted tabular-nums">
        {{ appliedTags.length }}/{{ allTags.length }}
      </span>
    </div>

    <div class="w-full h-1 bg-surface-muted mb-4">
      <div
        class="h-full bg-primary transition-all duration-500"
        :style="{ width: `${progress}%` }"
      />
    </div>

    <ul class="divide-y divide-surface-muted/60 border-t border-surface-muted/60">
      <li
        v-for="tag in allTags"
        :key="tag.id"
        class="flex items-center gap-3 py-2.5"
      >
        <span
          class="size-3.5 border-2 shrink-0 transition-colors"
          :class="appliedIds.has(tag.id) ? 'border-primary bg-primary' : 'border-surface-muted/60'"
        />
        <span
          class="text-sm flex-1 min-w-0 truncate transition-colors"
          :class="appliedIds.has(tag.id) ? 'font-medium text-default' : 'text-muted'"
        >{{ tag.name }}</span>
        <UTooltip v-if="tag.description" :text="tag.description" :delay-duration="150">
          <span
            class="inline-flex size-[17px] cursor-help items-center justify-center border border-muted/40 text-[9px] font-bold leading-none text-muted transition-colors hover:border-muted hover:text-default select-none shrink-0"
          >i</span>
        </UTooltip>
      </li>
    </ul>
  </PanelCard>
</template>
