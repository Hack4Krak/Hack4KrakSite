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

const isTasksCollapsed = ref(true)

const tasksHeader = {
  label: 'Zadania',
  slot: 'tasks-header' as const,
  ui: { item: 'cursor-pointer' },
  onSelect(e: Event) {
    e.preventDefault()
    isTasksCollapsed.value = !isTasksCollapsed.value
  },
}

const groups = computed(() => [
  {
    id: 'tasks',
    items: [
      tasksHeader,
      ...(isLoggedIn.value && team.value?.team_name
        ? [{ label: 'Złóż flagę', icon: 'pixelarticons:flag', kbds: ['S', 'F'], onSelect: openFlagModal }]
        : []),
      ...(tasks.value ?? []).map(task => ({
        label: task.name,
        icon: 'pixelarticons:flag',
        to: `/tasks/story/${task.id}`,
        onSelect: close,
      })),
    ],
    postFilter(searchTerm: string, filteredItems: any[]) {
      const rest = filteredItems.filter(i => i.slot !== 'tasks-header')
      return (!searchTerm && isTasksCollapsed.value) ? [tasksHeader] : [tasksHeader, ...rest]
    },
  },
  {
    id: 'navigation',
    label: 'Nawigacja',
    items: [
      { label: 'Strona główna', icon: 'pixelarticons:home', to: '/', onSelect: close },
      ...NAVBAR_ITEMS.flat()
        .filter(item => 'to' in item)
        .map(item => ({ label: item.label, icon: 'pixelarticons:link', to: item.to, onSelect: close })),
      ...(isLoggedIn.value
        ? [{ label: 'Panel', icon: 'pixelarticons:dashboard', to: '/panel/', onSelect: close }]
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
            { label: 'Zarejestruj się', icon: 'pixelarticons:user-plus', to: '/register', onSelect: close },
          ]
        : [
            { label: 'Profil', icon: 'pixelarticons:user', to: '/panel/profile', onSelect: close },
            ...(team.value?.team_name
              ? [{ label: 'Drużyna', icon: 'pixelarticons:users', to: '/panel/team', onSelect: close }]
              : []),
            { label: 'Wyloguj się', icon: 'pixelarticons:logout', onSelect: handleLogout },
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
        icon="pixelarticons:search"
        @update:open="isOpen = $event"
      >
        <template #tasks-header-leading />
        <template #tasks-header-label>
          <span class="font-semibold text-highlighted">Zadania</span>
        </template>
        <template #tasks-header-trailing>
          <UIcon
            :name="isTasksCollapsed ? 'pixelarticons:chevron-down' : 'pixelarticons:chevron-up'"
            class="size-5 text-primary"
          />
        </template>
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
