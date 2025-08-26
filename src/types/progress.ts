// 进度条相关类型定义

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
