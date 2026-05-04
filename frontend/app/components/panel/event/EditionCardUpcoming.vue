<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing/page'

const event = LANDING_CONTENT.event

const { isRegistered } = await useEventRegistration()
</script>

<template>
  <article class="border-2 border-surface-muted bg-default p-6 lg:p-7">
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
          <template v-if="!isRegistered">
            <ElevatedButton to="/panel/event/register">
              Zarejestruj się
            </ElevatedButton>
            <NuxtLink
              to="/panel/event"
              class="inline-flex items-center gap-2 px-5 py-2.5 border-2 border-surface-muted hover:border-primary text-sm font-bold transition-colors"
            >
              <UIcon name="pixelarticons:notes" class="size-4" />
              Szczegóły
            </NuxtLink>
          </template>
          <template v-else>
            <ElevatedButton to="/panel/event">
              Otwórz panel wydarzenia
            </ElevatedButton>
          </template>
        </div>
      </div>

      <div class="flex flex-col justify-center gap-3 border border-surface-muted/60 bg-surface-muted/20 px-6 py-5 lg:min-w-[19rem]">
        <p class="text-[10px] uppercase tracking-widest text-muted text-center lg:text-left">
          Do startu
        </p>
        <EventCountdown size="sm" />
      </div>
    </div>
  </article>
</template>
