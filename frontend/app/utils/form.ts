export function zPassword() {
  return z.string({ error: 'Hasło jest wymagane' })
    .min(8, 'Hasło musi mieć co najmniej 8 znaków')
}

export function zTeamName() {
  return z.string({ error: 'Nazwa drużyny jest wymagana' })
    .min(5, 'Nazwa drużyny musi mieć co najmniej 5 znaków')
}

export function zUsername() {
  return z.string({ error: 'Nazwa użytkownika jest wymagana' })
    .min(3, 'Nazwa użytkownika musi mieć co najmniej 3 znaki')
}
