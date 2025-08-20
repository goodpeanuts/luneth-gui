<template>
  <div class="progress-container">
    <div v-for="progress in progressList" :key="progress.id" class="progress-item">
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
import { ref, onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';

// Progress status types
type ProgressStatus = 'pending' | 'in-progress' | 'success' | 'failed' | 'mixed';

// Progress item interface
interface ProgressItem {
  id: string;
  name: string;
  current: number;
  total: number;
  status: ProgressStatus;
  errorMessage?: string;
}

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

const progressList = ref<ProgressItem[]>([]);
let unlistenFunctions: (() => void)[] = [];

// Reset progress when starting new crawl
const resetProgress = () => {
  progressList.value = [];
};

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

// Find or create progress item
const findOrCreateProgress = (name: string): ProgressItem => {
  let progress = progressList.value.find(p => p.name === name);
  if (!progress) {
    progress = {
      id: `${Date.now()}-${Math.random()}`,
      name,
      current: 0,
      total: -1,
      status: 'pending'
    };
    progressList.value.push(progress);
  }
  return progress;
};

// Update progress status based on results
const updateProgressStatus = (progress: ProgressItem) => {
  if (progress.current === 0) {
    progress.status = 'pending';
  } else if (progress.current < progress.total) {
    progress.status = 'in-progress';
  } else {
    // Check if there were any failures based on the name pattern
    // For auto mode, we'll rely on the crawl-codes-finished event
    progress.status = 'success';
  }
};

// Set up event listeners
onMounted(async () => {
  try {
    // Manual mode start
    const unlistenManualStart = await listen<CrawlManualStartEvent>('crawl-manual-start', (event) => {
      resetProgress();
      const progress = findOrCreateProgress('Manual Crawl');
      progress.total = event.payload.totalCount;
      progress.current = 0;
      progress.status = 'in-progress';
    });
    unlistenFunctions.push(unlistenManualStart);

    // Page start (auto mode)
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

      if (progressList.value.length === 1) {
        // Manual mode - single progress bar
        targetProgress = progressList.value[0];
      } else {
        // Auto mode - find the latest in-progress page
        targetProgress = progressList.value
          .filter(p => p.status === 'in-progress')
          .sort((a, b) => b.id.localeCompare(a.id))[0];
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

      if (progressList.value.length === 1) {
        // Manual mode
        targetProgress = progressList.value[0];
      } else {
        // Auto mode - find the latest in-progress page
        targetProgress = progressList.value
          .filter(p => p.status === 'in-progress')
          .sort((a, b) => b.id.localeCompare(a.id))[0];
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
  margin-top: 24px;
  border: 1px solid #e1e5e9;
  border-radius: 12px;
  background: linear-gradient(135deg, #f8f9fa 0%, #ffffff 100%);
  padding: 20px;
  max-height: 400px;
  overflow-y: auto;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}

.progress-item {
  margin-bottom: 16px;
  background: white;
  border-radius: 10px;
  padding: 16px;
  border: 1px solid #e9ecef;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  transition: all 0.2s ease;
}

.progress-item:hover {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  transform: translateY(-1px);
}

.progress-item:last-child {
  margin-bottom: 0;
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
  color: #2c3e50;
  font-size: 0.95rem;
}

.progress-count {
  font-size: 0.8rem;
  color: #6c757d;
  font-family: 'SF Mono', 'Monaco', 'Consolas', monospace;
}

.progress-status-badge {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 8px;
  border-radius: 6px;
  font-size: 0.75rem;
  font-weight: 500;
  white-space: nowrap;
}

.status-pending {
  background-color: #f8f9fa;
  color: #6c757d;
  border: 1px solid #dee2e6;
}

.status-progress {
  background-color: #e3f2fd;
  color: #1976d2;
  border: 1px solid #bbdefb;
}

.status-success {
  background-color: #e8f5e8;
  color: #2e7d32;
  border: 1px solid #c8e6c9;
}

.status-failed {
  background-color: #ffebee;
  color: #c62828;
  border: 1px solid #ffcdd2;
}

.status-mixed {
  background-color: #fff8e1;
  color: #f57c00;
  border: 1px solid #ffecb3;
}

.status-icon {
  font-size: 0.8rem;
}

.status-text {
  font-weight: 600;
}

.progress-bar-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 8px;
}

.progress-bar-track {
  flex: 1;
  height: 8px;
  background-color: #f1f3f4;
  border-radius: 4px;
  overflow: hidden;
  position: relative;
  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.1);
}

.progress-bar-fill {
  height: 100%;
  border-radius: 4px;
  transition: width 0.3s ease;
  position: relative;
  overflow: hidden;
}

.bar-pending {
  background-color: #9e9e9e;
}

.bar-progress {
  background: linear-gradient(90deg, #42a5f5 0%, #2196f3 100%);
}

.bar-success {
  background: linear-gradient(90deg, #66bb6a 0%, #4caf50 100%);
}

.bar-failed {
  background: linear-gradient(90deg, #ef5350 0%, #f44336 100%);
}

.bar-mixed {
  background: linear-gradient(90deg, #ffb74d 0%, #ff9800 100%);
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
  animation: shine 1.5s infinite;
}

@keyframes shine {
  0% {
    left: -100%;
  }
  100% {
    left: 100%;
  }
}

.progress-percentage {
  font-family: 'SF Mono', 'Monaco', 'Consolas', monospace;
  font-size: 0.8rem;
  font-weight: 600;
  color: #495057;
  min-width: 35px;
  text-align: right;
}

.error-message {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 8px;
  padding: 8px 12px;
  background-color: #ffebee;
  border: 1px solid #ffcdd2;
  border-radius: 6px;
  color: #c62828;
  font-size: 0.8rem;
}

.error-icon {
  font-size: 0.9rem;
  flex-shrink: 0;
}

.error-text {
  font-family: 'SF Mono', 'Monaco', 'Consolas', monospace;
  word-break: break-all;
}

/* Scrollbar styling */
.progress-container::-webkit-scrollbar {
  width: 6px;
}

.progress-container::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 3px;
}

.progress-container::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 3px;
}

.progress-container::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}
</style>
