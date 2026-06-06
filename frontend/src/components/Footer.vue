<script setup lang="ts">
const year = new Date().getFullYear()
</script>

<template>
  <footer class="border-t border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
      <div class="grid grid-cols-2 md:grid-cols-4 gap-8">
        <!-- Products -->
        <div>
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-4">Products</h3>
          <ul class="space-y-2">
            <li><a href="/products/attaseek" class="text-sm text-gray-500 dark:text-gray-400 hover:text-brand-600 dark:hover:text-brand-400 transition-colors">AttaSeek</a></li>
            <li><a href="/products/attacode" class="text-sm text-gray-500 dark:text-gray-400 hover:text-brand-600 dark:hover:text-brand-400 transition-colors">AttaCode</a></li>
            <li><a href="/products/attago" class="text-sm text-gray-500 dark:text-gray-400 hover:text-brand-600 dark:hover:text-brand-400 transition-colors">AttaGo</a></li>
            <li><a href="/products/clawpod" class="text-sm text-gray-500 dark:text-gray-400 hover:text-brand-600 dark:hover:text-brand-400 transition-colors">ClawPod</a></li>
          </ul>
        </div>

        <!-- Resources -->
        <div>
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-4">Resources</h3>
          <ul class="space-y-2">
            <li><a href="/docs" class="text-sm text-gray-500 dark:text-gray-400 hover:text-brand-600 dark:hover:text-brand-400 transition-colors">Documentation</a></li>
            <li><a href="/technology" class="text-sm text-gray-500 dark:text-gray-400 hover:text-brand-600 dark:hover:text-brand-400 transition-colors">Technology</a></li>
            <li><a href="/community" class="text-sm text-gray-500 dark:text-gray-400 hover:text-brand-600 dark:hover:text-brand-400 transition-colors">Community</a></li>
          </ul>
        </div>

        <!-- Company -->
        <div>
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-4">Company</h3>
          <ul class="space-y-2">
            <li><a href="/about" class="text-sm text-gray-500 dark:text-gray-400 hover:text-brand-600 dark:hover:text-brand-400 transition-colors">About</a></li>
            <li><a href="https://github.com/openatta" target="_blank" rel="noopener" class="text-sm text-gray-500 dark:text-gray-400 hover:text-brand-600 dark:hover:text-brand-400 transition-colors">GitHub</a></li>
          </ul>
        </div>

        <!-- Brand -->
        <div>
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-4">AttaSeek</h3>
          <p class="text-sm text-gray-500 dark:text-gray-400 leading-relaxed">
            AI Agent Workstation — run, inspect, and guide AI agents on your own hardware.
          </p>
        </div>
      </div>

      <div class="mt-10 pt-6 border-t border-gray-200 dark:border-gray-800">
        <p class="text-center text-sm text-gray-400 dark:text-gray-500">
          &copy; {{ year }} AttaSeek. All rights reserved.
        </p>
      </div>
    </div>
  </footer>
</template>
