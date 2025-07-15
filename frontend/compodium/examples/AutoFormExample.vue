<script setup lang="ts">
import * as z from 'zod'

const schema = z.object({
  email: z.email({ error: 'Adres e-mail jest wymagany' }),
  text: z.string({ error: 'Adres e-mail jest wymagany' }),
  number: z.number({ error: 'Adres e-mail jest wymagany' }).default(5),
  bool: z.boolean({ error: 'Adres e-mail jest wymagany' }).default(true),
})
</script>

<template>
  <AutoForm :schema="schema" />
</template>
