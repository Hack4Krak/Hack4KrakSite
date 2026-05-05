<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'
import type { EventRegistrationDraft } from '~/composables/useEventRegistration'

interface Props {
  redirectTo?: string
  maxTeamSize?: number
}

withDefaults(defineProps<Props>(), {
  redirectTo: '/panel/team',
  maxTeamSize: 5,
})

const registrationTeam = defineModel<EventRegistrationDraft['team']>('registrationTeam')

const { $auth } = useNuxtApp()
const toast = useToast()

const { data: team, error: teamError, refresh: refreshTeam } = await useAuth('/teams/membership/my_team', {
  onResponseError: undefined,
})

const { data: pendingInvites, refresh: refreshPendingInvites } = await useAuth('/teams/invitations/', {
  onResponseError: undefined,
})

const typedTeam = computed(() => {
  if (teamError.value || !team.value) {
    return null
  }

  return team.value as NonNullable<ApiResponse<'my_team'>>
})
const hasTeam = computed(() => typedTeam.value !== null)
const pendingInvitesList = computed(() => Array.isArray(pendingInvites.value) ? pendingInvites.value : [])
const invitedUsersList = computed(() => Array.isArray(typedTeam.value?.invited_users) ? typedTeam.value.invited_users : [])
const isLeader = computed(() => Array.isArray(typedTeam.value?.invited_users))

const newTeamName = ref('')
const leaveTeamModal = ref(false)
const deleteTeamModal = ref(false)
const kickUserModal = ref(false)
const revokeInvitationModal = ref(false)
const kickedUser = ref('')
const revokedInvitation = ref('')

async function syncAfterMutation() {
  await Promise.allSettled([
    refreshTeam(),
    refreshPendingInvites(),
    refreshNuxtData(),
  ])
}

async function createTeam(teamName: string) {
  const name = teamName.trim()
  if (!name) {
    return false
  }

  try {
    await $auth('/teams/create', {
      method: 'POST',
      body: { team_name: name },
    })
    toast.add({ title: 'Drużyna utworzona', color: 'success' })
    await syncAfterMutation()
    return true
  } catch (error) {
    toast.add({ title: 'Nie udało się utworzyć drużyny', description: (error as Error).message, color: 'error' })
    return false
  }
}

async function inviteUser(username: string) {
  try {
    await $auth('/teams/management/invite_user', {
      method: 'POST',
      body: { username },
    })
    toast.add({ title: 'Zaproszenie wysłane', color: 'success' })
    await syncAfterMutation()
    return true
  } catch (error) {
    toast.add({ title: 'Nie udało się zaprosić', description: (error as Error).message, color: 'error' })
    return false
  }
}

async function renameTeam(name: string) {
  try {
    await $auth('/teams/management/rename', {
      method: 'PATCH',
      body: { new_name: name },
    })
    toast.add({ title: 'Nazwa drużyny zmieniona', color: 'success' })
    await syncAfterMutation()
    return true
  } catch (error) {
    toast.add({ title: 'Nie udało się zmienić nazwy', description: (error as Error).message, color: 'error' })
    return false
  }
}

async function acceptInvitation(teamName: string) {
  try {
    await $auth('/teams/invitations/accept_invitation/{team_name}', {
      method: 'POST',
      path: { team_name: teamName },
    })
    toast.add({ title: 'Dołączono do drużyny', color: 'success' })
    await syncAfterMutation()
    return true
  } catch (error) {
    toast.add({ title: 'Nie udało się dołączyć', description: (error as Error).message, color: 'error' })
    return false
  }
}

watch(
  [hasTeam, () => typedTeam.value?.team_name],
  ([nextHasTeam, nextTeamName]) => {
    if (!registrationTeam.value) {
      return
    }

    registrationTeam.value.confirmed = nextHasTeam
    registrationTeam.value.teamName = nextTeamName ?? ''
  },
  { immediate: true },
)

async function handleCreateTeam() {
  const created = await createTeam(newTeamName.value)
  if (created) {
    newTeamName.value = ''
  }
}

function handleKick(username: string) {
  kickedUser.value = username
  kickUserModal.value = true
}

function handleRevoke(username: string) {
  revokedInvitation.value = username
  revokeInvitationModal.value = true
}
</script>

