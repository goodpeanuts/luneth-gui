<template>
  <div
    class="progress-container"
    v-if="progressState.isVisible"
    :data-count="progressState.progressList.length > 5 ? 'many' : 'few'"
  >
    <div v-for="progress in progressState.progressList" :key="progress.id" class="progress-item">
      <div class="progress-header">
        <div class="progress-info">
          <span class="progress-name">{{ progress.name }}</span>
          <span class="progress-count">{{ getStatusText(progress) }}</span>
        </div>
        <div class="progress-status-badge" :class="getStatusClass(progress.status)">
          <span class="status-icon">{{ getStatusIcon(progress.status) }}</span>
          <span class="status-text">{{ getStatusLabel(progress.status) }}</span>
        </div>
      </div>

      <div class="progress-bar-wrapper">
        <div class="progress-bar-track">
          <div
            class="progress-bar-fill"
            :class="getProgressBarClass(progress.status)"
            :style="{ width: getProgressWidth(progress) }"
          >
            <div class="progress-shine" v-if="progress.status === 'in-progress'"></div>
          </div>
        </div>
        <div class="progress-percentage">{{ getProgressPercentage(progress) }}</div>
      </div>

      <div v-if="progress.errorMessage" class="error-message">
        <span class="error-icon">⚠️</span>
        <span class="error-text">{{ truncateMessage(progress.errorMessage) }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';
import {
  progressState,
  scrapTaskState,
  findOrCreateProgress,
  updateProgressStatus,
  resetProgress
} from '@/store';
import type { ProgressStatus, ProgressItem } from '@/types';

// Event interfaces
interface CrawlPageStartEvent {
  pageName: string;
}

interface CrawlPageSuccessEvent {
  pageName: string;
  totalCount: number;
}

interface CrawlPageFailedEvent {
  pageName: string;
  errorMessage: string;
}

interface CrawlCodeReportEvent {
  code: string;
  status: 'Success' | 'Failed';
  message: string;
}

interface CrawlCodesFinishedEvent {
  successCount: number;
  errorCount: number;
  totalCount: number;
}

interface CrawlManualStartEvent {
  totalCount: number;
}

let unlistenFunctions: (() => void)[] = [];

// Get status class for styling
const getStatusClass = (status: ProgressStatus): string => {
  switch (status) {
    case 'pending': return 'status-pending';
    case 'in-progress': return 'status-progress';
    case 'success': return 'status-success';
    case 'failed': return 'status-failed';
    case 'mixed': return 'status-mixed';
    default: return '';
  }
};

// Get progress bar class for styling
const getProgressBarClass = (status: ProgressStatus): string => {
  switch (status) {
    case 'pending': return 'bar-pending';
    case 'in-progress': return 'bar-progress';
    case 'success': return 'bar-success';
    case 'failed': return 'bar-failed';
    case 'mixed': return 'bar-mixed';
    default: return 'bar-pending';
  }
};

// Get status text
const getStatusText = (progress: ProgressItem): string => {
  if (progress.status === 'failed' && progress.errorMessage) {
    return 'Failed';
  }
  return `${progress.current}/${progress.total === -1 ? '?' : progress.total}`;
};

// Get status icon
const getStatusIcon = (status: ProgressStatus): string => {
  switch (status) {
    case 'pending': return '⏳';
    case 'in-progress': return '🔄';
    case 'success': return '✅';
    case 'failed': return '❌';
    case 'mixed': return '⚠️';
    default: return '⏳';
  }
};

// Get status label
const getStatusLabel = (status: ProgressStatus): string => {
  switch (status) {
    case 'pending': return 'Pending';
    case 'in-progress': return 'Processing';
    case 'success': return 'Success';
    case 'failed': return 'Failed';
    case 'mixed': return 'Partial';
    default: return 'Pending';
  }
};

// Get progress percentage text
const getProgressPercentage = (progress: ProgressItem): string => {
  if (progress.status === 'failed') {
    return '—';
  }
  if (progress.total <= 0 || progress.total === -1) {
    return '—';
  }
  const percentage = Math.min((progress.current / progress.total) * 100, 100);
  return `${Math.round(percentage)}%`;
};

// Get progress width percentage
const getProgressWidth = (progress: ProgressItem): string => {
  if (progress.status === 'failed') {
    return '100%';
  }
  if (progress.total <= 0 || progress.total === -1) {
    return '0%';
  }
  return `${Math.min((progress.current / progress.total) * 100, 100)}%`;
};

// Truncate error message to fit in one line
const truncateMessage = (message: string): string => {
  const maxLength = 80;
  return message.length > maxLength ? `${message.substring(0, maxLength)}...` : message;
};

// Set up event listeners
onMounted(async () => {
  try {
    // Manual mode start
    const unlistenManualStart = await listen<CrawlManualStartEvent>('crawl-manual-start', (event) => {
      // 使用全局任务类型来判断是否是纯manual模式
      const isManualMode = scrapTaskState.taskType === 'manual';

      if (isManualMode) {
        // 纯manual模式：清空所有进度条并创建新的
        resetProgress();
        const progress = findOrCreateProgress('Manual Crawl');
        progress.total = event.payload.totalCount;
        progress.current = 0;
        progress.status = 'in-progress';
      } else {
        // Auto模式中的manual步骤：不重置进度条，更新最新的页面进度条
        const latestPageProgress = progressState.progressList
          .filter((p: ProgressItem) => p.name !== 'Manual Crawl')
          .sort((a: ProgressItem, b: ProgressItem) => b.createdAt - a.createdAt)[0];

        if (latestPageProgress) {
          latestPageProgress.total = event.payload.totalCount;
          latestPageProgress.current = 0;
          latestPageProgress.status = 'in-progress';
        }
      }
    });
    unlistenFunctions.push(unlistenManualStart);

    // Page start (auto mode) - 新页面应该在顶部创建
    const unlistenPageStart = await listen<CrawlPageStartEvent>('crawl-page-start', (event) => {
      const progress = findOrCreateProgress(event.payload.pageName);
      progress.status = 'pending';
      progress.total = -1; // Unknown total initially
      progress.current = 0;
    });
    unlistenFunctions.push(unlistenPageStart);

    // Page success (auto mode)
    const unlistenPageSuccess = await listen<CrawlPageSuccessEvent>('crawl-page-success', (event) => {
      const progress = findOrCreateProgress(event.payload.pageName);
      progress.total = event.payload.totalCount;
      progress.status = 'in-progress';
    });
    unlistenFunctions.push(unlistenPageSuccess);

    // Page failed (auto mode)
    const unlistenPageFailed = await listen<CrawlPageFailedEvent>('crawl-page-failed', (event) => {
      const progress = findOrCreateProgress(event.payload.pageName);
      progress.status = 'failed';
      progress.errorMessage = event.payload.errorMessage;
    });
    unlistenFunctions.push(unlistenPageFailed);

    // Code report (both modes)
    const unlistenCodeReport = await listen<CrawlCodeReportEvent>('crawl-code-report', (_event) => {
      // Find the appropriate progress bar to update
      let targetProgress: ProgressItem | undefined;

      if (scrapTaskState.taskType === 'auto') {
        // Auto模式：更新最新的in-progress页面进度条
        const autoModeProgresses = progressState.progressList.filter((p: ProgressItem) => p.name !== 'Manual Crawl');
        targetProgress = autoModeProgresses
          .filter((p: ProgressItem) => p.status === 'in-progress')
          .sort((a: ProgressItem, b: ProgressItem) => b.createdAt - a.createdAt)[0];
      } else {
        // Manual模式：更新Manual Crawl进度条
        targetProgress = progressState.progressList.find((p: ProgressItem) => p.name === 'Manual Crawl');
      }

      if (targetProgress) {
        targetProgress.current++;
        updateProgressStatus(targetProgress);
      }
    });
    unlistenFunctions.push(unlistenCodeReport);

    // Codes finished
    const unlistenCodesFinished = await listen<CrawlCodesFinishedEvent>('crawl-codes-finished', (event) => {
      // Find the progress to update
      let targetProgress: ProgressItem | undefined;

      if (scrapTaskState.taskType === 'auto') {
        // Auto模式：更新最新的in-progress页面进度条
        const autoModeProgresses = progressState.progressList.filter((p: ProgressItem) => p.name !== 'Manual Crawl');
        targetProgress = autoModeProgresses
          .filter((p: ProgressItem) => p.status === 'in-progress')
          .sort((a: ProgressItem, b: ProgressItem) => b.createdAt - a.createdAt)[0];
      } else {
        // Manual模式：更新Manual Crawl进度条
        targetProgress = progressState.progressList.find((p: ProgressItem) => p.name === 'Manual Crawl');
      }

      if (targetProgress) {
        const { successCount, errorCount, totalCount } = event.payload;
        targetProgress.current = totalCount;
        targetProgress.total = totalCount;

        if (errorCount === 0) {
          targetProgress.status = 'success';
        } else if (successCount === 0) {
          targetProgress.status = 'failed';
        } else {
          targetProgress.status = 'mixed';
        }
      }
    });
    unlistenFunctions.push(unlistenCodesFinished);

  } catch (error) {
    console.error('Failed to set up progress listeners:', error);
  }
});

// Clean up event listeners
onUnmounted(() => {
  unlistenFunctions.forEach(unlisten => unlisten());
  unlistenFunctions = [];
});

// Expose reset function for parent component
defineExpose({
  resetProgress
});
</script>

<style scoped>
.progress-container {
  padding: 20px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 16px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  margin: 16px 0;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  max-height: 70vh; /* 限制最大高度为视口高度的70% */
  overflow-y: auto; /* 支持垂直滚动 */
  overflow-x: hidden;
  position: relative;
}

.progress-item {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 12px;
  padding: 16px;
  margin-bottom: 12px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.05);
  transition: all 0.3s ease;
  border: 1px solid rgba(255, 255, 255, 0.2);
  position: relative;
  /* 新进度条的入场动画 */
  animation: slideInFromTop 0.4s ease-out;
}

