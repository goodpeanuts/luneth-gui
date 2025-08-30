<template>
  <div class="record-list-view">
    <div class="list-header">
      <h2 class="list-title">Records</h2>
      <button class="refresh-btn" @click="refreshData" :disabled="paginationState.isLoading">
        <span v-if="paginationState.isLoading">Loading...</span>
        <span v-else>🔄 Refresh</span>
      </button>
    </div>

    <!-- Search and Filter Controls -->
    <div class="filter-section">
      <div class="search-and-filters">
        <!-- Search Box -->
        <div class="search-container">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Search records..."
            class="search-input"
            @keyup.enter="applyFiltersAndSearch"
          />
          <button class="search-btn" @click="applyFiltersAndSearch">
            🔍
          </button>
        </div>

        <!-- Filter Buttons -->
        <div class="filter-buttons">
          <button
            class="filter-btn"
            :class="{ active: paginationState.filters.isLiked === true }"
            @click="toggleFilter('isLiked', true)"
          >
            <span class="filter-icon">❤️</span>
            Liked
          </button>
          <button
            class="filter-btn"
            :class="{ active: paginationState.filters.isViewed === true }"
            @click="toggleFilter('isViewed', true)"
          >
            <span class="filter-icon">👁️</span>
            Viewed
          </button>
          <button
            class="filter-btn"
            :class="{ active: paginationState.filters.isSubmitted === true }"
            @click="toggleFilter('isSubmitted', true)"
          >
            <span class="filter-icon">📤</span>
            Submitted
          </button>
          <button
            class="filter-btn"
            :class="{ active: paginationState.filters.hasLocalImages === true }"
            @click="toggleFilter('hasLocalImages', true)"
          >
            <span class="filter-icon">🖼️</span>
            local
          </button>
          <button
            class="clear-filters-btn"
            :disabled="!hasActiveFilters"
            @click="clearAllFilters"
          >
            Clear Filters
          </button>
        </div>
      </div>

      <!-- Filter Results Info -->
      <div class="filter-results" v-if="hasActiveFilters && !paginationState.isLoading">
        <span class="filter-results-text">
          Showing {{ paginationState.totalCount }} out of {{ paginationState.totalRecordsCount }} records
          <span v-if="paginationState.searchQuery" class="search-indicator">
            - Search: "{{ paginationState.searchQuery }}"
          </span>
        </span>
      </div>
    </div>

    <div v-if="paginationState.isLoading" class="loading-state">
      <div class="loading-spinner"></div>
      <p>Loading records...</p>
    </div>

    <div v-else-if="paginationState.error" class="error-state">
      <div class="error-icon">❌</div>
      <p class="error-text">{{ paginationState.error }}</p>
      <button class="retry-btn" @click="refreshData">Retry</button>
    </div>

    <div v-else-if="paginationState.totalCount === 0" class="empty-state">
      <div class="empty-icon">📝</div>
      <p class="empty-text">{{ hasActiveFilters ? 'No records match the current filters' : 'No records found' }}</p>
      <p class="empty-hint" v-if="!hasActiveFilters">Records will appear here after crawling data</p>
      <button
        class="clear-filters-btn"
        :disabled="!hasActiveFilters"
        @click="clearAllFilters"
      >
        Clear Filters
      </button>
    </div>

    <div v-else class="records-container">
      <!-- Records Grid -->
      <div class="records-list">
        <div v-for="record in paginationState.records" :key="record.id" class="record-item"
          :class="{ unviewed: !record.viewed }" @click="openRecordDetail(record)">
          <div class="record-thumbnail">
            <img
              v-if="record.cover"
              :src="record.cover"
              :alt="record.title"
              class="record-cover"
              @error="handleImageError"
            />
            <div v-else class="record-cover-placeholder">📸</div>
          </div>

          <div class="record-info">
            <div class="record-header">
              <h3 class="record-title" :title="record.title">{{ record.title }}</h3>
              <div class="record-badges">
                <div class="status-indicators">
                  <div class="status-icon liked" :class="{ active: record.is_liked }"
                    @click.stop="toggleRecordLike(record)">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path
                        d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />
                    </svg>
                  </div>
                  <div class="status-icon submitted" :class="{ active: record.is_submitted }">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M4 12l6 6L20 6" />
                    </svg>
                  </div>
                </div>
              </div>
            </div>

            <div class="record-meta">
              <div class="meta-item">
                <span class="meta-label">Date:</span>
                <span class="meta-value">{{ formatDate(record.release_date) }}</span>
              </div>
              <div class="meta-item">
                <span class="meta-label">Length:</span>
                <span class="meta-value">{{ record.length }}</span>
              </div>
              <div class="meta-item">
                <span class="meta-label">Images:</span>
                <span class="meta-value">{{ record.local_image_count }}</span>
              </div>
            </div>
          </div>

          <div class="record-arrow">→</div>
        </div>
      </div>

      <!-- Pagination Component -->
      <Pagination
        :current-page="paginationState.currentPage"
        :total-pages="paginationState.totalPages"
        :total-count="paginationState.totalCount"
        :page-size="paginationState.pageSize"
        :page-range="getPageRange()"
        @prev-page="prevPage"
        @next-page="nextPage"
        @go-to-page="goToPage"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import type { RecordModel, RecordFilterOptions } from '@/types';
