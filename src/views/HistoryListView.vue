<template>
  <div class="history-list-view">
    <div class="list-header">
      <h2 class="list-title">Operation History</h2>
      <div class="header-controls">
        <div class="filter-controls">
          <select v-model="filterOperation" class="filter-select">
            <option value="">All Operations</option>
            <option v-for="op in uniqueOperations" :key="op" :value="op">{{ op }}</option>
          </select>
          <select v-model="filterStatus" class="filter-select">
            <option value="">All Status</option>
            <option value="SUCCESS">Success</option>
            <option value="FAILED">Failed</option>
          </select>
          <button class="clear-filter-btn" @click="clearFilters" v-if="hasActiveFilters">
            Clear Filters
          </button>
        </div>
        <button class="refresh-btn" @click="loadHistory" :disabled="isLoading">
          <span v-if="isLoading && hasCached == false">Loading...</span>
          <span v-else>🔄 Refresh</span>
        </button>
      </div>
    </div>

    <div v-if="isLoading && history.length === 0" class="loading-state">
      <div class="loading-spinner"></div>
      <p>Loading history...</p>
    </div>

    <div v-else-if="error" class="error-state">
      <div class="error-icon">❌</div>
      <p class="error-text">{{ error }}</p>
      <button class="retry-btn" @click="loadHistory">Retry</button>
    </div>

    <div v-else-if="filteredHistory.length === 0" class="empty-state">
      <div class="empty-icon">📜</div>
      <p class="empty-text">{{ hasActiveFilters ? 'No records match the current filters' : 'No operation history found'
        }}</p>
      <p class="empty-hint" v-if="!hasActiveFilters">History will appear here after performing operations</p>
      <button v-if="hasActiveFilters" class="clear-filter-btn" @click="clearFilters">
        Clear Filters
      </button>
    </div>

    <div v-else class="history-container">
      <div class="history-stats" v-if="hasActiveFilters">
        <span class="stats-text">
          Showing {{ filteredHistory.length }} of {{ history.length }} records
        </span>
      </div>

      <div class="history-list">
        <div v-for="item in filteredHistory" :key="item.id" class="history-item"
          :class="{ 'success': item.status === 'SUCCESS', 'failed': item.status === 'FAILED' }">
          <div class="history-content">
            <div class="history-left">
              <div class="operation-header">
                <div class="operation-name">{{ item.operation || 'Unknown' }}</div>
                <div class="operation-status" :class="{ 'success': item.status === 'SUCCESS', 'failed': item.status === 'FAILED' }">
                  {{ item.status || 'Unknown' }}
                </div>
              </div>
              <div class="history-info">
                <div class="recorder-id">{{ item.recorder_id || 'N/A' }}</div>
                <div class="operation-timestamp">{{ formatDateTime(item.timestamp) }}</div>
              </div>
            </div>

            <div class="history-right" v-if="item.error_message">
              <div class="error-message">
                {{ item.error_message }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { HistoryOpModel } from '@/types';
import { getCachedHistory, setCachedHistory, setHistoryLoading } from '@/store';

const history = ref<HistoryOpModel[]>([]);
const isLoading = ref(false);
const hasCached = ref(false);
const error = ref('');

// 过滤状态
const filterOperation = ref('');
const filterStatus = ref('');

onMounted(() => {
  // 首先显示缓存数据（如果有的话）
  const cachedHistory = getCachedHistory();
  if (cachedHistory.length > 0) {
    history.value = cachedHistory;
    hasCached.value = true;
  }

  // 然后加载新数据
  loadHistory();
});

async function loadHistory() {
  isLoading.value = true;
  setHistoryLoading(true);
  error.value = '';

  try {
    const result = await invoke<HistoryOpModel[]>('get_all_op_history');
    console.log('Raw history data:', result);
    // 按创建时间从新到旧排序
    const sortedHistory = result.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime());
    history.value = sortedHistory;
    console.log('Sorted history data:', history.value);
    // 更新缓存
    setCachedHistory(sortedHistory);
  } catch (err) {
    console.error('Failed to load history:', err);
    error.value = `Failed to load history: ${err}`;
  } finally {
    isLoading.value = false;
    setHistoryLoading(false);
  }
}

// 计算属性
const uniqueOperations = computed(() => {
  const operations = new Set(history.value.map(item => item.operation).filter(Boolean));
  return Array.from(operations).sort();
});

const filteredHistory = computed(() => {
  let filtered = history.value;

  if (filterOperation.value) {
    filtered = filtered.filter(item => item.operation === filterOperation.value);
  }

  if (filterStatus.value) {
    filtered = filtered.filter(item => item.status === filterStatus.value);
  }

  return filtered;
});

const hasActiveFilters = computed(() => {
  return filterOperation.value !== '' || filterStatus.value !== '';
});

// 过滤方法
function clearFilters() {
  filterOperation.value = '';
  filterStatus.value = '';
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
  max-width: 100%;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 2px solid #e9ecef;
  flex-wrap: wrap;
  gap: 16px;
}

