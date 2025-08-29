// 进度状态管理

import { reactive } from 'vue';
import type { ProgressState, ProgressItem } from '@/types/progress';

// 全局进度状态
export const progressState = reactive<ProgressState>({
  progressList: [],
  isVisible: false,
});

// Progress item management functions
export function createProgressItem(name: string, total: number = -1): ProgressItem {
  const item: ProgressItem = {
    id: `${name}-${Date.now()}`,
    name,
    current: 0,
    total,
    status: 'pending',
    createdAt: Date.now(),
  };
  progressState.progressList.unshift(item); // 使用unshift在开头添加，最新的在上方
  progressState.isVisible = true;
  return item;
}

export function updateProgressItem(id: string, updates: Partial<ProgressItem>) {
  const item = progressState.progressList.find(p => p.id === id);
  if (item) {
    Object.assign(item, updates);
  }
}

export function clearProgressList() {
  progressState.progressList = [];
  progressState.isVisible = false;
}

// Additional progress helper functions
export function findOrCreateProgress(name: string): ProgressItem {
  let item = progressState.progressList.find(p => p.name === name);
  if (!item) {
    item = createProgressItem(name);
  }
  return item;
}

export function updateProgressStatus(item: ProgressItem) {
  // This function can be used to update progress status based on current/total
  if (item.current === item.total && item.total > 0) {
    item.status = 'success';
  } else if (item.current > 0) {
    item.status = 'in-progress';
  }
}

export function resetProgress() {
  clearProgressList();
}

export function showProgress() {
  progressState.isVisible = true;
}

export function hideProgress() {
  progressState.isVisible = false;
}
