import { reactive } from 'vue';
import type { AppState, ViewType, ScrapTaskState, ProgressState, ProgressItem, ProgressStatus } from '@/types';

// 全局应用状态
export const appState = reactive<AppState>({
  currentView: 'home',
  taskBaseUrl: null,
  isTaskBaseUrlSet: false,
  isProcessing: false,
  selectedRecord: null,
});

// 爬取任务状态
export const scrapTaskState = reactive<ScrapTaskState>({
  isProcessing: false,
  taskType: null,
  inputValue: '',
});

// 全局进度状态
export const progressState = reactive<ProgressState>({
  progressList: [],
  isVisible: false,
});

// 导航方法
export function navigateTo(view: ViewType) {
  appState.currentView = view;
}

// 设置任务基础URL
export function setTaskBaseUrl(url: string) {
  appState.taskBaseUrl = url;
  appState.isTaskBaseUrlSet = true;
}

// 检查是否设置了任务基础URL
export function checkTaskBaseUrl(): boolean {
  return appState.isTaskBaseUrlSet && !!appState.taskBaseUrl;
}

// 重置爬取任务状态
export function resetScrapTaskState() {
  scrapTaskState.isProcessing = false;
  scrapTaskState.taskType = null;
  scrapTaskState.inputValue = '';
}

// 进度管理函数
export function resetProgress() {
  progressState.progressList = [];
}

export function showProgress() {
  progressState.isVisible = true;
}

export function hideProgress() {
  progressState.isVisible = false;
}

export function findOrCreateProgress(name: string): ProgressItem {
  let progress = progressState.progressList.find(p => p.name === name);
  if (!progress) {
    progress = {
      id: `${Date.now()}-${Math.random()}`,
      name,
      current: 0,
      total: -1,
      status: 'pending' as ProgressStatus,
      createdAt: Date.now(),
    };
    // 新进度条插入到顶部（用于auto模式的正确显示顺序）
    progressState.progressList.unshift(progress);
  }
  return progress;
}

export function updateProgressStatus(progress: ProgressItem) {
  if (progress.current === 0) {
    progress.status = 'pending';
  } else if (progress.current < progress.total) {
    progress.status = 'in-progress';
  } else {
    progress.status = 'success';
  }
}
