<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'

useSeoMeta({
  title: 'Panel drużyny',
  description: 'Zarządzaj swoją drużyną, zapraszaj nowych członków i sprawdzaj postępy!',
})

const { data: team, refresh: refreshTeam } = await useAuth('/teams/membership/my_team', {
  onResponseError: undefined,
})

const { data: invitedUsers, refresh: refreshInvitations } = await useAuth('/teams/management/invited_users', {
  onResponseError: undefined,
})

const { data: pendingInvites, refresh: refreshPending } = await useAuth('/teams/invitations/', {
  onResponseError: undefined,
})

const { error } = await useAuth('/teams/management/', {
  onResponseError: undefined,
})

const isLeader = computed(() => error.value === undefined)
const typedTeam = computed(() => team.value as ApiResponse<'my_team'> | null)
const invitedUsersList = computed(() => Array.isArray(invitedUsers.value) ? invitedUsers.value : [])

const createTeamModal = ref(false)
const joinTeamModal = ref(false)
const leaveTeamModal = ref(false)
const deleteTeamModal = ref(false)
const kickUserModal = ref(false)
const kickedUser = ref('')
const revokeInvitationModal = ref(false)
const revokedInvitation = ref('')

async function handleInvite(username: string) {
  try {
    await useNuxtApp().$auth('/teams/management/invite_user', {
      method: 'POST',
      body: { username },
    })
    useToast().add({ title: 'Zaproszenie wysłane', color: 'success' })
    await refreshInvitations()
  } catch (err) {
    useToast().add({ title: 'Nie udało się zaprosić', description: (err as Error).message, color: 'error' })
  }
}

async function handleRename(name: string) {
  try {
    await useNuxtApp().$auth('/teams/management/rename', {
      method: 'PATCH',
      body: { new_name: name },
    })
    useToast().add({ title: 'Nazwa drużyny zmieniona', color: 'success' })
    await refreshTeam()
  } catch (err) {
    useToast().add({ title: 'Nie udało się zmienić nazwy', description: (err as Error).message, color: 'error' })
  }
}

