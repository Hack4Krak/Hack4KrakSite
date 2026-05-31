export function useTaskIcon(getTaskId: () => string | null | undefined) {
  const taskIconBaseUrl = `${useRuntimeConfig().public.openFetch.api.baseURL}/tasks/icon/`
  const iconErrored = ref(false)
  const imgSrc = computed(() => {
    const id = getTaskId()
    return iconErrored.value || !id ? '/img/task_unavailable.png' : `${taskIconBaseUrl}${id}`
  })
  return {
    imgSrc,
    onImgError: () => {
      iconErrored.value = true
    },
  }
}
