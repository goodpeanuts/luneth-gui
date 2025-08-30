// 分页状态管理

import { reactive } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { RecordModel, RecordFilterOptions } from '@/types/record';

export interface PaginationState {
  currentPage: number;
  pageSize: number;
  totalCount: number;
  totalPages: number;
  records: RecordModel[];
  isLoading: boolean;
  error: string;
  searchQuery: string;
  filters: RecordFilterOptions;
  totalRecordsCount: number; // 添加总记录数（未过滤的）
}

// 全局分页状态
export const paginationState = reactive<PaginationState>({
  currentPage: 1,
  pageSize: 30,
  totalCount: 0,
  totalPages: 0,
  records: [],
  isLoading: true, // 初始状态设为loading
  error: '',
  searchQuery: '',
  filters: {
    isLiked: null,
    isViewed: null,
    isSubmitted: null,
    hasLocalImages: null,
  },
  totalRecordsCount: 0, // 添加总记录数初始值
});

// 构建过滤器数组
function buildFilters(): string[] {
  const filters: string[] = [];

  // 注意：后端期望简单的字符串标识符
  if (paginationState.filters.isLiked === true) {
    filters.push('liked');
  }

  if (paginationState.filters.isViewed === true) {
    filters.push('viewed');
  }

  if (paginationState.filters.isSubmitted === true) {
    filters.push('submit');
  }

  if (paginationState.filters.hasLocalImages === true) {
    filters.push('local');
  }

  // TODO: 搜索功能暂时未实现
  // if (paginationState.searchQuery.trim()) {
  //   filters.push(`search:${paginationState.searchQuery.trim()}`);
  // }

  return filters;
}

// 计算总页数
function calculateTotalPages(totalCount: number, pageSize: number): number {
  return Math.ceil(totalCount / pageSize);
}

// 获取记录总数
export async function fetchRecordCount(): Promise<void> {
  try {
    const filters = buildFilters();
    console.log('[Pagination] Querying record count with filters:', filters);

    // 获取过滤后的记录数
    const count = await invoke<number>('query_record_count', { filters });
    console.log('[Pagination] Filtered record count result:', count);
    paginationState.totalCount = count;
    paginationState.totalPages = calculateTotalPages(count, paginationState.pageSize);

    // 获取总记录数（无过滤）
    const totalCount = await invoke<number>('query_record_count', { filters: [] });
    console.log('[Pagination] Total record count result:', totalCount);
    paginationState.totalRecordsCount = totalCount;

    // 如果当前页超出范围，跳转到最后一页（如果有页面的话）
    if (paginationState.currentPage > paginationState.totalPages && paginationState.totalPages > 0) {
      console.log(`[Pagination] Current page ${paginationState.currentPage} exceeds total pages ${paginationState.totalPages}, jumping to last page`);
      paginationState.currentPage = paginationState.totalPages;
    }
  } catch (error) {
    console.error('[Pagination] Failed to fetch record count:', error);
    paginationState.error = `Failed to fetch record count: ${error}`;
    throw error;
  }
}

// 获取当前页的记录
export async function fetchCurrentPageRecords(): Promise<void> {
  console.log('[Pagination] Starting fetchCurrentPageRecords...');
  paginationState.isLoading = true;
  paginationState.error = '';

  try {
    // 首先获取总数
    console.log('[Pagination] Fetching record count...');
    await fetchRecordCount();
    console.log(`[Pagination] Total count: ${paginationState.totalCount}, Total pages: ${paginationState.totalPages}`);

    // 计算偏移量
    const offset = (paginationState.currentPage - 1) * paginationState.pageSize;
    const filters = buildFilters();
    console.log(`[Pagination] Fetching records with offset: ${offset}, limit: ${paginationState.pageSize}, filters:`, filters);

    // 获取记录
    const records = await invoke<RecordModel[]>('get_local_records_paginator', {
      offset,
      limit: paginationState.pageSize,
      filters,
    });

    paginationState.records = records;
    console.log(`[Pagination] Loaded page ${paginationState.currentPage}, ${records.length} records`);
  } catch (error) {
    console.error('[Pagination] Failed to fetch records:', error);
    paginationState.error = `Failed to fetch records: ${error}`;
    paginationState.records = [];
  } finally {
    paginationState.isLoading = false;
  }
}

