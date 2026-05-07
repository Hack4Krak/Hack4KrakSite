<script setup lang="ts">
const LINKS = [
  { to: '/tasks', label: 'Zadania', background: '#6EEB8359' },
  { to: '/leaderboard', label: 'Ranking', background: '#F6B21670' },
  { to: '/account/events', label: 'Moje konto', background: '#7FDBFF70' },
]
</script>

<template>
  <div class="flex flex-col gap-5 items-center">
    <ElevatedButton
      v-for="link in LINKS"
      :key="link.to"
      :to="link.to"
      :background="link.background"
      class="w-60"
    >
      {{ link.label }}
    </ElevatedButton>
  </div>
</template>
