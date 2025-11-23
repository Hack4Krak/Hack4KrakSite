<script setup lang="ts">
definePageMeta({ layout: 'panel' })

useSeoMeta({
  title: 'Panel użytkownika',
  description: 'Zarządzaj swoim kontem i drużyną w naszym CTF-ie! Sprawdź swoje zadania i postępy!',
})

const { data: team } = await useAuth('/teams/membership/my_team', {
  onResponseError: () => {
    throw new Error('Response error')
  },
})

const { data: team_stats } = await useAuth('/teams/membership/stats', {
  onResponseError: () => {
    throw new Error('Response error')
  },
})

const { data: now, refresh: updateDate } = useAsyncData('formattedNow', async () => {
  const now = useNow()
  const formatted = useDateFormat(now, 'HH:mm:ss')
  return formatted.value
})

useRafFn(() => updateDate())
</script>

<template>
  <div
    class="grid grid-rows-[auto_auto_1fr_auto] divide-x m-10 outline min-w-fit flex-1"
    :class="{ 'grid-cols-[300px_1fr]': team }"
  >
    <!-- Top full-width bar -->
    <div class="col-span-2 border-b h-15 flex items-center divide-x font-bold">
      <span class="w-15 h-full flex items-center justify-center text-xl">
        X
      </span>
      <span v-if="team?.team_name" class="px-5 h-full flex items-center">
        {{ team?.team_name }}
      </span>
      <span class="px-5 h-full flex items-center">
        Hack4Krak CTF - Edycja dla szkół podstawowych
      </span>
      <span class="px-5 h-full flex items-center justify-end flex-1 text-xl">
        {{ now }}
      </span>
    </div>

    <PanelTileEventProgressBar class="border-b" />

    <!-- Sidebar -->
    <div v-if="team" class="row-span-3 p-4 flex flex-col justify-center space-y-2">
      <span class="font-bold">Moja drużyna</span>
      <USeparator :ui="{ border: 'border-neutral' }" />
      <div v-for="member in team?.members" :key="member.name">
        {{ member.name }}
      </div>
    </div>

    <!-- Top two boxes -->
    <div class="flex divide-x border-b font-pixelify" :class="{ 'col-span-2': !team }">
      <div class="flex flex-1 flex-col shadow items-center justify-center">
        <PanelTileFlagForm class="mx-20" />
      </div>
      <PanelTileLinks class="w-2/5 m-5" />
    </div>

    <PanelTileStats v-if="team_stats" :team-stats="team_stats" class="col-span-1" />
  </div>
</template>
