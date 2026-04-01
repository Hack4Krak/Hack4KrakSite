<script setup lang="ts">
import { FetchError } from 'ofetch'

const props = defineProps<{
  isLogin: boolean
}>()

type Schema = zInfer<typeof schema>
const schema = z.object({
  ...(props.isLogin ? {} : { name: zUsername() }),
  email: z.email({ error: 'Niepoprawny adres e-mail' }).meta({ title: 'Adres e-mail' }),
  password: zPassword(),
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
    }
    else {
      toast.add({ title: 'Sukces', description: 'Pomyślnie zarejestrowano! Wysłaliśmy Ci na podany adres email link do aktywacji konta', color: 'success' })
      await navigateTo('/login')
    }
  }
  catch (error) {
    console.error(error)
    if (!(error instanceof FetchError)) {
      throw error
    }
  }
  finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="auth-card w-full">
    <!-- Pixel-art accent top border -->
    <div class="h-1 w-full bg-primary mb-8" />

    <!-- Logo + platform tag -->
    <div class="flex items-center gap-3 mb-8">
      <NuxtImg src="/img/logo.png" alt="Hack4Krak" class="h-8 w-auto" />
      <div class="w-px h-5 bg-surface-muted" />
      <span class="text-[10px] font-bold tracking-[0.25em] uppercase text-muted">
        CTF Platform
      </span>
    </div>

    <!-- Heading -->
    <h1 class="font-pixelify text-3xl sm:text-4xl text-default mb-2 leading-tight">
      {{ isLogin ? 'Zaloguj się' : 'Zarejestruj się' }}
    </h1>
    <p class="text-sm text-muted mb-8">
      {{ isLogin ? 'Wróć do rywalizacji w Hack4Krak CTF.' : 'Dołącz do Hack4Krak CTF i sprawdź swoje umiejętności.' }}
    </p>

    <!-- OAuth — prominent, above the fold -->
    <div class="flex gap-3 mb-6">
      <a
        :href="`${OAuthBaseUrl}/google`"
        aria-label="Zaloguj przez Google"
        class="oauth-btn flex-1 group"
      >
        <UIcon name="logos:google-icon" class="size-5 flex-shrink-0" mode="svg" />
        <span class="text-sm font-semibold text-muted group-hover:text-default transition-colors">Google</span>
      </a>
      <a
        :href="`${OAuthBaseUrl}/github`"
        aria-label="Zaloguj przez GitHub"
        class="oauth-btn flex-1 group"
      >
        <UIcon name="mdi:github" class="size-5 flex-shrink-0" />
        <span class="text-sm font-semibold text-muted group-hover:text-default transition-colors">GitHub</span>
      </a>
    </div>

    <!-- Separator -->
    <USeparator class="mb-6" :ui="{ label: 'text-muted text-xs tracking-widest uppercase' }" label="lub przez email" />

    <!-- Form fields -->
    <AutoForm
      :schema="schema"
      class="text-left"
      :config="{
        submit: {
          props: {
            label: isLogin ? 'Zaloguj' : 'Utwórz konto',
            class: 'w-full font-pixelify text-base tracking-wide',
          },
        },
      }"
      @submit="onSubmit"
    >
      <template #email-hint>
        <NuxtLink v-if="isLogin" class="link-without-underline text-xs" to="/request_password_reset" tabindex="-1">
          Zresetuj hasło
        </NuxtLink>
      </template>
    </AutoForm>

    <!-- Switch mode link -->
    <p class="text-center text-sm text-muted mt-6 pt-5 border-t border-surface-muted">
      {{ isLogin ? 'Nie masz konta?' : 'Masz już konto?' }}
      <NuxtLink class="link font-semibold ml-1" :to="isLogin ? '/register' : '/login'">
        {{ isLogin ? 'Załóż je tutaj' : 'Zaloguj się' }}
      </NuxtLink>
    </p>
  </div>
</template>

<style scoped>
.oauth-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.625rem 1rem;
  border: 2px solid var(--color-surface-muted);
  background: transparent;
  cursor: pointer;
  transition: border-color 0.15s, background 0.15s;
  text-decoration: none;
  color: inherit;
}

.oauth-btn:hover {
  border-color: var(--color-primary);
  background: color-mix(in oklch, var(--color-primary) 6%, transparent);
}
</style>
