<template>
  <div class="crawl-view">
    <div class="crawl-container">
      <h2 class="crawl-title">Web Scraping</h2>
      <p class="crawl-description">
        Choose a scraping mode and configure options to start crawling.
      </p>

      <!-- Task Type Buttons -->
      <div class="task-buttons-section">
        <h3 class="section-title">Scraping Tasks</h3>
        <div class="task-buttons">
          <button
            class="task-btn"
            :class="{ active: activeTask === 'auto', disabled: isProcessing }"
            @click="selectTask('auto')"
            :disabled="isProcessing"
          >
            <div class="btn-icon">🤖</div>
            <div class="btn-text">
              <h4>Auto Scraping</h4>
              <p>Automatically crawl from a single URL</p>
            </div>
          </button>

          <button
            class="task-btn"
            :class="{ active: activeTask === 'batch', disabled: isProcessing }"
            @click="selectTask('batch')"
            :disabled="isProcessing"
          >
            <div class="btn-icon">✋</div>
            <div class="btn-text">
              <h4>Batch Scraping</h4>
              <p>Provide specific codes to crawl</p>
            </div>
          </button>

          <button
            class="task-btn"
            :class="{ active: activeTask === 'update', disabled: isProcessing }"
            @click="selectTask('update')"
            :disabled="isProcessing"
          >
            <div class="btn-icon">🔄</div>
            <div class="btn-text">
              <h4>Update Records</h4>
              <p>Update existing records with latest data</p>
            </div>
          </button>
        </div>
      </div>

      <!-- Configuration Panel -->
      <div v-if="activeTask" class="config-panel">
        <div class="config-header">
          <h3 class="section-title">Configuration for {{ getTaskTitle(activeTask) }}</h3>
        </div>

        <!-- Task-specific inputs -->
        <div v-if="activeTask === 'auto'" class="input-section">
          <label for="start-url" class="input-label">Start URL</label>
          <input
            id="start-url"
            v-model="config.startUrl"
            type="url"
            class="input-field"
            placeholder="https://example.com/page"
            :disabled="isProcessing"
          />
        </div>

        <div v-if="activeTask === 'batch'" class="input-section">
          <label for="codes-input" class="input-label">Codes to Process</label>
          <textarea
            id="codes-input"
            v-model="codesInput"
            class="input-field textarea"
            placeholder="Enter codes, one per line:&#10;CODE001&#10;CODE002&#10;CODE003"
            rows="6"
            :disabled="isProcessing"
          ></textarea>
          <div class="input-hint">
            Enter one code per line. Each line will be processed as a separate code.
          </div>
        </div>

        <div v-if="activeTask === 'update'" class="input-section">
          <label class="input-label">Update Filters</label>
          <div class="filter-options">
            <label class="filter-option">
              <input
                type="checkbox"
                v-model="updateFilters.isLiked"
                @change="updateFilteredRecordsCount"
                :disabled="isProcessing"
              />
              <span>Liked Records</span>
            </label>

            <label class="filter-option">
              <input
                type="checkbox"
                v-model="updateFilters.hasLocalImages"
                @change="updateFilteredRecordsCount"
                :disabled="isProcessing"
              />
              <span>Has Local Images</span>
            </label>

            <label class="filter-option">
              <input
                type="checkbox"
                v-model="updateFilters.isViewed"
                @change="updateFilteredRecordsCount"
                :disabled="isProcessing"
              />
              <span>Viewed Records</span>
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

        <!-- Common Configuration Options -->
        <div class="common-config">
          <h4 class="config-subtitle">Options</h4>

          <div class="config-row">
            <label class="checkbox-option">
              <input
                type="checkbox"
                v-model="config.withImage"
                :disabled="isProcessing"
              />
              <span class="checkbox-text">
                <strong>Download Images</strong>
                <small>Download cover and sample images to local storage</small>
              </span>
            </label>
          </div>

          <div class="config-row">
            <label class="checkbox-option">
              <input
                type="checkbox"
                v-model="config.headless"
                :disabled="isProcessing"
              />
              <span class="checkbox-text">
                <strong>Headless Mode</strong>
                <small>Run browser in background without UI</small>
              </span>
            </label>
          </div>

          <h4 class="config-subtitle">Advanced Settings</h4>

          <div class="config-grid">
            <div class="config-item">
              <label for="load-timeout" class="config-label">Load Timeout (seconds)</label>
              <input
                id="load-timeout"
                v-model.number="config.loadTimeout"
                type="number"
                min="10"
                max="300"
                class="config-input"
                :disabled="isProcessing"
              />
            </div>

            <div class="config-item">
              <label for="request-delay" class="config-label">Request Delay (seconds)</label>
              <input
                id="request-delay"
                v-model.number="config.requestDelay"
                type="number"
                min="0"
                max="10"
                step="0.1"
                class="config-input"
                :disabled="isProcessing"
              />
            </div>

            <div class="config-item">
              <label for="webdriver-port" class="config-label">WebDriver Port</label>
              <input
                id="webdriver-port"
                v-model.number="config.webdriverPort"
                type="number"
                min="1000"
                max="65535"
                class="config-input"
                :disabled="isProcessing"
              />
            </div>
          </div>
        </div>

        <!-- Launch Button -->
        <div class="action-section">
          <button
            class="launch-btn"
            :class="{
              'processing': isProcessing,
              'disabled': !canLaunch
            }"
            :disabled="!canLaunch"
            @click="handleLaunch"
          >
            <span v-if="isProcessing" class="processing-content">
              <div class="spinner"></div>
              Processing...
            </span>
            <span v-else>🚀 Launch {{ getTaskTitle(activeTask) }}</span>
          </button>

          <div v-if="errorMessage" class="error-message">
            {{ errorMessage }}
          </div>

          <div v-if="successMessage" class="success-message">
            {{ successMessage }}
          </div>
        </div>
      </div>

      <!-- Progress Tracker -->
      <ProgressTracker ref="progressTracker" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import {
  navigateTo,
  checkTaskBaseUrl,
  progressState,
  scrapTaskState,
  showProgress as showProgressGlobal,
  hideProgress,
  resetProgress,
  appState
} from '@/store';
import type { ScrapTaskType } from '@/types';
import ProgressTracker from '@/components/ProgressTracker.vue';

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

