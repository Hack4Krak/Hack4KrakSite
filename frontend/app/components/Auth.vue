<script setup lang="ts">
import type { FormSubmitEvent } from '#ui/types'
import { FetchError } from 'ofetch'
import * as z from 'zod'

const props = defineProps<{
  isLogin: boolean
}>()

type Schema = z.output<typeof schema>
const schema = z.object({
  email: z.string({ required_error: 'Adres e-mail jest wymagany' }).email('Niepoprawny adres e-mail'),
  password: z.string({ required_error: 'Hasło jest wymagane' }).min(8, 'Hasło musi mieć minimum 8 znaków'),
  ...(props.isLogin ? {} : { name: z.string({ required_error: 'Nazwa użytkownika jest wymagana' }).min(3, 'Nazwa użytkownika musi mieć co najmniej 3 znaki') }),
})

const loading = ref(false)
const toast = useToast()
const state = reactive<Partial<Schema>>({
  email: undefined,
  password: undefined,
  name: undefined,
})

const isButtonEnabled = computed(() => {
  return schema.safeParse(state).success
})

const OAuthBaseUrl = `${useRuntimeConfig().public.openFetch.api.baseURL}/auth/oauth`

async function onSubmit(event: FormSubmitEvent<Schema>) {
  event.preventDefault()

  loading.value = true

  try {
    const address = props.isLogin ? '/auth/login' : '/auth/register'
    await useNuxtApp().$api(address, {
      method: 'POST',
      credentials: 'include',
      body: event.data,
    })

    await toast.add({ title: 'Sukces', description: 'Pomyślnie zalogowano!', color: 'success' })
    await navigateTo('/panel/')
  } catch (error) {
    console.error(error)
    if (!(error instanceof FetchError)) {
      throw error
    }

    if (error.data) {
      await toast.add({ title: 'Błąd logowania', description: error.data.message, color: 'error' })
    } else {
      await toast.add({ title: 'Błąd logowania', color: 'error' })
    }
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="flex flex-col md:flex-row justify-center items-center bg-gray-100 dark:bg-zinc-900 rendering-pixelated bg-[url(assets/img/background.webp)] bg-cover bg-center">
    <div class="space-y-4 w-90 bg-default p-8 rounded-2xl">
      <h1 class="text-2xl font-medium">
        {{ isLogin ? 'Zaloguj się' : 'Zarejestruj się' }}
      </h1>

      <UForm :schema="schema" :state="state" class="space-y-4 text-center" @submit="onSubmit">
        <UFormField v-if="!isLogin" label="Nazwa użytkownika" name="name">
          <TransparentInput v-model="state.name" />
        </UFormField>

        <UFormField label="Email" name="email">
          <TransparentInput v-model="state.email" type="email" />
        </UFormField>

        <UFormField label="Hasło" name="password">
          <TransparentInput v-model="state.password" type="password" />
        </UFormField>

        <div class="space-y-2">
          <UButton type="submit" class="w-full text-center inline rounded-3xl py-2 bg-neutral-300" :disabled="loading" :class="isButtonEnabled ? 'bg-[var(--ui-primary)]' : ''">
            {{ isLogin ? 'Zaloguj' : 'Zarejestruj' }}
          </UButton>

          <span class="text-sm text-neutral-500">
            {{ isLogin ? 'Nie masz konta?' : 'Masz już konto?' }}
            <NuxtLink class="underline text-white cursor-pointer hover:text-[var(--ui-primary)]" :to="props.isLogin ? '/register' : '/login'">
              {{ isLogin ? 'Załóż je' : 'Zaloguj się' }}
            </NuxtLink>
          </span>
        </div>
      </UForm>

      <div class="w-full text-center">
        <USeparator class="my-3" label="Albo kontynuuj z" :ui="{ label: 'text-zinc-400' }" />

        <div class="flex text-center gap-2 justify-center items-center">
          <a :href="`${OAuthBaseUrl}/google`"><Icon name="logos:google-icon" size="45" class="cursor-pointer hover:scale-110 duration-300" mode="svg" /></a>
          <a :href="`${OAuthBaseUrl}/github`"><Icon name="uil:github" size="50" class="cursor-pointer hover:scale-110 duration-300" /></a>
        </div>
      </div>
    </div>
  </div>
</template>
