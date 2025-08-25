import { reactive } from 'vue';
import type { AppState, ViewType, ScrapTaskState, ProgressState, ProgressItem, ProgressStatus, RecordModel, HistoryOpModel } from '@/types';

// 缓存状态接口
export interface CacheState {
  records: {
    data: RecordModel[];
    timestamp: number;
    isLoading: boolean;
  };
  history: {
    data: HistoryOpModel[];
    timestamp: number;
    isLoading: boolean;
  };
}

// 全局应用状态
export const appState = reactive<AppState>({
  currentView: 'home',
  taskBaseUrl: null,
  isTaskBaseUrlSet: false,
  isProcessing: false,
  selectedRecord: null,
  clientAuth: {
    isSet: false,
    url: '',
  },
  pullRecordsState: {
    isLoading: false,
    lastSuccessTime: null,
    lastTotalCount: null,
    lastNewCount: null,
    lastError: null,
  },
});

// 全局缓存状态
export const cacheState = reactive<CacheState>({
  records: {
    data: [],
    timestamp: 0,
    isLoading: false,
  },
  history: {
    data: [],
    timestamp: 0,
    isLoading: false,
  },
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

// 设置客户端认证
export function setClientAuth(url: string) {
  appState.clientAuth.isSet = true;
  appState.clientAuth.url = url;
}

// 清除客户端认证
export function clearClientAuth() {
  appState.clientAuth.isSet = false;
  appState.clientAuth.url = '';
}

// 检查是否设置了客户端认证
export function checkClientAuth(): boolean {
  return appState.clientAuth.isSet && !!appState.clientAuth.url;
}

// 设置拉取记录状态
export function setPullRecordsLoading(isLoading: boolean) {
  appState.pullRecordsState.isLoading = isLoading;
}

// 设置拉取记录成功状态
export function setPullRecordsSuccess(totalCount: number, newCount: number) {
  appState.pullRecordsState.lastSuccessTime = new Date().toLocaleString();
  appState.pullRecordsState.lastTotalCount = totalCount;
  appState.pullRecordsState.lastNewCount = newCount;
  appState.pullRecordsState.lastError = null;
  appState.pullRecordsState.isLoading = false;
}

// 设置拉取记录失败状态
export function setPullRecordsError(error: string) {
  appState.pullRecordsState.lastError = error;
  appState.pullRecordsState.lastSuccessTime = new Date().toLocaleString();
  appState.pullRecordsState.lastTotalCount = null;
  appState.pullRecordsState.lastNewCount = null;
  appState.pullRecordsState.isLoading = false;
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

// 缓存管理函数
const CACHE_EXPIRE_TIME = 5 * 60 * 1000; // 5 minutes cache expiration time

export function isCacheValid(timestamp: number): boolean {
  return Date.now() - timestamp < CACHE_EXPIRE_TIME;
}

export function setCachedRecords(records: RecordModel[]) {
  cacheState.records.data = records;
  cacheState.records.timestamp = Date.now();
}

export function getCachedRecords(): RecordModel[] {
  if (isCacheValid(cacheState.records.timestamp)) {
    return cacheState.records.data;
  }
  return [];
}

export function setCachedHistory(history: HistoryOpModel[]) {
  cacheState.history.data = history;
  cacheState.history.timestamp = Date.now();
}

export function getCachedHistory(): HistoryOpModel[] {
  if (isCacheValid(cacheState.history.timestamp)) {
    return cacheState.history.data;
  }
  return [];
}

export function setRecordsLoading(isLoading: boolean) {
  cacheState.records.isLoading = isLoading;
}

export function setHistoryLoading(isLoading: boolean) {
  cacheState.history.isLoading = isLoading;
}

// 记录交互功能
export async function markRecordViewed(recordId: string): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  try {
    await invoke('mark_record_viewed', { code: recordId });

    // 更新本地状态
    updateRecordInCache(recordId, { viewed: true });

    // 如果当前选中的记录是这个，也要更新
    if (appState.selectedRecord?.id === recordId) {
      appState.selectedRecord.viewed = true;
    }
  } catch (error) {
    console.error('Failed to mark record as viewed:', error);
    throw error;
  }
}

export async function markRecordLiked(recordId: string): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  try {
    await invoke('mark_record_liked', { code: recordId });

    // 更新本地状态
    updateRecordInCache(recordId, { is_liked: true });

    // 如果当前选中的记录是这个，也要更新
    if (appState.selectedRecord?.id === recordId) {
      appState.selectedRecord.is_liked = true;
    }
  } catch (error) {
    console.error('Failed to mark record as liked:', error);
    throw error;
  }
}

export async function markRecordUnliked(recordId: string): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  try {
    await invoke('mark_record_unliked', { code: recordId });

    // 更新本地状态
    updateRecordInCache(recordId, { is_liked: false });

    // 如果当前选中的记录是这个，也要更新
    if (appState.selectedRecord?.id === recordId) {
      appState.selectedRecord.is_liked = false;
    }
  } catch (error) {
    console.error('Failed to mark record as unliked:', error);
    throw error;
  }
}

// 辅助函数：更新缓存中的记录
function updateRecordInCache(recordId: string, updates: Partial<RecordModel>): void {
  const record = cacheState.records.data.find(r => r.id === recordId);
  if (record) {
    Object.assign(record, updates);
  }
}