// 跳转到指定页面
export async function goToPage(page: number): Promise<void> {
  if (page < 1 || page > paginationState.totalPages) {
    return;
  }

  paginationState.currentPage = page;
  await fetchCurrentPageRecords();
}

// 下一页
export async function nextPage(): Promise<void> {
  if (paginationState.currentPage < paginationState.totalPages) {
    await goToPage(paginationState.currentPage + 1);
  }
}

// 上一页
export async function prevPage(): Promise<void> {
  if (paginationState.currentPage > 1) {
    await goToPage(paginationState.currentPage - 1);
  }
}

// 设置搜索查询
export function setSearchQuery(query: string): void {
  paginationState.searchQuery = query;
}

// 设置过滤器
export function setFilters(filters: Partial<RecordFilterOptions>): void {
  Object.assign(paginationState.filters, filters);
}

// 清除过滤器
export function clearFilters(): void {
  paginationState.filters = {
    isLiked: null,
    isViewed: null,
    isSubmitted: null,
    hasLocalImages: null,
  };
  paginationState.searchQuery = '';
}

// 应用过滤器并重新加载
export async function applyFiltersAndReload(): Promise<void> {
  paginationState.currentPage = 1; // 重置到第一页
  await fetchCurrentPageRecords();
}

// 立即应用过滤器（用于过滤按钮点击）
export async function applyFilterImmediately(filterKey: keyof RecordFilterOptions, value: boolean | null): Promise<void> {
  setFilters({ [filterKey]: value });
  await applyFiltersAndReload();
}

// 刷新当前页
export async function refreshCurrentPage(): Promise<void> {
  await fetchCurrentPageRecords();
}

// 初始化分页状态
export function initializePagination(): void {
  console.log('[Pagination] Initializing pagination state...');
  paginationState.isLoading = true;
  paginationState.error = '';
  paginationState.records = [];
  paginationState.totalCount = 0;
  paginationState.totalPages = 0;
  // 不重置当前页，搜索查询和过滤器，这些应该保持状态
}

// 重置分页状态
export function resetPaginationState(): void {
  paginationState.currentPage = 1;
  paginationState.totalCount = 0;
  paginationState.totalPages = 0;
  paginationState.records = [];
  paginationState.isLoading = false;
  paginationState.error = '';
  paginationState.totalRecordsCount = 0;
}

// 获取页码范围（用于显示页码选择器）
export function getPageRange(): number[] {
  const current = paginationState.currentPage;
  const total = paginationState.totalPages;
  const range: number[] = [];

  if (total <= 7) {
    // 如果总页数不超过7，显示所有页码
    for (let i = 1; i <= total; i++) {
      range.push(i);
    }
  } else {
    // 总是包含第一页
    range.push(1);

    if (current <= 4) {
      // 当前页在前面，显示 1,2,3,4,5...total
      for (let i = 2; i <= 5; i++) {
        range.push(i);
      }
      if (total > 6) {
        range.push(-1); // -1 表示省略号
      }
      range.push(total);
    } else if (current >= total - 3) {
      // 当前页在后面，显示 1...total-4,total-3,total-2,total-1,total
      if (total > 6) {
        range.push(-1);
      }
      for (let i = total - 4; i <= total; i++) {
        if (i > 1) {
          range.push(i);
        }
      }
    } else {
      // 当前页在中间，显示 1...current-1,current,current+1...total
      range.push(-1);
      for (let i = current - 1; i <= current + 1; i++) {
        range.push(i);
      }
      range.push(-1);
      range.push(total);
    }
  }

  return range;
}
