<template>
  <div class="manage-view">
    <div class="manage-container">
      <h2 class="manage-title">Task Management</h2>

      <div class="task-grid">
        <!-- Idol Crawl Task -->
        <div class="task-card">
          <div class="task-header">
            <h3 class="task-name">Idol Image Crawl</h3>
            <div class="task-description">
              Crawl and download idol images
            </div>
          </div>

          <div class="task-status">
            <div class="status-container">
              <div class="status-info">
                <div class="status-line">{{ getStatusText('idolCrawl') }}</div>
                <div v-if="manageTaskState.idolCrawl.message" class="status-message">
                  {{ manageTaskState.idolCrawl.message }}
                </div>
                <div v-if="manageTaskState.idolCrawl.progress" class="status-progress">
                  Progress: {{ manageTaskState.idolCrawl.progress.processed }}/{{ manageTaskState.idolCrawl.progress.total }}
                </div>
              </div>
            </div>

            <button
              class="task-btn"
              :class="{
                disabled: !isConfigured || isAnyTaskRunning,
                running: manageTaskState.idolCrawl.status === 'running'
              }"
              @click="startIdolCrawl"
              :disabled="!isConfigured || isAnyTaskRunning"
            >
              <span v-if="manageTaskState.idolCrawl.status === 'running'">Running...</span>
              <span v-else>Start Crawl</span>
            </button>
          </div>

          <div v-if="!isConfigured" class="config-warning">
            ⚠️ Configuration required
          </div>
        </div>

        <!-- Record Pull Task -->
        <div class="task-card">
          <div class="task-header">
            <h3 class="task-name">Record Pull (Slim)</h3>
            <div class="task-description">
              Pull records with minimal data
            </div>
          </div>

          <div class="task-status">
            <div class="status-container">
              <div class="status-info">
                <div class="status-line">{{ getStatusText('recordPull') }}</div>
                <div v-if="manageTaskState.recordPull.message" class="status-message">
                  {{ manageTaskState.recordPull.message }}
                </div>
                <div v-if="manageTaskState.recordPull.progress" class="status-progress">
                  Progress: {{ manageTaskState.recordPull.progress.processed }}/{{ manageTaskState.recordPull.progress.total }}
                </div>
              </div>
            </div>

            <button
              class="task-btn"
              :class="{
                disabled: !isConfigured || isAnyTaskRunning,
                running: manageTaskState.recordPull.status === 'running'
              }"
              @click="startRecordPull"
              :disabled="!isConfigured || isAnyTaskRunning"
            >
              <span v-if="manageTaskState.recordPull.status === 'running'">Running...</span>
              <span v-else>Start Pull</span>
            </button>
          </div>

          <div v-if="!isConfigured" class="config-warning">
            ⚠️ Configuration required
          </div>
        </div>

        <!-- Submit Task -->
        <div class="task-card">
          <div class="task-header">
            <h3 class="task-name">Submit Records</h3>
            <div class="task-description">
              Submit records to server based on conditions
            </div>
          </div>

          <!-- Filter Options -->
          <div class="filter-section">
            <h4 class="filter-title">Filter Conditions:</h4>
            <div class="filter-options">
              <label class="filter-option">
                <input
                  type="checkbox"
                  v-model="submitFilters.isLiked"
                  @change="updateFilteredCount"
                />
                <span>Liked Records</span>
              </label>

              <label class="filter-option">
                <input
                  type="checkbox"
                  v-model="submitFilters.isSubmitted"
                  @change="updateFilteredCount"
                />
                <span>Already Submitted</span>
              </label>

              <label class="filter-option">
                <input
                  type="checkbox"
                  v-model="submitFilters.hasLocalImages"
                  @change="updateFilteredCount"
                />
                <span>Has Local Images</span>
              </label>
            </div>

            <div class="filter-result">
              <span v-if="filteredRecordsCount >= 0">
                {{ filteredRecordsCount }} records match the criteria
              </span>
              <span v-else class="loading-text">
                Loading records...
              </span>
            </div>
          </div>

          <div class="task-status">
            <div class="status-container">
              <div class="status-info">
                <div class="status-line">{{ getStatusText('submit') }}</div>
                <div v-if="manageTaskState.submit.message" class="status-message">
                  {{ manageTaskState.submit.message }}
                </div>
                <div v-if="manageTaskState.submit.progress" class="status-progress">
                  Progress: {{ manageTaskState.submit.progress.processed }}/{{ manageTaskState.submit.progress.total }}
                </div>
              </div>
            </div>

            <button
              class="task-btn"
              :class="{
                disabled: !isConfigured || isAnyTaskRunning || filteredRecordsCount <= 0,
                running: manageTaskState.submit.status === 'running'
              }"
              @click="startSubmitTask"
              :disabled="!isConfigured || isAnyTaskRunning || filteredRecordsCount <= 0"
            >
              <span v-if="manageTaskState.submit.status === 'running'">Submitting...</span>
              <span v-else>Submit Records ({{ filteredRecordsCount }})</span>
            </button>
          </div>

          <div v-if="!isConfigured" class="config-warning">
            ⚠️ Configuration required
          </div>
        </div>

        <!-- Update Task -->
        <div class="task-card">
          <div class="task-header">
            <h3 class="task-name">Update Records</h3>
            <div class="task-description">
              Update existing records with latest data and images
            </div>
          </div>

          <!-- Filter Options -->
          <div class="filter-section">
            <h4 class="filter-title">Filter Conditions:</h4>
            <div class="filter-options">
              <label class="filter-option">
                <input
                  type="checkbox"
                  v-model="updateFilters.isLiked"
                  @change="updateUpdateFilteredCount"
                />
                <span>Liked Records</span>
              </label>

              <label class="filter-option">
                <input
                  type="checkbox"
                  v-model="updateFilters.hasLocalImages"
                  @change="updateUpdateFilteredCount"
                />
                <span>Has Local Images</span>
              </label>

              <label class="filter-option">
                <input
                  type="checkbox"
                  v-model="updateFilters.isViewed"
                  @change="updateUpdateFilteredCount"
                />
                <span>Viewed Records</span>
              </label>
            </div>

            <div class="filter-result">
              <span v-if="updateFilteredRecordsCount >= 0">
                {{ updateFilteredRecordsCount }} records match the criteria
              </span>
              <span v-else class="loading-text">
                Loading records...
              </span>
            </div>
          </div>

          <div class="task-status">
            <div class="status-container">
              <div class="status-info">
                <div class="status-line">{{ getStatusText('update') }}</div>
                <div v-if="manageTaskState.update.message" class="status-message">
                  {{ manageTaskState.update.message }}
                </div>
                <div v-if="manageTaskState.update.progress" class="status-progress">
                  Progress: {{ manageTaskState.update.progress.processed }}/{{ manageTaskState.update.progress.total }}
                </div>
              </div>
            </div>

            <button
              class="task-btn"
              :class="{
                disabled: !isConfigured || isAnyTaskRunning || updateFilteredRecordsCount <= 0,
                running: manageTaskState.update.status === 'running'
              }"
              @click="startUpdateTask"
              :disabled="!isConfigured || isAnyTaskRunning || updateFilteredRecordsCount <= 0"
            >
              <span v-if="manageTaskState.update.status === 'running'">Updating...</span>
              <span v-else>Update Records ({{ updateFilteredRecordsCount }})</span>
            </button>
          </div>

          <div v-if="!isConfigured" class="config-warning">
            ⚠️ Configuration required
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import {
  manageTaskState,
  updateTaskStatus,
  updateTaskMessage
} from '@/store';

