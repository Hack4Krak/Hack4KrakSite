<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing/page'

const props = defineProps<{
  username: string
  teamName?: string | null
  ticketNumber?: string
  locked?: boolean
}>()

const event = LANDING_CONTENT.event
const ticketDisplay = computed(() => props.ticketNumber ?? '0042')
const qrPayload = computed(() => `hack4krak://2026/check-in/${props.username}/${ticketDisplay.value}`)

function downloadTicket() {
  const content = [
    'Hack4Krak 2026 - bilet uczestnika',
    `Uczestnik: @${props.username}`,
    `Druzyna: ${props.teamName ?? '-'}`,
    `Numer: ${ticketDisplay.value}`,
    `Termin: ${event.dateDisplay}`,
    `Miejsce: ${event.venue.name}`,
    `Adres: ${event.venue.address}`,
    `QR: ${qrPayload.value}`,
  ].join('\n')

  const url = URL.createObjectURL(new Blob([content], { type: 'text/plain;charset=utf-8' }))
  const link = document.createElement('a')
  link.href = url
  link.download = `hack4krak-bilet-${props.username}.txt`
  link.click()
  URL.revokeObjectURL(url)
}
</script>

<template>
  <article class="panel-card">
    <header class="panel-card-header border-b-2 border-surface-muted bg-surface-muted/20">
      <h2 class="panel-card-title">
        Bilet uczestnika
      </h2>
      <span class="text-xs text-muted tabular-nums">
        #{{ ticketDisplay }}
      </span>
    </header>

    <div class="panel-card-body grid grid-cols-1 lg:grid-cols-[1fr_300px] gap-5 lg:gap-6">
      <div class="space-y-5">
        <div>
          <p class="text-xs uppercase tracking-wider text-muted">
            Uczestnik
          </p>
          <p class="font-pixelify text-2xl lg:text-3xl mt-1 leading-none">
            @{{ username }}
          </p>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
          <div class="panel-subcard">
            <p class="text-xs uppercase tracking-wider text-muted">
              Drużyna
            </p>
            <p class="font-medium mt-1 text-primary truncate">
              {{ teamName ?? '—' }}
            </p>
          </div>
          <div class="panel-subcard">
            <p class="text-xs uppercase tracking-wider text-muted">
              Termin
            </p>
            <p class="font-medium mt-1">
              {{ event.dateDisplay }}
            </p>
          </div>
          <div class="panel-subcard sm:col-span-2">
            <p class="text-xs uppercase tracking-wider text-muted">
              Miejsce
            </p>
            <p class="font-medium mt-1">
              {{ event.venue.name }}
            </p>
            <p class="text-xs text-muted mt-0.5">
              {{ event.venue.address }}
            </p>
          </div>
        </div>

        <p class="text-xs text-muted pt-4 border-t border-surface-muted leading-relaxed">
          Pokaż kod QR organizatorowi przy wejściu. Kod aktywuje się po ukończeniu zgłoszenia i dołączeniu do drużyny.
        </p>
      </div>

      <div class="panel-subcard flex flex-col items-center justify-center gap-4">
        <div class="relative size-52 bg-default p-3 border-2 border-surface-muted shadow-[8px_8px_0_rgb(38_38_38/0.7)]">
          <NuxtQrcode
            v-if="!locked"
            :value="qrPayload"
            :size="184"
            color="#fafafa"
            background="#171717"
          />
          <div
            v-else
            class="absolute inset-0 flex flex-col items-center justify-center text-center bg-default gap-3"
          >
            <UIcon name="pixelarticons:lock" class="size-10 text-muted/60" />
            <p class="text-xs text-muted">
              QR pojawi się<br>po rejestracji
            </p>
          </div>
        </div>

        <button
          type="button"
          :disabled="locked"
          class="w-full px-3 py-2 border-2 border-surface-muted hover:border-primary disabled:opacity-30 disabled:cursor-not-allowed text-sm text-default cursor-pointer transition-colors flex items-center justify-center gap-2"
          @click="downloadTicket"
        >
          <UIcon name="pixelarticons:download" class="size-4" />
          Zapisz na telefon
        </button>
      </div>
    </div>
  </article>
</template>
