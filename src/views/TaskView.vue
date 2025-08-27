<template>
  <div class="task-view">
    <div class="task-container">
      <h1 class="task-title">Task Management</h1>
      <p class="task-description">
        Manage all crawling and processing tasks from this unified interface
      </p>

      <!-- 任务类型选择区域 -->
      <div class="task-selection-section">
        <h2 class="section-title">Select Task Type</h2>
        <div class="task-buttons">
          <!-- 自动爬取任务 -->
          <button
            :class="['task-btn', { active: activeTask === 'auto', disabled: isAnyTaskRunning }]"
            @click="selectTask('auto')"
            :disabled="isAnyTaskRunning"
          >
            <div class="btn-icon">🤖</div>
            <div class="btn-text">
              <h4>Auto Scraping</h4>
              <p>Automatically crawl pages from the configured base URL</p>
            </div>
          </button>

          <!-- 批量爬取任务 -->
          <button
            :class="['task-btn', { active: activeTask === 'batch', disabled: isAnyTaskRunning }]"
            @click="selectTask('batch')"
            :disabled="isAnyTaskRunning"
          >
            <div class="btn-icon">📋</div>
            <div class="btn-text">
              <h4>Batch Scraping</h4>
              <p>Crawl specific records by providing their IDs</p>
            </div>
          </button>

          <!-- 更新任务 -->
          <button
            :class="['task-btn', { active: activeTask === 'update', disabled: isAnyTaskRunning }]"
            @click="selectTask('update')"
            :disabled="isAnyTaskRunning"
          >
            <div class="btn-icon">🔄</div>
            <div class="btn-text">
              <h4>Update Records</h4>
              <p>Update existing records based on filter criteria</p>
            </div>
          </button>

          <!-- Idol爬取任务 -->
          <button
            :class="['task-btn', { active: activeTask === 'idolCrawl', disabled: isAnyTaskRunning }]"
            @click="selectTask('idolCrawl')"
            :disabled="isAnyTaskRunning"
          >
            <div class="btn-icon">👤</div>
            <div class="btn-text">
              <h4>Idol Image Crawl</h4>
              <p>Crawl idol images from configured sources</p>
            </div>
          </button>

          <!-- 记录拉取任务 -->
          <button
            :class="['task-btn', { active: activeTask === 'recordPull', disabled: isAnyTaskRunning }]"
            @click="selectTask('recordPull')"
            :disabled="isAnyTaskRunning"
          >
            <div class="btn-icon">⬇️</div>
            <div class="btn-text">
              <h4>Pull Records</h4>
              <p>Pull latest records from remote server</p>
            </div>
          </button>

          <!-- 提交任务 -->
          <button
            :class="['task-btn', { active: activeTask === 'submit', disabled: isAnyTaskRunning }]"
            @click="selectTask('submit')"
            :disabled="isAnyTaskRunning"
          >
            <div class="btn-icon">📤</div>
            <div class="btn-text">
              <h4>Submit Records</h4>
              <p>Submit selected records to remote server</p>
            </div>
          </button>
        </div>
      </div>

      <!-- 配置面板 -->
      <div v-if="activeTask" class="config-panel">
        <div class="config-header">
          <h3>{{ getTaskTitle(activeTask) }} Configuration</h3>
        </div>

        <!-- 自动爬取配置 -->
        <div v-if="activeTask === 'auto'" class="auto-config">
          <div class="input-section">
            <label class="input-label">Start URL</label>
            <input
              v-model="config.startUrl"
              type="text"
              class="input-field"
              placeholder="Enter the starting URL for auto crawling"
            />
            <div class="input-hint">
              This URL will be used as the starting point for automatic crawling
            </div>
          </div>

          <div class="common-config">
            <h4 class="config-subtitle">Crawling Options</h4>
            <div class="checkbox-option">
              <input
                v-model="config.withImage"
                type="checkbox"
                id="auto-with-image"
              />
              <div class="checkbox-text">
                <label for="auto-with-image"><strong>Download Images</strong></label>
                <small>Download and cache images locally during crawling</small>
              </div>
            </div>

            <div class="checkbox-option">
              <input
                v-model="config.headless"
                type="checkbox"
                id="auto-headless"
              />
              <div class="checkbox-text">
                <label for="auto-headless"><strong>Headless Mode</strong></label>
                <small>Run browser in headless mode (no GUI)</small>
              </div>
            </div>

            <div class="config-grid">
              <div class="config-item">
                <label class="config-label">Load Timeout (seconds)</label>
                <input
                  v-model.number="config.loadTimeout"
                  type="number"
                  class="config-input"
                  min="10"
                  max="300"
                />
              </div>
              <div class="config-item">
                <label class="config-label">Request Delay (seconds)</label>
                <input
                  v-model.number="config.requestDelay"
                  type="number"
                  class="config-input"
                  min="0"
                  max="10"
                  step="0.1"
                />
              </div>
              <div class="config-item">
                <label class="config-label">WebDriver Port</label>
                <input
                  v-model.number="config.webdriverPort"
                  type="number"
                  class="config-input"
                  min="1000"
                  max="65535"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- 批量爬取配置 -->
        <div v-if="activeTask === 'batch'" class="batch-config">
          <div class="input-section">
            <label class="input-label">Record IDs</label>
            <textarea
              v-model="codesInput"
              class="input-field textarea"
              placeholder="Enter record IDs, one per line&#10;Example:&#10;ABC-123&#10;DEF-456&#10;GHI-789"
            />
            <div class="input-hint">
              Enter one record ID per line. Empty lines will be ignored.
            </div>
          </div>

          <div class="common-config">
            <h4 class="config-subtitle">Crawling Options</h4>
            <div class="checkbox-option">
              <input
                v-model="config.withImage"
                type="checkbox"
                id="batch-with-image"
              />
              <div class="checkbox-text">
                <label for="batch-with-image"><strong>Download Images</strong></label>
                <small>Download and cache images locally during crawling</small>
              </div>
            </div>

            <div class="checkbox-option">
              <input
                v-model="config.headless"
                type="checkbox"
                id="batch-headless"
              />
              <div class="checkbox-text">
                <label for="batch-headless"><strong>Headless Mode</strong></label>
                <small>Run browser in headless mode (no GUI)</small>
              </div>
            </div>

            <div class="config-grid">
              <div class="config-item">
                <label class="config-label">Load Timeout (seconds)</label>
                <input
                  v-model.number="config.loadTimeout"
                  type="number"
                  class="config-input"
                  min="10"
                  max="300"
                />
              </div>
              <div class="config-item">
                <label class="config-label">Request Delay (seconds)</label>
                <input
                  v-model.number="config.requestDelay"
                  type="number"
                  class="config-input"
                  min="0"
                  max="10"
                  step="0.1"
                />
              </div>
              <div class="config-item">
                <label class="config-label">WebDriver Port</label>
                <input
                  v-model.number="config.webdriverPort"
                  type="number"
                  class="config-input"
                  min="1000"
                  max="65535"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- 更新任务配置 -->
        <div v-if="activeTask === 'update'" class="update-config">
          <div class="filter-section">
            <h4 class="filter-title">Record Filter Options</h4>
            <div class="filter-options">
              <div class="filter-option">
                <input
                  v-model="updateFilters.isLiked"
                  type="checkbox"
                  id="update-liked"
                />
                <label for="update-liked">Liked Records</label>
              </div>
              <div class="filter-option">
                <input
                  v-model="updateFilters.hasLocalImages"
                  type="checkbox"
                  id="update-images"
                />
                <label for="update-images">Has Local Images</label>
              </div>
              <div class="filter-option">
                <input
                  v-model="updateFilters.isViewed"
                  type="checkbox"
                  id="update-viewed"
                />
                <label for="update-viewed">Viewed Records</label>
              </div>
            </div>
            <div class="filter-result">
              <span v-if="filteredRecordsCount === -1" class="loading-text">
                Loading records...
              </span>
              <span v-else>
                {{ filteredRecordsCount }} records match the current filters
              </span>
            </div>
          </div>

          <div class="common-config">
            <h4 class="config-subtitle">Update Options</h4>
            <div class="checkbox-option">
              <input
                v-model="config.withImage"
                type="checkbox"
                id="update-with-image"
              />
              <div class="checkbox-text">
                <label for="update-with-image"><strong>Download Images</strong></label>
                <small>Download and cache images locally during update</small>
              </div>
            </div>

            <div class="checkbox-option">
              <input
                v-model="config.headless"
                type="checkbox"
                id="update-headless"
              />
              <div class="checkbox-text">
                <label for="update-headless"><strong>Headless Mode</strong></label>
                <small>Run browser in headless mode (no GUI)</small>
              </div>
            </div>

            <div class="config-grid">
              <div class="config-item">
                <label class="config-label">Load Timeout (seconds)</label>
                <input
                  v-model.number="config.loadTimeout"
                  type="number"
                  class="config-input"
                  min="10"
                  max="300"
                />
              </div>
              <div class="config-item">
                <label class="config-label">Request Delay (seconds)</label>
                <input
                  v-model.number="config.requestDelay"
                  type="number"
                  class="config-input"
                  min="0"
                  max="10"
                  step="0.1"
                />
              </div>
              <div class="config-item">
                <label class="config-label">WebDriver Port</label>
                <input
                  v-model.number="config.webdriverPort"
                  type="number"
                  class="config-input"
                  min="1000"
                  max="65535"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- Idol爬取任务状态 -->
        <div v-if="activeTask === 'idolCrawl'" class="task-status-config">
          <div class="status-container">
            <div v-if="manageTaskState.idolCrawl.status !== 'idle'" class="status-info">
              <div class="status-line">
                Status: {{ getStatusText('idolCrawl') }}
              </div>
              <div v-if="manageTaskState.idolCrawl.message" class="status-message">
                {{ manageTaskState.idolCrawl.message }}
              </div>
              <div v-if="manageTaskState.idolCrawl.progress" class="status-progress">
                Progress: {{ manageTaskState.idolCrawl.progress.processed }}/{{ manageTaskState.idolCrawl.progress.total }}
              </div>
            </div>
            <div v-else class="status-info">
              <div class="status-line">Ready to start idol image crawling</div>
            </div>
          </div>
        </div>

        <!-- 记录拉取任务状态 -->
        <div v-if="activeTask === 'recordPull'" class="task-status-config">
          <div class="status-container">
            <div v-if="manageTaskState.recordPull.status !== 'idle'" class="status-info">
              <div class="status-line">
                Status: {{ getStatusText('recordPull') }}
              </div>
              <div v-if="manageTaskState.recordPull.message" class="status-message">
                {{ manageTaskState.recordPull.message }}
              </div>
              <div v-if="manageTaskState.recordPull.progress" class="status-progress">
                Progress: {{ manageTaskState.recordPull.progress.processed }}/{{ manageTaskState.recordPull.progress.total }}
              </div>
            </div>
            <div v-else class="status-info">
              <div class="status-line">Ready to start record pulling</div>
            </div>
          </div>
        </div>

        <!-- 提交任务配置 -->
        <div v-if="activeTask === 'submit'" class="submit-config">
          <div class="filter-section">
            <h4 class="filter-title">Submit Filter Options</h4>
            <div class="filter-options">
              <div class="filter-option">
                <input
                  v-model="submitFilters.isLiked"
                  type="checkbox"
                  id="submit-liked"
                />
                <label for="submit-liked">Liked Records</label>
              </div>
              <div class="filter-option">
                <input
                  v-model="submitFilters.isSubmitted"
                  type="checkbox"
                  id="submit-submitted"
                />
                <label for="submit-submitted">Already Submitted</label>
              </div>
              <div class="filter-option">
                <input
                  v-model="submitFilters.hasLocalImages"
                  type="checkbox"
                  id="submit-images"
                />
                <label for="submit-images">Has Local Images</label>
              </div>
            </div>
            <div class="filter-result">
              <span v-if="filteredSubmitRecordsCount === -1" class="loading-text">
                Loading records...
              </span>
              <span v-else>
                {{ filteredSubmitRecordsCount }} records match the current filters
              </span>
            </div>
          </div>

          <div class="task-status-config">
            <div class="status-container">
              <div v-if="manageTaskState.submit.status !== 'idle'" class="status-info">
                <div class="status-line">
                  Status: {{ getStatusText('submit') }}
                </div>
                <div v-if="manageTaskState.submit.message" class="status-message">
                  {{ manageTaskState.submit.message }}
                </div>
                <div v-if="manageTaskState.submit.progress" class="status-progress">
                  Progress: {{ manageTaskState.submit.progress.processed }}/{{ manageTaskState.submit.progress.total }}
                </div>
              </div>
              <div v-else class="status-info">
                <div class="status-line">Ready to submit records</div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 配置错误警告 -->
      <div v-if="activeTask && !isConfigured" class="config-warning">
        Please configure the required settings in the Config page before starting tasks.
      </div>

      <!-- 启动按钮 -->
      <div v-if="activeTask" class="action-section">
        <button
          :class="['launch-btn', {
            processing: isProcessing,
            disabled: !canLaunch
          }]"
          @click="handleLaunch"
          :disabled="!canLaunch"
        >
          <div v-if="isProcessing" class="processing-content">
            <div class="spinner"></div>
            <span>{{ getProcessingText() }}</span>
          </div>
          <div v-else class="launch-content">
            🚀 Start {{ getTaskTitle(activeTask) }}
          </div>
        </button>
      </div>

      <!-- 错误和成功消息 -->
      <div v-if="errorMessage" class="error-message">
        {{ errorMessage }}
      </div>
      <div v-if="successMessage" class="success-message">
        {{ successMessage }}
      </div>

      <!-- 进度跟踪器 -->
      <ProgressTracker
        v-if="progressState.isVisible"
        ref="progressTracker"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import {
  navigateTo,
  checkTaskBaseUrl,
  checkClientAuth,
  progressState,
  scrapTaskState,
  manageTaskState,
  showProgress as showProgressGlobal,
  resetProgress,
  appState,
  updateTaskStatus
} from '@/store';
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
const activeTask = ref<'auto' | 'batch' | 'update' | 'idolCrawl' | 'recordPull' | 'submit' | null>(null);

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

