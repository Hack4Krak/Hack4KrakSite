<script setup lang="ts">
const LINKS = [
  { to: '/tasks', label: 'Zadania', background: '#6EEB8359' },
  { to: '/leaderboard', label: 'Ranking', background: '#F6B21670' },
  { to: '/panel/profile', label: 'Moje konto', background: '#7FDBFF70' },
]
</script>

<template>
  <div class="flex flex-col gap-5 items-center justify-center overflow-clip">
    <h3 class="text-xl font-bold">
      Ważne linki
    </h3>
    <NuxtLink v-for="link in LINKS" :key="link.to" :to="link.to">
      <ElevatedButton class="w-60" :background="link.background">
        {{ link.label }}
      </ElevatedButton>
    </NuxtLink>
  </div>
</template>
