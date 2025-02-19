<script setup lang="ts">
const { data } = await useAuth('/teams/invitations/', {
  key: 'teams-invitations',
})

const open = defineModel<boolean>()
</script>

<template>
  <UModal v-model:open="open" title="Zaproszenia" description="Zbierz brygade swoich poteżnych team-matów do walki!" :ui="{ footer: 'justify-end' }">
    <template #body>
      <div v-if="data?.length === 0">
        Brak danych do wyświetlenia
      </div>
      <div v-for="d in data" :key="d">
        Unimplemented:
        {{ d }}
      </div>
    </template>

    <template #footer>
      <UButton label="Zamknij" color="neutral" variant="outline" @click="open = false" />
    </template>
  </UModal>
</template>
