<script setup lang="ts">
import type { FormSubmitEvent } from '@nuxt/ui'
import * as party from 'party-js'

const { showHeading = true } = defineProps<{
  showHeading?: boolean
}>()

const emit = defineEmits<{
  success: []
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

  burstPoints.value = {
    id: Date.now(),
    value: earned,
    task: taskName,
  }

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

  emit('success')
}

function onError() {
  triggerShake()
}

onBeforeUnmount(() => {
  if (burstTimer)
    clearTimeout(burstTimer)
})
</script>