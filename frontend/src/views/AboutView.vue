<script setup lang="ts">
</script>

<template>
  <div>
    <!-- Header -->
    <section class="bg-gradient-to-br from-brand-50 to-white dark:from-gray-950 dark:to-gray-900 py-16">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">About</h1>
        <p class="text-lg text-gray-500 dark:text-gray-400 max-w-2xl">
          We believe AI agents should run on your hardware, under your control.
        </p>
      </div>
    </section>

    <!-- Mission -->
    <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
      <div class="max-w-3xl">
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-4">Our Mission</h2>
        <p class="text-gray-500 dark:text-gray-400 leading-relaxed mb-6">
          AttaSeek exists to give developers full sovereignty over their AI toolchain.
          We build open-source tools that let you run, inspect, and guide AI agents on your own machines —
          no cloud dependency, no vendor lock-in, no surveillance.
        </p>
        <p class="text-gray-500 dark:text-gray-400 leading-relaxed mb-6">
          The name "Atta" comes from the leafcutter ant genus — industrious, collaborative, and capable of
          carrying many times its own weight. That is the spirit we bring to AI agent tooling.
        </p>
      </div>
    </section>

    <!-- Products recap -->
    <section class="bg-gray-50 dark:bg-gray-900 py-16">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-8">The Atta Product Family</h2>
        <div class="grid sm:grid-cols-2 lg:grid-cols-4 gap-6">
          <div class="p-6 rounded-2xl bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">AttaSeek</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Desktop AI Agent Workstation — the flagship GUI for inspecting and guiding agents.</p>
          </div>
          <div class="p-6 rounded-2xl bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">AttaCode</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Rust-native CLI engine — 42+ tools, multi-agent teams, full TUI.</p>
          </div>
          <div class="p-6 rounded-2xl bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">AttaGo</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Mobile remote control — secure pairing, real-time monitoring, iOS + Android.</p>
          </div>
          <div class="p-6 rounded-2xl bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">ClawPod</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Desktop agent runtime manager — install, configure, bridge AI runtimes.</p>
          </div>
        </div>
      </div>
    </section>

    <!-- Contact -->
    <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
      <div class="max-w-xl">
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-4">Contact Us</h2>
        <p class="text-gray-500 dark:text-gray-400 mb-8">
          Have questions, feedback, or want to collaborate? Reach out and we will get back to you.
        </p>

        <form class="space-y-5" @submit.prevent>
          <div>
            <label for="name" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">Name</label>
            <input
              id="name"
              type="text"
              required
              class="w-full px-4 py-2.5 rounded-xl border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-900 dark:text-white text-sm focus:ring-2 focus:ring-brand-500 focus:border-transparent outline-none transition-shadow"
              placeholder="Your name"
            />
          </div>
          <div>
            <label for="email" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">Email</label>
            <input
              id="email"
              type="email"
              required
              class="w-full px-4 py-2.5 rounded-xl border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-900 dark:text-white text-sm focus:ring-2 focus:ring-brand-500 focus:border-transparent outline-none transition-shadow"
              placeholder="you@example.com"
            />
          </div>
          <div>
            <label for="subject" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">Subject</label>
            <input
              id="subject"
              type="text"
              required
              class="w-full px-4 py-2.5 rounded-xl border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-900 dark:text-white text-sm focus:ring-2 focus:ring-brand-500 focus:border-transparent outline-none transition-shadow"
              placeholder="What is this about?"
            />
          </div>
          <div>
            <label for="message" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">Message</label>
            <textarea
              id="message"
              rows="4"
              required
              class="w-full px-4 py-2.5 rounded-xl border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-900 dark:text-white text-sm focus:ring-2 focus:ring-brand-500 focus:border-transparent outline-none transition-shadow resize-none"
              placeholder="Your message..."
            ></textarea>
          </div>
          <button
            type="submit"
            class="inline-flex items-center px-6 py-3 rounded-xl text-base font-semibold text-white bg-brand-600 hover:bg-brand-700 shadow-lg shadow-brand-500/25 transition-all"
          >
            Send Message
          </button>
        </form>
      </div>
    </section>
  </div>
</template>
