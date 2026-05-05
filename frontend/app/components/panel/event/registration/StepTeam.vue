<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'
import type { EventRegistrationDraft } from '~/composables/useEventRegistration'

const team = defineModel<EventRegistrationDraft['team']>('team', { required: true })

const { data: existingTeam, refresh: refreshTeam } = await useAuth('/teams/membership/my_team', {
  onResponseError: undefined,
})

const hasTeam = computed(() => Boolean(existingTeam.value))
const typedTeam = computed(() => existingTeam.value as ApiResponse<'my_team'> | null)

// Management data — only fetched when user has a team (avoids 403 for non-leaders)
const teamManagement = ref<null | object>(null)
const invitedUsersList = ref<string[]>([])
const isLeader = computed(() => Boolean(teamManagement.value))

async function loadManagementData() {
  const $auth = useNuxtApp().$auth
  const [mgmt, invited] = await Promise.all([
    $auth('/teams/management/', { onResponseError: undefined }),
    $auth('/teams/management/invited_users', { onResponseError: undefined }),
  ])
  teamManagement.value = (mgmt && typeof mgmt === 'object' && !('error' in (mgmt as object))) ? mgmt as object : null
  invitedUsersList.value = Array.isArray(invited) ? invited as string[] : []
}

// Pending invitations — only fetched when user opens the invitations tab
const pendingInvitesList = ref<string[]>([])
const loadingInvitations = ref(false)

async function loadInvitations() {
  loadingInvitations.value = true
  try {
    const data = await useNuxtApp().$auth('/teams/invitations/', { onResponseError: undefined })
    pendingInvitesList.value = Array.isArray(data) ? data as string[] : []
  }
  finally {
    loadingInvitations.value = false
  }
}

const mode = ref<'overview' | 'choose' | 'create' | 'invitations'>(hasTeam.value ? 'overview' : 'choose')

if (hasTeam.value) {
  team.value.confirmed = true
  team.value.teamName = existingTeam.value?.team_name ?? ''
  await loadManagementData()
}
else {
  team.value.confirmed = false
  team.value.teamName = ''
}

watch(hasTeam, async (next) => {
  if (next) {
    mode.value = 'overview'
    team.value.confirmed = true
    team.value.teamName = existingTeam.value?.team_name ?? ''
    await loadManagementData()
    return
  }
  team.value.confirmed = false
  team.value.teamName = ''
})

const newTeamName = ref('')
const submitting = ref(false)

async function selectMode(nextMode: 'create' | 'invitations') {
  mode.value = nextMode
  if (nextMode === 'invitations') {
    await loadInvitations()
  }
}

async function createTeam() {
  if (!newTeamName.value.trim() || submitting.value)
    return
  submitting.value = true
  try {
    await useNuxtApp().$auth('/teams/create', {
      method: 'POST',
      body: { team_name: newTeamName.value.trim() },
    })
    useToast().add({ title: 'Drużyna utworzona!', color: 'success' })
    await refreshTeam()
    await refreshNuxtData()
  }
  catch (error: unknown) {
    useToast().add({ title: 'Nie udało się utworzyć drużyny', description: (error as Error).message, color: 'error' })
  }
  finally {
    submitting.value = false
  }
}

async function inviteByUsername(username: string) {
  try {
    await useNuxtApp().$auth('/teams/management/invite_user', {
      method: 'POST',
      body: { username },
    })
    useToast().add({ title: 'Zaproszenie wysłane', color: 'success' })
    await loadManagementData()
  }
  catch (error) {
    useToast().add({ title: 'Nie udało się zaprosić', description: (error as Error).message, color: 'error' })
  }
}

async function acceptInvitation(teamName: string) {
  try {
    await useNuxtApp().$auth('/teams/invitations/accept_invitation/{team_name}', {
      method: 'POST',
      path: { team_name: teamName },
    })
    useToast().add({ title: 'Dołączono do drużyny', color: 'success' })
    await refreshTeam()
    await loadInvitations()
    await refreshNuxtData()
  }
  catch (error) {
    useToast().add({ title: 'Nie udało się dołączyć', description: (error as Error).message, color: 'error' })
  }
}
</script>

