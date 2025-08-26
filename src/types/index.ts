// 统一导出所有类型定义

// 应用相关类型
export type {
  ViewType,
  ClientAuthState,
  PullRecordsState,
  AppState
} from './app';

// 记录相关类型
export type {
  RecordModel,
  MagnetLink,
  HistoryOpModel,
  RecordFilterOptions
} from './record';

// 任务相关类型
export type {
  ScrapTaskType,
  ScrapTaskState,
  ManageTaskStatus,
  ManageTaskProgress,
  ManageTaskState,
  ManageTasksState
} from './task';

// 进度相关类型
export type {
  ProgressStatus,
  ProgressItem,
  ProgressState
} from './progress';

// 缓存相关类型
export type {
  CacheState
} from './cache';

// 事件相关类型
export type {
  CrawlPageStartEvent,
  CrawlPageSuccessEvent,
  CrawlPageFailedEvent,
  CrawlCodeReportEvent,
  CrawlCodesFinishedEvent,
  BatchCrawlStartEvent,
  IdolCrawlStartEvent,
  IdolCrawlProgressEvent,
  IdolCrawlCompleteEvent,
  IdolCrawlFailedEvent,
  RecordPullStartEvent,
  RecordPullProgressEvent,
  RecordPullCompleteEvent,
  RecordPullFailedEvent,
  SubmitStartEvent,
  SubmitCodeStartEvent,
  SubmitCodeResultEvent,
  SubmitFinishedEvent
} from './events';
