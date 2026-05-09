<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'
import { EVENT_BRING_ITEMS } from '~~/content/event-bring-items'

definePageMeta({
  layout: 'panel',
  middleware: ['event-registration-guard'],
})

useSeoMeta({
  title: 'Moje zgłoszenie',
})

const { data: user } = await useAuth('/account/', {
  redirect: 'error',
  onResponseError: undefined,
})
const { data: identification } = await useAuth('/account/identification')
const { data: allParticipantTags } = await useApi('/event/participant-tags', { onResponseError: undefined })
const { $auth } = useNuxtApp()
const { data: team, error: teamError, refresh: refreshTeam } = await useAuth('/teams/membership/my_team', {
  onResponseError: undefined,
})
const { data: pendingInvites, refresh: refreshPendingInvites } = await useAuth('/teams/invitations/', {
  onResponseError: undefined,
})
const { refreshParticipation } = useEventRegistration()

const typedTeam = computed(() => {
  if (teamError.value || !team.value) {
    return null
  }

  return team.value as NonNullable<ApiResponse<'my_team'>>
})
const hasTeam = computed(() => typedTeam.value !== null)
const pendingInvitesList = computed(() => Array.isArray(pendingInvites.value) ? pendingInvites.value : [])

const manageTeamModal = ref(false)
const cancelParticipationModal = ref(false)

async function acceptInvitation(teamName: string) {
  try {
    await $auth('/teams/invitations/accept_invitation/{team_name}', {
      method: 'POST',
      path: { team_name: teamName },
    })
    useToast().add({ title: 'Dołączono do drużyny', color: 'success' })
    await Promise.allSettled([
      refreshTeam(),
      refreshPendingInvites(),
    ])
  } catch (error) {
    useToast().add({ title: 'Nie udało się dołączyć', description: (error as Error).message, color: 'error' })
  }
}

async function rejectInvitation(_teamName: string) {
  useToast().add({ title: 'Zaproszenie odrzucone', color: 'neutral' })
}

async function handleCancelParticipation() {
  await refreshParticipation()
  await refreshNuxtData(['event-participation', 'event-registration'])
}
</script>

<template>
  <LazyPanelModalConfirmDeleteModal
    v-model="cancelParticipationModal"
    url="/event/participate"
    modal-title="Wypisz się z wydarzenia"
    modal-description="Czy na pewno chcesz wypisać się z wydarzenia? Twoje dane zgłoszeniowe zostaną usunięte."
    toast-success-message="Pomyślnie wypisano z wydarzenia"
    :request-body="undefined"
    redirect-to="/account/events"
    confirm-label="Wypisz się"
    hydrate-on-idle
    @success="handleCancelParticipation"
  />
  <UModal
    v-model:open="manageTeamModal"
    title="Zarządzaj drużyną"
    description="Tu zmienisz nazwę, zaprosisz osoby i użyjesz opcji administracyjnych."
    :ui="{ content: 'max-w-4xl', body: 'p-0 sm:p-0' }"
  >
    <template #body>
      <div class="max-h-[80vh] overflow-y-auto p-4 sm:p-6 bg-default">
        <PanelTeamManagementWorkspace
          v-if="manageTeamModal"
          redirect-to="/panel/event"
        />
      </div>
    </template>
  </UModal>

  <UContainer class="w-full py-8 lg:py-10 space-y-8">
    <header class="grid grid-cols-1 lg:grid-cols-[1fr_auto] gap-6 items-end pb-6 border-b-2 border-surface-muted">
      <div>
        <p class="mb-2 text-xs font-bold uppercase tracking-wider text-primary">
          Hack4Krak 2026
        </p>
        <h1 class="font-pixelify text-4xl leading-none lg:text-5xl">
          Moje zgłoszenie
        </h1>
        <p class="mt-3 max-w-2xl text-sm leading-relaxed text-muted">
          Tu znajdziesz kod QR, informacje organizacyjne, drużynę i listę rzeczy do zabrania.
        </p>
      </div>

      <EventCountdown size="sm" class="lg:min-w-72" />
    </header>

    <PanelCard
      v-if="pendingInvitesList.length > 0"
      accent
      body
    >
      <PanelSectionTitle class="mb-4">
        Zaproszenia do drużyny
      </PanelSectionTitle>
      <ul class="space-y-3">
        <li
          v-for="invite in pendingInvitesList"
          :key="invite"
          class="grid grid-cols-1 sm:grid-cols-[1fr_auto_auto] items-center gap-3 py-2"
        >
          <div class="min-w-0">
            <p class="font-medium">
              <span class="text-primary font-bold">{{ invite }}</span> zaprasza Cię do drużyny
            </p>
            <p class="text-xs text-muted mt-1">
              Możesz dołączyć od razu albo odrzucić zaproszenie.
            </p>
          </div>
          <PanelActionButton
            filled
            @click="acceptInvitation(invite)"
          >
            Dołącz
          </PanelActionButton>
          <PanelActionButton
            tone="danger"
            @click="rejectInvitation(invite)"
          >
            Odrzuć
          </PanelActionButton>
        </li>
      </ul>
    </PanelCard>

    <PanelEventBoardingPass
      :username="user?.username ?? 'guest'"
      :team-name="typedTeam?.team_name"
      :verification-code="identification?.identification_code"
      :locked="!identification?.identification_code"
    />

    <div class="grid grid-cols-1 xl:grid-cols-[minmax(0,1fr)_360px] gap-6 items-start">
      <div class="space-y-6 min-w-0">
        <PanelEventWhatToBringList :items="EVENT_BRING_ITEMS" />

        <PanelEventSocialUpdates />
      </div>

      <aside class="space-y-6 xl:sticky xl:top-28">
        <PanelEventTeamRoster
          v-if="hasTeam"
          variant="compact"
          :team="typedTeam"
        >
          <template #footer>
            <PanelActionButton
              tone="neutral"
              @click="manageTeamModal = true"
            >
              <UIcon name="pixelarticons:sliders-2" class="size-4" />
              Zarządzaj drużyną
            </PanelActionButton>
          </template>
        </PanelEventTeamRoster>

        <PanelTileParticipantTags
          v-if="allParticipantTags?.length"
          :all-tags="allParticipantTags"
          :applied-tags="identification?.applied_tags ?? []"
        />

        <PanelCard v-if="!hasTeam" body dashed>
          <PanelSectionTitle class="mb-4">
            Drużyna
          </PanelSectionTitle>
          <p class="font-bold text-lg">
            Grasz solo
          </p>
          <p class="text-sm text-muted mt-2">
            Rejestracja jest aktywna. Drużynę możesz stworzyć tutaj albo dobrać skład na Discordzie Hack4Krak.
          </p>
          <PanelActionButton
            tone="neutral"
            class="mt-5"
            @click="manageTeamModal = true"
          >
            <UIcon name="pixelarticons:sliders-2" class="size-4" />
            Otwórz zarządzanie drużyną
          </PanelActionButton>
        </PanelCard>

        <PanelCard body>
          <PanelSectionTitle class="mb-4">
            Zgłoszenie
          </PanelSectionTitle>
          <p class="font-bold text-lg">
            Chcesz zrezygnować z udziału?
          </p>
          <p class="text-sm text-muted mt-2">
            Jeśli zmienisz zdanie, możesz zapisać się ponownie, dopóki rejestracja jest jeszcze otwarta.
          </p>
          <PanelActionButton
            tone="danger"
            class="mt-5"
            @click="cancelParticipationModal = true"
          >
            Wypisz się z wydarzenia
          </PanelActionButton>
        </PanelCard>
      </aside>
    </div>
  </UContainer>
</template>
