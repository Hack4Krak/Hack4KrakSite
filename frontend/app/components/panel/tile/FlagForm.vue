<script setup lang="ts">
import type { FormSubmitEvent } from '@nuxt/ui'
import * as party from 'party-js'

const emit = defineEmits<{
  submitted: []
}>()
const flagPattern = /^hack4KrakCTF\{.*\}$/
const schema = z.object({
  flag: z.string({ error: 'Wpisz flagę' })
    .regex(flagPattern, { error: 'Flaga musi być w formacie "hack4KrakCTF{...}"' }),
})

type Schema = zInfer<typeof schema>

const state = reactive<Partial<Schema>>({
  flag: undefined,
})

const toast = useToast()
const { $auth } = useNuxtApp()

const formRef = ref<HTMLElement | null>(null)
const shaking = ref(false)
const submitting = ref(false)
const burstPoints = ref<{ id: number, value: number, task: string } | null>(null)

let burstTimer: ReturnType<typeof setTimeout> | null = null

function triggerShake() {
  shaking.value = false
  // restart animation cleanly
  requestAnimationFrame(() => {
    shaking.value = true
    setTimeout(() => {
      shaking.value = false
    }, 500)
  })
}

async function onSubmit(event: FormSubmitEvent<Schema>) {
  submitting.value = true
  const response = await $auth('/flag/submit', {
    method: 'POST',
    body: { flag: event.data.flag },
  }).catch(() => undefined)
  submitting.value = false

  if (!response) {
    triggerShake()
    return
  }

  const target = event.target as HTMLElement | undefined
  if (target) {
    party.confetti(target, {
      count: party.variation.range(300, 700),
      spread: 30,
    })
  }

  const earned = response.points
  const taskName = response.task_title

  burstPoints.value = { id: Date.now(), value: earned, task: taskName }
  if (burstTimer)
    clearTimeout(burstTimer)
  burstTimer = setTimeout(() => {
    burstPoints.value = null
  }, 2600)

  toast.add({
    title: `+${earned} punktów`,
    description: getRandomJoke(),
    color: 'success',
    duration: 8000,
    icon: 'pixelarticons:trophy',
  })
  state.flag = undefined
  emit('submitted')
}

function onError() {
  triggerShake()
}

onBeforeUnmount(() => {
  if (burstTimer)
    clearTimeout(burstTimer)
})
</script>

<template>
  <section class="relative overflow-hidden border-2 border-primary bg-default p-5 lg:p-6">
    <div class="mb-4 flex items-center gap-2">
      <UIcon name="pixelarticons:flag" class="size-5 text-primary" />
      <PanelSectionTitle>
        Prześlij flagę
      </PanelSectionTitle>
    </div>

    <UForm
      ref="formRef"
      :schema="schema"
      :state="state"
      class="flex flex-col gap-3 sm:flex-row sm:items-start"
      :class="{ 'shake-x': shaking }"
      @submit="onSubmit"
      @error="onError"
    >
      <UFormField name="flag" class="flex-1">
        <UInput
          v-model="state.flag"
          placeholder="hack4KrakCTF{...}"
          autocomplete="off"
          class="w-full"
          :ui="{ base: 'h-12 font-pixelify text-base tracking-wider' }"
        />
      </UFormField>

      <ElevatedButton type="submit" :disabled="submitting">
        <template #leading>
          <UIcon
            :name="submitting ? 'pixelarticons:loader' : 'pixelarticons:check'"
            class="size-4"
            :class="{ 'animate-spin': submitting }"
          />
        </template>
        {{ submitting ? 'Sprawdzam' : 'Sprawdź flagę' }}
      </ElevatedButton>
    </UForm>

    <Transition name="burst">
      <div
        v-if="burstPoints"
        :key="burstPoints.id"
        class="pointer-events-none absolute inset-0 flex items-center justify-center"
      >
        <div class="rotate-[-3deg] border-2 border-emerald-400 bg-default px-8 py-4 text-center">
          <p class="font-pixelify text-xs uppercase tracking-[0.3em] text-emerald-300">
            Flaga zaliczona
          </p>
          <p class="mt-1 font-pixelify text-5xl text-emerald-300">
            +{{ burstPoints.value }}
          </p>
          <p class="mt-1 truncate text-sm text-muted">
            {{ burstPoints.task }}
          </p>
        </div>
      </div>
    </Transition>
  </section>
</template>

<style scoped>
.shake-x {
  animation: shake-x 0.45s cubic-bezier(0.36, 0.07, 0.19, 0.97) both;
}

@keyframes shake-x {
  10%,
  90% {
    transform: translateX(-2px);
  }
  20%,
  80% {
    transform: translateX(4px);
  }
  30%,
  50%,
  70% {
    transform: translateX(-8px);
  }
  40%,
  60% {
    transform: translateX(8px);
  }
}

.burst-enter-active {
  transition:
    transform 0.35s cubic-bezier(0.2, 0.9, 0.3, 1.4),
    opacity 0.25s ease-out;
}
.burst-leave-active {
  transition:
    transform 0.45s ease-in,
    opacity 0.4s ease-in;
}
.burst-enter-from {
  transform: scale(0.4);
  opacity: 0;
}
.burst-leave-to {
  transform: translateY(-30px) scale(0.95);
  opacity: 0;
}
</style>
