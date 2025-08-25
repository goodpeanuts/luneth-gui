// 数据类型定义

export interface RecordModel {
  id: string;
  cover: string;
  title: string;
  release_date: string;
  length: string;
  director: Record<string, string>;
  studio: Record<string, string>;
  label: Record<string, string>;
  series: Record<string, string>;
  genre: Record<string, string>;
  idols: Record<string, string>;
  share_magnet_links: MagnetLink[];
  sample_image_links: string[];
  local_image_count: number;
  viewed: boolean;
  is_liked: boolean;
  is_submitted: boolean;
  is_cached_locally: boolean;
  created_at: string;
  updated_at: string;
}

export interface MagnetLink {
  name: string;
  link: string;
  size: string;
}

export interface HistoryOpModel {
  id: number;
  recorder_id: string;
  operation: string;
  timestamp: string;
  status: string;
  user: string;
  error_message?: string;
  created_at: string;
  updated_at: string;
}

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

// 应用状态类型
export interface AppState {
  currentView: ViewType;
  taskBaseUrl: string | null;
  isTaskBaseUrlSet: boolean;
  isProcessing: boolean;
  selectedRecord: RecordModel | null;
  clientAuth: ClientAuthState;
  pullRecordsState: PullRecordsState;
}

export type ViewType = 'home' | 'record_list' | 'history_list' | 'crawl' | 'manage' | 'config' | 'record_detail';

// 爬取任务类型
export type ScrapTaskType = 'auto' | 'manual';

// 爬取任务状态
export interface ScrapTaskState {
  isProcessing: boolean;
  taskType: ScrapTaskType | null;
  inputValue: string;
}

// 进度条状态类型
export type ProgressStatus = 'pending' | 'in-progress' | 'success' | 'failed' | 'mixed';

// 进度条项目
export interface ProgressItem {
  id: string;
  name: string;
  current: number;
  total: number;
  status: ProgressStatus;
  errorMessage?: string;
  createdAt: number; // 用于排序
}

// 全局进度状态
export interface ProgressState {
  progressList: ProgressItem[];
  isVisible: boolean;
}

// 管理任务状态类型
export type ManageTaskStatus = 'idle' | 'running' | 'success' | 'failed';

// 管理任务进度
export interface ManageTaskProgress {
  processed: number;
  total: number;
}

// 单个管理任务状态
export interface ManageTaskState {
  status: ManageTaskStatus;
  message?: string;
  progress?: ManageTaskProgress;
  startedAt?: string;
  completedAt?: string;
}

// 管理任务全局状态
export interface ManageTasksState {
  idolCrawl: ManageTaskState;
  recordPull: ManageTaskState;
}

// 记录筛选选项
export interface RecordFilterOptions {
  isLiked?: boolean | null; // null = all, true = liked only, false = not liked only
  isViewed?: boolean | null; // null = all, true = viewed only, false = not viewed only
  isSubmitted?: boolean | null; // null = all, true = submitted only, false = not submitted only
  hasLocalImages?: boolean | null; // null = all, true = has local images, false = no local images
}
