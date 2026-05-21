export interface TaskStats {
  solveCount: number
  points: number
}

export type TaskStatsMap = Record<string, TaskStats>

interface TaskLike {
  id: string
}

interface TeamWithTasksLike {
  tasks?: Record<string, unknown> | null
}

const DIFFICULTY_LABELS: Record<string, string> = {
  easy: 'Łatwe',
  rookie: 'Żółtodziób',
  medium: 'Średnie',
  hard: 'Trudne',
}

const LABELS: Record<string, string> = {
  'crypto': 'Kryptografia',
  'games': 'Gry',
  'misc': 'Różne',
  'reverse_engineering': 'Inż. wsteczna',
  'reverse-engineering': 'Inż. wsteczna',
  'web': 'Web',
}

const DIFFICULTY_ORDER: Record<string, number> = {
  easy: 0,
  rookie: 1,
  medium: 2,
  hard: 3,
}

export function normalizeTaskDifficulty(difficulty?: string | null) {
  const value = difficulty?.toLowerCase().trim() ?? ''

  if (value === 'easy' || value.includes('łatw'))
    return 'easy'
  if (value === 'rookie' || value.includes('żółt') || value.includes('zolt'))
    return 'rookie'
  if (value === 'medium' || value.includes('śred') || value.includes('sred'))
    return 'medium'
  if (value === 'hard' || value.includes('trud'))
    return 'hard'

  return value
}

export function taskDifficultyLabel(difficulty?: string | null) {
  const normalized = normalizeTaskDifficulty(difficulty)
  return DIFFICULTY_LABELS[normalized] ?? difficulty ?? 'Brak trudności'
}

export function taskDifficultyOrder(difficulty?: string | null) {
  return DIFFICULTY_ORDER[normalizeTaskDifficulty(difficulty)] ?? 99
}

export function taskDifficultyClass(difficulty?: string | null, active = false): string {
  const normalized = normalizeTaskDifficulty(difficulty)
  if (normalized === 'hard') {
    return active ? 'border-error bg-error/10 text-error' : 'border-error/70 bg-error/5 text-error'
  }
  if (normalized === 'medium' && !active) {
    return 'border-primary/70 bg-primary/5 text-primary'
  }
  return active ? 'border-primary bg-primary/10 text-primary' : 'border-surface-muted bg-default text-muted'
}

export function taskLabelText(label: string) {
  return LABELS[label.toLowerCase()] ?? label
}

export function calculateTaskPoints(solveCount: number, totalTeams: number) {
  const maxPoints = 500
  const minPoints = 100

  if (totalTeams <= 2 || solveCount <= 2)
    return maxPoints

  const decayFactor = (maxPoints - minPoints) / (totalTeams - 2)
  const points = maxPoints - ((solveCount - 2) * decayFactor)

  return Math.max(minPoints, Math.round(points))
}

export function buildTaskStats(tasks: TaskLike[] = [], teams: TeamWithTasksLike[] = []): TaskStatsMap {
  return tasks.reduce<TaskStatsMap>((acc, task) => {
    const solveCount = teams.filter(team => Boolean(team.tasks?.[task.id])).length
    acc[task.id] = {
      solveCount,
      points: calculateTaskPoints(solveCount, teams.length),
    }
    return acc
  }, {})
}

export function getTaskStats(stats: TaskStatsMap | undefined, taskId: string): TaskStats {
  return stats?.[taskId] ?? { solveCount: 0, points: 500 }
}
