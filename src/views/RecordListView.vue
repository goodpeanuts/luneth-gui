<template>
  <div class="record-list-view">
    <div class="list-header">
      <h2 class="list-title">Records</h2>
      <button class="refresh-btn" @click="loadRecords" :disabled="isLoading">
        <span v-if="isLoading">Loading...</span>
        <span v-else>🔄 Refresh</span>
      </button>
    </div>

    <!-- Filter Controls -->
    <div class="filter-section">
      <h3 class="filter-title">Filters</h3>

      <!-- 搜索框 -->
      <div class="search-section">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Search records..."
          class="search-input"
          @input="applyFilters"
        />
      </div>

      <!-- 过滤按钮 -->
      <div class="filter-buttons">
        <button
          :class="['filter-btn', { active: filters.isLiked === true }]"
          @click="toggleFilter('isLiked', true)"
        >
          <span class="filter-icon">❤️</span>
          Liked
        </button>

        <button
          :class="['filter-btn', { active: filters.isViewed === true }]"
          @click="toggleFilter('isViewed', true)"
        >
          <span class="filter-icon">👁️</span>
          Viewed
        </button>

        <button
          :class="['filter-btn', { active: filters.hasLocalImages === true }]"
          @click="toggleFilter('hasLocalImages', true)"
        >
          <span class="filter-icon">📁</span>
          Local
        </button>

        <button
          :class="['filter-btn', { active: filters.isSubmitted === true }]"
          @click="toggleFilter('isSubmitted', true)"
        >
          <span class="filter-icon">☁️</span>
          Submit
        </button>

        <button
          class="clear-filters-btn"
          @click="clearFilters"
          :disabled="!hasActiveFilters"
        >
          Clear All
        </button>
      </div>

      <div class="filter-results">
        {{ filteredRecords.length }} of {{ allRecords.length }} records
        <span v-if="searchQuery" class="search-indicator">
          (searching: "{{ searchQuery }}")
        </span>
      </div>
    </div>

    <div v-if="isLoading && !hasCached" class="loading-state">
      <div class="loading-spinner"></div>
      <p>Loading records...</p>
    </div>

    <div v-else-if="error" class="error-state">
      <div class="error-icon">❌</div>
      <p class="error-text">{{ error }}</p>
      <button class="retry-btn" @click="loadRecords">Retry</button>
    </div>

    <div v-else-if="filteredRecords.length === 0 && allRecords.length === 0" class="empty-state">
      <div class="empty-icon">📝</div>
      <p class="empty-text">No records found</p>
      <p class="empty-hint">Start crawling to create records</p>
      <button class="crawl-btn" @click="navigateTo('task')">
        Go to Tasks
      </button>
    </div>

    <div v-else-if="filteredRecords.length === 0 && allRecords.length > 0" class="empty-state">
      <div class="empty-icon">🔍</div>
      <p class="empty-text">No records match current filters</p>
      <p class="empty-hint">Try adjusting your filter criteria</p>
      <button class="clear-filters-btn" @click="clearFilters">
        Clear Filters
      </button>
    </div>

    <div v-else class="records-container">
      <div class="records-list">
        <div
          v-for="record in filteredRecords"
          :key="record.id"
          class="record-item"
          :class="{ 'unviewed': !record.viewed }"
          @click="openRecordDetail(record)"
        >
          <div class="record-thumbnail">
            <img
              v-if="getImageSrc(record)"
              :src="getImageSrc(record)"
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
                  <div
                    class="status-icon liked"
                    :class="{ active: record.is_liked }"
                    @click.stop="toggleRecordLike(record)"
                  >
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
import { ref, computed, onMounted } from 'vue';
import type { RecordModel, RecordFilterOptions } from '@/types';
import { navigateTo, appState, getCachedRecords, setRecordsLoading, markRecordLiked, markRecordUnliked, refreshRecordList } from '@/store';
import { getCachedImage } from '@/store/cache';
import { loadDisplayImage } from '@/utils/imageLoader';

const allRecords = ref<RecordModel[]>([]);
const isLoading = ref(false);
const hasCached = ref(false);
const error = ref('');

// 搜索查询
const searchQuery = ref('');

// 筛选选项
const filters = ref<RecordFilterOptions>({
  isLiked: null,
  isViewed: null,
  isSubmitted: null,
  hasLocalImages: null,
});

