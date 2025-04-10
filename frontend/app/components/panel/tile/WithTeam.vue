<script setup lang="ts">
const props = defineProps<{
  teamName: string
}>()

const { error } = await useApi('/event/status')

const isEventInProgress = computed(() => error.value === undefined)

const { data } = await useAuth('/teams/membership/completed_tasks')

const completedFlags = data.value ?? []

const { data: countData } = await useApi('/tasks/count')

const tasksCount = countData.value ?? 1

const value = computed(
  () => (completedFlags.length * 100) / tasksCount,
)

const flagsLeft = computed(
  () => tasksCount - completedFlags.length,
)
</script>

<template>
  <div class="p-5 flex gap-5 flex-col h-full py-10">
    <h1 class="font-bold text-2xl">
      Progress twojej drużyny:
    </h1>
    <UProgress v-model="value" size="md" :ui="{ base: 'bg-primary-darker' }" />
    <span class="flex-grow">{{ isEventInProgress ? `Pozostałe flagi: ${flagsLeft}` : "Liczba zdobytych flag będzie widoczna po rozpoczęciu wydarzenia" }}</span>

    <h1 class="font-semibold text-2xl">
      Nazwa zespołu: <span class="text-(--ui-primary)">{{ props.teamName }}</span>
    </h1>
    <NuxtLink to="/panel/team" class="flex items-center gap-5 border-neutral-800 border-2 rounded-2xl px-5 py-2 cursor-pointer hover:bg-neutral-800 transition-all duration-150">
      <UIcon name="mdi:account-group" class="size-15" />
      <h3 class="font-semibold">
        Panel zespołu
      </h3>
      <UIcon name="i-material-symbols-arrow-back-2-rounded" class="size-3 transform scale-x-[-1]" />
    </NuxtLink>
  </div>
</template>
