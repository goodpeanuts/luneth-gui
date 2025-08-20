<template>
  <div class="history-list-view">
    <div class="list-header">
      <h2 class="list-title">Operation History</h2>
      <button class="refresh-btn" @click="loadHistory" :disabled="isLoading">
        <span v-if="isLoading">Loading...</span>
        <span v-else">🔄 Refresh</span>
      </button>
    </div>

    <div v-if="isLoading" class="loading-state">
      <div class="loading-spinner"></div>
      <p>Loading history...</p>
    </div>

    <div v-else-if="error" class="error-state">
      <div class="error-icon">❌</div>
      <p class="error-text">{{ error }}</p>
      <button class="retry-btn" @click="loadHistory">Retry</button>
    </div>

    <div v-else-if="history.length === 0" class="empty-state">
      <div class="empty-icon">📜</div>
      <p class="empty-text">No operation history found</p>
      <p class="empty-hint">History will appear here after performing operations</p>
    </div>

    <div v-else class="history-container">
      <div class="history-list">
        <div
          v-for="item in history"
          :key="item.id"
          class="history-item"
          :class="{ 'success': item.status === 'Success', 'failed': item.status === 'Failed' }"
        >
          <div class="history-icon">
            <span v-if="item.operation === 'Create'">➕</span>
            <span v-else-if="item.operation === 'Update'">✏️</span>
            <span v-else-if="item.operation === 'Delete'">🗑️</span>
            <span v-else>🔄</span>
          </div>

          <div class="history-info">
            <div class="history-header">
              <h3 class="history-operation">{{ item.operation }}</h3>
              <div class="history-badges">
                <span
                  class="status-badge"
                  :class="{
                    'success': item.status === 'Success',
                    'failed': item.status === 'Failed'
                  }"
                >
                  {{ item.status }}
                </span>
              </div>
            </div>

            <div class="history-meta">
              <div class="meta-item">
                <span class="meta-label">Record ID:</span>
                <span class="meta-value">{{ item.id }}</span>
              </div>
              <div class="meta-item">
                <span class="meta-label">User:</span>
                <span class="meta-value">{{ item.user || 'System' }}</span>
              </div>
              <div class="meta-item">
                <span class="meta-label">Time:</span>
                <span class="meta-value">{{ formatDateTime(item.timestamp) }}</span>
              </div>
              <div class="meta-item">
                <span class="meta-label">Created:</span>
                <span class="meta-value">{{ formatDateTime(item.created_at) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { HistoryOpModel } from '@/types';

const history = ref<HistoryOpModel[]>([]);
const isLoading = ref(false);
const error = ref('');

onMounted(() => {
  loadHistory();
});

async function loadHistory() {
  isLoading.value = true;
  error.value = '';

  try {
    const result = await invoke<HistoryOpModel[]>('get_all_op_history');
    history.value = result;
  } catch (err) {
    error.value = `Failed to load history: ${err}`;
  } finally {
    isLoading.value = false;
  }
}

function formatDateTime(dateString: string): string {
  try {
    const date = new Date(dateString);
    return date.toLocaleString();
  } catch {
    return 'Invalid date';
  }
}
</script>

<style scoped>
.history-list-view {
  padding: 24px;
  height: 100vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 2px solid #e9ecef;
}

.list-title {
  font-size: 2rem;
  font-weight: 700;
  color: #333;
  margin: 0;
}

.refresh-btn {
  padding: 8px 16px;
  background-color: #28a745;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: background-color 0.2s ease;
}

.refresh-btn:hover:not(:disabled) {
  background-color: #218838;
}

.refresh-btn:disabled {
  background-color: #6c757d;
  cursor: not-allowed;
}

.loading-state, .error-state, .empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  gap: 16px;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #f3f3f3;
  border-top: 4px solid #007bff;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.error-icon, .empty-icon {
  font-size: 4rem;
  opacity: 0.6;
}

.error-text, .empty-text {
  font-size: 1.2rem;
  color: #666;
  margin: 0;
}

.empty-hint {
  color: #999;
  margin: 0;
}

.retry-btn {
  padding: 12px 24px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1rem;
  transition: background-color 0.2s ease;
}

.retry-btn:hover {
  background-color: #0056b3;
}

.history-container {
  flex: 1;
  overflow: hidden;
}

.history-list {
  height: 100%;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.history-item {
  display: flex;
  align-items: center;
  background: white;
  border: 1px solid #e9ecef;
  border-radius: 12px;
  padding: 16px;
  gap: 16px;
  transition: all 0.2s ease;
}

.history-item:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.history-item.success {
  border-left: 4px solid #28a745;
}

.history-item.failed {
  border-left: 4px solid #dc3545;
}

.history-icon {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  background-color: #f8f9fa;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  flex-shrink: 0;
}

.history-info {
  flex: 1;
  min-width: 0;
}

.history-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.history-operation {
  font-size: 1.2rem;
  font-weight: 600;
  color: #333;
  margin: 0;
}

.history-badges {
  display: flex;
  gap: 8px;
}

.status-badge {
  padding: 4px 12px;
  border-radius: 20px;
  font-size: 0.8rem;
  font-weight: 600;
  text-transform: uppercase;
}

.status-badge.success {
  background-color: #d4edda;
  color: #155724;
}

.status-badge.failed {
  background-color: #f8d7da;
  color: #721c24;
}

.history-meta {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.9rem;
}

.meta-label {
  font-weight: 600;
  color: #666;
  min-width: 80px;
}

.meta-value {
  color: #333;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