<template>
  <div class="space-y-6">
    <LazyPanelModalConfirmDeleteModal
      v-model="deleteTeamModal"
      url="/teams/management/delete"
      modal-title="Usuwanie drużyny"
      modal-description="Czy na pewno chcesz usunąć drużynę? Ta operacja jest nieodwracalna."
      toast-success-message="Pomyślnie usunięto drużynę"
      :request-body="undefined"
      :redirect-to="redirectTo"
      hydrate-on-idle
    />
    <LazyPanelModalConfirmDeleteModal
      v-model="leaveTeamModal"
      url="/teams/membership/leave_team"
      modal-title="Opuść drużynę"
      modal-description="Czy na pewno chcesz opuścić drużynę? Stracisz dostęp do jej ustawień."
      toast-success-message="Pomyślnie opuszczono drużynę"
      :request-body="undefined"
      :redirect-to="redirectTo"
      hydrate-on-idle
    />
    <LazyPanelModalConfirmDeleteModal
      v-model="kickUserModal"
      url="/teams/management/kick_user"
      modal-title="Wyrzucenie użytkownika"
      modal-description="Czy na pewno chcesz wyrzucić użytkownika z drużyny?"
      toast-success-message="Pomyślnie wyrzucono użytkownika"
      :request-body="{ username: kickedUser }"
      :redirect-to="redirectTo"
      hydrate-on-idle
    />
    <LazyPanelModalConfirmDeleteModal
      v-model="revokeInvitationModal"
      :url="`/teams/management/revoke_invitation/${revokedInvitation}` as any"
      modal-title="Cofnięcie zaproszenia"
      modal-description="Czy na pewno chcesz cofnąć zaproszenie?"
      toast-success-message="Pomyślnie cofnięto zaproszenie"
      :request-body="undefined"
      :redirect-to="redirectTo"
      hydrate-on-idle
    />

    <section
      v-if="!hasTeam && pendingInvitesList.length > 0"
      class="panel-card panel-card-accent"
    >
      <header class="panel-card-header">
        <h2 class="panel-card-title">
          Zaproszenia do Ciebie
        </h2>
      </header>
      <ul class="panel-card-body space-y-3">
        <li
          v-for="invite in pendingInvitesList"
          :key="invite"
          class="panel-subcard grid grid-cols-1 sm:grid-cols-[1fr_auto] items-center gap-3"
        >
          <div>
            <p class="font-medium">
              <span class="text-primary font-bold">{{ invite }}</span> zaprasza Cię do drużyny
            </p>
            <p class="text-xs text-muted mt-1">
              Możesz dołączyć od razu albo utworzyć własną drużynę.
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
    </section>

    <div v-if="!hasTeam" class="grid gap-6 md:grid-cols-2">
      <section class="panel-card">
        <header class="panel-card-header">
          <h2 class="panel-card-title">
            Stwórz drużynę
          </h2>
        </header>
        <form class="panel-card-body flex flex-col gap-4" @submit.prevent="handleCreateTeam">
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
            :disabled="!newTeamName.trim()"
            class="self-start"
          >
            Utwórz drużynę
          </ElevatedButton>
          <p class="text-xs text-muted leading-relaxed">
            Jeśli grasz solo, utwórz drużynę jednoosobową. Później nadal możesz ją rozbudować lub zmienić nazwę.
          </p>
        </form>
      </section>

      <section class="panel-card">
        <header class="panel-card-header">
          <h2 class="panel-card-title">
            Dołącz przez zaproszenie
          </h2>
        </header>
        <div class="panel-card-body flex flex-col gap-3">
          <p class="font-bold text-lg">
            Czekasz na zaproszenie?
          </p>
          <p class="text-sm text-muted leading-relaxed">
            Lider drużyny może zaprosić Cię po nicku. Gdy zaproszenie będzie gotowe, pojawi się ono powyżej.
          </p>
        </div>
      </section>
    </div>

    <template v-else>
      <PanelEventTeamRoster
        :team="typedTeam"
        :invited-users="invitedUsersList"
        :is-leader="isLeader"
        :max-team-size="maxTeamSize"
        @invite="inviteUser"
        @rename="renameTeam"
        @kick="handleKick"
        @revoke-invite="handleRevoke"
      />

      <section class="panel-card panel-card-danger">
        <header class="panel-card-header">
          <h2 class="text-sm font-bold uppercase tracking-wider text-error">
            Strefa zagrożenia
          </h2>
        </header>
        <div class="panel-card-body grid grid-cols-1 sm:grid-cols-[1fr_auto] gap-4 items-center">
          <div>
            <p v-if="isLeader" class="font-medium">
              Usuń drużynę
            </p>
            <p v-else class="font-medium">
              Opuść drużynę
            </p>
            <p class="text-sm text-muted mt-1">
              {{ isLeader
                ? 'Drużyna zostanie usunięta wraz z zaproszeniami. Operacja nieodwracalna.'
                : 'Stracisz dostęp do ustawień drużyny. Możesz wrócić z nowym zaproszeniem.' }}
            </p>
          </div>
          <button
            v-if="isLeader"
            type="button"
            class="px-5 py-2.5 border-2 border-error text-error text-sm font-bold uppercase tracking-wider hover:bg-error hover:text-default transition-colors cursor-pointer"
            @click="deleteTeamModal = true"
          >
            Usuń drużynę
          </button>
          <button
            v-else
            type="button"
            class="px-5 py-2.5 border-2 border-error text-error text-sm font-bold uppercase tracking-wider hover:bg-error hover:text-default transition-colors cursor-pointer"
            @click="leaveTeamModal = true"
          >
            Opuść drużynę
          </button>
        </div>
      </section>
    </template>
  </div>
</template>
