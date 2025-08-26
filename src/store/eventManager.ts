// 全局事件管理器

import { listen } from '@tauri-apps/api/event';
import type {
  CrawlPageStartEvent,
  CrawlPageSuccessEvent,
  CrawlPageFailedEvent,
  CrawlCodeReportEvent,
  CrawlCodesFinishedEvent,
  BatchCrawlStartEvent,
  IdolCrawlStartEvent,
  IdolCrawlProgressEvent,
  IdolCrawlCompleteEvent,
  IdolCrawlFailedEvent,
  RecordPullStartEvent,
  RecordPullProgressEvent,
  RecordPullCompleteEvent,
  RecordPullFailedEvent,
  SubmitStartEvent,
  SubmitCodeStartEvent,
  SubmitCodeResultEvent,
  SubmitFinishedEvent
} from '@/types/events';

import {
  progressState,
  findOrCreateProgress,
  updateProgressStatus,
  resetProgress
} from './progress';

import {
  scrapTaskState
} from './task';

import {
  updateTaskStatus,
  updateTaskMessage,
  updateTaskProgress
} from './task';

// 事件监听器函数引用，用于清理
let unlistenFunctions: (() => void)[] = [];

// 初始化全局事件监听
export async function initializeGlobalEventListeners() {
  // 清理之前的监听器
  cleanupEventListeners();

  try {
    // =================================
    // 爬取相关事件监听
    // =================================

    // Batch crawl start (unified event for all batch operations)
    const unlistenBatchStart = await listen<BatchCrawlStartEvent>('batch-crawl-start', (event) => {
      console.log('[Event] Batch crawl started:', event.payload);

      // 根据当前任务类型决定如何处理
      if (scrapTaskState.taskType === 'auto') {
        // Auto模式：更新最新的页面进度条
        const latestPageProgress = progressState.progressList
          .filter(p => p.name !== 'Manual Crawl')
          .sort((a, b) => b.createdAt - a.createdAt)[0];

        if (latestPageProgress) {
          latestPageProgress.total = event.payload.totalCount;
          latestPageProgress.current = 0;
          latestPageProgress.status = 'in-progress';
        }
      } else {
        // Manual模式：清空所有进度条并创建新的
        resetProgress();
        const progress = findOrCreateProgress('Manual Crawl');
        progress.total = event.payload.totalCount;
        progress.current = 0;
        progress.status = 'in-progress';
      }
    });
    unlistenFunctions.push(unlistenBatchStart);

    // Page start (auto mode)
    const unlistenPageStart = await listen<CrawlPageStartEvent>('crawl-page-start', (event) => {
      console.log('[Event] Page crawl started:', event.payload);

      const progress = findOrCreateProgress(event.payload.pageName);
      progress.status = 'pending';
      progress.total = -1; // Unknown total initially
      progress.current = 0;
    });
    unlistenFunctions.push(unlistenPageStart);

    // Page success (auto mode)
    const unlistenPageSuccess = await listen<CrawlPageSuccessEvent>('crawl-page-success', (event) => {
      console.log('[Event] Page crawl success:', event.payload);

      const progress = findOrCreateProgress(event.payload.pageName);
      progress.total = event.payload.totalCount;
      progress.status = 'in-progress';
    });
    unlistenFunctions.push(unlistenPageSuccess);

    // Page failed (auto mode)
    const unlistenPageFailed = await listen<CrawlPageFailedEvent>('crawl-page-failed', (event) => {
      console.log('[Event] Page crawl failed:', event.payload);

      const progress = findOrCreateProgress(event.payload.pageName);
      progress.status = 'failed';
      progress.errorMessage = event.payload.errorMessage;
    });
    unlistenFunctions.push(unlistenPageFailed);

    // Code report (both modes)
    const unlistenCodeReport = await listen<CrawlCodeReportEvent>('crawl-code-report', (event) => {
      console.log('[Event] Code crawl report:', event.payload);

      // Find the appropriate progress bar to update
      let targetProgress = undefined;

      if (scrapTaskState.taskType === 'auto') {
        // Auto模式：更新最新的in-progress页面进度条
        const autoModeProgresses = progressState.progressList.filter(p => p.name !== 'Manual Crawl');
        targetProgress = autoModeProgresses
          .filter(p => p.status === 'in-progress')
          .sort((a, b) => b.createdAt - a.createdAt)[0];
      } else {
        // Manual模式：更新Manual Crawl进度条
        targetProgress = progressState.progressList.find(p => p.name === 'Manual Crawl');
      }

      if (targetProgress) {
        targetProgress.current++;
        updateProgressStatus(targetProgress);
      }
    });
    unlistenFunctions.push(unlistenCodeReport);

    // Codes finished
    const unlistenCodesFinished = await listen<CrawlCodesFinishedEvent>('crawl-codes-finished', (event) => {
      console.log('[Event] Codes crawl finished:', event.payload);

      // Find the progress to update
      let targetProgress = undefined;

      if (scrapTaskState.taskType === 'auto') {
        // Auto模式：更新最新的in-progress页面进度条
        const autoModeProgresses = progressState.progressList.filter(p => p.name !== 'Manual Crawl');
        targetProgress = autoModeProgresses
          .filter(p => p.status === 'in-progress')
          .sort((a, b) => b.createdAt - a.createdAt)[0];
      } else {
        // Manual模式：更新Manual Crawl进度条
        targetProgress = progressState.progressList.find(p => p.name === 'Manual Crawl');
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

    // =================================
    // 管理任务相关事件监听
    // =================================

    // Idol crawl events
    const unlistenIdolStart = await listen<IdolCrawlStartEvent>('idol-crawl-start', (event) => {
      console.log('[Event] Idol crawl started:', event.payload);
      updateTaskStatus('idolCrawl', 'running');
      updateTaskMessage('idolCrawl', event.payload.message);
    });
    unlistenFunctions.push(unlistenIdolStart);

    const unlistenIdolProgress = await listen<IdolCrawlProgressEvent>('idol-crawl-progress', (event) => {
      console.log('[Event] Idol crawl progress:', event.payload);
      updateTaskProgress('idolCrawl', {
        processed: event.payload.processed,
        total: -1 // Unknown total for progress events
      });
      updateTaskMessage('idolCrawl', event.payload.message);
    });
    unlistenFunctions.push(unlistenIdolProgress);

    const unlistenIdolComplete = await listen<IdolCrawlCompleteEvent>('idol-crawl-complete', (event) => {
      console.log('[Event] Idol crawl completed:', event.payload);
      updateTaskStatus('idolCrawl', 'success');
      updateTaskMessage('idolCrawl', `Completed: ${event.payload.successCount}/${event.payload.totalCount} successful`);
    });
    unlistenFunctions.push(unlistenIdolComplete);

    const unlistenIdolFailed = await listen<IdolCrawlFailedEvent>('idol-crawl-failed', (event) => {
      console.log('[Event] Idol crawl failed:', event.payload);
      updateTaskStatus('idolCrawl', 'failed');
      updateTaskMessage('idolCrawl', event.payload.errorMessage);
    });
    unlistenFunctions.push(unlistenIdolFailed);

    // Record pull events
    const unlistenRecordStart = await listen<RecordPullStartEvent>('record-pull-start', (event) => {
      console.log('[Event] Record pull started:', event.payload);
      updateTaskStatus('recordPull', 'running');
      updateTaskMessage('recordPull', `Starting to pull ${event.payload.totalCount} records`);
    });
    unlistenFunctions.push(unlistenRecordStart);

    const unlistenRecordProgress = await listen<RecordPullProgressEvent>('record-pull-progress', (event) => {
      console.log('[Event] Record pull progress:', event.payload);
      updateTaskProgress('recordPull', {
        processed: event.payload.processed,
        total: -1 // Unknown total for progress events
      });
      updateTaskMessage('recordPull', event.payload.message);
    });
    unlistenFunctions.push(unlistenRecordProgress);

    const unlistenRecordComplete = await listen<RecordPullCompleteEvent>('record-pull-complete', (event) => {
      console.log('[Event] Record pull completed:', event.payload);
      updateTaskStatus('recordPull', 'success');
      updateTaskMessage('recordPull', `Completed: ${event.payload.successCount}/${event.payload.totalCount} successful`);
    });
    unlistenFunctions.push(unlistenRecordComplete);

    const unlistenRecordFailed = await listen<RecordPullFailedEvent>('record-pull-failed', (event) => {
      console.log('[Event] Record pull failed:', event.payload);
      updateTaskStatus('recordPull', 'failed');
      updateTaskMessage('recordPull', event.payload.errorMessage);
    });
    unlistenFunctions.push(unlistenRecordFailed);

    // Submit events
    const unlistenSubmitStart = await listen<SubmitStartEvent>('submit-start', (event) => {
      console.log('[Event] Submit started:', event.payload);
      updateTaskStatus('submit', 'running');
      updateTaskProgress('submit', {
        processed: 0,
        total: event.payload.totalCount
      });
      updateTaskMessage('submit', `Starting submission of ${event.payload.totalCount} records`);
    });
    unlistenFunctions.push(unlistenSubmitStart);

    const unlistenSubmitCodeStart = await listen<SubmitCodeStartEvent>('submit-code-start', (event) => {
      console.log('[Event] Submit code started:', event.payload);
      updateTaskMessage('submit', `Submitting: ${event.payload.code}`);
    });
    unlistenFunctions.push(unlistenSubmitCodeStart);

    const unlistenSubmitCodeResult = await listen<SubmitCodeResultEvent>('submit-code-result', (event) => {
      console.log('[Event] Submit code result:', event.payload);
      // This will be handled by submit-finished for final status
    });
    unlistenFunctions.push(unlistenSubmitCodeResult);

    const unlistenSubmitFinished = await listen<SubmitFinishedEvent>('submit-finished', (event) => {
      console.log('[Event] Submit finished:', event.payload);
      updateTaskStatus('submit', event.payload.errorCount > 0 ? 'failed' : 'success');
      updateTaskProgress('submit', {
        processed: event.payload.totalCount,
        total: event.payload.totalCount
      });
      updateTaskMessage('submit',
        `Submission completed: ${event.payload.successCount} successful, ${event.payload.errorCount} failed`
      );
    });
    unlistenFunctions.push(unlistenSubmitFinished);

    console.log('[EventManager] All event listeners initialized successfully');

  } catch (error) {
    console.error('[EventManager] Failed to initialize event listeners:', error);
  }
}

// 清理事件监听器
export function cleanupEventListeners() {
  console.log('[EventManager] Cleaning up event listeners');
  unlistenFunctions.forEach(unlisten => {
    try {
      unlisten();
    } catch (error) {
      console.error('[EventManager] Error cleaning up listener:', error);
    }
  });
  unlistenFunctions = [];
}

// 获取当前监听器数量（用于调试）
export function getActiveListenerCount(): number {
  return unlistenFunctions.length;
}
