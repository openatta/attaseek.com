<script setup lang="ts">
import HeroSection from '@/components/HeroSection.vue'
import ProductCard from '@/components/ProductCard.vue'

const products = [
  {
    title: 'AttaSeek',
    description: 'Desktop AI Agent Workstation. Inspect, review, and guide AI agents with a rich GUI — built on Electron, React, and TypeScript.',
    icon: '',
    to: '/products/attaseek',
    tags: ['Desktop', 'Electron', 'React', 'Agent Workbench'],
  },
  {
    title: 'AttaCode',
    description: 'A Rust-native CLI engine that reimagines Claude Code. 42+ built-in tools, multi-agent teams, plugin system, and full TUI.',
    icon: '⚡‍️',
    to: '/products/attacode',
    tags: ['CLI', 'Rust', '42+ Tools', 'TUI'],
  },
  {
    title: 'AttaGo',
    description: 'Mobile remote control for your AI agents. Connect securely from anywhere via Flutter — iOS and Android.',
    icon: '',
    to: '/products/attago',
    tags: ['Mobile', 'Flutter', 'iOS', 'Android'],
  },
  {
    title: 'ClawPod',
    description: 'Desktop agent management hub. Install, configure, and bridge AI runtimes — built on Tauri and Rust.',
    icon: '',
    to: '/products/clawpod',
    tags: ['Desktop', 'Tauri', 'Rust', 'Bridge'],
  },
]

const highlights = [
  {
    title: 'Local-First Privacy',
    description: 'Everything runs on your hardware. No cloud middleman sees your code, data, or agent conversations.',
    icon: '',
  },
  {
    title: 'Multi-Agent Teams',
    description: 'Orchestrate multiple AI agents working together — fan-out tasks, review outputs, and merge results.',
    icon: '',
  },
  {
    title: 'Cross-Device Control',
    description: 'Start a task on your desktop, monitor from your phone. Secure WebRTC + E2E encryption throughout.',
    icon: '',
  },
  {
    title: 'Open Protocol',
    description: 'The Atta protocol is open and documented. Build your own clients, tools, and integrations.',
    icon: '🔌',
  },
]
</script>

<template>
  <div>
    <!-- Hero -->
    <HeroSection
      headline="AI Agent Workstation"
      subheadline="Run, inspect, and guide AI agents on your own hardware. AttaSeek gives you full control — from CLI to desktop to mobile."
      cta-text="Explore Products"
      cta-to="/products"
      secondary-text="Read the Docs"
      secondary-to="/docs"
    />

    <!-- Products grid -->
    <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
      <div class="text-center mb-12">
        <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-4">The Atta Product Family</h2>
        <p class="text-gray-500 dark:text-gray-400 max-w-2xl mx-auto">
          Four products, one mission — make AI agents work for you, on your terms.
        </p>
      </div>
      <div class="grid sm:grid-cols-2 lg:grid-cols-4 gap-6">
        <ProductCard
          v-for="p in products"
          :key="p.to"
          v-bind="p"
        />
      </div>
    </section>

    <!-- Highlights -->
    <section class="bg-gray-50 dark:bg-gray-900 py-20">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="text-center mb-12">
          <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-4">Why AttaSeek</h2>
          <p class="text-gray-500 dark:text-gray-400 max-w-2xl mx-auto">
            Designed for developers who want full control over their AI toolchain.
          </p>
        </div>
        <div class="grid sm:grid-cols-2 lg:grid-cols-4 gap-8">
          <div v-for="h in highlights" :key="h.title" class="text-center">
            <div class="text-3xl mb-4">{{ h.icon }}</div>
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">{{ h.title }}</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400 leading-relaxed">{{ h.description }}</p>
          </div>
        </div>
      </div>
    </section>

    <!-- CTA -->
    <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20 text-center">
      <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-4">Ready to take control?</h2>
      <p class="text-gray-500 dark:text-gray-400 mb-8 max-w-xl mx-auto">
        Explore our open-source products and join the community of developers building the future of AI agents.
      </p>
      <div class="flex justify-center gap-4">
        <a href="/products" class="inline-flex items-center px-6 py-3 rounded-xl text-base font-semibold text-white bg-brand-600 hover:bg-brand-700 shadow-lg shadow-brand-500/25 transition-all">
          Get Started
        </a>
        <a href="https://github.com/openatta" target="_blank" rel="noopener" class="inline-flex items-center px-6 py-3 rounded-xl text-base font-semibold text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600 shadow-sm transition-all">
          <svg class="mr-2 w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
          GitHub
        </a>
      </div>
    </section>
  </div>
</template>
