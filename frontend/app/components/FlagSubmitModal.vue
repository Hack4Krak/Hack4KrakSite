<script setup lang="ts">
const isOpen = defineModel<boolean>({ default: false })
const toast = useToast()
const { $auth } = useNuxtApp()

const flagInput = ref('')

async function submitFlag() {
  try {
    await $auth('/flag/submit', {
      method: 'POST',
      body: { flag: flagInput.value },
    })
    toast.add({ title: 'Brawo!', description: 'To była poprawna flaga!', color: 'success' })
    isOpen.value = false
    flagInput.value = ''
  } catch {
    toast.add({ title: 'Niepoprawna flaga', color: 'error' })
  }
}

watch(isOpen, (open) => {
  if (open) {
    flagInput.value = ''
  }
})
</script>

<template>
  <UModal v-model:open="isOpen" title="Złóż flagę" description="Wpisz flagę w formacie hack4KrakCTF{...}">
    <template #body>
      <form class="flex flex-col gap-4" @submit.prevent="submitFlag">
        <UInput v-model="flagInput" placeholder="hack4KrakCTF{...}" autofocus />
        <UButton type="submit" block>
          Sprawdź
        </UButton>
      </form>
    </template>
  </UModal>
</template>
