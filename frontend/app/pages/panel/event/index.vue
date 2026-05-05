<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'

definePageMeta({
  middleware: ['event-registration-guard'],
})

useSeoMeta({
  title: 'Panel wydarzenia',
})

const { data: user } = await useAuth('/account/')
const { $auth } = useNuxtApp()
const { data: team, error: teamError } = await useAuth('/teams/membership/my_team', {
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

const manageTeamModal = ref(false)
const cancelParticipationModal = ref(false)
const isLeader = computed(() => typedTeam.value?.members?.some(member => member.is_leader && member.name === user.value?.username) ?? false)

const bringItems = [
  {
    icon: 'pixelarticons:device-laptop',
    title: 'Laptop + ładowarka',
    description: 'Mamy gniazdka pod każdym stanowiskiem · przynieś swój sprzęt.',
    required: true,
  },
  {
    icon: 'pixelarticons:contact',
    title: 'Dokument tożsamości',
    description: 'Legitymacja albo dowód · weryfikujemy przy wejściu.',
    required: true,
  },
  {
    icon: 'pixelarticons:script-text',
    title: 'Zgoda rodzica',
    description: 'Tylko jeśli poniżej 18 lat · pobierz PDF z regulaminu.',
    required: false,
  },
  {
    icon: 'pixelarticons:download',
    title: 'Twój QR kod',
    description: 'Pokaż przy rejestracji · masz go w tym panelu.',
    required: true,
  },
]

async function acceptInvitation(teamName: string) {
  try {
    await $auth('/teams/invitations/accept_invitation/{team_name}', {
      method: 'POST',
      path: { team_name: teamName },
    })
    useToast().add({ title: 'Dołączono do drużyny', color: 'success' })
    await Promise.allSettled([
      refreshPendingInvites(),
      refreshNuxtData(),
    ])
  } catch (error) {
    useToast().add({ title: 'Nie udało się dołączyć', description: (error as Error).message, color: 'error' })
  }
}

async function rejectInvitation(_teamName: string) {
  useToast().add({ title: 'Zaproszenie odrzucone', color: 'neutral' })
}
</script>

<template>
  <div class="panel-page">
    <LazyPanelModalConfirmDeleteModal
      v-model="cancelParticipationModal"
      url="/event/participate"
      modal-title="Wypisz się z wydarzenia"
      modal-description="Czy na pewno chcesz wypisać się z wydarzenia? Twoje dane zgłoszeniowe zostaną usunięte."
      toast-success-message="Pomyślnie wypisano z wydarzenia"
      :request-body="undefined"
      redirect-to="/panel/event"
      hydrate-on-idle
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
          <p class="panel-hero-kicker">
            Hack4Krak 2026
          </p>
          <h1 class="panel-title">
            Panel wydarzenia
          </h1>
          <p class="panel-copy">
            Tu znajdziesz kod QR, informacje organizacyjne, drużynę i listę rzeczy do zabrania.
          </p>
        </div>

        <EventCountdown size="sm" class="lg:min-w-72" />
      </header>

      <section
        v-if="pendingInvitesList.length > 0"
        class="panel-card panel-card-accent panel-card-body"
      >
        <p class="panel-section-title mb-4">
          Zaproszenia do drużyny
        </p>
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
            <button
              type="button"
              class="px-4 py-2 bg-primary text-default text-sm font-bold uppercase tracking-wider hover:bg-secondary cursor-pointer transition-colors"
              @click="acceptInvitation(invite)"
            >
              Dołącz
            </button>
            <button
              type="button"
              class="px-4 py-2 border-2 border-surface-muted hover:border-error hover:text-error text-sm font-medium cursor-pointer transition-colors"
              @click="rejectInvitation(invite)"
            >
              Odrzuć
            </button>
          </li>
        </ul>
      </section>

      <PanelEventBoardingPass
        :username="user?.username ?? 'guest'"
        :team-name="typedTeam?.team_name"
        :locked="!hasTeam"
      />

      <div class="grid grid-cols-1 xl:grid-cols-[minmax(0,1fr)_360px] gap-6 items-start">
        <div class="space-y-6 min-w-0">
          <PanelEventBringList :items="bringItems" />

          <PanelEventDangerZone
            :has-team="hasTeam"
            :is-leader="isLeader"
            :is-registered="true"
            @team-action="manageTeamModal = true"
            @cancel-participation="cancelParticipationModal = true"
          />

          <PanelEventSocialUpdates />
        </div>

        <aside class="space-y-6 xl:sticky xl:top-28">
          <PanelEventTeamRoster
            v-if="hasTeam"
            variant="compact"
            :team="typedTeam"
          >
            <template #footer>
              <button
                type="button"
                class="panel-action-link self-start cursor-pointer"
                @click="manageTeamModal = true"
              >
                <UIcon name="pixelarticons:settings" class="size-4" />
                Zarządzaj drużyną
              </button>
            </template>
          </PanelEventTeamRoster>

          <article v-if="!hasTeam" class="panel-card panel-card-body border-dashed">
            <div class="flex items-baseline justify-between gap-4 mb-4">
              <p class="panel-section-title">
                Drużyna
              </p>
              <span class="text-xs text-muted">
                wymagana
              </span>
            </div>
            <p class="font-bold text-lg">
              Nie masz jeszcze drużyny
            </p>
            <p class="text-sm text-muted mt-2">
              Uzupełnij skład albo utwórz nową drużynę bez opuszczania panelu wydarzenia.
            </p>
            <button
              type="button"
              class="panel-action-link self-start mt-5 cursor-pointer"
              @click="manageTeamModal = true"
            >
              <UIcon name="pixelarticons:settings" class="size-4" />
              Otwórz zarządzanie drużyną
            </button>
          </article>
        </aside>
      </div>
    </UContainer>
  </div>
</template>