// 移除本地imageSources，全部使用全局缓存

// 检查是否有激活的过滤器
const hasActiveFilters = computed(() => {
  return (
    filters.value.isLiked !== null ||
    filters.value.isViewed !== null ||
    filters.value.isSubmitted !== null ||
    filters.value.hasLocalImages !== null ||
    searchQuery.value.trim() !== ''
  );
});

// 计算筛选后的记录
const filteredRecords = computed(() => {
  return allRecords.value.filter(record => {
    // 搜索过滤
    if (searchQuery.value.trim()) {
      const query = searchQuery.value.trim().toLowerCase();
      const matchesSearch =
        record.title.toLowerCase().includes(query) ||
        record.id.toLowerCase().includes(query) ||
        record.release_date.toLowerCase().includes(query);

      if (!matchesSearch) {
        return false;
      }
    }

    // Filter by liked status
    if (filters.value.isLiked !== null) {
      if (record.is_liked !== filters.value.isLiked) {
        return false;
      }
    }

    // Filter by viewed status
    if (filters.value.isViewed !== null) {
      if (record.viewed !== filters.value.isViewed) {
        return false;
      }
    }

    // Filter by submitted status
    if (filters.value.isSubmitted !== null) {
      if (record.is_submitted !== filters.value.isSubmitted) {
        return false;
      }
    }

    // Filter by local images
    if (filters.value.hasLocalImages !== null) {
      const hasImages = record.local_image_count > 0;
      if (hasImages !== filters.value.hasLocalImages) {
        return false;
      }
    }

    return true;
  });
});

onMounted(() => {
  // 直接显示缓存数据
  const cachedRecords = getCachedRecords();
  if (cachedRecords.length > 0) {
    hasCached.value = true;
    allRecords.value = cachedRecords;
    // 对于缓存的记录，立即尝试显示已缓存的图片
    loadRecordImages(cachedRecords);
  }
  // 不在这里自动加载记录，使用全局缓存的数据
});

async function loadRecords() {
  isLoading.value = true;
  setRecordsLoading(true);
  error.value = '';

  try {
    // 使用全局刷新函数
    await refreshRecordList();

    // 获取刷新后的缓存数据
    const sortedRecords = getCachedRecords();
    allRecords.value = sortedRecords;

    // 加载图片
    await loadRecordImages(sortedRecords);
  } catch (err) {
    error.value = `Failed to load records: ${err}`;
  } finally {
    isLoading.value = false;
    setRecordsLoading(false);
  }
}

async function loadRecordImages(recordList: RecordModel[]) {
  // 对于有本地缓存的记录，尝试加载本地图片
  for (const record of recordList) {
    if (record.is_cached_locally && record.cover) {
      try {
        // loadDisplayImage内部会处理缓存
        await loadDisplayImage(record.id, record.cover);
      } catch (error) {
        console.warn(`Failed to load image for record ${record.id}:`, error);
      }
    }
  }
}

function getImageSrc(record: RecordModel): string {
  // 从全局缓存获取图片URL
  const cacheKey = `${record.id}_null`; // 封面图片的缓存key
  const cachedUrl = getCachedImage(cacheKey);
  return cachedUrl || record.cover || '';
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

// 切换喜欢状态
async function toggleRecordLike(record: RecordModel) {
  try {
    if (record.is_liked) {
      await markRecordUnliked(record.id);
      record.is_liked = false;
    } else {
      await markRecordLiked(record.id);
      record.is_liked = true;
    }
  } catch (error) {
    console.error('Failed to toggle like status:', error);
    // 可以在这里添加错误提示
  }
}

function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement;
  img.style.display = 'none';
}

// 筛选相关函数
function toggleFilter(filterKey: keyof RecordFilterOptions, value: boolean) {
  const currentValue = filters.value[filterKey];
  // 如果当前值等于要设置的值，则切换为null（取消过滤）
  // 否则设置为指定值
  filters.value[filterKey] = currentValue === value ? null : value;
  applyFilters();
}

function applyFilters() {
  // filteredRecords 是计算属性，会自动响应 filters 的变化
  console.log('Filters applied:', filters.value, 'Search:', searchQuery.value);
}

