<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing/page'

const props = defineProps<{
  username: string
  teamName?: string | null
  locked?: boolean
}>()

const event = LANDING_CONTENT.event
const qrPayload = computed(() => `hack4krak://2026/check-in/${props.username}`)

function downloadTicket() {
  const content = [
    'Hack4Krak 2026 - bilet uczestnika',
    `Uczestnik: @${props.username}`,
    `Druzyna: ${props.teamName ?? '-'}`,
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
  <PanelEventTicketFrame>
    <div class="grid grid-cols-1 overflow-hidden lg:grid-cols-[minmax(0,1fr)_18rem]">
      <section class="min-w-0 p-5 sm:p-6 lg:p-7">
        <header>
          <p class="text-xs font-bold uppercase tracking-[0.24em] text-primary">
            Bilet uczestnika
          </p>
          <p class="mt-3 truncate font-pixelify text-5xl leading-none text-default sm:text-6xl lg:text-7xl">
            @{{ username }}
          </p>
        </header>

        <dl class="mt-6 grid grid-cols-1 border-2 border-surface-muted bg-surface-muted/15 sm:grid-cols-2 xl:grid-cols-3">
          <div class="min-w-0 border-b-2 border-surface-muted p-4 sm:border-r-2 xl:border-b-0">
            <dt class="text-[10px] font-bold uppercase tracking-widest text-muted">
              Drużyna
            </dt>
            <dd class="mt-1 truncate font-bold leading-snug text-primary">
              {{ teamName ?? '—' }}
            </dd>
          </div>

          <div class="min-w-0 border-b-2 border-surface-muted p-4 sm:border-r-0 xl:border-b-0 xl:border-r-2">
            <dt class="text-[10px] font-bold uppercase tracking-widest text-muted">
              Termin
            </dt>
            <dd class="mt-1 font-bold leading-snug">
              {{ event.dateDisplay }}
            </dd>
          </div>

          <div class="min-w-0 border-b-0 p-4 sm:col-span-2 sm:border-r-0 xl:col-span-1">
            <dt class="text-[10px] font-bold uppercase tracking-widest text-muted">
              Miejsce
            </dt>
            <dd class="mt-1 font-bold leading-snug">
              {{ event.venue.name }}
            </dd>
            <dd class="mt-1 text-xs font-medium leading-snug text-muted">
              {{ event.venue.address }}
            </dd>
          </div>
        </dl>

        <p class="mt-5 border-t-2 border-dashed border-primary/35 pt-4 text-xs leading-relaxed text-muted">
          Pokaż kod QR organizatorowi przy wejściu. Kod aktywuje się po ukończeniu zgłoszenia i dołączeniu do drużyny.
        </p>
      </section>

      <aside class="flex flex-col items-center justify-between gap-4 border-t-2 border-dashed border-primary/60 p-8 sm:p-6 lg:border-l-2 lg:border-t-0 m-3">
        <div class="w-full text-center">
          <p class="text-[10px] font-bold uppercase tracking-widest text-muted">
            Kod potwierdzający
          </p>
        </div>

        <div class="relative size-44 border-2 border-primary bg-default p-2 shadow-[5px_5px_0_rgb(0_0_0/0.45)]">
          <NuxtQrcode
            v-if="!locked"
            :value="qrPayload"
            :size="156"
            color="#fafafa"
            background="#171717"
          />
          <div
            v-else
            class="absolute inset-0 flex flex-col items-center justify-center gap-3 bg-default text-center"
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
          class="flex w-full cursor-pointer items-center justify-center gap-2 border-2 border-primary/75 px-3 py-2 text-sm font-bold text-primary transition-colors hover:bg-primary hover:text-default disabled:cursor-not-allowed disabled:border-surface-muted disabled:text-muted disabled:opacity-40"
          @click="downloadTicket"
        >
          <UIcon name="pixelarticons:device-phone" class="size-5" />
          Zapisz na telefon
        </button>
      </aside>
    </div>
  </PanelEventTicketFrame>
</template>
