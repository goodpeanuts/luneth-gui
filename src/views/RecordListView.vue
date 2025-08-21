<template>
  <div class="record-list-view">
    <div class="list-header">
      <h2 class="list-title">Records</h2>
      <button class="refresh-btn" @click="loadRecords" :disabled="isLoading">
        <span v-if="isLoading">Loading...</span>
        <span v-else>🔄 Refresh</span>
      </button>
    </div>

    <div v-if="isLoading && hasCached == false" class="loading-state">
      <div class="loading-spinner"></div>
      <p>Loading records...</p>
    </div>

    <div v-else-if="error" class="error-state">
      <div class="error-icon">❌</div>
      <p class="error-text">{{ error }}</p>
      <button class="retry-btn" @click="loadRecords">Retry</button>
    </div>

    <div v-else-if="records.length === 0" class="empty-state">
      <div class="empty-icon">📝</div>
      <p class="empty-text">No records found</p>
      <p class="empty-hint">Start crawling to create records</p>
      <button class="crawl-btn" @click="navigateTo('crawl')">
        Go to Crawl
      </button>
    </div>

    <div v-else class="records-container">
      <div class="records-list">
        <div
          v-for="record in records"
          :key="record.id"
          class="record-item"
          @click="openRecordDetail(record)"
        >
          <div class="record-thumbnail">
            <img
              v-if="record.cover"
              :src="record.cover"
              :alt="record.title"
              class="record-cover"
              @error="handleImageError"
            />
            <div v-else class="record-cover-placeholder">
              <span>🎬</span>
            </div>
          </div>

          <div class="record-info">
            <div class="record-header">
              <h3 class="record-title">{{ record.title || 'Untitled' }}</h3>
              <div class="record-badges">
                <div class="status-indicators">
                  <!-- 喜欢状态 -->
                  <div class="status-icon liked" :class="{ active: record.is_liked }">
                    <svg width="20" height="20" viewBox="0 0 24 24" :fill="record.is_liked ? '#dc3545' : 'none'" stroke="#dc3545" stroke-width="2">
                      <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"></path>
                    </svg>
                  </div>
                  <!-- 提交状态 -->
                  <div class="status-icon submitted" :class="{ active: record.is_submitted }">
                    <svg width="20" height="20" viewBox="0 0 24 24" :fill="record.is_submitted ? '#007bff' : 'none'" stroke="#007bff" stroke-width="2">
                      <path d="M17.5 19H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9Z"></path>
                      <path v-if="record.is_submitted" d="m9 12 2 2 4-4"></path>
                    </svg>
                  </div>
                </div>
              </div>
            </div>

            <div class="record-meta">
              <div class="meta-item">
                <span class="meta-label">ID:</span>
                <span class="meta-value">{{ record.id }}</span>
              </div>
              <div class="meta-item">
                <span class="meta-label">Release:</span>
                <span class="meta-value">{{ record.release_date || 'Unknown' }}</span>
              </div>
              <div class="meta-item">
                <span class="meta-label">Created:</span>
                <span class="meta-value">{{ formatDate(record.created_at) }}</span>
              </div>
            </div>
          </div>

          <div class="record-arrow">
            <span>→</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { RecordModel } from '@/types';
import { navigateTo, appState, getCachedRecords, setCachedRecords, setRecordsLoading } from '@/store';

const records = ref<RecordModel[]>([]);
const isLoading = ref(false);
const hasCached = ref(false);
const error = ref('');

onMounted(() => {
  // 首先显示缓存数据（如果有的话）
  const cachedRecords = getCachedRecords();
  if (cachedRecords.length > 0) {
    hasCached.value = true;
    records.value = cachedRecords;
  }

  // 然后加载新数据
  loadRecords();
});

async function loadRecords() {
  isLoading.value = true;
  setRecordsLoading(true);
  error.value = '';

  try {
    const result = await invoke<RecordModel[]>('get_all_records');
    // 按创建时间从新到旧排序
    const sortedRecords = result.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime());
    records.value = sortedRecords;
    // 更新缓存
    setCachedRecords(sortedRecords);
  } catch (err) {
    error.value = `Failed to load records: ${err}`;
  } finally {
    isLoading.value = false;
    setRecordsLoading(false);
  }
}

function openRecordDetail(record: RecordModel) {
  // Store the selected record in app state for detail view
  appState.selectedRecord = record;
  navigateTo('record_detail');
}

function formatDate(dateString: string): string {
  try {
    return new Date(dateString).toLocaleDateString();
  } catch {
    return 'Invalid date';
  }
}

function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement;
  img.style.display = 'none';
}
</script>

<style scoped>
.record-list-view {
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

.retry-btn, .crawl-btn {
  padding: 12px 24px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1rem;
  transition: background-color 0.2s ease;
}

.retry-btn:hover, .crawl-btn:hover {
  background-color: #0056b3;
}

.records-container {
  flex: 1;
  overflow: hidden;
}

.records-list {
  height: 100%;
  overflow-y: auto;
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
  gap: 16px;
  padding: 4px; /* 为阴影留出空间 */
}

@media (max-width: 768px) {
  .records-list {
    grid-template-columns: 1fr;
  }
}

.record-item {
  display: flex;
  align-items: center;
  background: white;
  border: 1px solid #e9ecef;
  border-radius: 12px;
  padding: 16px;
  cursor: pointer;
  transition: all 0.2s ease;
  gap: 16px;
  max-width: 100%;
  min-height: 120px;
}

.record-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  border-color: #007bff;
}

.record-thumbnail {
  width: 80px;
  height: 80px;
  flex-shrink: 0;
  border-radius: 8px;
  overflow: hidden;
  background-color: #f8f9fa;
}

.record-cover {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.record-cover-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 2rem;
  color: #6c757d;
}

.record-info {
  flex: 1;
  min-width: 0;
}

.record-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 12px;
}

.record-title {
  font-size: 1.2rem;
  font-weight: 600;
  color: #333;
  margin: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 400px;
}

.record-badges {
  display: flex;
  gap: 8px;
  align-items: center;
}

.status-indicators {
  display: flex;
  gap: 8px;
  align-items: center;
}

.status-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background-color: #f8f9fa;
  transition: all 0.2s ease;
  cursor: pointer;
}

.status-icon:hover {
  background-color: #e9ecef;
  transform: scale(1.1);
}

.status-icon.liked.active {
  background-color: #ffe6e6;
}

.status-icon.submitted.active {
  background-color: #e6f3ff;
}

.status-icon svg {
  transition: all 0.2s ease;
}

.badge {
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.8rem;
  font-weight: 500;
}

.badge.liked {
  background-color: #ffe6e6;
  color: #dc3545;
}

.badge.submitted {
  background-color: #e6f7e6;
  color: #28a745;
}

.record-meta {
  display: flex;
  flex-direction: column;
  gap: 4px;
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
  min-width: 60px;
}

.meta-value {
  color: #333;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.record-arrow {
  color: #6c757d;
  font-size: 1.2rem;
  font-weight: bold;
}
</style>