function clearFilters() {
  filters.value = {
    isLiked: null,
    isViewed: null,
    isSubmitted: null,
    hasLocalImages: null,
  };
  searchQuery.value = '';
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

/* Filter section styles */
.filter-section {
  margin-bottom: 24px;
  padding: 16px;
  background: #f8f9fa;
  border-radius: 8px;
  border: 1px solid #e9ecef;
}

.filter-title {
  font-size: 1.2rem;
  font-weight: 600;
  color: #495057;
  margin: 0 0 16px 0;
}

/* 搜索框样式 */
.search-section {
  margin-bottom: 16px;
}

.search-input {
  width: 100%;
  max-width: 400px;
  padding: 8px 12px;
  border: 2px solid #e9ecef;
  border-radius: 6px;
  font-size: 0.9rem;
  transition: border-color 0.2s ease;
}

.search-input:focus {
  outline: none;
  border-color: #007bff;
}

/* 过滤按钮样式 */
.filter-buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 12px;
}

.filter-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background: white;
  border: 2px solid #e9ecef;
  border-radius: 20px;
  cursor: pointer;
  font-size: 0.85rem;
  font-weight: 500;
  transition: all 0.2s ease;
  color: #495057;
}

.filter-btn:hover {
  border-color: #007bff;
  background: #f8f9ff;
}

.filter-btn.active {
  background: #007bff;
  border-color: #007bff;
  color: white;
}

.filter-icon {
  font-size: 1rem;
}

.clear-filters-btn {
  padding: 8px 16px;
  background: #6c757d;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.85rem;
  transition: background-color 0.2s ease;
}

.clear-filters-btn:hover:not(:disabled) {
  background: #5a6268;
}

.clear-filters-btn:disabled {
  background: #adb5bd;
  cursor: not-allowed;
}

.filter-results {
  font-size: 0.9rem;
  color: #6c757d;
  font-weight: 500;
}

.search-indicator {
  color: #007bff;
  font-style: italic;
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
  display: flex;
  flex-direction: column;
}

.records-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
  gap: 16px;
  overflow-y: auto;
  padding: 4px;
}

.record-item {
  display: flex;
  background: white;
  border: 1px solid #e9ecef;
  border-radius: 8px;
  padding: 16px;
  cursor: pointer;
  transition: all 0.2s ease;
  gap: 16px;
}

.record-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  border-color: #007bff;
}

.record-item.unviewed {
  background-color: #e3f2fd;
  border-color: #bbdefb;
}

.record-thumbnail {
  flex-shrink: 0;
  width: 80px;
  height: 120px;
  overflow: hidden;
  border-radius: 6px;
  background: #f8f9fa;
  display: flex;
  align-items: center;
  justify-content: center;
}

.record-cover {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.record-cover-placeholder {
  color: #6c757d;
  font-size: 2rem;
}

.record-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 12px;
  min-width: 0;
}

.record-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 12px;
}

.record-title {
  margin: 0;
  font-size: 1.1rem;
  font-weight: 600;
  color: #2c3e50;
  line-height: 1.3;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.record-badges {
  flex-shrink: 0;
}

.status-indicators {
  display: flex;
  gap: 8px;
  align-items: center;
}

.status-icon {
  cursor: pointer;
  transition: transform 0.2s ease;
  padding: 4px;
  border-radius: 4px;
}

.status-icon:hover {
  transform: scale(1.1);
  background: rgba(0, 0, 0, 0.05);
}

.status-icon.liked.active svg {
  fill: #dc3545;
}

.status-icon.submitted.active svg {
  fill: #007bff;
}

.record-meta {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.meta-item {
  display: flex;
  gap: 8px;
  font-size: 0.85rem;
}

.meta-label {
  font-weight: 500;
  color: #6c757d;
  min-width: 60px;
}

.meta-value {
  color: #495057;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.record-arrow {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  color: #6c757d;
  font-size: 1.2rem;
}

/* Responsive design */
@media (max-width: 768px) {
  .filter-controls {
    flex-direction: column;
    align-items: stretch;
  }

  .filter-group {
    min-width: auto;
  }

  .filter-actions {
    margin-left: 0;
    margin-top: 16px;
  }

  .records-list {
    grid-template-columns: 1fr;
  }

  .record-item {
    flex-direction: column;
    text-align: center;
  }

  .record-header {
    justify-content: center;
  }

  .record-title {
    white-space: normal;
    text-align: center;
  }
}
</style>