<template>
  <section class="space-y-6">
    <header>
      <h2 class="font-pixelify text-3xl lg:text-4xl leading-none">
        Twoja <span class="text-primary">drużyna.</span>
      </h2>
      <p class="text-sm text-muted mt-3 max-w-xl leading-relaxed">
        Każdy udział jest przypisany do drużyny. Jeśli grasz solo, po prostu utwórz drużynę jednoosobową i przejdź dalej.
      </p>
    </header>

    <div v-if="hasTeam" class="space-y-4">
      <PanelEventTeamRoster
        :team="typedTeam"
        :invited-users="invitedUsersList"
        :is-leader="isLeader"
        @invite="inviteByUsername"
      />
      <p class="text-xs text-muted flex items-center gap-2">
        <UIcon name="pixelarticons:check" class="size-4 text-success" />
        Drużyna gotowa. Możesz przejść do następnego kroku.
      </p>
    </div>

    <div v-else class="space-y-4">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-px bg-surface-muted border-2 border-surface-muted">
        <button
          type="button"
          class="bg-default px-4 py-4 text-left transition-colors cursor-pointer"
          :class="mode === 'create' ? 'bg-primary/5' : 'hover:bg-surface-muted/30'"
          @click="selectMode('create')"
        >
          <span
            class="block text-sm font-bold uppercase tracking-wider"
            :class="mode === 'create' ? 'text-primary' : 'text-default'"
          >
            Stwórz drużynę
          </span>
          <span class="block text-xs text-muted mt-1.5 normal-case tracking-normal">
            Zostaniesz liderem i zaprosisz osoby po nicku.
          </span>
        </button>
        <button
          type="button"
          class="bg-default px-4 py-4 text-left transition-colors cursor-pointer relative"
          :class="mode === 'invitations' ? 'bg-primary/5' : 'hover:bg-surface-muted/30'"
          @click="selectMode('invitations')"
        >
          <span
            class="block text-sm font-bold uppercase tracking-wider"
            :class="mode === 'invitations' ? 'text-primary' : 'text-default'"
          >
            Sprawdź zaproszenia
          </span>
          <span class="block text-xs text-muted mt-1.5 normal-case tracking-normal">
            Przyjmij zaproszenie od lidera lub nauczyciela.
          </span>
        </button>
      </div>

      <div v-if="mode === 'choose'" class="border-2 border-dashed border-surface-muted p-5 text-sm text-muted">
        Wybierz jedną z opcji powyżej, żeby przejść dalej.
      </div>

      <form
        v-else-if="mode === 'create'"
        class="border-2 border-surface-muted p-5 space-y-4"
        @submit.prevent="createTeam"
      >
        <label class="block">
          <span class="text-xs uppercase tracking-wider text-muted font-bold">
            Nazwa drużyny
          </span>
          <UInput
            v-model="newTeamName"
            placeholder="np. byte_busters"
            size="lg"
            class="mt-2"
            :ui="{ base: 'rounded-none' }"
          />
        </label>
        <ElevatedButton
          type="submit"
          :disabled="!newTeamName.trim() || submitting"
          class="w-full"
        >
          Utwórz drużynę
        </ElevatedButton>
        <p class="text-xs text-muted text-center">
          Zostaniesz liderem. Jeśli grasz solo, zostaw drużynę jednoosobową i przejdź dalej.
        </p>
      </form>

      <div v-else-if="mode === 'invitations'" class="border-2 border-surface-muted">
        <header class="px-5 py-3 border-b-2 border-surface-muted bg-surface-muted/30 flex items-center justify-between gap-4">
          <h3 class="text-sm font-bold uppercase tracking-wider text-primary">
            Twoje zaproszenia
          </h3>
          <span v-if="pendingInvitesList.length > 0" class="text-xs text-muted tabular-nums">
            {{ pendingInvitesList.length }} aktywne
          </span>
        </header>
        <div v-if="loadingInvitations" class="p-6 text-center text-sm text-muted">
          Ładowanie...
        </div>
        <ul v-else-if="pendingInvitesList.length > 0" class="divide-y-2 divide-surface-muted">
          <li
            v-for="invite in pendingInvitesList"
            :key="invite"
            class="grid grid-cols-1 sm:grid-cols-[1fr_auto] items-center gap-3 px-5 py-4"
          >
            <div>
              <p class="font-medium">
                <span class="font-bold text-primary">{{ invite }}</span> zaprasza Cię do drużyny
              </p>
              <p class="text-xs text-muted mt-1">
                Po dołączeniu od razu trafisz do tej drużyny.
              </p>
            </div>
            <button
              type="button"
              class="px-4 py-2 bg-primary text-default text-sm font-bold uppercase tracking-wider hover:bg-secondary cursor-pointer transition-colors"
              @click="acceptInvitation(invite)"
            >
              Dołącz
            </button>
          </li>
        </ul>
        <div v-else class="p-6 text-center space-y-3">
          <p class="font-bold">
            Brak nowych zaproszeń
          </p>
          <p class="text-xs text-muted leading-relaxed">
            Poproś lidera istniejącej drużyny o wysłanie zaproszenia po Twoim nicku.
            Jeśli nie masz zaproszenia, utwórz własną drużynę i kontynuuj rejestrację.
          </p>
        </div>
      </div>
    </div>
  </section>
</template>
