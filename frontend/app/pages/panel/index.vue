<script setup lang="ts">
const { $api } = useNuxtApp()

const { data } = await useAuth('/user/', {
  method: 'GET',
})

async function logout() {
  $api('/auth/logout', {
    method: 'POST',
    credentials: 'include',
  })

  await navigateTo('/login')
}
</script>

<template>
  <div class="flex flex-col p-6 pb-12 items-center gap-5">
    <div>
      <h1 class="text-5xl font-bold text-center">
        Witaj {{ data?.username }}!
      </h1>
      <h2 class="text-lg text-center">
        Życzymy powodzenia na wydarzeniu!
      </h2>
    </div>

    <div class="container mx-auto grid grid-cols-1 md:grid-cols-3 gap-4 flex-grow">
      <PanelTile class="row-span-2" />
      <PanelTile class="row-span-2" />
      <PanelTile />
      <PanelTile>
        <h1>
          Ustawienia konta
        </h1>
        <UButton @click="logout">
          Wyloguj się
        </UButton>
      </PanelTile>
    </div>
  </div>
</template>