// 提交任务的筛选条件
const submitFilters = ref({
  isLiked: false,
  isSubmitted: false,
  hasLocalImages: false
});

// 状态
const isProcessing = ref(false);
const errorMessage = ref('');
const successMessage = ref('');
const progressTracker = ref<InstanceType<typeof ProgressTracker>>();

// 本地记录和筛选结果
const allRecords = ref<RecordLocal[]>([]);
const filteredRecordsCount = ref(-1);
const filteredSubmitRecordsCount = ref(-1);

// 在组件挂载时，如果有进度数据就显示进度组件
onMounted(() => {
  if (progressState.progressList.length > 0) {
    showProgressGlobal();
  }
  // 初始化配置中的startUrl为taskBaseUrl
  if (appState.taskBaseUrl) {
    config.value.startUrl = appState.taskBaseUrl;
  }
});

// 检查是否有任务正在运行
const isAnyTaskRunning = computed(() => {
  return Object.values(manageTaskState).some(task => task.status === 'running') || isProcessing.value;
});

// 配置检查
const isConfigured = computed(() => {
  if (activeTask.value === 'recordPull' || activeTask.value === 'submit') {
    return checkClientAuth();
  }
  return checkTaskBaseUrl();
});

// 选择任务类型
function selectTask(taskType: 'auto' | 'batch' | 'update' | 'idolCrawl' | 'recordPull' | 'submit') {
  activeTask.value = taskType;
  errorMessage.value = '';
  successMessage.value = '';

  // 根据任务类型进行初始化
  if ((taskType === 'auto' || taskType === 'batch') && appState.taskBaseUrl) {
    config.value.startUrl = appState.taskBaseUrl;
  }

  if (taskType === 'update' || taskType === 'submit') {
    loadAllRecords();
  }
}

