<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'
import type { EventRegistrationDraft } from '~/composables/useEventRegistration'

interface Props {
  redirectTo?: string
  maxTeamSize?: number
  showDangerZone?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  redirectTo: '/panel/team',
  maxTeamSize: 5,
  showDangerZone: true,
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
const modals = reactive({
  leaveTeam: false,
  deleteTeam: false,
  kickUser: false,
  revokeInvitation: false,
})
const selectedUsers = reactive({
  kicked: '',
  revokedInvitation: '',
})

async function refreshAll() {
  await Promise.allSettled([
    refreshTeam(),
    refreshPendingInvites(),
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
    await refreshAll()
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
    await refreshAll()
    return true
  } catch {
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
    await refreshAll()
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
    await refreshAll()
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
  selectedUsers.kicked = username
  modals.kickUser = true
}

function handleRevoke(username: string) {
  selectedUsers.revokedInvitation = username
  modals.revokeInvitation = true
}
</script>

<template>
  <div class="space-y-6">
    <LazyPanelModalConfirmDeleteModal
      v-model="modals.deleteTeam"
      url="/teams/management/delete"
      modal-title="Usuwanie drużyny"
      modal-description="Czy na pewno chcesz usunąć drużynę? Ta operacja jest nieodwracalna."
      toast-success-message="Pomyślnie usunięto drużynę"
      :request-body="undefined"
      :redirect-to="props.redirectTo"
      hydrate-on-idle
    />
    <LazyPanelModalConfirmDeleteModal
      v-model="modals.leaveTeam"
      url="/teams/membership/leave_team"
      modal-title="Opuść drużynę"
      modal-description="Czy na pewno chcesz opuścić drużynę? Stracisz dostęp do jej ustawień."
      toast-success-message="Pomyślnie opuszczono drużynę"
      :request-body="undefined"
      :redirect-to="props.redirectTo"
      hydrate-on-idle
    />
    <LazyPanelModalConfirmDeleteModal
      v-model="modals.kickUser"
      url="/teams/management/kick_user"
      modal-title="Wyrzucenie użytkownika"
      modal-description="Czy na pewno chcesz wyrzucić użytkownika z drużyny?"
      toast-success-message="Pomyślnie wyrzucono użytkownika"
      :request-body="{ username: selectedUsers.kicked }"
      :redirect-to="props.redirectTo"
      hydrate-on-idle
    />
    <LazyPanelModalConfirmDeleteModal
      v-model="modals.revokeInvitation"
      :url="`/teams/management/revoke_invitation/${selectedUsers.revokedInvitation}` as any"
      modal-title="Cofnięcie zaproszenia"
      modal-description="Czy na pewno chcesz cofnąć zaproszenie?"
      toast-success-message="Pomyślnie cofnięto zaproszenie"
      :request-body="undefined"
      :redirect-to="props.redirectTo"
      hydrate-on-idle
    />

    <ul class="space-y-3">
      <li
        v-for="invite in pendingInvitesList"
        :key="invite"
        class="grid grid-cols-1 items-center gap-3 border-2 border-surface-muted bg-surface-muted/20 p-4 sm:grid-cols-[1fr_auto]"
      >
        <div>
          <p class="font-medium">
            <span class="text-primary font-bold">{{ invite }}</span> zaprasza Cię do drużyny
          </p>
          <p class="text-xs text-muted mt-1">
            Możesz dołączyć od razu albo utworzyć własną drużynę.
          </p>
        </div>
        <PanelActionButton
          filled
          @click="acceptInvitation(invite)"
        >
          Dołącz
        </PanelActionButton>
      </li>
    </ul>

    <div v-if="!hasTeam" class="grid gap-6 md:grid-cols-2">
      <PanelCard title="Stwórz drużynę">
        <form class="flex flex-col gap-4" @submit.prevent="handleCreateTeam">
          <UFormField label="Nazwa drużyny">
            <UInput
              v-model="newTeamName"
              placeholder="np. Zielone Smoki z Nowej Huty"
              size="lg"
              class="mt-2"
            />
          </UFormField>
          <ElevatedButton
            type="submit"
            :disabled="!newTeamName.trim()"
            class="self-start ml-1"
          >
            Utwórz drużynę
          </ElevatedButton>
          <p class="text-xs text-muted leading-relaxed">
            Nazwa może nawiązywać do Krakowa: smoki, kopce, dzielnice albo ulubione miejsce spotkań. Skład i nazwę możesz zmienić później.
          </p>
        </form>
      </PanelCard>

      <PanelCard title="Dołącz przez zaproszenie">
        <div class="flex flex-col gap-3">
          <p class="font-bold text-lg">
            Czekasz na zaproszenie?
          </p>
          <p class="text-sm text-muted leading-relaxed">
            Lider drużyny może zaprosić Cię po nicku. Jeśli nie masz jeszcze składu, poznasz ludzi na Discordzie Hack4Krak.
          </p>
        </div>
      </PanelCard>
    </div>

    <template v-else>
      <PanelEventTeamRoster
        :team="typedTeam"
        :invited-users="invitedUsersList"
        :is-leader="isLeader"
        :max-team-size="props.maxTeamSize"
        @invite="inviteUser"
        @rename="renameTeam"
        @kick="handleKick"
        @revoke-invite="handleRevoke"
      />

      <PanelCard v-if="props.showDangerZone" danger title="Strefa zagrożenia" title-class="text-error">
        <div class="grid grid-cols-1 items-center gap-4 sm:grid-cols-[1fr_auto]">
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
          <PanelActionButton
            v-if="isLeader"
            tone="danger"
            @click="modals.deleteTeam = true"
          >
            Usuń drużynę
          </PanelActionButton>
          <PanelActionButton
            v-else
            tone="danger"
            @click="modals.leaveTeam = true"
          >
            Opuść drużynę
          </PanelActionButton>
        </div>
      </PanelCard>
    </template>
  </div>
</template>
