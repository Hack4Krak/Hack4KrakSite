<script setup lang="ts">
const { $api } = useNuxtApp()

const { data: user } = await useAuth('/account/')
const { data: verification } = await useAuth('/account/verification')

const joinExternalTeamModal = ref(false)
const updateAccountModal = ref(false)
const changePasswordModal = ref(false)
const deleteAccountModal = ref(false)
const qrCodeModal = ref(false)

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
  <LazyPanelModalJoinExternalTeam v-model="joinExternalTeamModal" hydrate-on-idle />
  <LazyPanelModalUpdateAccountModal v-model="updateAccountModal" hydrate-on-idle />
  <LazyPanelModalChangePasswordModal v-model="changePasswordModal" hydrate-on-idle />
  <LazyPanelModalConfirmDeleteModal
    v-model="deleteAccountModal"
    url="/account/delete"
    modal-title="Usuń konto"
    modal-description="Czy na pewno chcesz usunąć swoje konto? Ta operacja jest nieodwracalna."
    toast-success-message="Pomyślnie usunięto konto"
    :request-body="undefined"
    redirect-to="/"
    hydrate-on-idle
  />
  <LazyPanelModalQRCode v-model="qrCodeModal" :verification-id="verification?.verification_id" hydrate-on-idle />

  <div class="grid grid-cols-[400px_1fr] divide-x m-10 border min-w-fit flex-1">
    <div class="h-full flex flex-col divide-y">
      <div class="grow items-center justify-center flex flex-col">
        <h3 class="text-xl font-bold">
          {{ user?.username }}
        </h3>
        <span>Przesłane flagi: 0</span>
      </div>

      <div v-if="verification?.applied_tags && verification.applied_tags.length > 0" class="p-5">
        <h1 class="font-bold text-xl mb-3">
          Przypisane tagi
        </h1>
        <div class="flex flex-col gap-2">
          <div
            v-for="tag in verification.applied_tags"
            :key="tag.id"
            class="p-2 rounded bg-elevated border border-accented"
          >
            <p class="font-semibold text-sm">
              {{ tag.name }}
            </p>
            <p class="text-xs text-gray-400">
              {{ tag.description }}
            </p>
          </div>
        </div>
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
          <UButton icon="mdi:qrcode" variant="ghost" color="neutral" @click="qrCodeModal = true">
            Zobacz swój identyfikacyjny kod QR
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
