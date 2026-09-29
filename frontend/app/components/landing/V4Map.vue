<script setup lang="ts">
import type { V4CountryCode } from '~~/content/landing/countries'
import { V4_COUNTRIES } from '~~/content/landing/countries'
import { V4_MAP } from '~~/content/landing/v4-map'

const BORDER_FILL = 'color-mix(in oklab, var(--ui-text) 30%, transparent)'

// Poland is always coloured in. The others follow while the map scrolls up, once its middle (where the V4 is drawn)
// passes these points of the viewport, so they colour in about three quarters of the way down the screen
const REVEAL_AT: Partial<Record<V4CountryCode, number>> = {
  cz: 0.88,
  sk: 0.76,
  hu: 0.64,
}

const countries = V4_COUNTRIES.map(({ code }) => ({
  code,
  outline: V4_MAP.layers.find(layer => layer.key === `v4-border:${code}`)!,
  flag: V4_MAP.layers
    .filter(layer => layer.key.startsWith(`flag:${code}:`))
    .map(layer => ({ ...layer, fill: layer.key.slice(`flag:${code}:`.length) })),
}))

const borders = V4_MAP.layers.filter(layer => layer.key === 'border' || layer.key === 'v4-border')

// The pin's tip points at the middle of Kraków's cell
const pin = {
  left: `${((V4_MAP.krakow.x + 0.5) / V4_MAP.width) * 100}%`,
  top: `${((V4_MAP.krakow.y + 0.5) / V4_MAP.height) * 100}%`,
}

const root = useTemplateRef('root')
const { top, height } = useElementBounding(root)
const { height: viewportHeight } = useWindowSize()

// Everything is coloured in on the server, so the map is complete without JavaScript
const mounted = useMounted()

function revealed(code: V4CountryCode) {
  const at = REVEAL_AT[code]
  return !mounted.value || at === undefined || top.value + height.value / 2 <= viewportHeight.value * at
}
</script>

<template>
  <figure ref="root">
    <!-- The pin is positioned against this box, which is exactly the map's size even when the figure gets stretched by the grid -->
    <div class="relative">
      <svg
        :viewBox="`0 0 ${V4_MAP.width} ${V4_MAP.height}`"
        shape-rendering="crispEdges"
        class="map block w-full"
        role="img"
        aria-label="Mapa Europy Środkowej: Polska, Czechy, Słowacja i Węgry wypełnione flagami, z pinezką w Krakowie"
      >
        <path v-for="layer in borders" :key="layer.key" :d="layer.d" :fill="BORDER_FILL" />
        <g v-for="country in countries" :key="country.code" class="country" :class="{ revealed: revealed(country.code) }">
          <path v-for="layer in country.flag" :key="layer.key" :d="layer.d" :fill="layer.fill" />
          <path :d="country.outline.d" fill="var(--ui-primary)" />
        </g>
      </svg>

      <div class="absolute -translate-x-1/2 -translate-y-full" :style="pin">
        <svg viewBox="0 0 9 12" shape-rendering="crispEdges" class="block w-6 lg:w-8" aria-hidden="true">
          <path fill="var(--color-ink)" d="M2 0h5v1h1v1h1v5H8v2H7v1H6v1H5v1H4v-1H3v-1H2V9H1V7H0V2h1V1h1z" />
          <path fill="var(--ui-primary)" d="M2 1h5v1h1v5H7v2H6v1H5v1H4v-1H3V9H2V7H1V2h1z" />
          <path fill="var(--color-ink)" d="M3 3h3v3H3z" />
        </svg>
      </div>
    </div>
  </figure>
</template>

<style scoped>
/* Fade the rest of Europe out towards the edges, so the V4 stays in focus */
.map {
  mask-image: radial-gradient(ellipse 50% 52% at 50% 46%, black 55%, transparent 100%);
}

/* Until it is revealed a country is only its grey outline, like the rest of the map */
.country {
  opacity: 0;
}

.country.revealed {
  opacity: 1;
}

@media (prefers-reduced-motion: no-preference) {
  .country {
    transition: opacity 0.4s steps(4);
  }
}
</style>
