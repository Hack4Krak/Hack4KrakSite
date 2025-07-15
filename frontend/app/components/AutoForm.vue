<script setup lang="ts">
import { UCheckbox, UInput, UInputNumber } from '#components'
import * as z from 'zod'

const props = defineProps<{
  schema: z.ZodObject<any>
}>()

type Schema = z.output<typeof props.schema>
const state = reactive<Partial<Schema>>({})

const shape = (props.schema as z.ZodObject<any>).shape

// todo: type
function getComponentForZodType(key: string, zodType: any) {
  let component = UInput as any
  let props = {}

  if (zodType instanceof z.ZodEmail) {
    props = {
      type: 'email',
    }
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
    component,
    props,
  }
}

const fields = Object.entries(shape).map(([key, zodType]) => {
  return getComponentForZodType(key, zodType)
})

const isButtonEnabled = computed(() => {
  return props.schema.safeParse(state).success
})
</script>

<template>
  <UForm :schema="schema" :state="state" class="space-y-4 text-center">
    <UFormField
      v-for="field in fields"
      :key="field.key"
      :label="field.key"
      :name="field.key"
    >
      <component
        :is="field.component"
        v-bind="field.props"
        v-model="(state as any)[field.key]"
      />
    </UFormField>

    <div class="space-y-2">
      <UButton type="submit" class="w-full text-center inline py-2 bg-neutral-300" :class="isButtonEnabled ? 'bg-primary' : ''">
        Wyślij
      </UButton>
    </div>
  </UForm>
</template>
