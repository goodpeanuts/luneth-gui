// 任务相关类型定义

// 爬取任务类型
export type ScrapTaskType = 'auto' | 'manual';

// 爬取任务状态
export interface ScrapTaskState {
  isProcessing: boolean;
  taskType: ScrapTaskType | null;
  inputValue: string;
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
  submit: ManageTaskState;
}
