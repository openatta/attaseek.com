<script setup lang="ts">
const layers = [
  {
    title: 'Atta Protocol',
    description: 'The open protocol that connects all Atta products. Protobuf-based RPC over WebRTC DataChannels with E2E encryption on top of DTLS.',
    items: ['channel.proto', 'signaling.proto', 'e2e.proto', 'Shared .proto definitions + generated code for Rust, Dart, TypeScript'],
  },
  {
    title: 'AttaCloud',
    description: 'Lightweight cloud relay — anonymous Bridge registration, STUN/TURN orchestration, and short-lived signaling. Cloud never sees agent content.',
    items: ['Anonymous Bridge registration (KEY + LTT)', 'coturn node pool with short-term credentials', 'Redis-based signaling relay and rate limiting', 'Admin console (separate auth domain)'],
  },
  {
    title: 'Bridge',
    description: 'On-host daemon that connects local agent runtimes to the Atta network. Channel abstraction supports any agent protocol.',
    items: ['Stdio, Unix socket, HTTP, and MCP channel drivers', 'Anonymous cloud registration (no account required)', 'QR code pairing with one-time Pair Tokens', 'Runs standalone or embedded in ClawPod'],
  },
  {
    title: 'E2E Security',
    description: 'Defense in depth — DTLS for transport, plus an application-layer encryption based on long-term identity keys. Cloud cannot MITM.',
    items: ['WebRTC DTLS-SRTP for transport encryption', 'App-layer E2E encryption with identity keypairs', 'SDP fingerprint verification prevents cloud MITM', 'Pair Token: one-time, 5-min TTL, never reused'],
  },
]
</script>

<template>
  <div>
    <!-- Header -->
    <section class="bg-gradient-to-br from-brand-50 to-white dark:from-gray-950 dark:to-gray-900 py-16">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">Technology</h1>
        <p class="text-lg text-gray-500 dark:text-gray-400 max-w-2xl">
          How the Atta protocol stack connects CLI engines, desktop workstations, mobile apps, and cloud relay — securely and efficiently.
        </p>
      </div>
    </section>

    <!-- Architecture diagram placeholder -->
    <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
      <div class="rounded-2xl border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900 p-8 text-center">
        <pre class="text-left text-sm text-gray-600 dark:text-gray-400 font-mono leading-relaxed inline-block">
┌──────────────────────────────────────────────────────┐
│                   AttaSeek (Desktop)                  │
│  ┌────────────┐  ┌──────────┐  ┌──────────────────┐ │
│  │ Agent View │  │ Diff     │  │ Terminal         │ │
│  │ Conversation│  │ Inspector│  │ (xterm.js)       │ │
│  └────────────┘  └──────────┘  └──────────────────┘ │
│  ┌────────────────────────────────────────────────┐  │
│  │              AttaCode Engine (Rust)             │  │
│  │   42+ tools · Multi-agent · Plugin system      │  │
│  └────────────────────────────────────────────────┘  │
└──────────────────────┬───────────────────────────────┘
                       │ Atta Protocol (WebRTC + Proto)
        ┌──────────────┼──────────────┐
        │              │              │
   ┌────┴────┐   ┌────┴────┐   ┌────┴────┐
   │ AttaGo  │   │ AttaCloud│   │ ClawPod │
   │ Mobile  │   │ Relay    │   │ Desktop │
   └─────────┘   └─────────┘   └─────────┘</pre>
      </div>
    </section>

    <!-- Layers -->
    <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pb-20">
      <div class="space-y-12">
        <div v-for="layer in layers" :key="layer.title">
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-3">{{ layer.title }}</h2>
          <p class="text-gray-500 dark:text-gray-400 mb-4">{{ layer.description }}</p>
          <ul class="space-y-2">
            <li v-for="item in layer.items" :key="item" class="flex items-start gap-3 text-sm text-gray-600 dark:text-gray-400">
              <svg class="w-5 h-5 text-brand-500 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
              {{ item }}
            </li>
          </ul>
        </div>
      </div>
    </section>
  </div>
</template>
