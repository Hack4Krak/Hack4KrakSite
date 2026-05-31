<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing/page'

defineProps<{
  isEventLive: boolean
}>()

const event = LANDING_CONTENT.event

const { data: registrationInformation } = await useApi('/event/registration')
const { isRegistered } = useEventRegistration()

const registrationStarted = computed(() => {
  const startDate = registrationInformation.value?.start_date
  return startDate ? Date.now() >= new Date(startDate).getTime() : false
})
const canOpenSubmission = computed(() => isRegistered.value)
</script>

<template>
  <PanelCard class="p-6 lg:p-7">
    <div class="grid grid-cols-1 lg:grid-cols-[1fr_auto] gap-6 lg:gap-10 items-center">
      <div class="space-y-5">
        <div>
          <p class="text-xs uppercase tracking-wider text-primary mb-3 font-bold">
            Nadchodzące wydarzenie
          </p>
          <h3 class="font-pixelify text-3xl leading-none">
            <span class="text-default">Hack4Krak </span>
            <span class="text-primary">2026</span>
          </h3>
        </div>

        <ul class="space-y-3 text-sm">
          <li class="flex items-start gap-3">
            <UIcon name="pixelarticons:calendar" class="size-5 text-primary mt-0.5 shrink-0" />
            <span class="text-primary font-bold">{{ event.dateDisplay }}</span>
          </li>
          <li class="flex items-start gap-3">
            <UIcon name="pixelarticons:map-pin" class="size-5 text-primary mt-0.5 shrink-0" />
            <span>
              <span class="block font-medium">{{ event.venue.name }}</span>
              <span class="block text-muted text-xs mt-0.5">{{ event.venue.address }}</span>
            </span>
          </li>
          <li class="flex items-start gap-3">
            <UIcon name="pixelarticons:users" class="size-5 text-primary mt-0.5 shrink-0" />
            <span class="text-muted">
              Drużyny do <span class="text-default font-bold">5 osób</span> · uczniowie szkół ponadpodstawowych
            </span>
          </li>
        </ul>

        <div class="flex flex-wrap gap-5 items-center pt-2">
          <template v-if="!canOpenSubmission">
            <ElevatedButton
              to="/panel/event/register"
              :disabled="!registrationStarted"
            >
              Zarejestruj się na wydarzenie
            </ElevatedButton>
          </template>
          <template v-else>
            <ElevatedButton to="/panel/event">
              Otwórz swoje zgłoszenie
            </ElevatedButton>
            <PanelActionButton
              v-if="isEventLive"
              to="/panel"
              tone="neutral"
              icon="pixelarticons:dashboard"
            >
              Otwórz panel CTFu
            </PanelActionButton>
          </template>
        </div>
      </div>

      <div class="lg:pl-10 lg:border-l-2 lg:border-surface-muted lg:min-w-72">
        <PanelDataLabel class="mb-3 block">
          Do startu
        </PanelDataLabel>
        <EventCountdown size="sm" />
      </div>
    </div>
  </PanelCard>
</template>
