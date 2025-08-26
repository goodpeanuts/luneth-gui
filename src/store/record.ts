// 记录筛选和交互功能

import { reactive } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { RecordFilterOptions, RecordModel } from '@/types/record';
import { updateRecordInCache } from './cache';
import { appState } from './app';

// 记录筛选状态
export const recordFilterState = reactive<RecordFilterOptions>({
  isLiked: null,
  isViewed: null,
  isSubmitted: null,
  hasLocalImages: null,
});

// 记录筛选函数
export function setRecordFilter(options: Partial<RecordFilterOptions>) {
  Object.assign(recordFilterState, options);
}

export function clearRecordFilters() {
  recordFilterState.isLiked = null;
  recordFilterState.isViewed = null;
  recordFilterState.isSubmitted = null;
  recordFilterState.hasLocalImages = null;
}

export function getFilteredRecords(records: RecordModel[]): RecordModel[] {
  return records.filter(record => {
    // Filter by liked status
    if (recordFilterState.isLiked !== null) {
      if (record.is_liked !== recordFilterState.isLiked) {
        return false;
      }
    }

    // Filter by viewed status
    if (recordFilterState.isViewed !== null) {
      if (record.viewed !== recordFilterState.isViewed) {
        return false;
      }
    }

    // Filter by submitted status
    if (recordFilterState.isSubmitted !== null) {
      if (record.is_submitted !== recordFilterState.isSubmitted) {
        return false;
      }
    }

    // Filter by local images
    if (recordFilterState.hasLocalImages !== null) {
      const hasImages = record.local_image_count > 0;
      if (hasImages !== recordFilterState.hasLocalImages) {
        return false;
      }
    }

    return true;
  });
}

// 记录交互功能
export async function markRecordViewed(recordId: string): Promise<void> {
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
