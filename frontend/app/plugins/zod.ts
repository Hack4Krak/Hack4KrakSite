import * as z from 'zod'

export default defineNuxtPlugin(() => {
  z.config(z.locales.pl())
})