async function acceptInvitation(teamName: string) {
  try {
    await useNuxtApp().$auth('/teams/invitations/accept_invitation/{team_name}', {
      method: 'POST',
      path: { team_name: teamName },
    })
    useToast().add({ title: 'Dołączono do drużyny', color: 'success' })
    await Promise.all([refreshTeam(), refreshPending()])
  } catch (err) {
    useToast().add({ title: 'Błąd', description: (err as Error).message, color: 'error' })
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
  <div class="panel-page">
    <LazyPanelModalCreateTeam v-model="createTeamModal" hydrate-on-idle />
    <LazyPanelModalJoinExternalTeam v-model="joinTeamModal" hydrate-on-idle />
    <LazyPanelModalConfirmDeleteModal
      v-model="deleteTeamModal"
      url="/teams/management/delete"
      modal-title="Usuwanie drużyny"
      modal-description="Czy na pewno chcesz usunąć drużynę? Ta operacja jest nieodwracalna."
      toast-success-message="Pomyślnie usunięto drużynę"
      :request-body="undefined"
      redirect-to="/panel/team"
      hydrate-on-idle
    />
    <LazyPanelModalConfirmDeleteModal
      v-model="leaveTeamModal"
      url="/teams/membership/leave_team"
      modal-title="Opuść drużynę"
      modal-description="Czy na pewno chcesz opuścić drużynę? Ta operacja jest nieodwracalna."
      toast-success-message="Pomyślnie opuszczono drużynę"
      :request-body="undefined"
      redirect-to="/panel/team"
      hydrate-on-idle
    />
    <LazyPanelModalConfirmDeleteModal
      v-model="kickUserModal"
      url="/teams/management/kick_user"
      modal-title="Wyrzucenie użytkownika"
      modal-description="Czy na pewno chcesz wyrzucić użytkownika z drużyny?"
      toast-success-message="Pomyślnie wyrzucono użytkownika"
      :request-body="{ username: kickedUser }"
      redirect-to="/panel/team"
      hydrate-on-idle
    />
    <LazyPanelModalConfirmDeleteModal
      v-model="revokeInvitationModal"
      :url="`/teams/management/revoke_invitation/${revokedInvitation}` as any"
      modal-title="Cofnięcie zaproszenia"
      modal-description="Czy na pewno chcesz cofnąć zaproszenie?"
      toast-success-message="Pomyślnie cofnięto zaproszenie"
      :request-body="undefined"
      redirect-to="/panel/team"
      hydrate-on-idle
    />

    <UContainer class="w-full py-8 lg:py-10 space-y-8">
      <header class="panel-hero">
        <NuxtLink
          to="/panel/event"
          class="text-xs uppercase tracking-wider text-muted hover:text-default flex items-center gap-1.5 mb-4 transition-colors"
        >
          <UIcon name="pixelarticons:arrow-left" class="size-4" />
          Wróć do panelu wydarzenia
        </NuxtLink>
        <p class="panel-hero-kicker">
          Hack4Krak 2026
        </p>
        <h1 class="panel-title">
          Twoja drużyna
        </h1>
        <p class="panel-copy">
          Stwórz drużynę, dołącz do istniejącej albo zarządzaj składem.
        </p>
      </header>

      <section
        v-if="(pendingInvites ?? []).length > 0"
        class="panel-card panel-card-accent"
      >
        <header class="panel-card-header">
          <h2 class="panel-card-title">
            Zaproszenia do Ciebie
          </h2>
        </header>
        <ul class="panel-card-body space-y-3">
          <li
            v-for="invite in pendingInvites"
            :key="invite"
            class="panel-subcard grid grid-cols-1 sm:grid-cols-[1fr_auto] items-center gap-3"
          >
            <div>
              <p class="font-medium">
                <span class="text-primary font-bold">{{ invite }}</span> zaprasza Cię do drużyny
              </p>
              <p class="text-xs text-muted mt-1">
                Możesz dołączyć od razu.
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

      <div v-if="!team" class="grid md:grid-cols-2 gap-6">
        <section class="panel-card">
          <header class="panel-card-header">
            <h2 class="panel-card-title">
              Stwórz drużynę
            </h2>
          </header>
          <div class="panel-card-body flex flex-col gap-3">
            <p class="font-bold text-lg">
              Załóż nową drużynę
            </p>
            <p class="text-sm text-muted">
              Zostaniesz liderem i zaprosisz do 5 osób po nazwie użytkownika.
            </p>
            <ElevatedButton type="button" class="mt-3 self-start" @click="createTeamModal = true">
              Stwórz drużynę
            </ElevatedButton>
          </div>
        </section>

        <section class="panel-card">
          <header class="panel-card-header">
            <h2 class="panel-card-title">
              Dołącz do drużyny
            </h2>
          </header>
          <div class="panel-card-body flex flex-col gap-3">
            <p class="font-bold text-lg">
              Masz kod od organizatora?
            </p>
            <p class="text-sm text-muted">
              Wpisz 6-znakowy kod od nauczyciela albo lidera, żeby dołączyć do istniejącego zgłoszenia.
            </p>
            <button
              type="button"
              class="panel-action-link mt-3 self-start cursor-pointer"
              @click="joinTeamModal = true"
            >
              <UIcon name="pixelarticons:login" class="size-4" />
              Dołącz po kodzie
            </button>
          </div>
        </section>
      </div>

      <template v-else>
        <PanelEventTeamRoster
          :team="typedTeam"
          :invited-users="invitedUsersList"
          :is-leader="isLeader"
          @invite="handleInvite"
          @rename="handleRename"
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
                  : 'Stracisz dostęp do panelu drużyny. Możesz wrócić z nowym zaproszeniem.' }}
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
    </UContainer>
  </div>
</template>