// 当前选中的任务类型
const activeTask = ref<'auto' | 'batch' | 'update' | null>(null);

// 输入数据
const codesInput = ref('');

// 配置选项（带默认值）
const config = ref({
  startUrl: '',
  withImage: true,
  headless: true,
  loadTimeout: 90,
  requestDelay: 1,
  webdriverPort: 9032
});

// 更新任务的筛选条件
const updateFilters = ref({
  isLiked: false,
  hasLocalImages: false,
  isViewed: false
});

// 状态
const isProcessing = ref(false);
const errorMessage = ref('');
const successMessage = ref('');
const progressTracker = ref<InstanceType<typeof ProgressTracker>>();

// 本地记录和筛选结果
const allRecords = ref<RecordLocal[]>([]);
const filteredRecordsCount = ref(-1);

// 在组件挂载时，如果有进度数据就显示进度组件
onMounted(() => {
  if (progressState.progressList.length > 0) {
    showProgressGlobal();
  }
  // 初始化配置中的startUrl为taskBaseUrl
  if (appState.taskBaseUrl) {
    config.value.startUrl = appState.taskBaseUrl;
  }
  // 如果是更新任务，加载记录
  if (activeTask.value === 'update') {
    loadAllRecords();
  }
});

// 选择任务类型
function selectTask(taskType: 'auto' | 'batch' | 'update') {
  activeTask.value = taskType;
  errorMessage.value = '';
  successMessage.value = '';

  // 根据任务类型进行初始化
  if (taskType === 'auto' && appState.taskBaseUrl) {
    config.value.startUrl = appState.taskBaseUrl;
  }

  if (taskType === 'update') {
    loadAllRecords();
  }
}

// 获取任务标题
function getTaskTitle(taskType: string): string {
  switch (taskType) {
    case 'auto': return 'Auto Scraping';
    case 'batch': return 'Batch Scraping';
    case 'update': return 'Update Records';
    default: return 'Task';
  }
}

// 加载所有记录
async function loadAllRecords() {
  try {
    const records = await invoke<RecordLocal[]>('get_all_records');
    allRecords.value = records;
    updateFilteredRecordsCount();
  } catch (error) {
    console.error('Failed to load records:', error);
    allRecords.value = [];
    filteredRecordsCount.value = 0;
  }
}

// 更新筛选的记录数量
function updateFilteredRecordsCount() {
  if (allRecords.value.length === 0) {
    filteredRecordsCount.value = 0;
    return;
  }

  let filtered = allRecords.value;

  // 应用筛选条件
  if (updateFilters.value.isLiked) {
    filtered = filtered.filter(record => record.is_liked);
  }
  if (updateFilters.value.hasLocalImages) {
    filtered = filtered.filter(record => record.is_cached_locally);
  }
  if (updateFilters.value.isViewed) {
    filtered = filtered.filter(record => record.viewed);
  }

  filteredRecordsCount.value = filtered.length;
}

// 计算属性
const canLaunch = computed(() => {
  if (isProcessing.value || !activeTask.value) return false;

  switch (activeTask.value) {
    case 'auto':
      return config.value.startUrl.trim() !== '';
    case 'batch':
      return codesInput.value.trim() !== '';
    case 'update':
      return filteredRecordsCount.value > 0;
    default:
      return false;
  }
});

