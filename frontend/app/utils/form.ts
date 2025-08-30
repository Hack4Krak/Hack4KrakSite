import { AInputPasswordToggle } from '#components'

export function zPassword(errorMessage?: string) {
  return z.string({ error: errorMessage || 'Hasło jest wymagane' })
    .min(8, 'Hasło musi mieć co najmniej 8 znaków')
    .meta({ input: { component: AInputPasswordToggle } })
}

export function zTeamName() {
  return z.string({ error: 'Nazwa drużyny jest wymagana' })
    .min(5, 'Nazwa drużyny musi mieć co najmniej 5 znaków')
}

export function zUsername() {
  return z.string({ error: 'Nazwa użytkownika jest wymagana' })
    .min(3, 'Nazwa użytkownika musi mieć co najmniej 3 znaki')
    .meta({ title: 'Nazwa użytkownika' })
}
