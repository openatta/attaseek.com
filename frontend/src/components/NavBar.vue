<script setup lang="ts">
import { ref } from 'vue'
import { RouterLink, useRouter } from 'vue-router'

const router = useRouter()
const mobileOpen = ref(false)

const navItems = [
  { label: 'Home', to: '/' },
  { label: 'Products', to: '/products' },
  { label: 'Technology', to: '/technology' },
  { label: 'Docs', to: '/docs' },
  { label: 'Community', to: '/community' },
  { label: 'About', to: '/about' },
]

function toggleMobile() {
  mobileOpen.value = !mobileOpen.value
}

function closeMobile() {
  mobileOpen.value = false
}
</script>

<template>
  <nav class="sticky top-0 z-50 border-b border-gray-200 dark:border-gray-800 bg-white/80 dark:bg-gray-950/80 backdrop-blur-md">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between h-16">
        <!-- Logo -->
        <RouterLink to="/" class="flex items-center gap-2 font-bold text-xl text-brand-600 dark:text-brand-400" @click="closeMobile">
          <svg class="w-8 h-8" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
            <rect width="32" height="32" rx="8" fill="currentColor" class="text-brand-500" />
            <path d="M8 12l4-4 4 4M8 20l4 4 4-4M16 8l4-4 4 4M16 24l4-4 4 4" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          AttaSeek
        </RouterLink>

        <!-- Desktop nav -->
        <div class="hidden md:flex items-center gap-1">
          <RouterLink
            v-for="item in navItems"
            :key="item.to"
            :to="item.to"
            class="px-3 py-2 rounded-lg text-sm font-medium text-gray-600 dark:text-gray-300 hover:text-brand-600 dark:hover:text-brand-400 hover:bg-brand-50 dark:hover:bg-brand-950/50 transition-colors"
            active-class="text-brand-600 dark:text-brand-400 bg-brand-50 dark:bg-brand-950/50"
          >
            {{ item.label }}
          </RouterLink>
        </div>

        <!-- Mobile toggle -->
        <button
          class="md:hidden p-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800"
          @click="toggleMobile"
          aria-label="Toggle menu"
        >
          <svg v-if="!mobileOpen" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
          </svg>
          <svg v-else class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Mobile menu -->
      <div v-if="mobileOpen" class="md:hidden pb-4 border-t border-gray-200 dark:border-gray-800">
        <div class="flex flex-col gap-1 pt-3">
          <RouterLink
            v-for="item in navItems"
            :key="item.to"
            :to="item.to"
            class="px-3 py-2.5 rounded-lg text-base font-medium text-gray-600 dark:text-gray-300 hover:text-brand-600 dark:hover:text-brand-400 hover:bg-brand-50 dark:hover:bg-brand-950/50 transition-colors"
            active-class="text-brand-600 dark:text-brand-400 bg-brand-50 dark:bg-brand-950/50"
            @click="closeMobile"
          >
            {{ item.label }}
          </RouterLink>
        </div>
      </div>
    </div>
  </nav>
</template>
