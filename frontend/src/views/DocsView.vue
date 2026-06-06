<script setup lang="ts">
const docEntries = [
  {
    product: 'AttaSeek',
    description: 'Desktop AI Agent Workstation — architecture, UI specs, development guide.',
    links: [
      { label: 'Project Overview', href: 'https://github.com/openatta/AttaSeek' },
      { label: 'Architecture', href: 'https://github.com/openatta/AttaSeek/blob/main/docs/design/' },
    ],
  },
  {
    product: 'AttaCode',
    description: 'CLI Agent Engine — crate architecture, tool API, compaction strategy, protocol specs.',
    links: [
      { label: 'CLAUDE.md (Project Guide)', href: 'https://github.com/openatta/AttaCode/blob/main/CLAUDE.md' },
      { label: 'Architecture Docs', href: 'https://github.com/openatta/AttaCode/tree/main/docs/' },
    ],
  },
  {
    product: 'AttaGo',
    description: 'Mobile Remote Control — UI design, app architecture, proto generation.',
    links: [
      { label: 'CLAUDE.md (Project Guide)', href: 'https://github.com/openatta/AttaGo/blob/main/CLAUDE.md' },
      { label: 'UI / Design Docs', href: 'https://github.com/openatta/AttaGo/tree/main/docs/' },
    ],
  },
  {
    product: 'ClawPod',
    description: 'Desktop Agent Manager — sandbox architecture, bridge protocol, CLI reference.',
    links: [
      { label: 'CLAUDE.md (Project Guide)', href: 'https://github.com/openatta/ClawPod/blob/main/CLAUDE.md' },
      { label: 'Architecture Docs', href: 'https://github.com/openatta/ClawPod/tree/main/docs/' },
    ],
  },
  {
    product: 'Atta Protocol',
    description: 'Cross-component protocol definitions — channel, signaling, E2E encryption.',
    links: [
      { label: 'Proto Schema Docs', href: 'https://github.com/openatta/AttaMeta/tree/main/Proto/' },
      { label: 'E2E Security Model', href: 'https://github.com/openatta/docs/blob/main/E2E_SECURITY.md' },
    ],
  },
]
</script>

<template>
  <div>
    <!-- Header -->
    <section class="bg-gradient-to-br from-brand-50 to-white dark:from-gray-950 dark:to-gray-900 py-16">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">Documentation</h1>
        <p class="text-lg text-gray-500 dark:text-gray-400 max-w-2xl">
          Deep dives into each product — architecture, protocols, and development guides. All open source, all on GitHub.
        </p>
      </div>
    </section>

    <!-- Doc cards -->
    <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
      <div class="grid sm:grid-cols-2 gap-6">
        <div
          v-for="entry in docEntries"
          :key="entry.product"
          class="p-6 rounded-2xl border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900"
        >
          <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-2">{{ entry.product }}</h2>
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">{{ entry.description }}</p>
          <div class="flex flex-wrap gap-3">
            <a
              v-for="link in entry.links"
              :key="link.href"
              :href="link.href"
              target="_blank"
              rel="noopener"
              class="inline-flex items-center text-sm font-medium text-brand-600 dark:text-brand-400 hover:text-brand-700 dark:hover:text-brand-300 transition-colors"
            >
              {{ link.label }}
              <svg class="ml-1 w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
              </svg>
            </a>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>
