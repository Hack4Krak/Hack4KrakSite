<script setup lang="ts">
import type { FormSubmitEvent } from '@nuxt/ui'
import * as z from 'zod'

const schema = z
  .object({
    new_password: z
      .string({ error: 'Hasło jest wymagane' })
      .refine(val => !val || val.length >= 8, {
        message: 'Hasło musi mieć min 8 znaków',
      }),
    confirm_new_password: z.string({
      error: 'Potwierdzenie hasła jest wymagane',
    }),
    old_password: z.string({ error: 'Stare hasło jest wymagane' }),
  })
  .superRefine((data, ctx) => {
    if (data.new_password !== data.confirm_new_password) {
      const message = 'Hasła nie są takie same'

      ctx.addIssue({
        code: z.ZodIssueCode.custom,
        message,
        path: ['confirm_new_password'],
      })
      ctx.addIssue({
        code: z.ZodIssueCode.custom,
        message,
        path: ['new_password'],
      })
    }
  })

type Schema = z.output<typeof schema>

const state = reactive<Partial<Schema>>({
  new_password: undefined,
  confirm_new_password: undefined,
  old_password: undefined,
})

const toast = useToast()
const open = defineModel<boolean>()
const formRef = useTemplateRef('form')

const { $auth } = useNuxtApp()

async function onSubmit(event: FormSubmitEvent<Schema>) {
  event.preventDefault()

  const response = await $auth('/account/update/password', {
    method: 'PATCH',
    body: {
      old_password: event.data.old_password,
      new_password: event.data.new_password,
    },
  })

  if ((response as any).error) {
    return
  }

  toast.add({ title: 'Sukces', description: 'Pomyślnie zaktualizowano ustawienia konta', color: 'success' })

  open.value = false
}
</script>

<template>
  <UModal v-model:open="open" title="Ustawienia konta" description="Zmień hasło" :ui="{ footer: 'justify-end' }">
    <template #body>
      <UForm ref="form" :schema="schema" :state="state" class="space-y-4" @submit="onSubmit">
        <UFormField label="Stare hasło" name="old_password">
          <TransparentInput v-model="state.old_password" class="w-full" type="password" />
        </UFormField>
        <UFormField label="Nowe hasło" name="new_password">
          <TransparentInput v-model="state.new_password" class="w-full" type="password" />
        </UFormField>
        <UFormField label="Potwierdź Nowe hasło" name="confirm_new_password">
          <TransparentInput v-model="state.confirm_new_password" class="w-full" type="password" />
        </UFormField>
        <NuxtLink class="link" to="/request_password_reset">
          Zresetuj hasło
        </NuxtLink>
      </UForm>
    </template>

    <template #footer>
      <UButton label="Anuluj" color="neutral" variant="outline" @click="open = false" />
      <UButton label="Potwierdź" @click="formRef?.submit()" />
    </template>
  </UModal>
</template>
