<script setup lang="ts">
definePageMeta({
  layout: 'account',
})

const { data: user } = await useAuth('/account/')

const updateAccountModal = ref(false)
const changePasswordModal = ref(false)
const deleteAccountModal = ref(false)

useSeoMeta({
  title: 'Ustawienia konta',
  description: 'Zarządzaj danymi konta i bezpieczeństwem.',
})
</script>

<template>
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

  <div class="space-y-8 max-w-2xl">
    <header>
      <h1 class="font-pixelify text-3xl lg:text-4xl leading-none">
        Ustawienia konta
      </h1>
      <p class="text-sm text-muted mt-2">
        Dane konta i bezpieczeństwo.
      </p>
    </header>

    <div class="space-y-px bg-surface-muted border-2 border-surface-muted">
      <PanelTileSettingsRow label="Email" :value="user?.email ?? ''">
        <span class="text-[10px] font-pixelify uppercase tracking-widest text-success border border-success px-2 py-1">
          zweryfikowany
        </span>
      </PanelTileSettingsRow>

      <PanelTileSettingsRow label="Nazwa wyświetlana" :value="user?.username ?? ''">
        <UButton variant="ghost" color="neutral" size="sm" icon="pixelarticons:edit" @click="updateAccountModal = true">
          Edytuj
        </UButton>
      </PanelTileSettingsRow>

      <PanelTileSettingsRow label="Hasło" value="••••••••">
        <UButton variant="ghost" color="neutral" size="sm" icon="pixelarticons:lock" @click="changePasswordModal = true">
          Zmień
        </UButton>
      </PanelTileSettingsRow>
    </div>

    <PanelTileSettingsRow danger label="Strefa zagrożenia" value="Operacja nieodwracalna. Usuniemy konto i powiązane dane.">
      <UButton variant="outline" color="error" size="sm" icon="pixelarticons:trash" @click="deleteAccountModal = true">
        Usuń konto
      </UButton>
    </PanelTileSettingsRow>
  </div>
</template>
