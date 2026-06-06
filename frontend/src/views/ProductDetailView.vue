<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{ slug: string }>()

interface ProductDetail {
  title: string
  tagline: string
  icon: string
  description: string
  features: { title: string; description: string }[]
  techStack: string[]
  links: { label: string; href: string }[]
}

const products: Record<string, ProductDetail> = {
  attaseek: {
    title: 'AttaSeek',
    tagline: 'Desktop AI Agent Workstation',
    icon: '',
    description:
      'AttaSeek is the flagship desktop application for working with AI agents. It provides a rich graphical interface to inspect, review, and guide agents through complex tasks — all running on your own hardware.',
    features: [
      { title: 'Multi-Panel Workspace', description: 'Drag-and-drop panels for conversation, artifacts, terminal, diff viewer, and file browser. Customize your layout and save workspaces.' },
      { title: 'Agent Conversation View', description: 'Full agent conversation rendering — streaming messages, tool call cards with expand/collapse, inline permission approvals, and model selector.' },
      { title: 'Artifact Inspection', description: 'Rich renderers for code, diffs, HTML, Markdown, SVGs, and tables. Inspect every artifact an agent produces.' },
      { title: 'Integrated Terminal', description: 'Built-in xterm.js terminal with full PTY support. Run commands alongside your agent conversations.' },
      { title: 'Plugin System', description: 'MCP protocol support — connect agents to external tools and data sources through a standardized interface.' },
      { title: 'Cross-Platform', description: 'Native experience on macOS, Windows, and Linux. Window controls blend seamlessly with OS chrome.' },
    ],
    techStack: ['Electron', 'React 18', 'TypeScript', 'Tailwind CSS', 'Jotai', 'SQLite', 'Monaco Editor', 'xterm.js', 'MCP Protocol'],
    links: [
      { label: 'GitHub', href: 'https://github.com/openatta/AttaSeek' },
      { label: 'Documentation', href: '/docs' },
    ],
  },
  attacode: {
    title: 'AttaCode',
    tagline: 'Rust-Native CLI Agent Engine',
    icon: '⚡‍️',
    description:
      'AttaCode is a high-performance rewrite of the Claude Code engine in Rust. It is not a fork or derivative — it is an independent implementation with its own architecture, designed for terminal power users and tool builders.',
    features: [
      { title: '42+ Built-in Tools', description: 'Bash, FileRead/Write/Edit, Glob, Grep, WebFetch, WebSearch, Agent, TodoWrite, LSP integration, NotebookEdit, and dozens more.' },
      { title: 'Multi-Agent Teams', description: 'Orchestrate up to 16 agents with a multi-stage pipeline. Agents communicate via mailbox and share scratchpad files.' },
      { title: 'Full TUI', description: 'Ratatui-based terminal interface with 7+ screens: transcript, inline input, todo panel, agent sidebar, slash completion, permission dialogs, Vim mode.' },
      { title: 'Plugin + Skill System', description: 'TOML-based plugin manifests. 10 built-in skills. Load skills from disk or plugins. Hooks for 9 lifecycle events.' },
      { title: 'Context Compaction', description: '4 compaction strategies: snip, micro_compact, collapse_context, and full_compact — plus session-level persistent memory.' },
      { title: 'Daemon Mode', description: 'JSON-RPC 2.0 over Unix socket, TCP, and Windows named pipes. 11+ RPC methods with streaming EngineEvent frames. IDE bridge included.' },
    ],
    techStack: ['Rust', 'Tokio', 'Ratatui', 'Axum', 'rmcp', 'tiktoken-rs', 'OpenTelemetry'],
    links: [
      { label: 'GitHub', href: 'https://github.com/openatta/AttaCode' },
      { label: 'Documentation', href: '/docs' },
    ],
  },
  attago: {
    title: 'AttaGo',
    tagline: 'Mobile Remote Control for AI Agents',
    icon: '',
    description:
      'AttaGo brings AI agent control to your pocket. Pair with your desktop agents via QR code, monitor tasks in real-time, and send instructions from anywhere — all with end-to-end encryption.',
    features: [
      { title: 'QR Code Pairing', description: 'Scan a QR code from your desktop Bridge to instantly pair your phone. One-time pair tokens expire in 5 minutes for security.' },
      { title: 'Real-Time Monitoring', description: 'Watch agent task progress live. Stream conversation output and tool calls directly to your phone.' },
      { title: 'Secure by Default', description: 'WebRTC DataChannel with an additional E2E encryption layer on top of DTLS. Cloud never sees your content.' },
      { title: 'Multi-Device', description: 'Connect to multiple agents across multiple machines. Full iOS and Android support via Flutter.' },
      { title: 'Biometric Auth', description: 'Fingerprint and face unlock for quick, secure access to your agent dashboard.' },
      { title: 'Deep Links', description: 'Open AttaGo directly from notifications, QR scans, and external triggers with deep link routing.' },
    ],
    techStack: ['Flutter', 'Dart', 'WebRTC', 'Riverpod', 'SQLCipher', 'Drift'],
    links: [
      { label: 'GitHub', href: 'https://github.com/openatta/AttaGo' },
      { label: 'Documentation', href: '/docs' },
    ],
  },
  clawpod: {
    title: 'ClawPod',
    tagline: 'Desktop Agent Runtime Manager',
    icon: '',
    description:
      'ClawPod is the on-ramp to the Atta network. It installs, configures, and bridges AI agent runtimes — from OpenClaw to custom shims. Available as a Tauri desktop app or a standalone headless daemon.',
    features: [
      { title: 'Agent Runtime Management', description: 'Install, start, stop, and configure multiple AI agent runtimes. Channel abstraction supports stdio, Unix socket, HTTP, and MCP transports.' },
      { title: 'Bridge Networking', description: 'Anonymous registration with AttaCloud, KEY generation, LTT-based authentication, and PT-based pairing — all without tying Bridge identity to user accounts.' },
      { title: 'Dual Deployment', description: 'Full Tauri desktop GUI for interactive use, or compile as a standalone `attago-bridge` binary for headless servers. Same codebase, two deployment modes.' },
      { title: 'Application Management', description: 'Install, list, and uninstall Application packages onto agent runtimes. Each channel driver handles the runtime-specific package format.' },
      { title: 'Minimal Privilege', description: 'Bridge declares allowed agent connections and capabilities at startup. Application tools use a three-state permission model: Allow / Ask / Deny.' },
      { title: 'Tauri-Native Performance', description: 'Built with Tauri v2 for minimal resource footprint. Rust backend with a lightweight web frontend. Native on macOS, Windows, and Linux.' },
    ],
    techStack: ['Tauri', 'Rust', 'Tokio', 'WebRTC', 'Protobuf', 'SQLite'],
    links: [
      { label: 'GitHub', href: 'https://github.com/openatta/ClawPod' },
      { label: 'Documentation', href: '/docs' },
    ],
  },
}

