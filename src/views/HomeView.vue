<template>
  <div class="home-view">
    <div class="welcome-content">
      <div class="welcome-logo">
        <img src="/tauri.svg" alt="Luneth Logo" class="large-logo" />
      </div>
      <h1 class="welcome-title">Welcome to Luneth</h1>
      <p class="welcome-description">
        A powerful web scraping and record management tool built with Tauri and Vue.
      </p>
      <div class="quick-actions">
        <button
          class="action-btn primary"
          @click="navigateTo('record_list')"
        >
          View Records
        </button>
        <button
          class="action-btn secondary"
          @click="navigateTo('crawl')"
        >
          Start Crawling
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { navigateTo } from '@/store';
</script>

<style scoped>
.home-view {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.welcome-content {
  text-align: center;
  max-width: 500px;
  padding: 40px;
}

.welcome-logo {
  margin-bottom: 30px;
}

.large-logo {
  width: 120px;
  height: 120px;
  filter: brightness(0) invert(1);
}

.welcome-title {
  font-size: 3rem;
  font-weight: 700;
  margin-bottom: 20px;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

.welcome-description {
  font-size: 1.2rem;
  margin-bottom: 40px;
  opacity: 0.9;
  line-height: 1.6;
}

.quick-actions {
  display: flex;
  gap: 20px;
  justify-content: center;
}

.action-btn {
  padding: 12px 24px;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  min-width: 140px;
}

.action-btn.primary {
  background-color: #007bff;
  color: white;
}

.action-btn.primary:hover {
  background-color: #0056b3;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 123, 255, 0.3);
}

.action-btn.secondary {
  background-color: transparent;
  color: white;
  border: 2px solid white;
}

.action-btn.secondary:hover {
  background-color: white;
  color: #667eea;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(255, 255, 255, 0.2);
}
</style>
