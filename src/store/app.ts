// 应用状态管理

import { reactive } from 'vue';
import type { AppState, ViewType } from '@/types/app';

// 全局应用状态
export const appState = reactive<AppState>({
  currentView: 'home',
  taskBaseUrl: null,
  isTaskBaseUrlSet: false,
  isProcessing: false,
  selectedRecord: null,
  clientAuth: {
    isSet: false,
    url: '',
  },
  pullRecordsState: {
    isLoading: false,
    lastSuccessTime: null,
    lastTotalCount: null,
    lastNewCount: null,
    lastError: null,
  },
});

// 导航方法
export function navigateTo(view: ViewType) {
  appState.currentView = view;
}

// 设置任务基础URL
export function setTaskBaseUrl(url: string) {
  appState.taskBaseUrl = url;
  appState.isTaskBaseUrlSet = true;
}

// 检查是否设置了任务基础URL
export function checkTaskBaseUrl(): boolean {
  return appState.isTaskBaseUrlSet && !!appState.taskBaseUrl;
}

// 设置客户端认证
export function setClientAuth(url: string) {
  appState.clientAuth.isSet = true;
  appState.clientAuth.url = url;
}

// 清除客户端认证
export function clearClientAuth() {
  appState.clientAuth.isSet = false;
  appState.clientAuth.url = '';
}

// 检查是否设置了客户端认证
export function checkClientAuth(): boolean {
  return appState.clientAuth.isSet && !!appState.clientAuth.url;
}

// 设置拉取记录状态
export function setPullRecordsLoading(isLoading: boolean) {
  appState.pullRecordsState.isLoading = isLoading;
}

// 设置拉取记录成功状态
export function setPullRecordsSuccess(totalCount: number, newCount: number) {
  appState.pullRecordsState.lastSuccessTime = new Date().toLocaleString();
  appState.pullRecordsState.lastTotalCount = totalCount;
  appState.pullRecordsState.lastNewCount = newCount;
  appState.pullRecordsState.lastError = null;
  appState.pullRecordsState.isLoading = false;
}

// 设置拉取记录失败状态
export function setPullRecordsError(error: string) {
  appState.pullRecordsState.lastError = error;
  appState.pullRecordsState.lastSuccessTime = new Date().toLocaleString();
  appState.pullRecordsState.lastTotalCount = null;
  appState.pullRecordsState.lastNewCount = null;
  appState.pullRecordsState.isLoading = false;
}
