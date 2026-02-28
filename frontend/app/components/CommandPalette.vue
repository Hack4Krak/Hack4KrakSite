<script setup lang="ts">
const open = defineModel<boolean>({ default: false })

function close() {
  open.value = false
}

const { data: tasks } = await useApi('/tasks/list')

const groups = computed(() => [
  {
    id: 'tasks',
    label: 'Zadania',
    items: (tasks.value ?? []).map(task => ({
      label: task.name,
      icon: 'mdi:flag',
      to: `/tasks/story/${task.id}`,
      onSelect: close,
    })),
  },
  {
    id: 'navigation',
    label: 'Nawigacja',
    items: [
      { label: 'Strona główna', icon: 'mdi:home', to: '/', onSelect: close },
      { label: 'Zadania', icon: 'mdi:map', to: '/tasks', onSelect: close },
      { label: 'Ranking', icon: 'mdi:trophy', to: '/leaderboard', onSelect: close },
      { label: 'Regulamin', icon: 'mdi:file-document', to: '/docs/rules', onSelect: close },
      { label: 'FAQ', icon: 'mdi:help-circle', to: '/docs/faq', onSelect: close },
      { label: 'O nas', icon: 'mdi:information', to: '/about_us', onSelect: close },
    ],
  },
  {
    id: 'account',
    label: 'Konto',
    items: [
      { label: 'Zaloguj się', icon: 'pixelarticons:login', to: '/login', onSelect: close },
      { label: 'Zarejestruj się', icon: 'mdi:account-plus', to: '/register', onSelect: close },
      { label: 'Panel', icon: 'mdi:view-dashboard', to: '/panel/', onSelect: close },
      { label: 'Profil', icon: 'mdi:account', to: '/panel/profile', onSelect: close },
      { label: 'Drużyna', icon: 'mdi:account-group', to: '/panel/team', onSelect: close },
    ],
  },
  {
    id: 'contact',
    label: 'Kontakt',
    items: [
      { label: 'Discord', icon: 'ic:baseline-discord', to: 'https://discord.gg/ASPqckzEd8', target: '_blank', onSelect: close },
      { label: 'Instagram', icon: 'mdi:instagram', to: 'https://www.instagram.com/hack4krak/', target: '_blank', onSelect: close },
      { label: 'LinkedIn', icon: 'mdi:linkedin', to: 'https://pl.linkedin.com/company/hack4krak', target: '_blank', onSelect: close },
      { label: 'Email', icon: 'mdi:envelope', to: 'mailto:hack4krak@gmail.com', onSelect: close },
    ],
  },
])
</script>

<template>
  <UModal v-model:open="open">
    <template #content>
      <UCommandPalette
        :groups="groups"
        placeholder="Szukaj stron..."
        close
        icon="mdi:magnify"
        @update:open="open = $event"
      >
        <template #footer>
          <div class="flex items-center justify-between gap-2 text-xs text-muted">
            <div class="flex items-center gap-2">
              <UKbd value="↑" size="sm" />
              <UKbd value="↓" size="sm" />
              <span>Nawigacja</span>
            </div>
            <div class="flex items-center gap-2">
              <UKbd value="↵" size="sm" />
              <span>Otwórz</span>
            </div>
            <div class="flex items-center gap-2">
              <UKbd value="esc" size="sm" />
              <span>Zamknij</span>
            </div>
          </div>
        </template>
      </UCommandPalette>
    </template>
  </UModal>
</template>