import { navigateTo, appState, markRecordLiked, markRecordUnliked } from '@/store';
import {
  paginationState,
  initializePagination,
  fetchCurrentPageRecords,
  goToPage,
  nextPage,
  prevPage,
  setSearchQuery,
  clearFilters,
  applyFiltersAndReload,
  applyFilterImmediately,
  getPageRange,
} from '@/store/pagination';
import Pagination from '@/components/Pagination.vue';

const searchQuery = ref('');

// 检查是否有激活的过滤器
const hasActiveFilters = computed(() => {
  return (
    paginationState.filters.isLiked !== null ||
    paginationState.filters.isViewed !== null ||
    paginationState.filters.isSubmitted !== null ||
    paginationState.filters.hasLocalImages !== null ||
    paginationState.searchQuery.trim() !== ''
  );
});

// 监听搜索查询的变化，同步到分页状态
watch(searchQuery, (newValue) => {
  setSearchQuery(newValue);
});

// 初始化时从分页状态恢复搜索查询
onMounted(async () => {
  console.log('[RecordListView] Component mounted');
  searchQuery.value = paginationState.searchQuery;

  // 初始化分页状态
  initializePagination();

  // 加载第一页数据
  console.log('[RecordListView] Loading first page...');
  await fetchCurrentPageRecords();
});

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
    } else {
      await markRecordLiked(record.id);
    }
  } catch (error) {
    console.error('Failed to toggle like status:', error);
  }
}

function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement;
  img.style.display = 'none';
}

// 筛选相关函数
async function toggleFilter(filterKey: keyof RecordFilterOptions, value: boolean) {
  const currentValue = paginationState.filters[filterKey];
  const newValue = currentValue === value ? null : value;

  // 立即应用过滤器
  await applyFilterImmediately(filterKey, newValue);
}

async function applyFiltersAndSearch() {
  await applyFiltersAndReload();
}

async function clearAllFilters() {
  searchQuery.value = '';
  setSearchQuery('');
  clearFilters();
  await applyFiltersAndReload();
}

async function refreshData() {
  await fetchCurrentPageRecords();
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

/* 搜索和筛选容器 */
.search-and-filters {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
  margin-bottom: 12px;
}

/* 搜索容器样式 */
.search-container {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 0 0 auto;
}

.search-input {
  width: 300px;
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

.search-btn {
  padding: 8px 12px;
  background: #007bff;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: background-color 0.2s ease;
}

.search-btn:hover {
  background: #0056b3;
}

/* 过滤按钮样式 */
.filter-buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
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
  flex: 1;
  margin-bottom: 16px;
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
  .search-and-filters {
    flex-direction: column;
    align-items: stretch;
    gap: 12px;
  }

  .search-container {
    flex-direction: column;
  }

  .search-input {
    width: 100%;
  }

  .filter-buttons {
    justify-content: center;
  }

  .records-list {
    grid-template-columns: 1fr;
  }

  .record-item {
    flex-direction: column;
    gap: 12px;
  }

  .record-header {
    flex-direction: column;
    gap: 8px;
  }

  .record-title {
    white-space: normal;
  }
}
</style>
