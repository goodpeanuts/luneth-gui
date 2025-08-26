// 缓存相关类型定义

import type { RecordModel, HistoryOpModel } from './record';

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
