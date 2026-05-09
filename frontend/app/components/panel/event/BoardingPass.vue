<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing/page'

const props = defineProps<{
  username: string
  teamName?: string | null
  verificationCode?: string | null
  locked?: boolean
}>()

const event = LANDING_CONTENT.event
const savingQr = ref(false)
const qrPayload = computed(() => props.verificationCode ?? '')
const qrCode = useQrcode(qrPayload, {
  toBase64: true,
  blackColor: '#fafafa',
  whiteColor: '#171717',
})

async function downloadQr() {
  const url = unref(qrCode)

  if (!url || savingQr.value) {
    return
  }

  savingQr.value = true

  try {
    const link = document.createElement('a')
    link.href = url
    link.download = `hack4krak-qr-${props.username}.png`
    link.click()
  } finally {
    savingQr.value = false
  }
}
</script>

<template>
  <div>
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
              <dt>
                <PanelDataLabel>
                  Drużyna
                </PanelDataLabel>
              </dt>
              <dd class="mt-1 truncate font-bold leading-snug text-primary">
                {{ teamName ?? 'Solo' }}
              </dd>
            </div>

            <div class="min-w-0 border-b-2 border-surface-muted p-4 sm:border-r-0 xl:border-b-0 xl:border-r-2">
              <dt>
                <PanelDataLabel>
                  Termin
                </PanelDataLabel>
              </dt>
              <dd class="mt-1 font-bold leading-snug">
                {{ event.dateDisplay }}
              </dd>
            </div>

            <div class="min-w-0 border-b-0 p-4 sm:col-span-2 sm:border-r-0 xl:col-span-1">
              <dt>
                <PanelDataLabel>
                  Miejsce
                </PanelDataLabel>
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
            Pokaż kod QR organizatorowi przy wejściu. Drużynę możesz uzupełnić później albo dobrać skład na Discordzie.
          </p>
        </section>

        <aside class="m-3 flex flex-col items-center justify-between gap-4 border-t-2 border-dashed border-primary/60 p-8 sm:p-6 lg:border-l-2 lg:border-t-0">
          <div class="w-full text-center">
            <PanelDataLabel>
              Kod potwierdzający
            </PanelDataLabel>
          </div>

          <div class="relative size-44 border-2 border-primary bg-default p-2 shadow-[5px_5px_0_rgb(0_0_0/0.45)]">
            <img
              v-if="!locked && qrPayload"
              :src="qrCode"
              alt="Kod QR uczestnika"
              class="size-full"
            >
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

          <PanelActionButton
            :disabled="locked || savingQr"
            icon="pixelarticons:device-phone"
            class="w-full"
            @click="downloadQr"
          >
            {{ savingQr ? 'Zapisywanie...' : 'Zapisz na telefon' }}
          </PanelActionButton>
        </aside>
      </div>
    </PanelEventTicketFrame>
  </div>
</template>
