<script setup lang="ts" generic="T extends z.ZodObject<any>">
import type { FormSubmitEvent, InferOutput } from '#ui/types'
import { UCheckbox, UInput, UInputNumber } from '#components'
import * as z from 'zod'

const props = defineProps<{ schema: T }>()

const emit = defineEmits<{
  (e: 'submit', data: InferOutput<T>): void
}>()

type Output = InferOutput<T>

const state = reactive({})

const shape = (props.schema as z.ZodObject<any>).shape

function getComponentForZodType(key: string, zodType: any) {
  let component = UInput as any
  let props = {}

  if (zodType instanceof z.ZodEmail) {
    props = { type: 'email' }
  } else if (zodType instanceof z.ZodNumber) {
    component = UInputNumber
  } else if (zodType instanceof z.ZodBoolean) {
    component = UCheckbox
  } else if (zodType instanceof z.ZodDefault) {
    (state as any)[key] = zodType.def.defaultValue
    return getComponentForZodType(key, zodType.unwrap())
  }

  return {
    key,
    label: zodType.description ?? key,
    component,
    props,
  }
}

const fields = Object.entries(shape).map(([key, zodType]) =>
  getComponentForZodType(key, zodType),
)

const isButtonEnabled = computed(() => props.schema.safeParse(state).success)

const loading = ref(false)

async function onSubmit(event: FormSubmitEvent<Output>) {
  event.preventDefault()
  loading.value = true
  try {
    emit('submit', toRaw(event.data))
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <UForm
    :schema="schema"
    :state="state"
    class="space-y-4 text-center"
    @submit="onSubmit"
  >
    <UFormField
      v-for="field in fields"
      :key="field.key"
      :label="field.label"
      :name="field.key"
    >
      <component
        :is="field.component"
        v-bind="field.props"
        v-model="(state as Record<string, any>)[field.key]"
      />
    </UFormField>

    <div class="space-y-2">
      <UButton
        type="submit"
        class="w-full text-center inline py-2 bg-neutral-300"
        :class="isButtonEnabled ? 'bg-primary' : ''"
      >
        Wyślij
      </UButton>
    </div>
  </UForm>
</template>
