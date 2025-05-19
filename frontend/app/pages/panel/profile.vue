<script setup lang="ts">
const { $api } = useNuxtApp()

const { data: user } = await useAuth('/account/')

const updateAccountModal = ref(false)
const deleteAccountModal = ref(false)
const changePasswordModal = ref(false)

const joinExternalTeamModal = ref(false)

async function logout() {
  await $api('/auth/logout', {
    method: 'POST',
    credentials: 'include',
  })

  await navigateTo('/login')
}
</script>

<template>
  <PanelModalJoinExternalTeam v-model="joinExternalTeamModal" />

  <div class="grid grid-cols-[400px_1fr] h-155  divide-x my-5 mx-15 border w-full">
    <div class=" h-full flex flex-col divide-y">
      <div class="grow items-center justify-center flex flex-col">
        <h3 class="text-xl font-bold">
          {{ user?.username }}
        </h3>
        <span>Przesłane flagi: 0</span>
      </div>
      <div class="p-5">
        <h1 class="font-bold text-xl">
          Ustawienia konta
        </h1>
        <div class="flex flex-col gap-3 mt-3 justify-center">
          <UButton icon="mdi:account" variant="ghost" color="neutral" @click="navigateTo('/account/submit_personal_info')">
            Zmień lub zobacz dane o końcie
          </UButton>
          <UButton icon="mdi:account-cog" variant="ghost" color="neutral" @click="updateAccountModal = true">
            Zmień ustawienia konta
          </UButton>
          <UButton icon="mdi:account-key" variant="ghost" color="neutral" @click="changePasswordModal = true">
            Zmień hasło
          </UButton>
          <UButton icon="mdi:account-remove" variant="ghost" color="neutral" @click="deleteAccountModal = true">
            Usuń konto
          </UButton>
          <UButton icon="mdi:logout" variant="ghost" color="neutral" @click="logout">
            Wyloguj się
          </UButton>
        </div>
      </div>
    </div>

    <div class="divide-y w-full h-full flex flex-col">
      <div class="w-full text-2xl font-bold p-5 flex justify-between items-center">
        Wydarzenia
        <JoinTeamButton @click="joinExternalTeamModal = true" />
      </div>
      <div class="grow">
        <Placeholder class="w-full h-full">
          Strona w trakcie tworzenia!<br>
        </Placeholder>
      </div>
    </div>
  </div>
</template>
