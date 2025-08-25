<template>
  <div class="sidebar">
    <!-- Logo -->
    <div class="logo-section" @click="navigateTo('home')">
      <div class="logo">
        <img src="/tauri.svg" alt="Logo" class="logo-img" />
        <span class="logo-text">Luneth</span>
      </div>
    </div>

    <!-- Navigation Menu -->
    <nav class="nav-menu">
      <div
        class="nav-item"
        :class="{ active: appState.currentView === 'record_list' }"
        @click="navigateTo('record_list')"
      >
        <div class="nav-icon">📋</div>
        <span class="nav-text">Records</span>
      </div>

      <div
        class="nav-item"
        :class="{ active: appState.currentView === 'history_list' }"
        @click="navigateTo('history_list')"
      >
        <div class="nav-icon">📜</div>
        <span class="nav-text">History</span>
      </div>

      <div
        class="nav-item"
        :class="{ active: appState.currentView === 'crawl' }"
        @click="navigateTo('crawl')"
      >
        <div class="nav-icon">🕷️</div>
        <span class="nav-text">Crawl</span>
      </div>

      <div
        class="nav-item"
        :class="{ active: appState.currentView === 'manage' }"
        @click="navigateTo('manage')"
      >
        <div class="nav-icon">🔧</div>
        <span class="nav-text">Manage</span>
      </div>

      <div
        class="nav-item"
        :class="{ active: appState.currentView === 'config' }"
        @click="navigateTo('config')"
      >
        <div class="nav-icon">⚙️</div>
        <span class="nav-text">Config</span>
      </div>
    </nav>
  </div>
</template>

<script setup lang="ts">
import { appState, navigateTo } from '@/store';
</script>

<style scoped>
.sidebar {
  width: 200px;
  height: 100vh;
  background-color: #f8f9fa;
  border-right: 1px solid #e9ecef;
  display: flex;
  flex-direction: column;
}

.logo-section {
  padding: 20px;
  border-bottom: 1px solid #e9ecef;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.logo-section:hover {
  background-color: #e9ecef;
}

.logo {
  display: flex;
  align-items: center;
  gap: 12px;
}

.logo-img {
  width: 32px;
  height: 32px;
}

.logo-text {
  font-size: 18px;
  font-weight: 600;
  color: #495057;
}

.nav-menu {
  flex: 1;
  padding: 20px 0;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 20px;
  cursor: pointer;
  transition: all 0.2s ease;
  border-left: 3px solid transparent;
}

.nav-item:hover {
  background-color: #e9ecef;
}

.nav-item.active {
  background-color: #007bff;
  color: white;
  border-left-color: #0056b3;
}

.nav-icon {
  font-size: 20px;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.nav-text {
  font-size: 14px;
  font-weight: 500;
}

.nav-item.active .nav-text {
  color: white;
}
</style>
