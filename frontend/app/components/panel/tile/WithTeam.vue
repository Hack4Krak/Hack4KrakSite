<script setup lang="ts">
const value = ref(50)
</script>

<template>
  <div class="p-5 flex gap-5 flex-col h-full py-10">
    <h1 class="font-bold text-2xl">
      Progress twojej drużyny:
    </h1>
    <UProgress v-model="value" size="md" :ui="{ base: 'bg-primary-darker' }" />
    <span class="flex-grow">Brakuje wam 25 flag</span>

    <h1 class="font-semibold text-2xl">
      Nazwa zespołu: <span class="text-(--ui-primary)">Szypki</span>
    </h1>
    <NuxtLink to="/panel/team" class="flex items-center gap-5 border-neutral-800 border-2 rounded-2xl px-5 py-2 cursor-pointer hover:bg-neutral-800 transition-all duration-150">
      <UIcon name="i-fluent-people-team-16-filled" class="size-15" />
      <h3 class="font-semibold">
        Panel zespołu
      </h3>
      <UIcon name="i-material-symbols-arrow-back-2-rounded" class="size-3 transform scale-x-[-1]" />
    </NuxtLink>
  </div>
</template>
