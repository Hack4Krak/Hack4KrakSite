<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'
import type { TableColumn } from '@nuxt/ui'
import type { Row, TableMeta } from '@tanstack/vue-table'
import dayjs from 'dayjs'

export type Team = ApiResponse<'teams_with_tasks'>[0]

const UIcon = resolveComponent('UIcon')

const { data: teams } = await useLazyApi('/leaderboard/teams_with_tasks')
const { data: tasks } = await useLazyApi('/tasks/list')

const top3PerTask = computed(() => {
  if (!teams.value?.length || !tasks.value?.length)
    return {} as Record<string, string[]>

  return tasks.value.reduce<Record<string, string[]>>((acc, task) => {
    const solves = teams.value?.map((team) => {
      const solvedAt = team.tasks?.[task.id]
      return solvedAt ? { teamId: team.team_id, time: new Date(solvedAt).getTime() } : null
    })
      .filter(Boolean) as { teamId: string, time: number }[]

    solves.sort((a, b) => a.time - b.time)

    acc[task.id] = solves.slice(0, 3).map(s => s.teamId)
    return acc
  }, {})
})

const taskColumns = computed<TableColumn<Team>[]>(() => tasks.value?.map((task) => {
  return {
    accessorFn: (row: Team) => row.tasks?.[task.id],
    id: task.id,
    header: task.name,
    meta: {
      class: {
        th: 'w-10 px-1',
        td: 'w-10 px-1',
      },
    },
  }
}) ?? [])

const defaultHeaderMeta = {
  class: {
    th: 'align-bottom text-center min-w-36 text-center',
    td: 'min-w-36 text-center',
  },
}

const columns = computed<TableColumn<Team>[]>(() => [
  {
    accessorFn: (_row, index) => index + 1,
    id: 'place',
    header: 'Miejsce',
    meta: defaultHeaderMeta,
  },
  {
    accessorKey: 'team_name',
    header: 'Nazwa drużyny',
    meta: defaultHeaderMeta,
  },
  {
    accessorKey: 'current_points',
    header: 'Ilość punktów',
    meta: defaultHeaderMeta,
  },
  {
    accessorKey: 'captured_flags',
    header: 'Zdobyte flagi',
    meta: defaultHeaderMeta,
  },
  ...taskColumns.value,
])

const meta = computed<TableMeta<Team>>(() => ({
  class: {
    tr: (row: Row<Team>) => {
      const place = row.index + 1
      if (place === 1) {
        return 'bg-[#3a2f00]/55 font-bold'
      } else if (place === 2) {
        return 'bg-[#2e2e2e]/55 font-bold'
      } else if (place === 3) {
        return 'bg-[#3b2a1a]/55 font-bold'
      }
      return ''
    },
  },
}))

function getTaskIcon(teamId: string, taskId: string) {
  const top3 = top3PerTask.value[taskId] || []
  if (top3[0] === teamId)
    return { icon: 'pixelarticons:trophy', color: 'text-yellow-400', label: 'First blood' }
  else if (top3[1] === teamId)
    return { icon: 'lucide:medal', color: 'text-gray-400', label: '2. rozwiązanie' }
  else if (top3[2] === teamId)
    return { icon: 'lucide:medal', color: 'text-amber-600', label: '3. rozwiązanie' }
  return { icon: 'pixelarticons:flag', color: 'text-green-600', label: 'Rozwiązane' }
}
</script>

<template>
  <UTable
    :data="teams"
    :meta="meta"
    :columns="columns"
    empty="Brak danych"
    class="flex-1 px-15"
    :ui="{
      td: 'text-neutral-50',
      th: 'text-neutral-50 text-center',
      tr: 'text-neutral-50',
    }"
  >
    <template
      v-for="task in tasks"
      :key="`${task.id}-header`"
      #[`${task.id}-header`]
    >
      <UTooltip :text="task.name" :delay-duration="200">
        <div
          class="border-b-0 border border-accented w-15 h-25 skew-x-45 relative -left-[80%]"
        >
          <div
            class="relative left-1/2 -translate-x-1/2 w-28 h-full flex items-center justify-center overflow-visible"
          >
            <NuxtLink
              class="whitespace-nowrap truncate flex-1 mt-2"
              style="transform: skew(-45deg) rotate(45deg);"
              :to="`/tasks/description/${task.id}`"
            >
              {{ task.name }}
            </NuxtLink>
          </div>
        </div>
      </UTooltip>
    </template>
    <template
      v-for="task in tasks"
      :key="`${task.id}-cell`"
      #[`${task.id}-cell`]="{ row }"
    >
      <div v-if="row.original.tasks?.[task.id]">
        <UIcon
          :name="getTaskIcon(row.original.team_id, task.id).icon"
          :class="`${getTaskIcon(row.original.team_id, task.id).color} text-xl`"
          :title="`${getTaskIcon(row.original.team_id, task.id).label} — ${dayjs(row.original.tasks[task.id]).format('DD.MM.YYYY HH:mm')}`"
        />
      </div>
    </template>
  </UTable>
  <div class="flex gap-6 justify-center mt-4 text-sm text-neutral-400">
    <span class="flex items-center gap-1">
      <UIcon name="pixelarticons:trophy" class="text-yellow-400" /> First blood
    </span>
    <span class="flex items-center gap-1">
      <UIcon name="lucide:medal" class="text-gray-400" /> 2. rozwiązanie
    </span>
    <span class="flex items-center gap-1">
      <UIcon name="lucide:medal" class="text-amber-600" /> 3. rozwiązanie
    </span>
    <span class="flex items-center gap-1">
      <UIcon name="pixelarticons:flag" class="text-green-600" /> Rozwiązane
    </span>
  </div>
</template>
