<script setup lang="ts">
definePageMeta({ layout: 'panel' })

const { $api } = useNuxtApp()

const { data: user } = await useAuth('/account/')

const joinExternalTeamModal = ref(false)
const updateAccountModal = ref(false)
const changePasswordModal = ref(false)
const deleteAccountModal = ref(false)

async function logout() {
  await $api('/auth/logout', {
    method: 'POST',
    credentials: 'include',
  })

  await refreshNuxtData()
  await navigateTo('/login')
}
</script>

<template>
  <LazyPanelModalJoinExternalTeam v-model="joinExternalTeamModal" hydrate-on-visible />
  <LazyPanelModalUpdateAccountModal v-model="updateAccountModal" hydrate-on-visible />
  <LazyPanelModalChangePasswordModal v-model="changePasswordModal" hydrate-on-visible />
  <LazyPanelModalConfirmDeleteModal
    v-model="deleteAccountModal"
    url="/account/delete"
    modal-title="Usuń konto"
    modal-description="Czy na pewno chcesz usunąć swoje konto? Ta operacja jest nieodwracalna."
    toast-success-message="Pomyślnie usunięto konto"
    :request-body="undefined"
    redirect-to="/"
    hydrate-on-visible
  />

  <div class="grid grid-cols-[400px_1fr] divide-x m-10 border min-w-fit flex-1">
    <div class="h-full flex flex-col divide-y">
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
            Zmień lub zobacz informacje o koncie
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

    <div class="divide-y flex flex-col">
      <div class="text-2xl font-bold flex justify-between items-center gap-5 p-5">
        Wydarzenia
        <JoinTeamButton class="whitespace-nowrap" @click="joinExternalTeamModal = true" />
      </div>
      <div class="grow">
        <Placeholder class="w-full h-full">
          Strona w trakcie tworzenia!<br>
        </Placeholder>
      </div>
    </div>
  </div>
</template>
