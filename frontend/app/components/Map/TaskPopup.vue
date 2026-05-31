<script setup lang="ts">
import type { TaskStats, TaskStatusDetails } from '~/utils/taskPresentation'
import { taskDifficultyClass, taskDifficultyLabel, taskLabelText, taskStatusClass, taskStatusIcon, taskStatusLabel } from '~/utils/taskPresentation'

const props = defineProps<{
  id: string
  name: string
  difficultyEstimate: string
  labels: string[]
  isCompleted: boolean
  stats?: TaskStats
  status?: TaskStatusDetails
  compact?: boolean
  hideName?: boolean
}>()

const labelDescription = useTaskLabelDescription()
const visibleLabelLimit = computed(() => props.compact ? 1 : 4)
</script>

<template>
  <div
    class="pointer-events-none relative border-2 border-surface-muted bg-default/95 shadow-lg shadow-black/30 transition-all before:absolute before:-top-1.5 before:left-5 before:size-2 before:rotate-45 before:border-l-2 before:border-t-2 before:border-surface-muted before:bg-default"
    :class="compact ? 'w-max max-w-64 p-2.5' : 'w-72 p-4'"
  >
    <div v-if="!hideName" class="mb-2 flex items-start justify-between gap-3">
      <span class="font-bold leading-tight" :class="compact ? 'text-sm' : 'text-base'">
        {{ name }}
      </span>
      <UIcon
        v-if="isCompleted"
        name="i-lucide-check"
        class="mt-0.5 size-4 shrink-0 text-primary"
      />
    </div>

    <div class="flex flex-wrap items-center gap-2">
      <span
        v-if="difficultyEstimate"
        class="border px-1.5 py-px text-[9px] font-bold uppercase tracking-wider"
        :class="taskDifficultyClass(difficultyEstimate)"
      >
        {{ taskDifficultyLabel(difficultyEstimate) }}
      </span>
      <UTooltip
        v-for="label in labels.slice(0, visibleLabelLimit)"
        :key="label"
        :text="labelDescription(label)"
        :disabled="!labelDescription(label)"
        :delay-duration="100"
      >
        <span class="text-[9px] font-bold uppercase tracking-wider text-muted">
          {{ taskLabelText(label) }}
        </span>
      </UTooltip>
      <span v-if="labels.length > visibleLabelLimit" class="text-[9px] font-bold text-muted">
        +{{ labels.length - visibleLabelLimit }}
      </span>
      <span class="inline-flex items-center gap-1 text-xs text-muted" title="Punkty">
        <UIcon name="pixelarticons:trophy" class="size-3.5 text-primary" />
        <span class="font-bold text-default">{{ stats?.points ?? 500 }}</span>
      </span>
      <span class="inline-flex items-center gap-1 text-xs text-muted" title="Rozwiązania">
        <UIcon name="pixelarticons:users" class="size-3.5 text-primary" />
        <span class="font-bold text-default">{{ stats?.solveCount ?? 0 }}</span>
      </span>
      <span
        v-if="status"
        class="inline-flex items-center gap-1 border px-1.5 py-px text-[9px] font-bold uppercase tracking-wider"
        :class="taskStatusClass(status.status)"
        :title="status.comment ?? taskStatusLabel(status.status)"
      >
        <UIcon :name="taskStatusIcon(status.status)" class="size-3" />
        {{ taskStatusLabel(status.status) }}
      </span>
    </div>
  </div>
</template>
