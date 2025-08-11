<script setup lang="ts">
definePageMeta({
  layout: 'centered',
})

useSeoMeta({
  title: 'Rejestracja drużyn',
})

const route = useRoute()
const confirmationCode = String(route.query.code)

const { data, refresh, error } = await useApi('/teams/external_invitations/info/{code}', {
  path: {
    code: confirmationCode,
  },
  onResponseError: undefined,
})

if (error.value) {
  showError({
    statusCode: 400,
    message: 'Zły kod rejestracji.\n Skontaktuj się z organizatorami...',
  })
}

function print() {
  if (import.meta.browser) {
    window.print()
  }
}
</script>

<template>
  <div class="max-w-3xl">
    <h1 class="text-2xl font-semibold text-center text-white mb-6">
      Zarejestruj drużyny
    </h1>
    <p>
      Dziękujemy za zainteresowanie przez szkołę
      <strong>{{ data?.organization }}</strong> konkursem Hack4Krak!
    </p>

    <div v-if="data && data.teams.length > 0" class="space-y-4 mt-6">
      <p>
        Poniżej znajduje się lista unikalnych kodów rejestracyjnych przypisanych do poszczególnych uczestników.
        Proszę przekazać każdemu uczniowi jego indywidualny kod — można go wydrukować i rozdać lub przekazać elektronicznie.
      </p>
      <p>
        Każdy uczeń powinien założyć konto na stronie <ULink to="/">
          hack4krak.pl
        </ULink>
        , otworzyć panel, kliknąć „Dołącz do drużyny”, a następnie wpisać lub zeskanować otrzymany kod.
      </p>

      <div
        v-for="item in data.teams"
        :key="item.team_name"
        class="bg-elevated  p-3 rounded space-y-2"
      >
        <p class="font-bold">
          Nazwa drużyny: {{ item.team_name }}
        </p>
        <div class="flex flex-wrap gap-2">
          <div
            v-for="code in item.codes"
            :key="code"
            class="p-2 rounded text-center border-1 border-accented"
          >
            {{ code }}
            <Qrcode :value="code" class="w-16 mt-1" />
          </div>
        </div>
      </div>

      <UButton icon="mdi:printer" class="print:hidden" @click="print">
        Wydrukuj
      </UButton>
    </div>

    <AdminFormRegisterExternalTeam
      v-else
      :confirmation-code="confirmationCode"
      :on-success="refresh"
    />
  </div>
</template>
