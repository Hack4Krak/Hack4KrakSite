<script setup lang="ts">
type AnnouncementLevel = 'info' | 'warning' | 'success' | 'critical'

type TaskStatus = 'up' | 'broken' | 'down'

interface Announcement {
  id: string
  level: AnnouncementLevel
  title: string
  details: string
  publishedAt: Date
}

const selectedAnnouncement = ref<Announcement | null>(null)
const announcementDetailsOpen = computed({
  get: () => selectedAnnouncement.value !== null,
  set: (open) => {
    if (!open)
      selectedAnnouncement.value = null
  },
})

const { data: fetchedAnnouncements } = await useLazyApi('/announcements/', {
  query: { limit: 10 },
})

const taskStatusLabel: Record<TaskStatus, string> = {
  up: 'Sprawne',
  broken: 'Problemy',
  down: 'Niedostępne',
}

function getTaskStatusLabel(status: string) {
  return taskStatusLabel[status as TaskStatus] ?? status
}

function getTaskStatusLevel(status: string): AnnouncementLevel {
  if (status === 'up')
    return 'success'
  if (status === 'down')
    return 'critical'
  return 'warning'
}

const announcements = computed<Announcement[]>(() => {
  if (!fetchedAnnouncements.value?.length)
    return []

  return fetchedAnnouncements.value.map((announcement) => {
    const action = announcement.action
    const isTaskUpdate = action.type === 'task_status_update'

    return {
      id: announcement.id,
      level: isTaskUpdate ? getTaskStatusLevel(action.status) : 'info',
      title: isTaskUpdate
        ? `Zadanie ${action.task_id}: ${getTaskStatusLabel(action.status)}`
        : action.text,
      details: isTaskUpdate
        ? action.comment ?? `Status: ${getTaskStatusLabel(action.status)}.`
        : action.text,
      publishedAt: new Date(announcement.created_at),
    }
  })
})

const accent: Record<AnnouncementLevel, string> = {
  info: 'text-sky-300',
  warning: 'text-primary',
  success: 'text-emerald-300',
  critical: 'text-red-450',
}

const icon: Record<AnnouncementLevel, string> = {
  info: 'pixelarticons:notification',
  warning: 'pixelarticons:alert',
  success: 'pixelarticons:check',
  critical: 'pixelarticons:flag',
}

function relativeTime(date: Date) {
  const diff = (Date.now() - date.getTime()) / 1000
  if (diff < 60)
    return 'teraz'
  if (diff < 3600)
    return `${Math.floor(diff / 60)} min`
  if (diff < 86400)
    return `${Math.floor(diff / 3600)} godz.`
  return `${Math.floor(diff / 86400)} dni`
}

const sorted = computed(() =>
  [...announcements.value].sort((a, b) => b.publishedAt.getTime() - a.publishedAt.getTime()),
)
</script>

<template>
  <section class="flex h-full min-h-0 flex-col border-2 border-surface-muted bg-default p-5 shadow-[inset_0_0_0_1px_rgb(250_250_250/0.025)] lg:p-6">
    <PanelSectionTitle class="mb-4 shrink-0">
      Ogłoszenia
    </PanelSectionTitle>

    <ul class="min-h-0 flex-1 divide-y-2 divide-surface-muted overflow-y-auto pr-1">
      <li v-if="!sorted.length" class="py-6 text-sm text-muted">
        Brak ogłoszeń.
      </li>
      <li
        v-for="a in sorted"
        :key="a.id"
        class="py-2.5 first:pt-0 last:pb-0"
      >
        <button
          type="button"
          class="flex w-full items-center gap-3 text-left transition-colors hover:text-primary"
          @click="selectedAnnouncement = a"
        >
          <UIcon :name="icon[a.level]" class="size-4 shrink-0" :class="accent[a.level]" />
          <span class="min-w-0 flex-1 truncate text-sm">
            {{ a.title }}
          </span>
          <span class="shrink-0 font-pixelify text-[11px] uppercase tracking-wider text-muted">
            {{ relativeTime(a.publishedAt) }}
          </span>
        </button>
      </li>
    </ul>

    <NuxtLink
      to="/announcements"
      class="mt-4 flex shrink-0 items-center justify-between border-t-2 border-surface-muted pt-3 font-pixelify text-xs uppercase tracking-wider text-muted transition-colors hover:text-primary"
    >
      Zobacz wszystkie
      <UIcon name="pixelarticons:arrow-right" class="size-4" />
    </NuxtLink>

    <UModal v-model:open="announcementDetailsOpen" title="Szczegóły ogłoszenia">
      <template #body>
        <div v-if="selectedAnnouncement" class="space-y-3">
          <p class="font-pixelify text-lg text-primary">
            {{ selectedAnnouncement.title }}
          </p>
          <p class="text-sm leading-relaxed text-muted">
            {{ selectedAnnouncement.details }}
          </p>
          <p class="font-pixelify text-xs uppercase tracking-wider text-muted">
            {{ selectedAnnouncement.publishedAt.toLocaleString('pl-PL') }}
          </p>
        </div>
      </template>
    </UModal>
  </section>
</template>
