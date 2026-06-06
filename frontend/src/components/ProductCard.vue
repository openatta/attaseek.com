<script setup lang="ts">
defineProps<{
  title: string
  description: string
  icon: string
  to: string
  tags?: string[]
}>()
</script>

<template>
  <a
    :href="to"
    class="group block p-6 rounded-2xl border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900 hover:border-brand-300 dark:hover:border-brand-700 hover:shadow-lg hover:shadow-brand-500/5 transition-all duration-300"
  >
    <div class="text-3xl mb-4">{{ icon }}</div>
    <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2 group-hover:text-brand-600 dark:group-hover:text-brand-400 transition-colors">
      {{ title }}
    </h3>
    <p class="text-sm text-gray-500 dark:text-gray-400 leading-relaxed mb-4">
      {{ description }}
    </p>
    <div v-if="tags?.length" class="flex flex-wrap gap-2">
      <span
        v-for="tag in tags"
        :key="tag"
        class="inline-block px-2.5 py-0.5 rounded-full text-xs font-medium bg-brand-50 dark:bg-brand-950 text-brand-600 dark:text-brand-400"
      >
        {{ tag }}
      </span>
    </div>
  </a>
</template>
