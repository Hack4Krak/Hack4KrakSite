<script setup lang="ts">
import { AInputPasswordToggle } from '#components'
import { FetchError } from 'ofetch'

const props = defineProps<{
  isLogin: boolean
}>()

type Schema = zInfer<typeof schema>
const schema = z.object({
  ...(props.isLogin ? {} : { name: zUsername() }),
  email: z.email({ error: 'Niepoprawny adres e-mail' }),
  password: zPassword().meta({ title: 'Hasło', input: { component: AInputPasswordToggle } }),
})

const loading = ref(false)
const toast = useToast()

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

async function onSubmit(event: Schema) {
  loading.value = true

  try {
    const address = props.isLogin ? '/auth/login' : '/auth/register'
    if (!props.isLogin) {
      toast.add({ title: 'Oczekiwanie', description: 'Wysyłanie emaila…', color: 'info' })
    }

    await useNuxtApp().$api(address, {
      method: 'POST',
      credentials: 'include',
      body: event,
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
</script>

<template>
  <div class="space-y-4">
    <h1 class="text-2xl font-medium">
      {{ isLogin ? 'Zaloguj się' : 'Zarejestruj się' }}
    </h1>

    <AutoForm :schema="schema" class="text-center" :config="{ submit: { props: { label: isLogin ? 'Zaloguj' : 'Zarejestruj' } } }" @submit="onSubmit">
      <template #email-hint>
        <NuxtLink v-if="isLogin" class="link-without-underline" to="/request_password_reset" tabindex="-1">
          Zresetuj hasło
        </NuxtLink>
      </template>
    </AutoForm>

    <div class="flex flex-col gap-1 w-full text-center">
      <span class="text-sm text-neutral-400">
        {{ isLogin ? 'Nie masz konta?' : 'Masz już konto?' }}
        <NuxtLink class="link" :to="isLogin ? '/register' : '/login'">
          {{ isLogin ? 'Załóż je' : 'Zaloguj się' }}
        </NuxtLink>
      </span>
    </div>

    <div class="w-full text-center">
      <USeparator class="my-3" label="Albo kontynuuj z" :ui="{ label: 'text-zinc-400' }" />

      <div class="flex text-center gap-2 justify-center items-center">
        <a :href="`${OAuthBaseUrl}/google`" aria-label="Login with Google"><UIcon name="logos:google-icon" size="45" class="cursor-pointer hover:scale-110 duration-300" mode="svg" /></a>
        <a :href="`${OAuthBaseUrl}/github`" aria-label="Login with GitHub"><UIcon name="mdi:github" size="50" class="cursor-pointer hover:scale-110 duration-300" /></a>
      </div>
    </div>
  </div>
</template>
