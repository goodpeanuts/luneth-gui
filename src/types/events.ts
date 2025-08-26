// 事件相关类型定义

// 爬取事件接口
export interface CrawlPageStartEvent {
  pageName: string;
}

export interface CrawlPageSuccessEvent {
  pageName: string;
  totalCount: number;
}

export interface CrawlPageFailedEvent {
  pageName: string;
  errorMessage: string;
}

export interface CrawlCodeReportEvent {
  code: string;
  status: 'Success' | 'Failed';
  message: string;
}

export interface CrawlCodesFinishedEvent {
  successCount: number;
  errorCount: number;
  totalCount: number;
}

export interface CrawlManualStartEvent {
  totalCount: number;
}

// 管理任务事件接口
export interface IdolCrawlStartEvent {
  message: string;
}

export interface IdolCrawlProgressEvent {
  processed: number;
  total: number;
  message: string;
}

export interface IdolCrawlCompleteEvent {
  total: number;
  successful: number;
  message: string;
}

export interface IdolCrawlFailedEvent {
  error: string;
}

export interface RecordPullStartEvent {
  message: string;
}

export interface RecordPullProgressEvent {
  processed: number;
  total: number;
  message: string;
}

export interface RecordPullCompleteEvent {
  total: number;
  newCount: number;
  message: string;
}

export interface RecordPullFailedEvent {
  error: string;
}

// 提交任务事件接口
export interface SubmitStartEvent {
  totalCount: number;
}

export interface SubmitCodeStartEvent {
  code: string;
}

export interface SubmitCodeResultEvent {
  code: string;
  status: 'Success' | 'Failed';
  message: string;
}

export interface SubmitFinishedEvent {
  successCount: number;
  errorCount: number;
  totalCount: number;
}
