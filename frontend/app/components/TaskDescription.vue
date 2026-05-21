<script setup lang="ts">
import { FetchError } from 'ofetch'
import { buildTaskStats, getTaskStats } from '~/utils/taskPresentation'

const props = defineProps<{
  taskId: string
}>()

const apiBaseURL = useRuntimeConfig().public.openFetch.api.baseURL

const description = ref('')
const ast = ref<any>(null)
const isLoading = ref(true)
const parse = useMarkdownParser()

const { data: tasks } = await useApi('/tasks/list')
const task = computed(() => tasks.value?.find(t => t.id === props.taskId))
const isLocked = computed(() => task.value && task.value.name == null)

try {
  if (!isLocked.value) {
    const data = await $fetch<string>(`/tasks/description/${props.taskId}`, { baseURL: apiBaseURL })
    description.value = String(data ?? '')
    if (description.value)
      ast.value = await parse(description.value)
  }
} catch (error) {
  if (!(error instanceof FetchError)) {
    isLoading.value = false
    throw error
  }
  if (error.statusCode === 404) {
    showError({ status: 404, message: 'Zadanie nie zostało znalezione' })
  } else {
    showError(error)
  }
} finally {
  isLoading.value = false
}

const { data: teams } = await useLazyApi('/leaderboard/teams_with_tasks')
const { data: completed } = await useAuth('/teams/membership/completed_tasks', {
  onResponseError: undefined,
  redirect: 'error',
})

const isCompleted = computed(() =>
  Array.isArray(completed.value) && completed.value.includes(props.taskId),
)
const taskStats = computed(() => buildTaskStats(tasks.value ?? [], teams.value ?? []))
const stats = computed(() => getTaskStats(taskStats.value, props.taskId))

const { data: assets } = await useAuth('/tasks/assets/list/{task_id}', {
  path: { task_id: props.taskId },
  key: `task-${props.taskId}`,
})

const { error: solutionError } = await useAuth('/tasks/solution/{task_id}', {
  path: { task_id: props.taskId },
  key: `task-solution-${props.taskId}`,
  onResponseError: () => {
    throw new Error('Response error')
  },
})

const baseAssetsPath = `${apiBaseURL}/tasks/assets/get`
</script>

<template>
  <UContainer class="w-full max-w-4xl py-8 lg:py-10">
    <div v-if="task" class="border-2 border-surface-muted bg-default">
      <TaskHeader
        :task-id="task.id"
        :name="task.name"
        :difficulty="task.difficulty_estimate"
        :labels="task.labels"
        :authors="task.authors"
        :stats="stats"
        :is-completed="isCompleted"
        :is-locked="!!isLocked"
        :available-since="task.available_since"
        show-icon
      />

      <div class="space-y-8 px-6 py-6 lg:px-10 lg:py-8">
        <div v-if="isLocked" class="flex flex-col items-center justify-center gap-2 py-12 text-center text-muted">
          <UIcon name="i-lucide-lock" class="size-10 text-warning/70" />
          <p class="text-sm uppercase tracking-widest">
            Zadanie jeszcze nie zostało odblokowane
          </p>
        </div>

        <div v-else-if="isLoading" class="flex h-32 items-center justify-center text-muted">
          <UIcon name="i-lucide-loader" class="mr-2 size-5 animate-spin" />
          <span class="text-sm">Ładowanie...</span>
        </div>

        <div
          v-else-if="ast?.body"
          class="prose prose-invert max-w-none prose-headings:font-pixelify prose-a:text-primary prose-code:border prose-code:border-surface-muted prose-code:bg-surface-muted/25 prose-code:px-1.5 prose-code:py-0.5 prose-blockquote:border-primary"
        >
          <MDCRenderer :body="ast.body" :data="ast.data" />
        </div>

        <div v-else class="text-sm text-muted">
          Brak opisu.
        </div>

        <section v-if="assets?.length" class="space-y-3">
          <p class="text-xs font-bold uppercase tracking-[0.24em] text-primary">
            Załączniki
          </p>
          <ul class="grid gap-2 sm:grid-cols-2">
            <li v-for="asset in assets" :key="asset.path">
              <a
                :href="`${baseAssetsPath}/${task.id}/${asset.path}`"
                download
                target="_blank"
                class="flex items-center gap-2 border-2 border-primary/70 bg-default px-3 py-2 text-sm font-bold text-primary transition-colors hover:bg-primary hover:text-default"
              >
                <UIcon name="i-lucide-paperclip" class="size-4 shrink-0" />
                {{ asset.description }}
              </a>
            </li>
          </ul>
        </section>

        <section v-if="!solutionError" class="space-y-3 border-t-2 border-surface-muted pt-6">
          <p class="text-xs font-bold uppercase tracking-[0.24em] text-primary">
            Rozwiązanie
          </p>
          <p class="text-sm text-muted">
            Wydarzenie już się zakończyło.
          </p>
          <NuxtLink
            :to="`/tasks/solution/${taskId}`"
            class="inline-flex items-center gap-2 border-2 border-primary/70 bg-default px-3 py-2 text-sm font-bold text-primary transition-colors hover:bg-primary hover:text-default"
          >
            <UIcon name="i-lucide-arrow-right" class="size-4" />
            Zobacz rozwiązanie
          </NuxtLink>
        </section>
      </div>
    </div>
  </UContainer>
</template>
