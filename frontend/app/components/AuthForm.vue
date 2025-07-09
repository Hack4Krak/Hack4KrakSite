<script setup lang="ts">
import type { FormSubmitEvent } from '#ui/types'
import { FetchError } from 'ofetch'
import * as z from 'zod'

const props = defineProps<{
  isLogin: boolean
}>()

type Schema = z.output<typeof schema>
const schema = z.object({
  email: z.string({ error: 'Adres e-mail jest wymagany' }).email('Niepoprawny adres e-mail'),
  password: z.string({ error: 'Hasło jest wymagane' }).min(8, 'Hasło musi mieć minimum 8 znaków'),
  ...(props.isLogin ? {} : { name: z.string({ error: 'Nazwa użytkownika jest wymagana' }).min(3, 'Nazwa użytkownika musi mieć co najmniej 3 znaki') }),
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

const route = useRoute()

if (route.query.redirect_from_confirmation === 'true' && import.meta.client) {
  toast.add({
    title: 'Sukces',
    description: 'Pomyślnie aktywowano konto! Możesz się teraz zalogować',
    color: 'success',
  })
  const query = Object.assign({}, route.query)
  delete query.redirect_from_confirmation
  useRouter().replace({ query })
}

async function onSubmit(event: FormSubmitEvent<Schema>) {
  event.preventDefault()

  loading.value = true

  try {
    const address = props.isLogin ? '/auth/login' : '/auth/register'
    if (!props.isLogin) {
      toast.add({ title: 'Oczekiwanie', description: 'Wysyłanie emaila…', color: 'info' })
    }

    await useNuxtApp().$api(address, {
      method: 'POST',
      credentials: 'include',
      body: event.data,
    })

    if (props.isLogin) {
      toast.add({ title: 'Sukces', description: 'Pomyślnie zalogowano!', color: 'success' })
      await navigateTo('/panel/')
    } else {
      toast.add({ title: 'Sukces', description: 'Pomyślnie zarejestrowano! Wysłaliśmy Ci na podany adres email link do aktywacji konta', color: 'success' })
      await navigateTo('/login')
    }
  } catch (error) {
    console.error(error)
    if (!(error instanceof FetchError)) {
      throw error
    }
  } finally {
    loading.value = false
  }
}

const show_password = ref(false)
</script>

<template>
  <div class="space-y-4">
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

      <div class="flex flex-col items-start gap-1">
        <UFormField label="Hasło" name="password" class="w-full">
          <UInput v-model="state.password" class="w-full" :type="show_password ? 'text' : 'password'">
            <template #trailing>
              <UButton
                color="neutral"
                variant="link"
                size="sm"
                :icon="show_password ? 'i-lucide-eye-off' : 'i-lucide-eye'"
                :aria-label="show_password ? 'Ukryj hasło' : 'Pokaż hasło'"
                :aria-pressed="show_password"
                aria-controls="password"
                @click="show_password = !show_password"
              />
            </template>
          </UInput>
          <template #hint>
            <NuxtLink v-if="isLogin" class="link-without-underline" to="/request_password_reset" tabindex="-1">
              Zresetuj hasło
            </NuxtLink>
          </template>
        </UFormField>
      </div>

      <div class="space-y-2">
        <UButton type="submit" class="w-full text-center inline rounded-3xl py-2 bg-neutral-300" :disabled="loading" :class="isButtonEnabled ? 'bg-primary' : ''">
          {{ isLogin ? 'Zaloguj' : 'Zarejestruj' }}
        </UButton>

        <div class="flex flex-col gap-1">
          <span class="text-sm text-neutral-400">
            {{ isLogin ? 'Nie masz konta?' : 'Masz już konto?' }}
            <NuxtLink class="link" :to="isLogin ? '/register' : '/login'">
              {{ isLogin ? 'Załóż je' : 'Zaloguj się' }}
            </NuxtLink>
          </span>
        </div>
      </div>
    </UForm>

    <div class="w-full text-center">
      <USeparator class="my-3" label="Albo kontynuuj z" :ui="{ label: 'text-zinc-400' }" />

      <div class="flex text-center gap-2 justify-center items-center">
        <a :href="`${OAuthBaseUrl}/google`" aria-label="Login with Google"><UIcon name="logos:google-icon" size="45" class="cursor-pointer hover:scale-110 duration-300" mode="svg" /></a>
        <a :href="`${OAuthBaseUrl}/github`" aria-label="Login with GitHub"><UIcon name="mdi:github" size="50" class="cursor-pointer hover:scale-110 duration-300" /></a>
      </div>
    </div>
  </div>
</template>
