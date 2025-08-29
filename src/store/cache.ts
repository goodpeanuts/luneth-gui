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
  images: {},
});

// 缓存管理函数
const CACHE_EXPIRE_TIME = 5 * 60 * 1000; // 5 minutes cache expiration time
const IMAGE_CACHE_EXPIRE_TIME = 30 * 60 * 1000; // 30 minutes for images

export function isCacheValid(timestamp: number): boolean {
  return Date.now() - timestamp < CACHE_EXPIRE_TIME;
}

export function isImageCacheValid(timestamp: number): boolean {
  return Date.now() - timestamp < IMAGE_CACHE_EXPIRE_TIME;
}

export function setCachedRecords(records: RecordModel[]) {
  cacheState.records.data = records;
  cacheState.records.timestamp = Date.now();
}

export function getCachedRecords(): RecordModel[] {
  return cacheState.records.data; // 总是返回缓存的数据，不检查过期时间
}

export function isCachedRecordsValid(): boolean {
  return cacheState.records.data.length > 0 && isCacheValid(cacheState.records.timestamp);
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

// 图片缓存管理函数
export function setCachedImage(cacheKey: string, url: string, isLocal: boolean): void {
  cacheState.images[cacheKey] = {
    url,
    isLocal,
    timestamp: Date.now(),
  };
}

export function getCachedImage(cacheKey: string): string | null {
  const cached = cacheState.images[cacheKey];
  if (cached && isImageCacheValid(cached.timestamp)) {
    return cached.url;
  }
  return null;
}

export function getCachedImageInfo(cacheKey: string): { url: string; isLocal: boolean } | null {
  const cached = cacheState.images[cacheKey];
  if (cached && isImageCacheValid(cached.timestamp)) {
    return { url: cached.url, isLocal: cached.isLocal };
  }
  return null;
}

export function removeCachedImage(cacheKey: string): void {
  delete cacheState.images[cacheKey];
}

export function clearExpiredImageCache(): void {
  Object.keys(cacheState.images).forEach(cacheKey => {
    const cached = cacheState.images[cacheKey];
    if (!isImageCacheValid(cached.timestamp)) {
      // 如果是本地blob URL，需要释放
      if (cached.isLocal && cached.url.startsWith('blob:')) {
        URL.revokeObjectURL(cached.url);
      }
      delete cacheState.images[cacheKey];
    }
  });
}
