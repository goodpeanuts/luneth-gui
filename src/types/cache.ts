// 缓存相关类型定义

import type { HistoryOpModel } from './record';

// 缓存状态接口 - 简化版，移除记录和图片缓存
export interface CacheState {
  history: {
    data: HistoryOpModel[];
    timestamp: number;
    isLoading: boolean;
  };
}
