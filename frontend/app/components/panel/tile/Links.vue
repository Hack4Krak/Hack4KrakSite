<template>
  <div class="flex flex-col gap-5 items-center justify-center overflow-clip">
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
</template>
