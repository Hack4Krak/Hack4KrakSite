<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'
import type { TaskStats } from '~/utils/taskPresentation'

type Task = ApiResponse<'task_list'>[number]

const props = defineProps<{
  task: Task | null
  isCompleted: boolean
  stats?: TaskStats
}>()

const emit = defineEmits<{
  close: []
}>()

const description = ref('')
const isLoading = ref(false)
const ast = ref<any>(null)
const assets = ref<Array<{ description: string, path: string }>>([])
const parse = useMarkdownParser()

const apiBaseURL = useRuntimeConfig().public.openFetch.api.baseURL

const isLocked = computed(() => props.task?.name == null)

watch(
  () => props.task?.id,
  async (id) => {
    description.value = ''
    ast.value = null
    assets.value = []
    if (!id || isLocked.value) {
      return
    }
    isLoading.value = true
    try {
      const [descData, assetsData] = await Promise.allSettled([
        $fetch<string>(`/tasks/description/${id}`, { baseURL: apiBaseURL }),
        $fetch<Array<{ description: string, path: string }>>(`/tasks/assets/list/${id}`, { baseURL: apiBaseURL }),
      ])
      if (descData.status === 'fulfilled') {
        description.value = String(descData.value ?? '')
        if (description.value)
          ast.value = await parse(description.value)
      }
      if (assetsData.status === 'fulfilled' && Array.isArray(assetsData.value))
        assets.value = assetsData.value
    } finally {
      isLoading.value = false
    }
  },
  { immediate: true },
)

const baseAssetsPath = `${apiBaseURL}/tasks/assets/get`
</script>

<template>
  <div v-if="task" class="h-full flex flex-col overflow-hidden bg-default text-default">
    <TaskHeader
      :task-id="task.id"
      :name="task.name"
      :difficulty="task.difficulty_estimate"
      :labels="task.labels"
      :authors="task.authors"
      :stats="stats"
      :is-completed="isCompleted"
      :is-locked="isLocked"
      :available-since="task.available_since"
      show-icon
      compact
    />

    <!-- Body -->
    <div class="flex-1 overflow-y-auto space-y-6 px-6 py-5 scrollbar-hide">
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

      <div v-else-if="ast?.body" class="prose prose-sm prose-invert max-w-none prose-headings:font-pixelify prose-a:text-primary prose-code:border prose-code:border-surface-muted prose-code:bg-surface-muted/25 prose-code:px-1.5 prose-code:py-0.5 prose-blockquote:border-primary">
        <MDCRenderer :body="ast.body" :data="ast.data" />
      </div>

      <div v-else-if="!isLoading" class="text-sm text-muted">
        Brak opisu.
      </div>

      <div v-if="assets?.length" class="space-y-2">
        <p class="text-xs text-primary uppercase tracking-[0.24em] font-bold">
          Załączniki
        </p>
        <ul class="space-y-1">
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
      </div>
    </div>

    <!-- Footer -->
    <div class="shrink-0 flex items-center justify-between border-t-2 border-surface-muted px-6 py-3">
      <NuxtLink
        v-if="!isLocked"
        :to="{ name: 'tasks-description-id', params: { id: task.id } }"
        class="flex items-center gap-2 border-2 border-primary/70 bg-default px-3 py-2 text-sm font-bold text-primary transition-colors hover:bg-primary hover:text-default"
      >
        <UIcon name="i-lucide-external-link" class="size-4" />
        Pełna strona zadania
      </NuxtLink>
      <span v-else />
      <button
        class="border-2 border-surface-muted bg-default px-3 py-2 text-sm font-bold uppercase tracking-wider text-muted transition-colors hover:border-default hover:text-default"
        @click="emit('close')"
      >
        Zamknij
      </button>
    </div>
  </div>
</template>
