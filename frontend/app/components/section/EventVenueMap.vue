<script setup lang="ts">
interface Venue {
  name: string
  building: string
  address: string
  city: string
  mapsUrl: string
  lat: number
  lon: number
}

const props = defineProps<{
  venue: Venue
}>()

const venueQuery = encodeURIComponent(`${props.venue.name}, ${props.venue.address}`)

const mapLinks = [
  {
    label: 'Google Maps',
    href: `https://www.google.com/maps/search/?api=1&query=${props.venue.lat},${props.venue.lon}`,
  },
  {
    label: 'Apple Maps',
    href: `https://maps.apple.com/?ll=${props.venue.lat},${props.venue.lon}&q=${venueQuery}`,
  },
  {
    label: 'OpenStreetMap',
    href: props.venue.mapsUrl,
  },
]

const osmEmbedUrl = `https://www.openstreetmap.org/export/embed.html?bbox=${props.venue.lon - 0.008},${props.venue.lat - 0.005},${props.venue.lon + 0.008},${props.venue.lat + 0.005}&layer=mapnik&marker=${props.venue.lat},${props.venue.lon}`
</script>

<template>
  <div class="flex h-full flex-col gap-4">
    <div class="relative w-full overflow-hidden border-2 border-surface-muted aspect-[16/9] lg:min-h-[320px] lg:flex-1 lg:aspect-auto">
      <iframe
        :src="osmEmbedUrl"
        class="absolute inset-0 w-full h-full"
        frameborder="0"
        scrolling="no"
        marginheight="0"
        marginwidth="0"
        :title="`Lokalizacja wydarzenia — ${venue.city}`"
        loading="lazy"
      />
    </div>

    <div class="border-2 border-surface-muted p-5 flex flex-col gap-3">
      <div class="flex items-start gap-3">
        <UIcon name="pixelarticons:map-pin" class="icon-md text-primary flex-shrink-0 mt-0.5" />
        <div>
          <p class="font-bold text-default text-base">
            {{ venue.name }}
          </p>
          <p class="text-sm text-muted">
            {{ venue.building }}
          </p>
          <p class="text-sm text-muted mt-1">
            {{ venue.address }}
          </p>
        </div>
      </div>
      <USeparator />

      <div class="flex flex-wrap gap-2">
        <a
          v-for="link in mapLinks"
          :key="link.label"
          :href="link.href"
          target="_blank"
          rel="noopener noreferrer"
          class="inline-flex items-center gap-2 border border-surface-muted px-3 py-2 text-sm text-primary transition-colors hover:border-primary/60 hover:bg-primary/5"
        >
          <UIcon name="pixelarticons:external-link" class="icon-sm" />
          {{ link.label }}
        </a>
      </div>
    </div>
  </div>
</template>
