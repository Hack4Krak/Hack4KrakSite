<script setup lang="ts">
import { LANDING_SOCIALS } from '~~/content/landing-socials'
import { NAVBAR_ITEMS } from '~~/content/navbar'

const isOpen = defineModel<boolean>({ default: false })

const isFlagModalOpen = useFlagModalState()
const { logout } = useLogout()

function close() {
  isOpen.value = false
}

const { data: tasks } = useLazyApi('/tasks/list')

function openFlagModal() {
  close()
  isFlagModalOpen.value = true
}

async function handleLogout() {
  close()
  await logout()
}

const { data: isLoggedIn } = useAuth('/auth/status', {
  redirect: 'error',
  onResponseError: undefined,
})

const { data: team } = useAuth('/teams/membership/my_team', {
  onResponseError: undefined,
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
      ...NAVBAR_ITEMS.flat()
        .filter(item => 'to' in item)
        .map(item => ({ label: item.label, icon: 'mdi:link', to: item.to, onSelect: close })),
      ...(isLoggedIn.value
        ? [{ label: 'Panel', icon: 'mdi:view-dashboard', to: '/panel/', onSelect: close }]
        : []),
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
            { label: 'Wyloguj się', icon: 'mdi:logout', onSelect: handleLogout },
          ]),
    ],
  },
  {
    id: 'contact',
    label: 'Kontakt',
    items: LANDING_SOCIALS.map(social => ({ ...social, onSelect: close })),
  },
])
</script>

<template>
  <UModal v-model:open="isOpen" title="Paleta poleceń" description="Szukaj i nawiguj po stronie">
    <template #content>
      <UCommandPalette
        :groups="groups"
        placeholder="Szukaj..."
        close
        icon="mdi:magnify"
        @update:open="isOpen = $event"
      >
        <template #empty>
          Brak wyników
        </template>
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
