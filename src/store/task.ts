// 任务状态管理

import { reactive } from 'vue';
import type {
  ScrapTaskState,
  ManageTasksState,
  ManageTaskStatus,
  ManageTaskProgress
} from '@/types/task';

// 爬取任务状态
export const scrapTaskState = reactive<ScrapTaskState>({
  isProcessing: false,
  taskType: null,
  inputValue: '',
});

// 管理任务状态
export const manageTaskState = reactive<ManageTasksState>({
  idolCrawl: {
    status: 'idle',
  },
  recordPull: {
    status: 'idle',
  },
  submit: {
    status: 'idle',
  },
  update: {
    status: 'idle',
  },
});

// 重置爬取任务状态
export function resetScrapTaskState() {
  scrapTaskState.isProcessing = false;
  scrapTaskState.taskType = null;
  scrapTaskState.inputValue = '';
}

// 管理任务状态管理函数
export function setManageTaskStatus(taskType: 'idolCrawl' | 'recordPull' | 'submit' | 'update', status: ManageTaskStatus) {
  const task = manageTaskState[taskType];
  task.status = status;

  const now = new Date().toLocaleString();
  if (status === 'running') {
    task.startedAt = now;
    task.completedAt = undefined;
  } else if (status === 'success' || status === 'failed') {
    task.completedAt = now;
  }
}

export function setManageTaskMessage(taskType: 'idolCrawl' | 'recordPull' | 'submit' | 'update', message: string) {
  manageTaskState[taskType].message = message;
}

export function setManageTaskProgress(taskType: 'idolCrawl' | 'recordPull' | 'submit' | 'update', progress: ManageTaskProgress) {
  manageTaskState[taskType].progress = progress;
}

export function resetManageTaskState(taskType: 'idolCrawl' | 'recordPull' | 'submit' | 'update') {
  manageTaskState[taskType] = {
    status: 'idle',
  };
}

// 管理任务状态更新函数
export function updateTaskStatus(taskType: 'idolCrawl' | 'recordPull' | 'submit' | 'update', status: ManageTaskStatus) {
  manageTaskState[taskType].status = status;
  if (status === 'running') {
    manageTaskState[taskType].startedAt = new Date().toISOString();
  } else if (status === 'success' || status === 'failed') {
    manageTaskState[taskType].completedAt = new Date().toISOString();
  }
}

export function updateTaskMessage(taskType: 'idolCrawl' | 'recordPull' | 'submit' | 'update', message: string) {
  manageTaskState[taskType].message = message;
}

export function updateTaskProgress(taskType: 'idolCrawl' | 'recordPull' | 'submit' | 'update', progress: ManageTaskProgress) {
  manageTaskState[taskType].progress = progress;
}
