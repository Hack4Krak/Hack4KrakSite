<script setup lang="ts">
import { FetchError } from 'ofetch'

const links = [
  { name: 'Users', url: '/admin/users/' },
  { name: 'Teams', url: '/admin/teams' },
  { name: 'Tasks', url: '/admin/tasks' },
]

if (import.meta.server) {
  try {
    const { data, error } = await useAuth('/admin/users/list', {
      method: 'GET',
    })

    if (error.value) {
      showError({
        statusCode: 403,
        statusMessage: 'You are not authorized to view this page',
      })
      console.error(error)
    }

    const users = data.value
  } catch (error) {
    console.error(error)
    if (!(error instanceof FetchError)) {
      throw error
    }

    showError(error)
  }
}
</script>

<template>
  <div class="flex">
    <SideNavigation :links="links" />
    {{ users }}
  </div>
</template>