// 监听任务类型变化
watch(activeTask, (newTask, oldTask) => {
  errorMessage.value = '';
  successMessage.value = '';

  // 切换到manual模式时清空进度条，auto模式保留
  if (newTask === 'batch' && oldTask !== 'batch') {
    hideProgress();
  }
});

// 监听输入变化，清空错误信息
watch([() => config.value.startUrl, codesInput, () => config.value.withImage], () => {
  errorMessage.value = '';
  successMessage.value = '';
});

// 监听筛选条件变化
watch([updateFilters], () => {
  updateFilteredRecordsCount();
}, { deep: true });

// 启动爬取任务
async function handleLaunch() {
  if (!canLaunch.value || !activeTask.value) return;

  // 检查是否设置了基础URL
  if (!checkTaskBaseUrl()) {
    errorMessage.value = 'Please set the task base URL in the configuration first.';
    setTimeout(() => {
      navigateTo('config');
    }, 2000);
    return;
  }

  isProcessing.value = true;
  errorMessage.value = '';
  successMessage.value = '';
  showProgressGlobal();

  // 设置全局任务类型
  scrapTaskState.taskType = activeTask.value as ScrapTaskType;
  scrapTaskState.isProcessing = true;

  // 获取筛选后的记录IDs（用于更新任务）
  const getFilteredRecordIds = (): string[] => {
    let filtered = allRecords.value;

    if (updateFilters.value.isLiked) {
      filtered = filtered.filter(record => record.is_liked);
    }
    if (updateFilters.value.hasLocalImages) {
      filtered = filtered.filter(record => record.is_cached_locally);
    }
    if (updateFilters.value.isViewed) {
      filtered = filtered.filter(record => record.viewed);
    }

    return filtered.map(record => record.id);
  };

  try {
    switch (activeTask.value) {
      case 'auto':
        // Manual模式才重置进度条，Auto模式保留已有进度条
        await invoke('launch_auto_scrap_task', {
          startUrl: config.value.startUrl.trim(),
          withImage: config.value.withImage,
          headless: config.value.headless,
          loadTimeout: config.value.loadTimeout,
          requestDelay: config.value.requestDelay,
          webdriverPort: config.value.webdriverPort
        });
        successMessage.value = 'Auto scraping task completed!';
        break;

      case 'batch':
        resetProgress(); // Batch模式重置进度条
        const codes = codesInput.value
          .split('\n')
          .map(line => line.trim())
          .filter(line => line !== '');

        await invoke('launch_batch_scrap_task', {
          batch: codes,
          withImage: config.value.withImage,
          headless: config.value.headless,
          loadTimeout: config.value.loadTimeout,
          requestDelay: config.value.requestDelay,
          webdriverPort: config.value.webdriverPort
        });
        successMessage.value = `Batch scraping task completed. Processed ${codes.length} codes.`;
        break;

      case 'update':
        resetProgress(); // Update模式重置进度条
        const updateCodes = getFilteredRecordIds();

        await invoke('launch_update_task', {
          batch: updateCodes,
          withImage: config.value.withImage,
          headless: config.value.headless,
          loadTimeout: config.value.loadTimeout,
          requestDelay: config.value.requestDelay,
          webdriverPort: config.value.webdriverPort
        });
        successMessage.value = `Update task completed. Processed ${updateCodes.length} records.`;
        break;
    }
  } catch (error) {
    errorMessage.value = `Task failed: ${error}`;
  } finally {
    isProcessing.value = false;
    scrapTaskState.isProcessing = false;
  }
}
</script>

<style scoped>
.crawl-view {
  padding: 40px;
  max-width: 1000px;
  margin: 0 auto;
}

.crawl-container {
  background: white;
  border-radius: 12px;
  padding: 40px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
}

.crawl-title {
  font-size: 2rem;
  font-weight: 700;
  color: #333;
  margin-bottom: 16px;
  text-align: center;
}

.crawl-description {
  color: #666;
  font-size: 1.1rem;
  line-height: 1.6;
  margin-bottom: 32px;
  text-align: center;
}

.section-title {
  font-size: 1.3rem;
  font-weight: 600;
  color: #333;
  margin-bottom: 16px;
}

/* Task Buttons */
.task-buttons-section {
  margin-bottom: 32px;
}

.task-buttons {
  display: grid;
  gap: 16px;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
}

.task-btn {
  background: white;
  border: 2px solid #e2e8f0;
  border-radius: 12px;
  padding: 20px;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 16px;
  text-align: left;
}

.task-btn:hover:not(.disabled) {
  border-color: #3b82f6;
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.15);
}

