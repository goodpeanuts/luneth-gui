<template>
  <div class="manage-view">
    <div class="manage-container">
      <h2 class="manage-title">Task Management</h2>

      <div class="task-grid">
        <-d Idol Crawl Task -->
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

        <-d Record Pull Task -->
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
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import {
  manageTaskState,
  updateTaskStatus,
  updateTaskMessage,
  updateTaskProgress
} from '@/store';

// Configuration check
const isConfigured = computed(() => {
  // Check if basic configuration is available
  // This would typically check for API keys, server URLs, etc.
  return true; // Placeholder - implement actual config check
});

const isAnyTaskRunning = computed(() => {
  return manageTaskState.idolCrawl.status === 'running' ||
         manageTaskState.recordPull.status === 'running';
});

// Status text helper
const getStatusText = (taskType: 'idolCrawl' | 'recordPull') => {
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

// Event listeners
let unlistenIdolStart: (() => void) | undefined;
let unlistenIdolProgress: (() => void) | undefined;
let unlistenIdolComplete: (() => void) | undefined;
let unlistenIdolFailed: (() => void) | undefined;
let unlistenRecordStart: (() => void) | undefined;
let unlistenRecordProgress: (() => void) | undefined;
let unlistenRecordComplete: (() => void) | undefined;
let unlistenRecordFailed: (() => void) | undefined;

onMounted(async () => {
  // Idol crawl events
  unlistenIdolStart = await listen('idol-crawl-start', (event) => {
    console.log('Idol crawl started:', event.payload);
    updateTaskStatus('idolCrawl', 'running');
    updateTaskMessage('idolCrawl', 'Starting idol crawl...');
  });

  unlistenIdolProgress = await listen('idol-crawl-progress', (event) => {
    const progress = event.payload as any;
    console.log('Idol crawl progress:', progress);
    updateTaskProgress('idolCrawl', progress);
    updateTaskMessage('idolCrawl', `Processing: ${progress.current_url || 'images'}`);
  });

  unlistenIdolComplete = await listen('idol-crawl-complete', (event) => {
    console.log('Idol crawl completed:', event.payload);
    updateTaskStatus('idolCrawl', 'success');
    updateTaskMessage('idolCrawl', 'Crawl completed successfully');
  });

  unlistenIdolFailed = await listen('idol-crawl-failed', (event) => {
    console.error('Idol crawl failed:', event.payload);
    updateTaskStatus('idolCrawl', 'failed');
    updateTaskMessage('idolCrawl', `Crawl failed: ${event.payload}`);
  });

  // Record pull events
  unlistenRecordStart = await listen('record-pull-start', (event) => {
    console.log('Record pull started:', event.payload);
    updateTaskStatus('recordPull', 'running');
    updateTaskMessage('recordPull', 'Starting record pull...');
  });

  unlistenRecordProgress = await listen('record-pull-progress', (event) => {
    const progress = event.payload as any;
    console.log('Record pull progress:', progress);
    updateTaskProgress('recordPull', progress);
    updateTaskMessage('recordPull', `Processing records...`);
  });

  unlistenRecordComplete = await listen('record-pull-complete', (event) => {
    console.log('Record pull completed:', event.payload);
    updateTaskStatus('recordPull', 'success');
    updateTaskMessage('recordPull', 'Record pull completed successfully');
  });

  unlistenRecordFailed = await listen('record-pull-failed', (event) => {
    console.error('Record pull failed:', event.payload);
    updateTaskStatus('recordPull', 'failed');
    updateTaskMessage('recordPull', `Record pull failed: ${event.payload}`);
  });
});

onUnmounted(() => {
  // Clean up event listeners
  unlistenIdolStart?.();
  unlistenIdolProgress?.();
  unlistenIdolComplete?.();
  unlistenIdolFailed?.();
  unlistenRecordStart?.();
  unlistenRecordProgress?.();
  unlistenRecordComplete?.();
  unlistenRecordFailed?.();
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