// 获取任务标题
function getTaskTitle(taskType: string): string {
  switch (taskType) {
    case 'auto': return 'Auto Scraping';
    case 'batch': return 'Batch Scraping';
    case 'update': return 'Update Records';
    case 'idolCrawl': return 'Idol Image Crawl';
    case 'recordPull': return 'Pull Records';
    case 'submit': return 'Submit Records';
    default: return 'Task';
  }
}

// 获取正在处理时的文本
function getProcessingText(): string {
  switch (activeTask.value) {
    case 'auto': return 'Auto crawling...';
    case 'batch': return 'Batch crawling...';
    case 'update': return 'Updating records...';
    case 'idolCrawl': return 'Crawling idol images...';
    case 'recordPull': return 'Pulling records...';
    case 'submit': return 'Submitting records...';
    default: return 'Processing...';
  }
}

// 加载所有记录
async function loadAllRecords() {
  try {
    const records = await invoke<RecordLocal[]>('get_all_records');
    allRecords.value = records;
    updateFilteredRecordsCount();
    updateFilteredSubmitRecordsCount();
  } catch (error) {
    console.error('Failed to load records:', error);
    allRecords.value = [];
    filteredRecordsCount.value = 0;
    filteredSubmitRecordsCount.value = 0;
  }
}

