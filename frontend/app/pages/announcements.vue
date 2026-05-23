<script setup lang="ts">
definePageMeta({
  middleware: ['event-access-guard'],
})

useSeoMeta({
  title: 'Ogłoszenia',
  description: 'Aktualne ogłoszenia i statusy zadań Hack4Krak CTF.',
})

const { data: announcements } = await useApi('/announcements/', {
  query: { limit: 100 },
})

type AnnouncementAction
  = | { type: 'task_status_update', task_id: string, status: string, comment?: string | null }
    | { type: 'normal', text: string }

type TaskStatus = 'up' | 'broken' | 'down'

const taskStatusLabel: Record<TaskStatus, string> = {
  up: 'Sprawne',
  broken: 'Problemy',
  down: 'Niedostępne',
}

function getTaskStatusLabel(status: string) {
  return taskStatusLabel[status as TaskStatus] ?? status
}

function getTaskStatusIcon(status: string) {
  if (status === 'up')
    return 'pixelarticons:check'
  if (status === 'down')
    return 'pixelarticons:close'
  return 'pixelarticons:alert'
}

function getTaskStatusTone(status: string) {
  if (status === 'up')
    return 'text-emerald-300'
  if (status === 'down')
    return 'text-red-450'
  return 'text-primary'
}

function formatAction(action: AnnouncementAction) {
  if (action.type === 'task_status_update') {
    return {
      title: `Zadanie ${action.task_id}: ${getTaskStatusLabel(action.status)}`,
      description: action.comment ?? `Status: ${getTaskStatusLabel(action.status)}.`,
      icon: getTaskStatusIcon(action.status),
      tone: getTaskStatusTone(action.status),
    }
  }

  return {
    title: action.text,
    description: action.text,
    icon: 'pixelarticons:notification',
    tone: 'text-sky-300',
  }
}
</script>

<template>
  <UContainer class="py-8 lg:py-10">
    <header class="mb-8 border-b-2 border-surface-muted pb-6">
      <p class="mb-2 text-xs font-bold uppercase tracking-wider text-primary">
        Hack4Krak 2026
      </p>
      <h1 class="font-pixelify text-4xl leading-none lg:text-5xl">
        Ogłoszenia
      </h1>
    </header>

    <div v-if="announcements?.length" class="space-y-4">
      <article
        v-for="announcement in announcements"
        :key="announcement.id"
        class="border-2 border-surface-muted bg-default p-5"
      >
        <div class="flex items-start gap-3">
          <UIcon
            :name="formatAction(announcement.action).icon"
            class="mt-1 size-5 shrink-0"
            :class="formatAction(announcement.action).tone"
          />
          <div class="min-w-0 flex-1">
            <h2 class="font-pixelify text-xl text-primary">
              {{ formatAction(announcement.action).title }}
            </h2>
            <p class="mt-2 text-sm leading-relaxed text-muted">
              {{ formatAction(announcement.action).description }}
            </p>
            <p class="mt-3 font-pixelify text-xs uppercase tracking-wider text-muted">
              {{ new Date(announcement.created_at).toLocaleString('pl-PL') }}
            </p>
          </div>
        </div>
      </article>
    </div>

    <PanelCard v-else body dashed>
      <PanelSectionTitle class="mb-3">
        Brak ogłoszeń
      </PanelSectionTitle>
      <p class="text-sm text-muted">
        Gdy organizatorzy opublikują aktualizacje, pojawią się tutaj.
      </p>
    </PanelCard>
  </UContainer>
</template>
