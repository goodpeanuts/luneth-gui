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
          @click="navigateTo('task')"
        >
          Manage Tasks
        </button>
      </div>

      <!-- Pull Records Section -->
      <div class="pull-records-section">
        <div class="section-title">Sync Remote Records</div>
        <div class="pull-records-container">
          <button
            class="pull-btn"
            :class="{
              'loading': appState.pullRecordsState.isLoading,
              'disabled': appState.pullRecordsState.isLoading
            }"
            :disabled="appState.pullRecordsState.isLoading"
            @click="handlePullRecords"
          >
            <span v-if="appState.pullRecordsState.isLoading">Pulling...</span>
            <span v-else>Pull Records</span>
          </button>

          <!-- Status Display -->
          <div v-if="pullStatusMessage" class="pull-status">
            <div
              v-if="appState.pullRecordsState.lastTotalCount !== null && appState.pullRecordsState.lastNewCount !== null"
              class="success-status"
            >
              {{ appState.pullRecordsState.lastSuccessTime }} 成功拉取了{{ appState.pullRecordsState.lastTotalCount }}条记录，新增{{ appState.pullRecordsState.lastNewCount }}条
            </div>

            <div
              v-if="appState.pullRecordsState.lastError"
              class="error-status"
            >
              {{ appState.pullRecordsState.lastSuccessTime }} 拉取失败：{{ appState.pullRecordsState.lastError }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import {
  navigateTo,
  appState,
  checkClientAuth,
  setPullRecordsLoading,
  setPullRecordsSuccess,
  setPullRecordsError
} from '@/store';

// 计算属性
const pullStatusMessage = computed(() => {
  return (appState.pullRecordsState.lastTotalCount !== null && appState.pullRecordsState.lastNewCount !== null) ||
         appState.pullRecordsState.lastError;
});

// 处理拉取记录
async function handlePullRecords() {
  // 检查是否设置了客户端认证
  if (!checkClientAuth()) {
    // 如果没有设置，跳转到设置页
    navigateTo('config');
    return;
  }

  // 开始拉取
  setPullRecordsLoading(true);

  try {
    const [totalCount, newCount] = await invoke('pull_record_slim') as [number, number];
    setPullRecordsSuccess(totalCount, newCount);
  } catch (error) {
    setPullRecordsError(error as string);
  }
}
</script>

<style scoped>
.home-view {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: calc(100vh - 80px);
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 20px;
}

.welcome-content {
  text-align: center;
  max-width: 600px;
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
  margin-bottom: 50px;
  flex-wrap: wrap;
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

.pull-records-section {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  padding: 30px;
  margin-top: 20px;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.section-title {
  font-size: 1.4rem;
  font-weight: 600;
  margin-bottom: 20px;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
}

.pull-records-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.pull-btn {
  padding: 12px 32px;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  background-color: #28a745;
  color: white;
  min-width: 140px;
}

.pull-btn:hover:not(.disabled) {
  background-color: #218838;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(40, 167, 69, 0.3);
}

.pull-btn.loading {
  background-color: #6c757d;
  cursor: wait;
  transform: none;
  box-shadow: none;
}

.pull-btn.disabled {
  background-color: #6c757d;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.pull-status {
  max-width: 400px;
  margin-top: 10px;
}

.success-status {
  background: rgba(40, 167, 69, 0.2);
  border: 1px solid rgba(40, 167, 69, 0.3);
  border-radius: 6px;
  padding: 12px;
  font-size: 14px;
  line-height: 1.4;
  text-align: center;
}

.error-status {
  background: rgba(220, 53, 69, 0.2);
  border: 1px solid rgba(220, 53, 69, 0.3);
  border-radius: 6px;
  padding: 12px;
  font-size: 14px;
  text-align: center;
}

@media (max-width: 600px) {
  .welcome-title {
    font-size: 2.5rem;
  }

  .welcome-content {
    padding: 20px;
  }

  .quick-actions {
    flex-direction: column;
    align-items: center;
  }

  .action-btn {
    min-width: 200px;
  }
}
</style>
