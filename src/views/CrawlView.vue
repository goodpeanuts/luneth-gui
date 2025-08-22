<template>
  <div class="crawl-view">
    <div class="crawl-container">
      <h2 class="crawl-title">Web Scraping</h2>
      <p class="crawl-description">
        Choose a scraping mode and provide the necessary input to start crawling.
      </p>

      <!-- Task Type Selection -->
      <div class="task-type-section">
        <h3 class="section-title">Scraping Mode</h3>
        <div class="task-type-options">
          <label class="task-option" :class="{ active: taskType === 'auto' }">
            <input type="radio" v-model="taskType" value="auto" :disabled="isProcessing" />
            <div class="option-content">
              <div class="option-icon">🤖</div>
              <div class="option-text">
                <h4>Auto Mode</h4>
                <p>Automatically crawl from a single URL</p>
              </div>
            </div>
          </label>

          <label class="task-option" :class="{ active: taskType === 'manual' }">
            <input type="radio" v-model="taskType" value="manual" :disabled="isProcessing" />
            <div class="option-content">
              <div class="option-icon">✋</div>
              <div class="option-text">
                <h4>Manual Mode</h4>
                <p>Provide specific codes to crawl</p>
              </div>
            </div>
          </label>
        </div>
      </div>

      <!-- Input Section -->
      <div v-if="taskType" class="input-section">
        <h3 class="section-title">
          {{ taskType === 'auto' ? 'URL Input' : 'Codes Input' }}
        </h3>

        <div v-if="taskType === 'auto'" class="url-input">
          <input v-model="urlInput" type="url" class="input-field" placeholder="https://example.com/page"
            :disabled="isProcessing" />
        </div>

        <div v-else class="codes-input">
          <textarea v-model="codesInput" class="input-field textarea"
            placeholder="Enter codes, one per line:&#10;CODE001&#10;CODE002&#10;CODE003" rows="8"
            :disabled="isProcessing"></textarea>
          <div class="codes-hint">
            Enter one code per line. Each line will be processed as a separate code.
          </div>
        </div>
      </div>

      <!-- Action Section -->
      <div v-if="taskType" class="action-section">
        <button class="launch-btn" :class="{
          'processing': isProcessing,
          'disabled': !canLaunch
        }" :disabled="!canLaunch" @click="handleLaunch">
          <span v-if="isProcessing" class="processing-content">
            <div class="spinner"></div>
            Processing...
          </span>
          <span v-else>🚀 Launch Scraping</span>
        </button>

        <div v-if="errorMessage" class="error-message">
          {{ errorMessage }}
        </div>

        <div v-if="successMessage" class="success-message">
          {{ successMessage }}
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
  resetProgress
} from '@/store';
import type { ScrapTaskType } from '@/types';
import ProgressTracker from '@/components/ProgressTracker.vue';

const urlInput = ref('');
const codesInput = ref('');
const isProcessing = ref(false);
const errorMessage = ref('');
const successMessage = ref('');
const progressTracker = ref<InstanceType<typeof ProgressTracker>>();

// 使用全局任务类型状态
const taskType = computed({
  get: () => scrapTaskState.taskType || '',
  set: (value: ScrapTaskType | '') => {
    scrapTaskState.taskType = value as ScrapTaskType | null;
  }
});

// 在组件挂载时，如果有进度数据就显示进度组件
onMounted(() => {
  if (progressState.progressList.length > 0) {
    showProgressGlobal();
  }
});

// 计算属性
const canLaunch = computed(() => {
  if (isProcessing.value || !taskType.value) return false;

  if (taskType.value === 'auto') {
    return urlInput.value.trim() !== '';
  } else {
    return codesInput.value.trim() !== '';
  }
});

// 监听任务类型变化，清空错误信息
watch(taskType, (newType) => {
  errorMessage.value = '';
  successMessage.value = '';
  // 切换到manual模式时清空进度条，auto模式保留
  if (newType === 'manual') {
    hideProgress();
  }
});

