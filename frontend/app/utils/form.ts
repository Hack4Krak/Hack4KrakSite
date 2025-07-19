export function zPassword() {
  return z.string({ error: 'Hasło jest wymagane' })
    .min(8, 'Hasło musi mieć minimum 8 znaków')
}

export function zTeamName() {
  return z.string({ error: 'Nazwa drużyny jest wymagana' })
    .min(5, 'Nazwa drużyny musi mieć min 5 znaków')
}

declare module 'zod' {
  interface GlobalMeta {
    /** Configuration for <AutoForm> Vue component */
    autoForm?: {
      /** Float input box to the right of the label */
      floatRight?: boolean
    }
  }
}
