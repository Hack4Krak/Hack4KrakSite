<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'
import LANDING_CONTENT from '~~/content/landing/page'

useSeoMeta({
  title: 'Hack4Krak 2026 - panel wydarzenia',
})

const event = LANDING_CONTENT.event

const { data: eventInformation } = await useApi('/event/info', {
  key: 'panel-event-header-info',
  onResponseError: undefined,
})
const [eventStart] = await useEventStartAndEnd(eventInformation)
const eventStarted = computed(() => eventStart ? Date.now() >= eventStart.getTime() : false)

const { data: user } = await useAuth('/account/')
const { data: existingTeam, refresh: refreshTeam } = await useAuth('/teams/membership/my_team', {
  onResponseError: undefined,
})
const { data: invitedUsers, refresh: refreshInvitations } = await useAuth(
  '/teams/management/invited_users',
  { onResponseError: undefined },
)
const { data: teamManagement } = await useAuth('/teams/management/', {
  onResponseError: undefined,
})
const { data: pendingInvites, refresh: refreshPending } = await useAuth('/teams/invitations/', {
  onResponseError: undefined,
})

const { isRegistered, draft } = await useEventRegistration()

const isLeader = computed(() => Boolean(teamManagement.value))
const hasTeam = computed(() => Boolean(existingTeam.value))
const typedTeam = computed(() => existingTeam.value as ApiResponse<'my_team'> | null)
const invitedUsersList = computed(() => Array.isArray(invitedUsers.value) ? invitedUsers.value : [])

const consentsDone = computed(() =>
  isRegistered.value
  && draft.value.agreements.rules
  && draft.value.agreements.rodo
  && draft.value.agreements.parental,
)

const checklistItems = computed(() => [
  {
    id: 'account',
    title: 'Konto aktywne',
    meta: user.value?.email ?? undefined,
    done: true,
  },
  {
    id: 'consents',
    title: 'Zgody na udział',
    meta: 'regulamin, RODO i zgoda opiekuna',
    done: consentsDone.value,
    meAction: !consentsDone.value
      ? { label: 'Uzupełnij', to: '/panel/event/register' }
      : undefined,
  },
  {
    id: 'team',
    title: hasTeam.value
      ? `Drużyna ${typedTeam.value?.team_name}`
      : 'Stwórz drużynę albo dołącz do istniejącej',
    meta: hasTeam.value ? `${typedTeam.value?.members?.length ?? 0}/5 osób` : 'wymagane do udziału',
    done: hasTeam.value,
    meAction: !hasTeam.value
      ? { label: 'Drużyna', to: '/panel/team' }
      : undefined,
  },
  {
    id: 'arrival',
    title: 'Rejestracja na miejscu',
    meta: 'pierwszy dzień wydarzenia',
    done: false,
  },
  ...(draft.value.agreements.parental
    ? [{
        id: 'parental',
        title: 'Przynieś podpisaną zgodę rodzica/opiekuna',
        meta: 'jeśli masz mniej niż 18 lat',
        done: false,
        meAction: { label: 'Regulamin', to: '/docs/rules' },
      }]
    : []),
])

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

async function handleInvite(username: string) {
  try {
    await useNuxtApp().$auth('/teams/management/invite_user', {
      method: 'POST',
      body: { username },
    })
    useToast().add({ title: 'Zaproszenie wysłane', color: 'success' })
    await refreshInvitations()
  } catch (error) {
    useToast().add({ title: 'Błąd', description: (error as Error).message, color: 'error' })
  }
}

async function handleRevoke(username: string) {
  try {
    await useNuxtApp().$auth('/teams/management/revoke_invitation/{username}', {
      method: 'DELETE',
      path: { username },
    })
    useToast().add({ title: 'Zaproszenie cofnięte', color: 'success' })
    await refreshInvitations()
  } catch (error) {
    useToast().add({ title: 'Błąd', description: (error as Error).message, color: 'error' })
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
  } catch (error) {
    useToast().add({ title: 'Nie udało się zmienić nazwy', description: (error as Error).message, color: 'error' })
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
  } catch (error) {
    useToast().add({ title: 'Błąd', description: (error as Error).message, color: 'error' })
  }
}

async function rejectInvitation(_teamName: string) {
  useToast().add({ title: 'Zaproszenie odrzucone', color: 'neutral' })
  await refreshPending()
}

const ticketNumber = computed(() => {
  const seed = (user.value?.username ?? 'guest').split('').reduce((acc, c) => acc + c.charCodeAt(0), 0)
  return String(((seed * 37) % 9000) + 1000)
})
</script>