const product = computed(() => products[props.slug] ?? null)
</script>

<template>
  <div v-if="product">
    <!-- Header -->
    <section class="bg-gradient-to-br from-brand-50 to-white dark:from-gray-950 dark:to-gray-900 py-16">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center gap-3 mb-4">
          <span class="text-4xl">{{ product.icon }}</span>
          <div>
            <h1 class="text-4xl font-bold text-gray-900 dark:text-white">{{ product.title }}</h1>
            <p class="text-lg text-brand-600 dark:text-brand-400 font-medium">{{ product.tagline }}</p>
          </div>
        </div>
        <p class="text-lg text-gray-500 dark:text-gray-400 max-w-3xl mt-6">
          {{ product.description }}
        </p>
        <div class="flex gap-4 mt-8">
          <a
            v-for="link in product.links"
            :key="link.href"
            :href="link.href"
            :target="link.href.startsWith('http') ? '_blank' : undefined"
            :rel="link.href.startsWith('http') ? 'noopener' : undefined"
            class="inline-flex items-center px-5 py-2.5 rounded-xl text-sm font-semibold text-white bg-brand-600 hover:bg-brand-700 shadow-sm transition-all"
          >
            {{ link.label }}
          </a>
        </div>
      </div>
    </section>

    <!-- Features -->
    <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
      <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-10">Key Features</h2>
      <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-6">
        <div
          v-for="f in product.features"
          :key="f.title"
          class="p-6 rounded-2xl border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900"
        >
          <h3 class="text-base font-semibold text-gray-900 dark:text-white mb-2">{{ f.title }}</h3>
          <p class="text-sm text-gray-500 dark:text-gray-400 leading-relaxed">{{ f.description }}</p>
        </div>
      </div>
    </section>

    <!-- Tech stack -->
    <section class="bg-gray-50 dark:bg-gray-900 py-16">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">Tech Stack</h2>
        <div class="flex flex-wrap gap-3">
          <span
            v-for="tech in product.techStack"
            :key="tech"
            class="inline-block px-3.5 py-1.5 rounded-lg text-sm font-medium bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-700 dark:text-gray-300"
          >
            {{ tech }}
          </span>
        </div>
      </div>
    </section>

    <!-- Back link -->
    <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
      <a href="/products" class="inline-flex items-center text-sm font-medium text-brand-600 dark:text-brand-400 hover:text-brand-700 dark:hover:text-brand-300 transition-colors">
        <svg class="mr-1 w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
        </svg>
        All Products
      </a>
    </section>
  </div>

  <!-- Not found -->
  <div v-else class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-32 text-center">
    <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">Product Not Found</h1>
    <p class="text-gray-500 dark:text-gray-400 mb-8">The product "{{ slug }}" does not exist.</p>
    <a href="/products" class="text-brand-600 dark:text-brand-400 hover:underline font-medium">View all products</a>
  </div>
</template>
