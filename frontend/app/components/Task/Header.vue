<script setup lang="ts">
import type { TaskStats } from '~/utils/taskPresentation'
import { taskDifficultyClass, taskDifficultyLabel, taskLabelText } from '~/utils/taskPresentation'

interface Author { name: string, url?: string | null }

const props = defineProps<{
  taskId: string
  name?: string | null
  difficulty?: string | null
  labels?: string[] | null
  authors?: Author[] | null
  stats?: TaskStats
  isCompleted?: boolean
  isLocked?: boolean
  availableSince?: string | null
  showIcon?: boolean
  compact?: boolean
}>()

const { imgSrc, onImgError } = useTaskIcon(() => props.taskId)
const countdownText = useTaskCountdown(() => props.availableSince)
const labelDescription = useTaskLabelDescription()
</script>

<template>
  <div
    class="relative flex items-center gap-4 border-b-2 border-surface-muted px-6 py-4"
    :class="isLocked ? 'bg-surface-muted/20' : ''"
  >
    <!-- Lock stripe -->
    <div
      v-if="isLocked"
      aria-hidden="true"
      class="pointer-events-none absolute inset-y-0 left-0 w-1 bg-warning"
    />

    <img
      v-if="showIcon"
      :src="imgSrc"
      :alt="name ?? '?'"
      class="shrink-0 object-contain rendering-pixelated"
      :class="[compact ? 'size-14' : 'size-16', isLocked ? 'opacity-40 grayscale' : '']"
      @error="onImgError"
    >

    <div class="min-w-0 flex-1">
      <div class="flex items-start justify-between gap-3">
        <!-- Left: kicker + title + meta-left -->
        <div class="min-w-0 flex-1">
          <div class="flex items-center gap-2">
            <p class="text-[10px] font-bold uppercase tracking-widest text-primary">
              Zadanie CTF
            </p>
            <span
              v-if="isLocked"
              class="inline-flex items-center gap-1 border-2 border-warning/70 bg-warning/10 px-1.5 py-px text-[9px] font-bold uppercase tracking-wider text-warning"
            >
              <UIcon name="i-lucide-lock" class="size-3" />
              Zablokowane
            </span>
          </div>

          <div class="mt-1 flex flex-wrap items-center gap-x-3 gap-y-1">
            <UIcon
              v-if="isCompleted && !isLocked"
              name="i-lucide-circle-check"
              class="size-5 shrink-0 text-muted"
              title="Ukończone"
            />
            <h2
              class="font-pixelify font-bold leading-none"
              :class="compact ? 'text-xl' : 'text-2xl'"
            >
              {{ name ?? countdownText ?? 'Zadanie niedostępne' }}
            </h2>
          </div>

          <div class="mt-2.5 flex items-center gap-3 text-xs">
            <span class="inline-flex items-center gap-1.5" title="Punkty">
              <UIcon name="pixelarticons:trophy" class="size-4 text-primary" />
              <span class="font-bold text-default">{{ stats?.points ?? 500 }}</span>
              <span class="text-muted">pkt</span>
            </span>
            <span class="inline-flex items-center gap-1.5" title="Rozwiązania">
              <UIcon name="pixelarticons:users" class="size-4 text-primary" />
              <span class="font-bold text-default">{{ stats?.solveCount ?? 0 }}</span>
              <span class="text-muted">rozwiązań</span>
            </span>
          </div>

          <div v-if="authors?.length" class="mt-2 flex flex-wrap items-center gap-x-2 gap-y-1">
            <span class="text-[10px] uppercase tracking-widest text-muted">Autorzy:</span>
            <template v-for="author in authors" :key="author.name">
              <a
                v-if="author.url"
                :href="author.url"
                target="_blank"
                rel="noreferrer"
                class="text-xs text-muted hover:text-primary transition-colors"
              >
                {{ author.name }}
              </a>
              <span v-else class="text-xs text-muted">{{ author.name }}</span>
            </template>
          </div>
        </div>

        <!-- Right: difficulty + label chips -->
        <div class="flex max-w-[55%] flex-wrap justify-end gap-1">
          <span
            v-if="difficulty"
            class="border-2 px-2 py-0.5 text-[10px] font-bold uppercase tracking-wider"
            :class="taskDifficultyClass(difficulty)"
          >
            {{ taskDifficultyLabel(difficulty) }}
          </span>
          <UTooltip
            v-for="label in labels ?? []"
            :key="label"
            :text="labelDescription(label)"
            :disabled="!labelDescription(label)"
            :delay-duration="100"
          >
            <span
              class="cursor-help border-2 border-surface-muted px-2 py-0.5 text-[10px] font-bold uppercase tracking-wider text-muted"
            >
              {{ taskLabelText(label) }}
            </span>
          </UTooltip>
        </div>
      </div>
    </div>
  </div>
</template>
