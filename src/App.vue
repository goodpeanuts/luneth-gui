<template>
  <div id="app">
    <div class="app-layout">
      <!-- Sidebar -->
      <nav class="sidebar">
        <div class="logo-section">
          <img src="/tauri.svg" alt="Luneth" class="logo" />
          <h1>Luneth</h1>
        </div>

        <ul class="nav-menu">
          <li>
            <button
              :class="{ active: appState.currentView === 'home' }"
              @click="navigateTo('home')"
            >
              🏠 Home
            </button>
          </li>
          <li>
            <button
              :class="{ active: appState.currentView === 'record_list' }"
              @click="navigateTo('record_list')"
            >
              📝 Records
            </button>
          </li>
          <li>
            <button
              :class="{ active: appState.currentView === 'history_list' }"
              @click="navigateTo('history_list')"
            >
              📜 History
            </button>
          </li>
          <li>
            <button
              :class="{ active: appState.currentView === 'crawl' }"
              @click="navigateTo('crawl')"
            >
              🕷️ Crawl
            </button>
          </li>
          <li>
            <button
              :class="{ active: appState.currentView === 'manage' }"
              @click="navigateTo('manage')"
            >
              🔧 Manage
            </button>
          </li>
          <li>
            <button
              :class="{ active: appState.currentView === 'config' }"
              @click="navigateTo('config')"
            >
              ⚙️ Config
            </button>
          </li>
        </ul>
      </nav>

      <!-- Main Content -->
      <main class="main-content">
        <component :is="currentViewComponent" />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue';
import { appState, navigateTo, initializeGlobalEventListeners, cleanupEventListeners } from '@/store';

// Components
import HomeView from '@/views/HomeView.vue';
import ConfigView from '@/views/ConfigView.vue';
import RecordListView from '@/views/RecordListView.vue';
import HistoryListView from '@/views/HistoryListView.vue';
import CrawlView from '@/views/CrawlView.vue';
import ManageView from '@/views/ManageView.vue';
import RecordDetailView from '@/views/RecordDetailView.vue';

// View component mapping
const viewComponents = {
  home: HomeView,
  config: ConfigView,
  record_list: RecordListView,
  history_list: HistoryListView,
  crawl: CrawlView,
  manage: ManageView,
  record_detail: RecordDetailView,
};

// Current view component
const currentViewComponent = computed(() => {
  return viewComponents[appState.currentView] || HomeView;
});

// 初始化全局事件监听
onMounted(async () => {
  console.log('[App] Initializing global event listeners');
  await initializeGlobalEventListeners();
});

// 清理事件监听
onUnmounted(() => {
  console.log('[App] Cleaning up global event listeners');
  cleanupEventListeners();
});
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  background-color: #f8f9fa;
}

#app {
  height: 100vh;
  overflow: hidden;
}

.app-layout {
  display: flex;
  height: 100vh;
}

.sidebar {
  width: 250px;
  background: linear-gradient(180deg, #2c3e50 0%, #34495e 100%);
  color: white;
  display: flex;
  flex-direction: column;
  padding: 20px 0;
}

.logo-section {
  display: flex;
  align-items: center;
  padding: 0 20px 30px 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  margin-bottom: 30px;
}

.logo {
  width: 32px;
  height: 32px;
  margin-right: 12px;
  filter: brightness(0) invert(1);
}

.logo-section h1 {
  font-size: 1.5rem;
  font-weight: 700;
}

.nav-menu {
  list-style: none;
  flex: 1;
}

.nav-menu li {
  margin: 0;
}

.nav-menu button {
  width: 100%;
  background: none;
  border: none;
  color: rgba(255, 255, 255, 0.8);
  padding: 16px 20px;
  text-align: left;
  cursor: pointer;
  font-size: 1rem;
  font-family: inherit;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 12px;
}

.nav-menu button:hover {
  background-color: rgba(255, 255, 255, 0.1);
  color: white;
}

.nav-menu button.active {
  background-color: rgba(255, 255, 255, 0.15);
  color: white;
  border-right: 3px solid #3498db;
}

.main-content {
  flex: 1;
  overflow-y: auto;
  background-color: #ffffff;
  height: 100vh;
  scroll-behavior: smooth; /* 添加平滑滚动 */
}

/* Scrollbar styling */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: #f1f1f1;
}

::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}

/* Common button styles */
button {
  font-family: inherit;
}

/* Link styles */
a {
  color: #007bff;
  text-decoration: none;
}

a:hover {
  text-decoration: underline;
}
</style>
