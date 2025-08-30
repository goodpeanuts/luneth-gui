// 简化的缓存状态管理 - 移除图片缓存和记录缓存

import { reactive } from 'vue';
import type { HistoryOpModel } from '@/types/record';

// 基础缓存状态 - 现在只保留历史记录缓存
export interface CacheState {
  history: {
    data: HistoryOpModel[];
    timestamp: number;
    isLoading: boolean;
  };
}

// 全局缓存状态
export const cacheState = reactive<CacheState>({
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

export function setHistoryLoading(isLoading: boolean) {
  cacheState.history.isLoading = isLoading;
}
