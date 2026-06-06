<script setup lang="ts">
defineProps<{
  headline: string
  subheadline: string
  ctaText?: string
  ctaTo?: string
  secondaryText?: string
  secondaryTo?: string
}>()
</script>

<template>
  <section class="relative overflow-hidden">
    <!-- Background gradient -->
    <div class="absolute inset-0 bg-gradient-to-br from-brand-50 via-white to-blue-50 dark:from-gray-950 dark:via-gray-950 dark:to-brand-950/30" />
    <div class="absolute inset-0 bg-[radial-gradient(ellipse_at_top_right,_var(--tw-gradient-stops))] from-brand-100/40 via-transparent to-transparent dark:from-brand-900/20" />

    <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24 sm:py-32 lg:py-40">
      <div class="max-w-3xl">
        <h1 class="text-4xl sm:text-5xl lg:text-6xl font-bold tracking-tight text-gray-900 dark:text-white mb-6">
          {{ headline }}
        </h1>
        <p class="text-lg sm:text-xl text-gray-500 dark:text-gray-400 leading-relaxed mb-10 max-w-2xl">
          {{ subheadline }}
        </p>
        <div class="flex flex-wrap gap-4">
          <a
            v-if="ctaText && ctaTo"
            :href="ctaTo"
            class="inline-flex items-center px-6 py-3 rounded-xl text-base font-semibold text-white bg-brand-600 hover:bg-brand-700 shadow-lg shadow-brand-500/25 hover:shadow-brand-500/40 transition-all"
          >
            {{ ctaText }}
            <svg class="ml-2 w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
          </a>
          <a
            v-if="secondaryText && secondaryTo"
            :href="secondaryTo"
            class="inline-flex items-center px-6 py-3 rounded-xl text-base font-semibold text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600 shadow-sm transition-all"
          >
            {{ secondaryText }}
          </a>
        </div>
      </div>
    </div>
  </section>
</template>
