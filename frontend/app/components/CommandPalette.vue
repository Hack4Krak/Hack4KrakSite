<script setup lang="ts">
const open = defineModel<boolean>({ default: false })
const toast = useToast()
const { $auth } = useNuxtApp()

const flagModalOpen = useFlagModalState()
const flagInput = ref('')

function close() {
  open.value = false
}

const { data: tasks } = await useApi('/tasks/list')

function openFlagModal() {
  close()
  flagInput.value = ''
  flagModalOpen.value = true
}

async function submitFlag() {
  try {
    await $auth('/flag/submit', {
      method: 'POST',
      body: { flag: flagInput.value },
    })
    toast.add({ title: 'Brawo!', description: 'To była poprawna flaga!', color: 'success' })
    flagModalOpen.value = false
    flagInput.value = ''
  } catch {
    toast.add({ title: 'Niepoprawna flaga', color: 'error' })
  }
}

async function logout() {
  close()
  await $auth('/auth/logout', {
    method: 'POST',
    credentials: 'include',
  })
  await refreshNuxtData()
  await navigateTo('/login')
}

const { data: isLoggedIn } = useAuth('/auth/status', {
  redirect: 'error',
  onResponseError: () => {},
})

const { data: team } = await useAuth('/teams/membership/my_team', {
  onResponseError: () => {},
  redirect: 'error',
})

const groups = computed(() => [
  {
    id: 'tasks',
    label: 'Zadania',
    items: [
      { label: 'Mapa zadań', icon: 'mdi:map', to: '/tasks', onSelect: close },
      ...(isLoggedIn.value && team.value
        ? [{ label: 'Złóż flagę', icon: 'mdi:flag-plus', kbds: ['S', 'F'], onSelect: openFlagModal }]
        : []),
      ...(tasks.value ?? []).map(task => ({
        label: task.name,
        icon: 'mdi:flag',
        to: `/tasks/story/${task.id}`,
        onSelect: close,
      })),
    ],
  },
  {
    id: 'navigation',
    label: 'Nawigacja',
    items: [
      { label: 'Strona główna', icon: 'mdi:home', to: '/', onSelect: close },
      { label: 'Ranking', icon: 'mdi:trophy', to: '/leaderboard', onSelect: close },
      ...(isLoggedIn.value
        ? [{ label: 'Panel', icon: 'mdi:view-dashboard', to: '/panel/', onSelect: close }]
        : []),
      { label: 'Regulamin', icon: 'mdi:file-document', to: '/docs/rules', onSelect: close },
    ],
  },
  {
    id: 'account',
    label: 'Konto',
    items: [
      ...(!isLoggedIn.value
        ? [
            { label: 'Zaloguj się', icon: 'pixelarticons:login', to: '/login', onSelect: close },
            { label: 'Zarejestruj się', icon: 'mdi:account-plus', to: '/register', onSelect: close },
          ]
        : [
            { label: 'Profil', icon: 'mdi:account', to: '/panel/profile', onSelect: close },
            ...(team.value
              ? [{ label: 'Drużyna', icon: 'mdi:account-group', to: '/panel/team', onSelect: close }]
              : []),
            { label: 'Wyloguj się', icon: 'mdi:logout', onSelect: logout },
          ]),
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
  <UModal v-model:open="flagModalOpen" title="Złóż flagę" description="Wpisz flagę w formacie hack4KrakCTF{...}">
    <template #body>
      <form class="flex flex-col gap-4" @submit.prevent="submitFlag">
        <UInput v-model="flagInput" placeholder="hack4KrakCTF{...}" autofocus />
        <UButton type="submit" block>
          Sprawdź
        </UButton>
      </form>
    </template>
  </UModal>

  <UModal v-model:open="open" title="Paleta poleceń" description="Szukaj i nawiguj po stronie">
    <template #content>
      <UCommandPalette
        :groups="groups"
        placeholder="Szukaj..."
        close
        icon="mdi:magnify"
        @update:open="open = $event"
      >
        <template #footer>
          <div class="flex items-center justify-between gap-2 text-sm text-muted">
            <div class="flex items-center gap-2">
              <UKbd>
                <UIcon name="pixelarticons:arrow-up" />
              </UKbd>
              <UKbd>
                <UIcon name="pixelarticons:arrow-down" />
              </UKbd>
              <span>Nawigacja</span>
            </div>
            <div class="flex items-center gap-2">
              <UKbd>
                <UIcon name="pixelarticons:corner-down-left" />
              </UKbd>
              <span>Otwórz</span>
            </div>
            <div class="flex items-center gap-2">
              <UKbd value="esc" />
              <span>Zamknij</span>
            </div>
          </div>
        </template>
      </UCommandPalette>
    </template>
  </UModal>
</template>
