<script setup lang="ts">
const { $api } = useNuxtApp()

const { data } = await useAuth('/account/')

const updateAccountModal = ref(false)
const deleteAccountModal = ref(false)
const changePasswordModal = ref(false)

const { data: team, error } = await useAuth('/teams/membership/my_team', {
  onResponseError: () => {
    throw new Error('Response error')
  },
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
    <PanelModalUpdateAccountModal v-model="updateAccountModal" />
    <PanelModalChangePasswordModal v-model="changePasswordModal" />
    <PanelModalConfirmDeleteModal
      v-model="deleteAccountModal"
      url="/account/delete"
      modal-title="Usuwanie konta"
      modal-description="Czy na pewno chcesz usunąć konto? Ta operacja jest nieodwracalna."
      toast-success-message="Pomyślnie usunięto konto"
      :request-body="undefined"
      redirect-to="/"
    />

    <div class="flex flex-col flex-grow items-center justify-center max-h-[15em]">
      <div class="text-center">
        <h1 class="text-5xl font-bold">
          Witaj {{ data?.username }}!
        </h1>
        <h2 class="text-4xl font-light mt-2">
          Życzymy powodzenia na wydarzeniu!
        </h2>
      </div>
    </div>

    <div class="container mx-auto grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-x-15 gap-y-5">
      <PanelTile class="row-span-2 min-h-100 min-w-70">
        <PanelTileWithTeam v-if="!error" :team-name="team!.team_name" />
        <PanelTileWithoutTeam v-else />
      </PanelTile>
      <PanelTile class="row-span-2">
        <PanelTileMain />
      </PanelTile>
      <PanelTile class="min-h-25 overflow-hidden" />
      <PanelTile>
        <div class="p-5">
          <h1 class="font-bold text-2xl">
            Ustawienia konta
          </h1>
          <div class="flex flex-col gap-3 mt-3 justify-center">
            <GhostButtonWithIcon icon="mdi:account" description="Zmień lub zobacz dane o końcie" @click="navigateTo('/account/submit_personal_info')" />
            <GhostButtonWithIcon icon="mdi:account-cog" description="Zmień ustawienia konta" @click="updateAccountModal = true" />
            <GhostButtonWithIcon icon="mdi:account-key" description="Zmień hasło" @click="changePasswordModal = true" />
            <GhostButtonWithIcon icon="mdi:account-remove" description="Usuń konto" @click="deleteAccountModal = true" />
            <GhostButtonWithIcon icon="mdi:logout" description="Wyloguj się" @click="logout" />
          </div>
        </div>
      </PanelTile>
    </div>
  </div>
</template>
