// 应用相关类型定义

export type ViewType = 'home' | 'record_list' | 'history_list' | 'crawl' | 'manage' | 'config' | 'record_detail';

// 客户端认证状态
export interface ClientAuthState {
  isSet: boolean;
  url: string;
}

// 拉取记录状态
export interface PullRecordsState {
  isLoading: boolean;
  lastSuccessTime: string | null;
  lastTotalCount: number | null;
  lastNewCount: number | null;
  lastError: string | null;
}

// 应用状态类型 (RecordModel 通过 index.ts 中的重新导出来处理循环引用)
export interface AppState {
  currentView: ViewType;
  taskBaseUrl: string | null;
  isTaskBaseUrlSet: boolean;
  isProcessing: boolean;
  selectedRecord: any | null; // 使用 any 避免循环引用，实际类型为 RecordModel
  clientAuth: ClientAuthState;
  pullRecordsState: PullRecordsState;
}
