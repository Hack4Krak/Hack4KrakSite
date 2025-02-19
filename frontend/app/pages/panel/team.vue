<script setup lang="ts">
const { data: team } = await useAuth('/teams/membership/my_team', {
  method: 'GET',
})

const members = computed(() => {
  const teamMembers = team?.value?.members || []
  return [...teamMembers, ...Array.from({ length: 5 }).fill(null)].slice(0, 5)
})
</script>

<template>
  <div>
    <NuxtLink to="/panel/">
      Wróć
    </NuxtLink>
    <div class="flex mx-20 gap-20">
      <div class="border-2 border-neutral-600 p-5 min-w-70 rounded-2xl flex flex-col">
        <h1 class="flex-grow text-3xl font-bold">
          {{ team?.team_name }}
        </h1>
        <UButton class="w-full">
          Usuń
        </UButton>
      </div>
      <div class="border-2 border-neutral-600 flex-grow rounded-2xl">
        <div v-for="(user, index) in members" :key="index" class="border-b-1 border-neutral-600 last-of-type:border-0 p-5">
          <div v-if="user">
            {{ user }}
            <UButton>Remove</UButton>
          </div>
          <div v-else>
            <UButton>Dodaj</UButton>
            Dodaj do zespołu
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