// 更新筛选的记录数量（用于更新任务）
function updateFilteredRecordsCount() {
  if (allRecords.value.length === 0) {
    filteredRecordsCount.value = 0;
    return;
  }

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

  // 如果没有选择任何筛选条件，返回0
  if (!updateFilters.value.isLiked && !updateFilters.value.hasLocalImages && !updateFilters.value.isViewed) {
    filteredRecordsCount.value = 0;
    return;
  }

  filteredRecordsCount.value = filtered.length;
}

// 更新筛选的记录数量（用于提交任务）
function updateFilteredSubmitRecordsCount() {
  if (allRecords.value.length === 0) {
    filteredSubmitRecordsCount.value = 0;
    return;
  }

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

  // 如果没有选择任何筛选条件，返回0
  if (!submitFilters.value.isLiked && !submitFilters.value.isSubmitted && !submitFilters.value.hasLocalImages) {
    filteredSubmitRecordsCount.value = 0;
    return;
  }

  filteredSubmitRecordsCount.value = filtered.length;
}

// 计算属性
const canLaunch = computed(() => {
  if (isProcessing.value || !activeTask.value || !isConfigured.value) return false;

  switch (activeTask.value) {
    case 'auto':
      return !!config.value.startUrl.trim();
    case 'batch':
      return !!codesInput.value.trim();
    case 'update':
      return filteredRecordsCount.value > 0;
    case 'idolCrawl':
    case 'recordPull':
      return true;
    case 'submit':
      return filteredSubmitRecordsCount.value > 0;
    default:
      return false;
  }
});