// 记录模型类型定义（简化版本）
interface RecordLocal {
  id: string;
  title: string;
  viewed: boolean;
  is_liked: boolean;
  is_submitted: boolean;
  is_cached_locally: boolean;
  local_image_count: number;
}

// 筛选条件
const submitFilters = ref({
  isLiked: false,
  isSubmitted: false,
  hasLocalImages: false,
});

// 更新任务筛选条件
const updateFilters = ref({
  isLiked: false,
  hasLocalImages: false,
  isViewed: false,
});

// 所有记录
const allRecords = ref<RecordLocal[]>([]);

// 筛选后的记录数量
const filteredRecordsCount = ref(-1);

// 更新任务筛选后的记录数量
const updateFilteredRecordsCount = ref(-1);

// Configuration check
const isConfigured = computed(() => {
  // Check if basic configuration is available
  // This would typically check for API keys, server URLs, etc.
  return true; // Placeholder - implement actual config check
});

const isAnyTaskRunning = computed(() => {
  return manageTaskState.idolCrawl.status === 'running' ||
         manageTaskState.recordPull.status === 'running' ||
         manageTaskState.submit.status === 'running' ||
         manageTaskState.update.status === 'running';
});

// Status text helper
const getStatusText = (taskType: 'idolCrawl' | 'recordPull' | 'submit' | 'update') => {
  const status = manageTaskState[taskType].status;
  switch (status) {
    case 'idle':
      return 'Ready to start';
    case 'running':
      return 'Running';
    case 'success':
      return 'Completed';
    case 'failed':
      return 'Failed';
    default:
      return 'Unknown';
  }
};

