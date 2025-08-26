// 缓存状态管理

import { reactive } from 'vue';
import type { CacheState } from '@/types/cache';
import type { RecordModel, HistoryOpModel } from '@/types/record';

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

// 辅助函数：更新缓存中的记录
export function updateRecordInCache(recordId: string, updates: Partial<RecordModel>): void {
  const record = cacheState.records.data.find(r => r.id === recordId);
  if (record) {
    Object.assign(record, updates);
  }
}