// 监听任务类型变化
watch(activeTask, (newTask, oldTask) => {
  errorMessage.value = '';
  successMessage.value = '';

  // 切换到batch模式时清空进度条，auto模式保留
  if (newTask === 'batch' && oldTask !== 'batch') {
    resetProgress();
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

watch([submitFilters], () => {
  updateFilteredSubmitRecordsCount();
}, { deep: true });

// 监听来自 extract 页面的输入内容
watch(() => scrapTaskState.inputValue, (newValue) => {
  if (newValue) {
    // 如果当前已经是 batch 任务，直接更新输入框
    if (activeTask.value === 'batch') {
      codesInput.value = newValue;
      scrapTaskState.inputValue = '';
    }
    // 如果不是 batch 任务但有输入内容，说明需要切换到 batch 任务
    else if (appState.currentView === 'task') {
      activeTask.value = 'batch';
      codesInput.value = newValue;
      scrapTaskState.inputValue = '';
    }
  }
});

// 监听路由变化，如果来自 extract 页面且有输入内容，则自动选择 batch 任务
watch(() => appState.currentView, (newView) => {
  if (newView === 'task' && scrapTaskState.inputValue) {
    activeTask.value = 'batch';
    codesInput.value = scrapTaskState.inputValue;
    scrapTaskState.inputValue = '';
  }
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

// 启动任务
async function handleLaunch() {
  if (!canLaunch.value || !activeTask.value) return;

  // 检查配置
  if (!isConfigured.value) {
    if (activeTask.value === 'recordPull' || activeTask.value === 'submit') {
      if (!checkClientAuth()) {
        errorMessage.value = 'Please configure client authentication in Config page first.';
        navigateTo('config');
        return;
      }
    } else {
      if (!checkTaskBaseUrl()) {
        errorMessage.value = 'Please set the task base URL in Config page first.';
        navigateTo('config');
        return;
      }
    }
  }

  isProcessing.value = true;
  errorMessage.value = '';
  successMessage.value = '';

  try {
    switch (activeTask.value) {
      case 'auto':
        await handleAutoScraping();
        break;
      case 'batch':
        await handleBatchScraping();
        break;
      case 'update':
        await handleUpdateTask();
        break;
      case 'idolCrawl':
        await handleIdolCrawlTask();
        break;
      case 'recordPull':
        await handleRecordPullTask();
        break;
      case 'submit':
        await handleSubmitTask();
        break;
    }
  } catch (error) {
    errorMessage.value = `Task failed: ${error}`;
  } finally {
    isProcessing.value = false;
  }
}

// 处理自动爬取
async function handleAutoScraping() {
  showProgressGlobal();
  scrapTaskState.taskType = 'auto' as const;
  scrapTaskState.isProcessing = true;

  const params = {
    start_url: config.value.startUrl.trim(),
    with_image: config.value.withImage,
    headless: config.value.headless,
    load_timeout: config.value.loadTimeout,
    request_delay: config.value.requestDelay,
    webdriver_port: config.value.webdriverPort
  };

  await invoke('launch_auto_scrap_task', params);
  successMessage.value = 'Auto crawling started successfully!';
}

// 处理批量爬取
async function handleBatchScraping() {
  const codes = codesInput.value
    .split('\n')
    .map(line => line.trim())
    .filter(line => line.length > 0);

  if (codes.length === 0) {
    errorMessage.value = 'Please enter at least one record ID.';
    return;
  }

  showProgressGlobal();
  scrapTaskState.taskType = 'batch' as const;
  scrapTaskState.isProcessing = true;

  const params = {
    codes,
    with_image: config.value.withImage,
    headless: config.value.headless,
    load_timeout: config.value.loadTimeout,
    request_delay: config.value.requestDelay,
    webdriver_port: config.value.webdriverPort
  };

  await invoke('launch_batch_scrap_task', params);
  successMessage.value = `Batch crawling started for ${codes.length} records!`;
}

// 处理更新任务
async function handleUpdateTask() {
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

  const params = {
    records: recordIds,
    with_image: config.value.withImage,
    headless: config.value.headless,
    load_timeout: config.value.loadTimeout,
    request_delay: config.value.requestDelay,
    webdriver_port: config.value.webdriverPort
  };

  await invoke('launch_update_task', params);
  successMessage.value = `Update task started for ${recordIds.length} records!`;
}

// 处理Idol爬取任务
async function handleIdolCrawlTask() {
  updateTaskStatus('idolCrawl', 'running');
  await invoke('launch_idol_scrap_task');
  successMessage.value = 'Idol crawl task started!';
}

// 处理记录拉取任务
async function handleRecordPullTask() {
  updateTaskStatus('recordPull', 'running');
  await invoke('launch_record_pull_task');
  successMessage.value = 'Record pull task started!';
}

// 处理提交任务
async function handleSubmitTask() {
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
  successMessage.value = `Submit task started for ${recordIds.length} records!`;
}
</script>

<style scoped>
.task-view {
  padding: 40px;
  max-width: 1200px;
  margin: 0 auto;
}

.task-container {
  background: white;
  border-radius: 12px;
  padding: 40px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
}

.task-title {
  font-size: 2rem;
  font-weight: 700;
  color: #333;
  margin-bottom: 16px;
  text-align: center;
}

.task-description {
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

/* Task Selection */
.task-selection-section {
  margin-bottom: 32px;
}

.task-buttons {
  display: grid;
  gap: 16px;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
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

.config-header h3 {
  font-size: 1.2rem;
  font-weight: 600;
  color: #374151;
  margin: 0;
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
  font-size: 0.9rem;
  color: #495057;
}

.filter-option:hover {
  background-color: #f1f5f9;
}

.filter-option input[type="checkbox"] {
  margin: 0;
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

/* Task Status Config */
.task-status-config {
  margin-bottom: 16px;
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

.checkbox-option {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  cursor: pointer;
  padding: 12px;
  border-radius: 8px;
  transition: background-color 0.2s ease;
  margin-bottom: 8px;
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
  margin-top: 16px;
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

/* Responsive design */
@media (max-width: 768px) {
  .task-view {
    padding: 20px;
  }

  .task-container {
    padding: 20px;
  }

  .task-buttons {
    grid-template-columns: 1fr;
  }

  .config-grid {
    grid-template-columns: 1fr;
  }
}
</style>
