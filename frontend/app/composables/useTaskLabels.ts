export interface LabelDescriptor {
  id: string
  name: string
  description: string
}

export function useTaskLabels() {
  return useLazyAsyncData<LabelDescriptor[]>(
    'task-labels',
    () => $fetch<LabelDescriptor[]>('/tasks/labels', {
      baseURL: useRuntimeConfig().public.openFetch.api.baseURL,
    }),
    { default: () => [] },
  )
}

export function useTaskLabelDescription() {
  const { data } = useTaskLabels()
  const descriptionMap = computed(() => {
    const descriptionByLabelId: Record<string, string> = {}
    for (const label of data.value ?? [])
      descriptionByLabelId[label.id.toLowerCase()] = label.description
    return descriptionByLabelId
  })
  return (id: string) => descriptionMap.value[id.toLowerCase()] ?? ''
}
