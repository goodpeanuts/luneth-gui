// 数据类型定义

export interface RecordModel {
  id: string;
  url: string;
  html: string;
  base_url: string;
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
  id: string;
  operation: string;
  timestamp: string;
  status: string;
  user: string;
  created_at: string;
  updated_at: string;
}

// 应用状态类型
export interface AppState {
  currentView: ViewType;
  taskBaseUrl: string | null;
  isTaskBaseUrlSet: boolean;
  isProcessing: boolean;
  selectedRecord: RecordModel | null;
}

export type ViewType = 'home' | 'record_list' | 'history_list' | 'crawl' | 'config' | 'record_detail';

// 爬取任务类型
export type ScrapTaskType = 'auto' | 'manual';

// 爬取任务状态
export interface ScrapTaskState {
  isProcessing: boolean;
  taskType: ScrapTaskType | null;
  inputValue: string;
}
