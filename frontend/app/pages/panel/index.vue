<script setup lang="ts">
useSeoMeta({
  title: 'Panel użytkownika',
  description: 'Zarządzaj swoim kontem i drużyną w naszym CTF-ie! Sprawdź swoje zadania i postępy!',
})

const { data: team } = await useAuth('/teams/membership/my_team', {
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
  <div class="grid grid-rows-[auto_auto_1fr_auto] grid-cols-[300px_1fr] h-155 divide-x my-5 mx-15 outline">
    <!-- Top full-width bar -->
    <div class="col-span-2 border-b h-15 flex items-center divide-x">
      <span class="w-15 h-full flex items-center justify-center font-bold">
        X
      </span>
      <span v-if="team?.team_name" class="px-5 h-full flex items-center">
        {{ team?.team_name }}
      </span>
      <span class="px-5 h-full flex items-center">
        Hack4Krak CTF - Edycja dla szkół podstawowych
      </span>
      <span class="px-5 h-full flex items-center justify-end flex-1">
        {{ now }}
      </span>
    </div>

    <PanelTileEventProgressBar class="border-b" />

    <!-- Sidebar -->
    <div class="row-span-3 p-4 flex flex-col justify-center">
      <span class="font-bold">Moja drużyna</span>
      <USeparator :ui="{ border: 'border-neutral' }" />
      <div v-if="(team?.members ?? []).length">
        <div v-for="member in team?.members" :key="member.name">
          {{ member.name }}
        </div>
      </div>
      <div v-else>
        Proszę stworzyć drużynę!
      </div>
    </div>

    <!-- Top two boxes -->
    <div class="flex divide-x border-b font-pixelify">
      <div class="flex-1  shadow items-center justify-center flex-col flex">
        <PanelFlagForm />
      </div>
      <div class="w-2/5 flex flex-col gap-5 items-center justify-center overflow-clip">
        <h3 class="text-xl font-bold">
          Ważne linki
        </h3>
        <NuxtLink to="/tasks">
          <ElevatedButton class="w-60">
            Zadania
          </ElevatedButton>
        </NuxtLink>
        <NuxtLink to="/leaderboard">
          <ElevatedButton class="w-60">
            Ranking
          </ElevatedButton>
        </NuxtLink>
        <NuxtLink to="/panel/profile">
          <ElevatedButton class="w-60">
            Moje konto
          </ElevatedButton>
        </NuxtLink>
      </div>
    </div>

    <!-- Bottom 3 boxes -->
    <!--    <div class="col-span-1 flex divide-x font-pixelify text-center"> -->
    <!--      <div class="flex-1 p-4"> -->
    <!--        <div class="text-sm"> -->
    <!--          Miejsce drużyny -->
    <!--        </div> -->
    <!--        <div class="text-xl font-bold"> -->
    <!--          1/24 -->
    <!--        </div> -->
    <!--      </div> -->
    <!--      <div class="flex-1 p-4"> -->
    <!--        <div class="text-sm"> -->
    <!--          Zdobyte flagi -->
    <!--        </div> -->
    <!--        <div class="text-xl text-green-400 font-bold"> -->
    <!--          6 -->
    <!--        </div> -->
    <!--      </div> -->
    <!--      <div class="flex-1 p-4"> -->
    <!--        <div class="text-sm"> -->
    <!--          Pozostałe flagi -->
    <!--        </div> -->
    <!--        <div class="text-xl text-yellow-400 font-bold"> -->
    <!--          16 -->
    <!--        </div> -->
    <!--      </div> -->
    <!--    </div> -->
  </div>
</template>
