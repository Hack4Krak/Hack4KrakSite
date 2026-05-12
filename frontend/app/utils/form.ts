import { AInputPasswordToggle } from '#components'

const NAME_CHARS_REGEX = /^[\p{Script=Latin}\p{N}\p{P} ]*$/u

function zNameChars() {
  return z.string({ error: 'Pole jest wymagane' })
    .regex(NAME_CHARS_REGEX, 'Pole może zawierać tylko litery alfabetu łacińskiego, cyfry, spacje i znaki interpunkcyjne')
}

export function zPassword(errorMessage?: string) {
  return z.string({ error: errorMessage || 'Hasło jest wymagane' })
    .min(8, 'Hasło musi mieć co najmniej 8 znaków')
    .max(32, 'Pole może mieć maksymalnie 32 znaki')
    .meta({ title: 'Hasło', input: { component: AInputPasswordToggle } })
}

export function zTeamName() {
  return zNameChars()
    .min(3, 'Pole musi mieć co najmniej 3 znaki')
    .max(32, 'Pole może mieć maksymalnie 32 znaki')
    .meta({ title: 'Nazwa drużyny' })
}

export function zUsername() {
  return zNameChars()
    .min(3, 'Pole musi mieć co najmniej 3 znaki')
    .max(32, 'Pole może mieć maksymalnie 32 znaki')
    .meta({ title: 'Nazwa użytkownika' })
}

export function zFirstName() {
  return z.string({ error: 'Imię jest wymagane' })
    .min(1, 'Pole nie może być puste')
    .max(64, 'Pole może mieć maksymalnie 64 znaki')
}

export function zOrganization() {
  return zNameChars()
    .min(3, 'Pole musi mieć co najmniej 3 znaki')
    .max(128, 'Pole może mieć maksymalnie 128 znaków')
}

export function zLocation() {
  return zNameChars()
    .min(2, 'Pole musi mieć co najmniej 2 znaki')
    .max(128, 'Pole może mieć maksymalnie 128 znaków')
}