// 获取所有记录
const fetchAllRecords = async () => {
  try {
    allRecords.value = await invoke<RecordLocal[]>('get_all_records');
    updateFilteredCount();
    updateUpdateFilteredCount();
  } catch (error) {
    console.error('Failed to fetch records:', error);
    allRecords.value = [];
    filteredRecordsCount.value = 0;
    updateFilteredRecordsCount.value = 0;
  }
};

// 更新筛选后的记录数量
const updateFilteredCount = () => {
  if (allRecords.value.length === 0) {
    filteredRecordsCount.value = 0;
    return;
  }

  let filtered = allRecords.value;

  // 如果选择了喜欢的记录
  if (submitFilters.value.isLiked) {
    filtered = filtered.filter(record => record.is_liked);
  }

  // 如果选择了已提交的记录
  if (submitFilters.value.isSubmitted) {
    filtered = filtered.filter(record => record.is_submitted);
  }

  // 如果选择了有本地图片的记录
  if (submitFilters.value.hasLocalImages) {
    filtered = filtered.filter(record => record.is_cached_locally && record.local_image_count > 0);
  }

  // 如果没有选择任何筛选条件，返回0
  if (!submitFilters.value.isLiked && !submitFilters.value.isSubmitted && !submitFilters.value.hasLocalImages) {
    filteredRecordsCount.value = 0;
    return;
  }

  filteredRecordsCount.value = filtered.length;
};

// Task actions
const startIdolCrawl = async () => {
  if (!isConfigured.value || isAnyTaskRunning.value) return;

  try {
    updateTaskStatus('idolCrawl', 'running');
    await invoke('launch_idol_scrap_task');
  } catch (error) {
    console.error('Failed to start idol crawl:', error);
    updateTaskStatus('idolCrawl', 'failed');
    updateTaskMessage('idolCrawl', `Error: ${error}`);
  }
};

const startRecordPull = async () => {
  if (!isConfigured.value || isAnyTaskRunning.value) return;

  try {
    updateTaskStatus('recordPull', 'running');
    await invoke('launch_record_pull_task');
  } catch (error) {
    console.error('Failed to start record pull:', error);
    updateTaskStatus('recordPull', 'failed');
    updateTaskMessage('recordPull', `Error: ${error}`);
  }
};

const startSubmitTask = async () => {
  if (!isConfigured.value || isAnyTaskRunning.value || filteredRecordsCount.value <= 0) return;

  try {
    // 获取符合条件的记录ID
    let filtered = allRecords.value;

    if (submitFilters.value.isLiked) {
      filtered = filtered.filter(record => record.is_liked);
    }

    if (submitFilters.value.isSubmitted) {
      filtered = filtered.filter(record => record.is_submitted);
    }

    if (submitFilters.value.hasLocalImages) {
      filtered = filtered.filter(record => record.is_cached_locally && record.local_image_count > 0);
    }

    const recordIds = filtered.map(record => record.id);

    updateTaskStatus('submit', 'running');
    await invoke('launch_submit_task', { codes: recordIds });
  } catch (error) {
    console.error('Failed to start submit task:', error);
    updateTaskStatus('submit', 'failed');
    updateTaskMessage('submit', `Error: ${error}`);
  }
};

// 更新任务筛选后的记录数量
const updateUpdateFilteredCount = () => {
  if (allRecords.value.length === 0) {
    updateFilteredRecordsCount.value = 0;
    return;
  }

  let filtered = allRecords.value;

  // 如果选择了喜欢的记录
  if (updateFilters.value.isLiked) {
    filtered = filtered.filter(record => record.is_liked);
  }

  // 如果选择了有本地图片的记录
  if (updateFilters.value.hasLocalImages) {
    filtered = filtered.filter(record => record.is_cached_locally && record.local_image_count > 0);
  }

  // 如果选择了已查看的记录
  if (updateFilters.value.isViewed) {
    filtered = filtered.filter(record => record.viewed);
  }

  // 如果没有选择任何筛选条件，返回0
  if (!updateFilters.value.isLiked && !updateFilters.value.hasLocalImages && !updateFilters.value.isViewed) {
    updateFilteredRecordsCount.value = 0;
    return;
  }

  updateFilteredRecordsCount.value = filtered.length;
};

const startUpdateTask = async () => {
  if (!isConfigured.value || isAnyTaskRunning.value || updateFilteredRecordsCount.value <= 0) return;

  try {
    // 获取符合条件的记录ID
    let filtered = allRecords.value;

    if (updateFilters.value.isLiked) {
      filtered = filtered.filter(record => record.is_liked);
    }

    if (updateFilters.value.hasLocalImages) {
      filtered = filtered.filter(record => record.is_cached_locally && record.local_image_count > 0);
    }

    if (updateFilters.value.isViewed) {
      filtered = filtered.filter(record => record.viewed);
    }

    const recordIds = filtered.map(record => record.id);

    updateTaskStatus('update', 'running');
    await invoke('launch_update_task', { codes: recordIds });
  } catch (error) {
    console.error('Failed to start update task:', error);
    updateTaskStatus('update', 'failed');
    updateTaskMessage('update', `Error: ${error}`);
  }
};

