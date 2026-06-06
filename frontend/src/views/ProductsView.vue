<script setup lang="ts">
import ProductCard from '@/components/ProductCard.vue'

const products = [
  {
    title: 'AttaSeek',
    description: 'The flagship desktop workstation for AI agents. Multi-panel GUI with conversation view, artifact inspection, diff viewer, integrated terminal, and drag-and-drop workspace layout. Built on Electron with React + TypeScript.',
    icon: '',
    to: '/products/attaseek',
    tags: ['Desktop', 'Electron', 'React', 'TypeScript', 'Multi-Agent'],
  },
  {
    title: 'AttaCode',
    description: 'A high-performance CLI agent engine written in Rust. Features 42+ built-in tools, multi-agent team orchestration, a plugin system, Vim-mode TUI, and JSON-RPC daemon mode for IDE integration.',
    icon: '⚡‍️',
    to: '/products/attacode',
    tags: ['CLI', 'Rust', '42+ Tools', 'TUI', 'Plugin System'],
  },
  {
    title: 'AttaGo',
    description: 'Mobile companion app for remote agent control. Scan a QR code to securely pair with your desktop agents, monitor task progress, and send instructions — all with E2E encryption over WebRTC.',
    icon: '',
    to: '/products/attago',
    tags: ['Mobile', 'Flutter', 'iOS', 'Android', 'WebRTC'],
  },
  {
    title: 'ClawPod',
    description: 'Desktop agent runtime manager. Install, configure, and bridge multiple AI agent runtimes (OpenClaw, Hermes, custom). Includes a lightweight Tauri GUI and a standalone daemon mode for headless servers.',
    icon: '',
    to: '/products/clawpod',
    tags: ['Desktop', 'Tauri', 'Rust', 'Bridge', 'Daemon'],
  },
]
</script>

<template>
  <div>
    <!-- Header -->
    <section class="bg-gradient-to-br from-brand-50 to-white dark:from-gray-950 dark:to-gray-900 py-16">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">Products</h1>
        <p class="text-lg text-gray-500 dark:text-gray-400 max-w-2xl">
          The complete AttaSeek product family — from CLI to desktop to mobile, every tool you need to build with AI agents.
        </p>
      </div>
    </section>

    <!-- Product cards -->
    <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
      <div class="grid sm:grid-cols-2 gap-8">
        <ProductCard v-for="p in products" :key="p.to" v-bind="p" />
      </div>
    </section>
  </div>
</template>