<template>
  <div class="panel-page">
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

        <div class="border-2 border-surface-muted px-4 py-3 min-w-64">
          <Timer v-if="eventStart && !eventStarted" :target="eventStart">
            <template #default="{ allUnits, padded }">
              <div class="grid grid-cols-4 gap-3 text-center">
                <div v-for="unit in allUnits" :key="unit.key">
                  <span class="font-pixelify text-2xl text-primary block leading-none tabular-nums">
                    {{ padded(unit.value) }}
                  </span>
                  <span class="text-[10px] uppercase tracking-widest text-muted block mt-1">
                    {{ unit.shortLabel }}
                  </span>
                </div>
              </div>
            </template>
          </Timer>
          <div v-else-if="eventStarted" class="font-pixelify text-xl text-primary text-center">
            W trakcie!
          </div>
          <div v-else class="font-pixelify text-xl text-primary text-center">
            {{ event.dateDisplay }}
          </div>
        </div>
      </header>

      <section
        v-if="(pendingInvites ?? []).length > 0"
        class="panel-card panel-card-accent"
      >
        <header class="panel-card-header">
          <h2 class="panel-card-title">
            Zaproszenia do drużyny
          </h2>
        </header>
        <ul class="panel-card-body space-y-3">
          <li
            v-for="invite in pendingInvites"
            :key="invite"
            class="panel-subcard grid grid-cols-1 sm:grid-cols-[1fr_auto_auto] items-center gap-3"
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

      <section
        v-if="!isRegistered"
        class="panel-card panel-card-accent panel-card-body grid grid-cols-1 md:grid-cols-[1fr_auto] items-center gap-5"
      >
        <div>
          <h3 class="font-pixelify text-2xl">
            Dokończ zgłoszenie
          </h3>
          <p class="text-sm text-muted mt-2 max-w-md">
            Po rejestracji zobaczysz aktywny kod QR i pełny status udziału.
          </p>
        </div>
        <ElevatedButton to="/panel/event/register">
          Przejdź do rejestracji
        </ElevatedButton>
      </section>

      <PanelEventBoardingPass
        :username="user?.username ?? 'guest'"
        :team-name="typedTeam?.team_name"
        :ticket-number="ticketNumber"
        :locked="!isRegistered || !hasTeam"
      />

      <div class="grid grid-cols-1 xl:grid-cols-[minmax(0,1fr)_430px] gap-6 items-start">
        <div class="space-y-6 min-w-0">
          <article class="panel-card">
            <header class="panel-card-header border-b-2 border-surface-muted bg-surface-muted/20">
              <h2 class="panel-card-title">
                Lokalizacja i termin
              </h2>
            </header>
            <div class="panel-card-body grid grid-cols-1 md:grid-cols-3 gap-3">
              <div class="panel-subcard">
                <p class="text-xs uppercase tracking-wider text-muted">
                  Termin
                </p>
                <p class="font-bold text-primary mt-1">
                  {{ event.dateDisplay }}
                </p>
              </div>
              <div class="panel-subcard md:col-span-2">
                <p class="text-xs uppercase tracking-wider text-muted">
                  Miejsce
                </p>
                <p class="font-bold mt-1">
                  {{ event.venue.name }}
                </p>
                <p class="text-xs text-muted mt-1">
                  {{ event.venue.address }}
                </p>
              </div>
            </div>
          </article>

          <PanelEventBringList :items="bringItems" />
        </div>

        <aside class="space-y-6 xl:sticky xl:top-28">
          <PanelEventTeamRoster
            v-if="hasTeam"
            :team="typedTeam"
            :invited-users="invitedUsersList"
            :is-leader="isLeader"
            @invite="handleInvite"
            @rename="handleRename"
            @revoke-invite="handleRevoke"
          />

          <article v-else class="panel-card border-dashed">
            <header class="panel-card-header border-b-2 border-surface-muted bg-surface-muted/20">
              <h2 class="panel-card-title">
                Drużyna
              </h2>
              <span class="text-xs text-muted">
                wymagana
              </span>
            </header>
            <div class="panel-card-body">
              <p class="font-bold text-lg">
                Nie masz jeszcze drużyny
              </p>
              <p class="text-sm text-muted mt-2">
                Załóż własną i zaproś znajomych po nicku albo dołącz kodem.
              </p>
              <div class="flex flex-col gap-3 mt-5">
                <ElevatedButton to="/panel/team" class="self-start">
                  Stwórz drużynę
                </ElevatedButton>
                <NuxtLink
                  to="/panel/team"
                  class="panel-action-link self-start"
                >
                  <UIcon name="pixelarticons:login" class="size-4" />
                  Mam kod
                </NuxtLink>
              </div>
            </div>
          </article>

          <PanelEventChecklist :items="checklistItems" />
        </aside>
      </div>
    </UContainer>
  </div>
</template>