@keyframes slideInFromTop {
  0% {
    opacity: 0;
    transform: translateY(-20px);
    max-height: 0;
    margin-bottom: 0;
    padding-top: 0;
    padding-bottom: 0;
  }
  100% {
    opacity: 1;
    transform: translateY(0);
    max-height: 200px; /* 估算的最大高度 */
    margin-bottom: 12px;
    padding-top: 16px;
    padding-bottom: 16px;
  }
}

.progress-item:last-child {
  margin-bottom: 0;
}

.progress-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
}

.progress-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.progress-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.progress-name {
  font-weight: 600;
  font-size: 16px;
  color: #2d3748;
  word-break: break-word; /* 长文本换行 */
}

.progress-count {
  font-size: 14px;
  color: #718096;
  font-family: 'Courier New', monospace;
}

.progress-status-badge {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  white-space: nowrap; /* 防止状态文本换行 */
}

.status-pending {
  background: linear-gradient(135deg, #ffeaa7, #fdcb6e);
  color: #8b5a00;
}

.status-progress {
  background: linear-gradient(135deg, #74b9ff, #0984e3);
  color: white;
}

.status-success {
  background: linear-gradient(135deg, #55efc4, #00b894);
  color: white;
}

.status-failed {
  background: linear-gradient(135deg, #fd79a8, #e84393);
  color: white;
}

.status-mixed {
  background: linear-gradient(135deg, #fdcb6e, #e17055);
  color: white;
}

.progress-bar-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
}

.progress-bar-track {
  flex: 1;
  height: 8px;
  background: rgba(0, 0, 0, 0.05);
  border-radius: 8px;
  overflow: hidden;
  position: relative;
}

.progress-bar-fill {
  height: 100%;
  border-radius: 8px;
  transition: width 0.3s ease;
  position: relative;
  overflow: hidden;
}

.bar-pending {
  background: linear-gradient(90deg, #ffeaa7, #fdcb6e);
}

.bar-progress {
  background: linear-gradient(90deg, #74b9ff, #0984e3);
}

.bar-success {
  background: linear-gradient(90deg, #55efc4, #00b894);
}

.bar-failed {
  background: linear-gradient(90deg, #fd79a8, #e84393);
}

.bar-mixed {
  background: linear-gradient(90deg, #fdcb6e, #e17055);
}

.progress-shine {
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.4),
    transparent
  );
  animation: shine 2s infinite;
}

@keyframes shine {
  0% { left: -100%; }
  100% { left: 100%; }
}

.progress-percentage {
  font-size: 14px;
  font-weight: 600;
  color: #4a5568;
  min-width: 40px;
  text-align: right;
  font-family: 'Courier New', monospace;
}

.error-message {
  margin-top: 8px;
  padding: 8px 12px;
  background: rgba(255, 107, 107, 0.1);
  border: 1px solid rgba(255, 107, 107, 0.2);
  border-radius: 8px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.error-icon {
  font-size: 14px;
}

.error-text {
  font-size: 13px;
  color: #c53030;
  flex: 1;
  word-break: break-word;
}

/* 自定义滚动条样式 */
.progress-container::-webkit-scrollbar {
  width: 12px;
}

.progress-container::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  margin: 4px;
}

.progress-container::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.4);
  border-radius: 6px;
  border: 2px solid transparent;
  background-clip: content-box;
}

.progress-container::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.6);
  background-clip: content-box;
}

/* 滚动指示器 */
.progress-container::before {
  content: '';
  position: sticky;
  top: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.5), transparent);
  pointer-events: none;
  z-index: 1;
}

.progress-container::after {
  content: '';
  position: sticky;
  bottom: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.5), transparent);
  pointer-events: none;
  z-index: 1;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .progress-container {
    padding: 16px;
    margin: 12px 0;
    max-height: 60vh;
  }

  .progress-item {
    padding: 12px;
    margin-bottom: 8px;
  }

  .progress-header {
    flex-direction: column;
    align-items: stretch;
    gap: 8px;
  }

  .progress-status-badge {
    align-self: flex-start;
  }
}

/* 当进度条很多时的视觉提示 */
.progress-container[data-count="many"] {
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15), inset 0 1px 0 rgba(255, 255, 255, 0.1);
}
</style>