// 监听输入变化，清空错误信息
watch([urlInput, codesInput], () => {
  errorMessage.value = '';
  successMessage.value = '';
});

// 启动爬取任务
async function handleLaunch() {
  if (!canLaunch.value) return;

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
  scrapTaskState.taskType = taskType.value as ScrapTaskType;
  scrapTaskState.isProcessing = true;

  // Manual模式才重置进度条，Auto模式保留已有进度条
  if (taskType.value === 'manual') {
    resetProgress();
  }

  try {
    if (taskType.value === 'auto') {
      await invoke('launch_auto_scrap_task', { url: urlInput.value.trim() });
      successMessage.value = 'Auto scraping task completed!';
    } else {
      const codes = codesInput.value
        .split('\n')
        .map(line => line.trim())
        .filter(line => line !== '');

      await invoke('launch_manual_scrap_task', { codes });
      successMessage.value = `Manual scraping task completed. Processed ${codes.length} codes.`;
    }
  } catch (error) {
    errorMessage.value = `Scraping task failed: ${error}`;
  } finally {
    isProcessing.value = false;
    scrapTaskState.isProcessing = false;
  }
}
</script>

<style scoped>
.crawl-view {
  padding: 40px;
  max-width: 800px;
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

.task-type-section {
  margin-bottom: 32px;
}

.task-type-options {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.task-option {
  border: 2px solid #e9ecef;
  border-radius: 12px;
  padding: 20px;
  cursor: pointer;
  transition: all 0.3s ease;
  display: block;
}

.task-option:hover {
  border-color: #007bff;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 123, 255, 0.1);
}

.task-option.active {
  border-color: #007bff;
  background-color: #f8f9ff;
}

.task-option input[type="radio"] {
  display: none;
}

.option-content {
  display: flex;
  align-items: center;
  gap: 16px;
}

.option-icon {
  font-size: 2rem;
  width: 60px;
  height: 60px;
  border-radius: 50%;
  background-color: #f8f9fa;
  display: flex;
  align-items: center;
  justify-content: center;
}

.option-text h4 {
  font-size: 1.1rem;
  font-weight: 600;
  color: #333;
  margin: 0 0 4px 0;
}

.option-text p {
  font-size: 0.9rem;
  color: #666;
  margin: 0;
}

.input-section {
  margin-bottom: 32px;
}

.url-input,
.codes-input {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.input-field {
  padding: 12px 16px;
  border: 2px solid #e1e5e9;
  border-radius: 8px;
  font-size: 1rem;
  transition: all 0.3s ease;
  font-family: inherit;
}

.input-field:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
}

.input-field:disabled {
  background-color: #f8f9fa;
  color: #6c757d;
  cursor: not-allowed;
}

.textarea {
  resize: vertical;
  min-height: 120px;
}

.codes-hint {
  font-size: 0.9rem;
  color: #6c757d;
  font-style: italic;
}

.action-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.launch-btn {
  padding: 16px 32px;
  background-color: #28a745;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 1.1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  min-width: 200px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.launch-btn:hover:not(.disabled):not(.processing) {
  background-color: #218838;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(40, 167, 69, 0.3);
}

.launch-btn.disabled {
  background-color: #6c757d;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.launch-btn.processing {
  background-color: #007bff;
  cursor: wait;
}

.processing-content {
  display: flex;
  align-items: center;
  gap: 8px;
}

.spinner {
  width: 20px;
  height: 20px;
  border: 2px solid transparent;
  border-top: 2px solid white;
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

.error-message {
  color: #dc3545;
  background-color: #f8d7da;
  border: 1px solid #f5c6cb;
  border-radius: 6px;
  padding: 12px 16px;
  font-size: 0.95rem;
  text-align: center;
  max-width: 500px;
}

.success-message {
  color: #155724;
  background-color: #d4edda;
  border: 1px solid #c3e6cb;
  border-radius: 6px;
  padding: 12px 16px;
  font-size: 0.95rem;
  text-align: center;
  max-width: 500px;
}
</style>
