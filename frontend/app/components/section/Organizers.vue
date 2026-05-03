<script setup lang="ts">
import { organizers } from '~~/content/organizers'

const groupedOrganizers = computed(() => {
  const map = new Map<string, typeof organizers.team>()

  for (const member of organizers.team) {
    if (!map.has(member.section)) {
      map.set(member.section, [])
    }

    map.get(member.section)!.push(member)
  }

  return map
})
</script>

<template>
  <div class="flex flex-col gap-6">
    <section
      v-for="sectionName in organizers.hierarchy"
      :key="sectionName"
      class="flex flex-col w-full text-center items-center justify-center"
    >
      <h3 class="font-semibold">
        {{ sectionName.toUpperCase() }}
      </h3>
      <div class="w-full grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 my-6">
        <TeamMemberCard
          v-for="organizer in groupedOrganizers.get(sectionName) || []"
          :key="organizer.email"
          :name="organizer.name"
          :email="organizer.email"
          :role="organizer.role"
          :github="organizer.github"
          :linkedin="organizer.linkedin"
          :highlight="sectionName === 'zarząd'"
          class="w-full"
        />
      </div>
    </section>
  </div>
</template>