.task-btn.active {
  border-color: #3b82f6;
  background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);
  color: white;
}

.task-btn.disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-icon {
  font-size: 2rem;
  flex-shrink: 0;
}

.btn-text h4 {
  margin: 0 0 4px 0;
  font-size: 1.1rem;
  font-weight: 600;
}

.btn-text p {
  margin: 0;
  font-size: 0.9rem;
  opacity: 0.8;
}

/* Configuration Panel */
.config-panel {
  background: #f8fafc;
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 32px;
  border: 1px solid #e2e8f0;
}

.config-header {
  margin-bottom: 24px;
}

.input-section {
  margin-bottom: 24px;
}

.input-label {
  display: block;
  font-weight: 600;
  color: #374151;
  margin-bottom: 8px;
}

.input-field {
  width: 100%;
  padding: 12px 16px;
  border: 2px solid #e2e8f0;
  border-radius: 8px;
  font-size: 1rem;
  transition: border-color 0.3s ease;
}

.input-field:focus {
  outline: none;
  border-color: #3b82f6;
}

.input-field.textarea {
  resize: vertical;
  min-height: 120px;
  font-family: inherit;
}

.input-hint {
  margin-top: 8px;
  font-size: 0.875rem;
  color: #6b7280;
}

/* Filter Options */
.filter-options {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
  margin-bottom: 16px;
}

.filter-option {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  padding: 8px 12px;
  border-radius: 6px;
  transition: background-color 0.2s ease;
}

.filter-option:hover {
  background-color: #f1f5f9;
}

.filter-option input[type="checkbox"] {
  margin: 0;
}

.filter-result {
  padding: 12px;
  background: #f1f5f9;
  border-radius: 6px;
  font-size: 0.9rem;
  color: #475569;
}

.loading-text {
  color: #6b7280;
  font-style: italic;
}

/* Common Configuration */
.common-config {
  margin-top: 24px;
}

.config-subtitle {
  font-size: 1.1rem;
  font-weight: 600;
  color: #374151;
  margin: 20px 0 12px 0;
}

.config-row {
  margin-bottom: 16px;
}

.checkbox-option {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  cursor: pointer;
  padding: 12px;
  border-radius: 8px;
  transition: background-color 0.2s ease;
}

.checkbox-option:hover {
  background-color: #f1f5f9;
}

.checkbox-option input[type="checkbox"] {
  margin: 4px 0 0 0;
  flex-shrink: 0;
}

.checkbox-text {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.checkbox-text strong {
  font-weight: 600;
  color: #374151;
}

.checkbox-text small {
  color: #6b7280;
  font-size: 0.875rem;
}

.config-grid {
  display: grid;
  gap: 16px;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
}

.config-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.config-label {
  font-size: 0.9rem;
  font-weight: 500;
  color: #374151;
}

.config-input {
  padding: 8px 12px;
  border: 2px solid #e2e8f0;
  border-radius: 6px;
  font-size: 0.9rem;
  transition: border-color 0.3s ease;
}

.config-input:focus {
  outline: none;
  border-color: #3b82f6;
}

/* Action Section */
.action-section {
  text-align: center;
  margin-top: 32px;
}

.launch-btn {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
  border: none;
  border-radius: 12px;
  padding: 16px 32px;
  font-size: 1.1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  min-width: 200px;
  justify-content: center;
}

.launch-btn:hover:not(.disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(16, 185, 129, 0.3);
}

.launch-btn.processing {
  background: linear-gradient(135deg, #6b7280 0%, #4b5563 100%);
  cursor: not-allowed;
}

.launch-btn.disabled {
  background: #9ca3af;
  cursor: not-allowed;
  opacity: 0.6;
}

.processing-content {
  display: flex;
  align-items: center;
  gap: 8px;
}

.spinner {
  width: 16px;
  height: 16px;
  border: 2px solid transparent;
  border-top: 2px solid currentColor;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* Messages */
.error-message {
  background: #fef2f2;
  border: 1px solid #fecaca;
  color: #dc2626;
  padding: 12px 16px;
  border-radius: 8px;
  margin-top: 16px;
  font-size: 0.9rem;
}

.success-message {
  background: #f0fdf4;
  border: 1px solid #bbf7d0;
  color: #16a34a;
  padding: 12px 16px;
  border-radius: 8px;
  margin-top: 16px;
  font-size: 0.9rem;
}

/* Responsive design */
@media (max-width: 768px) {
  .crawl-view {
    padding: 20px;
  }

  .crawl-container {
    padding: 24px;
  }

  .task-buttons {
    grid-template-columns: 1fr;
  }

  .config-grid {
    grid-template-columns: 1fr;
  }
}
</style>
