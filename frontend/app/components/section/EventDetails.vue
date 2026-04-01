<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing-page.json'

const { event } = LANDING_CONTENT

const schedule = [
  { time: 'Sobota, 18 kwietnia', items: [
    { time: '08:00', label: 'Rejestracja uczestników' },
    { time: '09:00', label: 'Śniadanie' },
    { time: '09:30', label: 'Rozpoczęcie konkursu' },
    { time: '14:00', label: 'Przerwa obiadowa' },
    { time: '20:00', label: 'Przerwa na kolację' },
  ] },
  { time: 'Niedziela, 19 kwietnia', items: [
    { time: '09:00', label: 'Śniadanie' },
    { time: '14:00', label: 'Obiad' },
    { time: '16:00', label: 'Zakończenie konkursu' },
  ] },
]

// OSM embed URL centred on AGH Wydział Zarządzania
const osmEmbedUrl = `https://www.openstreetmap.org/export/embed.html?bbox=${event.venue.lon - 0.008},${event.venue.lat - 0.005},${event.venue.lon + 0.008},${event.venue.lat + 0.005}&layer=mapnik&marker=${event.venue.lat},${event.venue.lon}`
</script>

<template>
  <section class="w-full py-12 lg:py-20">
    <div class="text-center mb-10 lg:mb-14">
      <p class="text-xs font-bold tracking-[0.25em] uppercase text-muted mb-3">
        Gdzie i kiedy
      </p>
      <h2 class="font-pixelify text-3xl lg:text-5xl text-default">
        Lokalizacja i harmonogram
      </h2>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 lg:gap-8 items-start">
      <!-- Left: map + address -->
      <div class="flex flex-col gap-4">
        <!-- OSM embed -->
        <div class="relative w-full overflow-hidden border-2 border-surface-muted" style="aspect-ratio: 16/9">
          <iframe
            :src="osmEmbedUrl"
            class="absolute inset-0 w-full h-full"
            frameborder="0"
            scrolling="no"
            marginheight="0"
            marginwidth="0"
            title="Lokalizacja wydarzenia — AGH Kraków"
            loading="lazy"
          />
        </div>

        <!-- Address card -->
        <div class="border-2 border-surface-muted p-5 flex flex-col gap-3">
          <div class="flex items-start gap-3">
            <UIcon name="pixelarticons:map-pin" class="icon-md text-primary flex-shrink-0 mt-0.5" />
            <div>
              <p class="font-bold text-default text-base">
                {{ event.venue.name }}
              </p>
              <p class="text-sm text-muted">
                {{ event.venue.building }}
              </p>
              <p class="text-sm text-muted mt-1">
                {{ event.venue.address }}
              </p>
            </div>
          </div>
          <USeparator />
          <a
            :href="event.venue.mapsUrl"
            target="_blank"
            rel="noopener noreferrer"
            class="flex items-center gap-2 text-sm text-primary hover:underline w-fit"
          >
            <UIcon name="pixelarticons:external-link" class="icon-sm" />
            Otwórz w OpenStreetMap
          </a>
        </div>
      </div>

      <!-- Right: schedule -->
      <div class="flex flex-col gap-6">
        <div
          v-for="(day, di) in schedule"
          :key="di"
          class="border-2 border-surface-muted"
        >
          <div class="px-5 py-3 border-b-2 border-surface-muted bg-surface-muted/30">
            <p class="font-bold text-sm uppercase tracking-wider text-primary">
              {{ day.time }}
            </p>
          </div>
          <ul class="divide-y-2 divide-surface-muted">
            <li
              v-for="(item, ii) in day.items"
              :key="ii"
              class="flex items-center gap-4 px-5 py-3"
            >
              <span class="font-pixelify text-primary text-base font-bold w-12 flex-shrink-0 tabular-nums">
                {{ item.time }}
              </span>
              <span class="text-sm text-default">{{ item.label }}</span>
            </li>
          </ul>
        </div>

        <p class="text-xs text-muted text-center lg:text-left">
          * Harmonogram może ulec drobnym zmianom — śledź nasze social media po aktualności.
        </p>
      </div>
    </div>
  </section>
</template>
