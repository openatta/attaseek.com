<script setup lang="ts">
const channels = [
  {
    name: 'GitHub',
    description: 'All AttaSeek projects are open source. Explore the code, report issues, and contribute PRs.',
    icon: '',
    href: 'https://github.com/openatta',
    cta: 'View on GitHub',
  },
  {
    name: 'Discord',
    description: 'Join the community chat for real-time discussion, troubleshooting, and feature brainstorming.',
    icon: '',
    href: '#',
    cta: 'Coming Soon',
  },
  {
    name: 'Blog',
    description: 'Release notes, technical deep dives, and product updates from the AttaSeek team.',
    icon: '',
    href: '#',
    cta: 'Coming Soon',
  },
]
</script>

<template>
  <div>
    <!-- Header -->
    <section class="bg-gradient-to-br from-brand-50 to-white dark:from-gray-950 dark:to-gray-900 py-16">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">Community</h1>
        <p class="text-lg text-gray-500 dark:text-gray-400 max-w-2xl">
          AttaSeek is built in the open. Join the community of developers shaping the future of AI agent tooling.
        </p>
      </div>
    </section>

    <!-- Channels -->
    <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
      <div class="grid sm:grid-cols-3 gap-8">
        <div
          v-for="ch in channels"
          :key="ch.name"
          class="p-8 rounded-2xl border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900 text-center"
        >
          <div class="text-4xl mb-4">{{ ch.icon }}</div>
          <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-3">{{ ch.name }}</h2>
          <p class="text-sm text-gray-500 dark:text-gray-400 leading-relaxed mb-6">{{ ch.description }}</p>
          <a
            :href="ch.href"
            :target="ch.href.startsWith('http') ? '_blank' : undefined"
            :rel="ch.href.startsWith('http') ? 'noopener' : undefined"
            class="inline-flex items-center px-5 py-2.5 rounded-xl text-sm font-semibold text-white bg-brand-600 hover:bg-brand-700 shadow-sm transition-all"
          >
            {{ ch.cta }}
          </a>
        </div>
      </div>
    </section>

    <!-- Contributing -->
    <section class="bg-gray-50 dark:bg-gray-900 py-16">
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-4">Contributing</h2>
        <p class="text-gray-500 dark:text-gray-400 leading-relaxed">
          We welcome contributions of all kinds — code, documentation, bug reports, and feature ideas.
          Each repository has a <code class="px-1.5 py-0.5 rounded bg-gray-200 dark:bg-gray-800 text-sm">CLAUDE.md</code> file
          with project-specific guidelines. Start there, then open a PR.
        </p>
      </div>
    </section>
  </div>
</template>
