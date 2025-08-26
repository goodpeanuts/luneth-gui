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

export interface BatchCrawlStartEvent {
  totalCount: number;
}

// 管理任务事件接口
export interface IdolCrawlStartEvent {
  message: string;
}

export interface IdolCrawlProgressEvent {
  processed: number;
  message: string;
}

export interface IdolCrawlCompleteEvent {
  successCount: number;
  totalCount: number;
}

export interface IdolCrawlFailedEvent {
  errorMessage: string;
}

export interface RecordPullStartEvent {
  totalCount: number;
}

export interface RecordPullProgressEvent {
  processed: number;
  message: string;
}

export interface RecordPullCompleteEvent {
  successCount: number;
  totalCount: number;
}

export interface RecordPullFailedEvent {
  errorMessage: string;
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
