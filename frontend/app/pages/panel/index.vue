<script setup lang="ts">
const { $api } = useNuxtApp()

const { data } = await useAuth('/user/', {
  method: 'GET',
})

async function logout() {
  await $api('/auth/logout', {
    method: 'POST',
    credentials: 'include',
  })

  await navigateTo('/login')
}
</script>

<template>
  <div class="flex flex-col p-12 pb-12 items-center gap-12">
    <div>
      <h1 class="text-5xl font-bold text-center">
        Witaj {{ data?.username }}!
      </h1>
      <h2 class="text-2xl font-light text-center mt-2">
        Życzymy powodzenia na wydarzeniu!
      </h2>
    </div>

    <div class="container mx-auto grid grid-cols-1 md:grid-cols-3 gap-x-15 gap-y-5 flex-grow">
      <PanelTile class="row-span-2 min-h-100 min-w-70">
        <PanelTileWithTeam />
      </PanelTile>
      <PanelTile class="row-span-2">
        <PanelTileMain />
      </PanelTile>
      <PanelTile class="min-h-25" />
      <PanelTile>
        <div class="p-5">
          <h1>
            Ustawienia konta
          </h1>
          <UButton @click="logout">
            Wyloguj się
          </UButton>
        </div>
      </PanelTile>
    </div>
  </div>
</template>