.list-title {
  font-size: 2rem;
  font-weight: 700;
  color: #333;
  margin: 0;
}

.header-controls {
  display: flex;
  align-items: center;
  gap: 16px;
  flex-wrap: wrap;
}

.filter-controls {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.filter-select {
  padding: 8px 12px;
  border: 1px solid #ced4da;
  border-radius: 6px;
  background-color: white;
  color: #495057;
  font-size: 0.9rem;
  cursor: pointer;
  transition: border-color 0.2s ease;
  min-width: 140px;
}

.filter-select:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.25);
}

.clear-filter-btn {
  padding: 8px 16px;
  background-color: #6c757d;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: background-color 0.2s ease;
}

.clear-filter-btn:hover {
  background-color: #5a6268;
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

.loading-state,
.error-state,
.empty-state {
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
  0% {
    transform: rotate(0deg);
  }

  100% {
    transform: rotate(360deg);
  }
}

.error-icon,
.empty-icon {
  font-size: 4rem;
  opacity: 0.6;
}

.error-text,
.empty-text {
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
  max-width: 100%;
}

.history-stats {
  margin-bottom: 16px;
  padding: 8px 12px;
  background-color: #f8f9fa;
  border-radius: 6px;
  border-left: 4px solid #007bff;
}

.stats-text {
  color: #495057;
  font-size: 0.9rem;
  font-weight: 500;
}

.history-list {
  height: 100%;
  overflow-y: auto;
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(400px, 600px));
  gap: 16px;
  padding: 4px;
  max-width: 100%;
  justify-content: center;
}

/* 响应式网格 - 基于容器宽度动态显示列数 */
@media (max-width: 768px) {
  .history-list {
    grid-template-columns: 1fr;
    justify-content: stretch;
  }
}

.history-item {
  background: white;
  border: 1px solid #e9ecef;
  border-radius: 8px;
  margin-bottom: 12px;
  transition: all 0.2s ease;
  padding: 16px;
  width: 100%;
  height: auto;
  max-width: 600px;
}

.history-item:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.history-item.success {
  border-left: 4px solid #28a745;
  background-color: #f8fff9;
}

.history-item.failed {
  border-left: 4px solid #dc3545;
  background-color: #fffafa;
}

.history-content {
  display: flex;
  gap: 16px;
  align-items: flex-start;
}

.history-left {
  flex: 0 0 33%; /* left column ~1/3 */
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.operation-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  margin-bottom: 8px;
}

.operation-name {
  font-size: 1.1rem;
  font-weight: 600;
  color: #333;
  word-break: break-word;
  flex: 1;
}

.operation-status {
  display: inline-block;
  font-size: 0.9rem;
  font-weight: 500;
  padding: 4px 8px;
  border-radius: 12px;
  text-align: center;
  white-space: nowrap;
  flex-shrink: 0;
}

.operation-status.success {
  background-color: #d4edda;
  color: #155724;
}

.operation-status.failed {
  background-color: #f8d7da;
  color: #721c24;
}

.history-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.recorder-id {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 0.9rem;
  color: #495057;
  background-color: #f8f9fa;
  padding: 4px 8px;
  border-radius: 4px;
  word-break: break-all;
}

.operation-timestamp {
  font-size: 0.85rem;
  color: #6c757d;
}

.history-right {
  flex: 1 1 auto; /* right column grows */
  min-width: 0;
  max-width: 67%;
}

.error-message {
  background-color: #f8d7da;
  color: #721c24;
  padding: 12px;
  border-radius: 4px;
  font-size: 0.9rem;
  border-left: 3px solid #dc3545;
  word-break: break-word;
  line-height: 1.5;
  white-space: pre-wrap;
}

/* ensure badges don't color the whole card */
.history-item.success {
  border-left: 4px solid #28a745;
  background-color: #fff; /* keep card white, badge colors handled on .operation-status */
}

.history-item.failed {
  border-left: 4px solid #dc3545;
  background-color: #fff;
}

.operation-status.success {
  background-color: #d4edda;
  color: #155724;
}

.operation-status.failed {
  background-color: #f8d7da;
  color: #721c24;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .history-list {
    grid-template-columns: 1fr !important;
    justify-content: stretch !important;
  }

  .history-item {
    width: 100%;
    max-width: none;
  }

  .history-content {
    flex-direction: column;
    gap: 12px;
  }

  .history-right {
    max-width: none;
    min-width: auto;
  }

  .operation-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }

  .operation-status {
    align-self: flex-start;
  }

  .list-header {
    flex-direction: column;
    align-items: stretch;
  }

  .header-controls {
    flex-direction: column;
    align-items: stretch;
  }

  .filter-controls {
    flex-direction: column;
  }

  .filter-select {
    min-width: auto;
  }
}

/* 平板设备 */
@media (min-width: 769px) and (max-width: 1024px) {
  .history-list {
    grid-template-columns: repeat(auto-fill, minmax(350px, 500px));
  }
}
</style>
