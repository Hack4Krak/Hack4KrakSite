export function zPassword() {
  return z.string({ error: 'Hasło jest wymagane' })
    .min(8, 'Hasło musi mieć minimum 8 znaków')
}

export function zTeamName() {
  return z.string({ error: 'Nazwa drużyny jest wymagana' })
    .min(5, 'Nazwa drużyny musi mieć min 5 znaków')
}

export function zUsername() {
  return z.string({ error: 'Nazwa użytkownika' })
    .min(3, 'Nazwa użytkownika musi mieć min 3 znaki')
}
