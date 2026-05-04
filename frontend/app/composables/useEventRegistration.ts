export default async function useEventRegistration() {
  const { data: team } = await useAuth('/teams/membership/my_team', {
    onResponseError: undefined,
  })

  const isRegistered = computed(() => Boolean(team.value))

  return {
    isRegistered,
    team,
  }
}