// 初始化时获取记录
onMounted(() => {
  fetchAllRecords();
});
</script>

<style scoped>
.manage-view {
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

.manage-container {
  background: #ffffff;
  border-radius: 12px;
  padding: 2rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.manage-title {
  font-size: 2rem;
  font-weight: 600;
  color: #2c3e50;
  margin-bottom: 2rem;
  text-align: center;
}

.task-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
  gap: 2rem;
}

.task-card {
  background: #f8f9fa;
  border-radius: 8px;
  border: 1px solid #e9ecef;
  padding: 1.5rem;
  transition: all 0.3s ease;
}

.task-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transform: translateY(-2px);
}

.task-header {
  margin-bottom: 1.5rem;
}

.task-name {
  font-size: 1.25rem;
  font-weight: 600;
  color: #2c3e50;
  margin: 0 0 0.5rem 0;
}

.task-description {
  color: #6c757d;
  font-size: 0.9rem;
  line-height: 1.4;
}

.filter-section {
  background: #f8f9fa;
  border-radius: 6px;
  padding: 1rem;
  margin-bottom: 1rem;
  border: 1px solid #e9ecef;
}

.filter-title {
  font-size: 1rem;
  font-weight: 600;
  color: #495057;
  margin: 0 0 0.75rem 0;
}

.filter-options {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 0.75rem;
}

.filter-option {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  font-size: 0.9rem;
  color: #495057;
}

.filter-option input[type="checkbox"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
}

.filter-result {
  padding: 0.5rem;
  background: #ffffff;
  border-radius: 4px;
  border: 1px solid #dee2e6;
  font-size: 0.85rem;
  color: #28a745;
  font-weight: 500;
  text-align: center;
}

.loading-text {
  color: #6c757d !important;
  font-style: italic;
}

.task-status {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.status-container {
  background: #ffffff;
  border-radius: 6px;
  padding: 1rem;
  border: 1px solid #dee2e6;
  min-height: 80px;
}

.status-info {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.status-line {
  font-weight: 600;
  color: #495057;
  font-size: 0.95rem;
}

.status-message {
  color: #6c757d;
  font-size: 0.85rem;
  background: #f8f9fa;
  padding: 0.5rem;
  border-radius: 4px;
  border-left: 3px solid #007bff;
}

.status-progress {
  color: #28a745;
  font-size: 0.85rem;
  font-weight: 500;
}

.task-btn {
  width: 100%;
  padding: 0.75rem 1.5rem;
  background: #007bff;
  color: white;
  border: none;
  border-radius: 6px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: 0.95rem;
}

.task-btn:hover:not(.disabled):not([disabled]) {
  background: #0056b3;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 123, 255, 0.3);
}

.task-btn.running {
  background: #28a745;
  animation: pulse 2s infinite;
}

.task-btn.disabled,
.task-btn[disabled] {
  background: #6c757d;
  cursor: not-allowed;
  opacity: 0.6;
}

.config-warning {
  background: #fff3cd;
  color: #856404;
  padding: 0.75rem;
  border-radius: 6px;
  border: 1px solid #ffeaa7;
  font-size: 0.9rem;
  font-weight: 500;
  margin-top: 1rem;
  text-align: center;
}

@keyframes pulse {
  0% {
    box-shadow: 0 0 0 0 rgba(40, 167, 69, 0.7);
  }
  70% {
    box-shadow: 0 0 0 10px rgba(40, 167, 69, 0);
  }
  100% {
    box-shadow: 0 0 0 0 rgba(40, 167, 69, 0);
  }
}

@media (max-width: 768px) {
  .manage-view {
    padding: 1rem;
  }

  .manage-container {
    padding: 1.5rem;
  }

  .task-grid {
    grid-template-columns: 1fr;
    gap: 1.5rem;
  }

  .manage-title {
    font-size: 1.5rem;
    margin-bottom: 1.5rem;
  }
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .manage-container {
    background: #2d3748;
    color: #e2e8f0;
  }

  .task-card {
    background: #4a5568;
    border-color: #718096;
  }

  .status-container {
    background: #2d3748;
    border-color: #718096;
  }

  .task-name {
    color: #e2e8f0;
  }

  .status-line {
    color: #e2e8f0;
  }

  .status-message {
    background: #4a5568;
    color: #cbd5e0;
  }

  .manage-title {
    color: #e2e8f0;
  }
}
</style>
