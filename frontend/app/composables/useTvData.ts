import type { ApiResponse } from '#open-fetch'

export type TvTeam = ApiResponse<'teams_with_tasks'>[0]
export type TvTask = ApiResponse<'task_list'>[0]

interface TaskEvent {
  type: 'first-solve' | 'all-solved'
  taskName: string
  teamName?: string
  teamColor?: string
}

export function useTvData(refreshInterval = 10000) {
  const { $api } = useNuxtApp()

  const teams = ref<TvTeam[]>([])
  const tasks = ref<TvTask[]>([])
  const chartData = ref<ApiResponse<'chart'> | null>(null)
  const eventInfo = ref<ApiResponse<'info'> | null>(null)

  const previousSolves = ref<Record<string, Set<string>>>({})
  const taskEvents = ref<TaskEvent[]>([])

  async function fetchAll() {
    const [teamsRes, tasksRes, chartRes, eventRes] = await Promise.all([
      $api('/leaderboard/teams_with_tasks'),
      $api('/tasks/list'),
      $api('/leaderboard/chart'),
      $api('/event/info'),
    ])

    if (tasksRes)
      tasks.value = tasksRes
    if (chartRes)
      chartData.value = chartRes
    if (eventRes)
      eventInfo.value = eventRes

    if (teamsRes) {
      detectNewSolves(teamsRes)
      teams.value = teamsRes
    }
  }

  function detectNewSolves(newTeams: TvTeam[]) {
    if (!tasks.value.length)
      return

    const totalTeams = newTeams.length
    const newEvents: TaskEvent[] = []

    for (const task of tasks.value) {
      const prevSolvers = previousSolves.value[task.id] ?? new Set()
      const currentSolvers = new Set<string>()

      for (const team of newTeams) {
        if (team.tasks?.[task.id]) {
          currentSolvers.add(team.team_id)

          if (!prevSolvers.has(team.team_id)) {
            newEvents.push({
              type: 'first-solve',
              taskName: task.name,
              teamName: team.team_name,
              teamColor: team.color,
            })
          }
        }
      }

      if (currentSolvers.size === totalTeams && prevSolvers.size < totalTeams && totalTeams > 0) {
        newEvents.push({
          type: 'all-solved',
          taskName: task.name,
        })
      }

      previousSolves.value[task.id] = currentSolvers
    }

    if (newEvents.length > 0) {
      taskEvents.value = [...taskEvents.value, ...newEvents]
    }
  }

  function dismissEvent() {
    taskEvents.value = taskEvents.value.slice(1)
  }

  const currentEvent = computed<TaskEvent | undefined>(() => taskEvents.value[0])

  const sortedTeams = computed(() =>
    [...teams.value].sort((a, b) => b.current_points - a.current_points),
  )

  fetchAll()
  const { pause, resume } = useIntervalFn(fetchAll, refreshInterval)

  return {
    teams: sortedTeams,
    tasks,
    chartData,
    eventInfo,
    currentEvent,
    dismissEvent,
    pause,
    resume,
    refresh: fetchAll,
  }
}
