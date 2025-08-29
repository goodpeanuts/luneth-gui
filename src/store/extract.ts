// Extract 页面状态管理

import { reactive } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// 类型定义
interface LineResult {
  line_number: number;
  original_text: string;
  extracted_content?: string;
  status: 'Normal' | 'Error' | 'Duplicate' | 'Selected';
}

interface ProcessResult {
  input_lines: LineResult[];
  output_lines: string[];
  duplicate_groups: Record<string, number[]>;
}

// Extract 页面全局状态
export const extractState = reactive({
  inputText: '',
  processResult: null as ProcessResult | null,
  isProcessing: false,
  existRecords: [] as string[], // 缓存的已存在记录
  filterExistRecords: false, // 是否过滤已存在记录
  filteredOutputLines: [] as string[], // 过滤后的输出结果
});

// 获取已存在记录
export async function fetchExistRecords(): Promise<void> {
  try {
    const records = await invoke<string[]>('get_all_exist_records');
    extractState.existRecords = records;
    updateFilteredOutput();
  } catch (error) {
    console.error('获取已存在记录失败:', error);
  }
}

// 更新过滤后的输出
export function updateFilteredOutput(): void {
  if (!extractState.processResult) {
    extractState.filteredOutputLines = [];
    return;
  }

  if (extractState.filterExistRecords) {
    // 过滤掉已存在的记录
    extractState.filteredOutputLines = extractState.processResult.output_lines.filter(
      line => !extractState.existRecords.includes(line)
    );
  } else {
    // 不过滤，显示所有结果 - 确保创建新数组避免引用问题
    extractState.filteredOutputLines = [...extractState.processResult.output_lines];
  }
}

// 设置过滤状态
export function setFilterExistRecords(filter: boolean): void {
  extractState.filterExistRecords = filter;
  updateFilteredOutput();
}

// 处理文本
export async function processText(): Promise<void> {
  if (!extractState.inputText.trim()) {
    return;
  }

  extractState.isProcessing = true;
  try {
    const result = await invoke<ProcessResult>("process_text", {
      input: extractState.inputText
    });
    extractState.processResult = result;
    updateFilteredOutput();
  } catch (error) {
    console.error("处理文本时出错:", error);
  } finally {
    extractState.isProcessing = false;
  }
}

// 切换行选中状态
export async function toggleLineSelection(extractedContent: string): Promise<void> {
  if (!extractState.processResult) return;

  try {
    const result = await invoke<ProcessResult>("toggle_line_selection", {
      processResult: extractState.processResult,
      extractedContent
    });
    extractState.processResult = result;
    updateFilteredOutput();
  } catch (error) {
    console.error("切换行选中状态时出错:", error);
  }
}

// 重置所有状态
export function resetAll(): void {
  extractState.inputText = "";
  extractState.processResult = null;
  extractState.filteredOutputLines = [];
}

// 清除选中状态
export function clearSelection(): void {
  if (!extractState.processResult) return;

  // 将所有 Selected 状态恢复为 Duplicate
  extractState.processResult.input_lines.forEach(line => {
    if (line.status === 'Selected') {
      line.status = 'Duplicate';
    }
  });
  updateFilteredOutput();
}

// 获取当前显示的输出内容（用于复制到 batch 任务）
export function getCurrentOutputContent(): string {
  return extractState.filteredOutputLines.join('\n');
}

// 统计函数
export function getSuccessCount(): number {
  if (!extractState.processResult) return 0;
  return extractState.processResult.input_lines.filter(line =>
    line.status !== 'Error'
  ).length;
}

export function getErrorCount(): number {
  if (!extractState.processResult) return 0;
  return extractState.processResult.input_lines.filter(line =>
    line.status === 'Error'
  ).length;
}

export function getDuplicateCount(): number {
  if (!extractState.processResult) return 0;
  return Object.keys(extractState.processResult.duplicate_groups).length;
}

export function hasSelectedLines(): boolean {
  if (!extractState.processResult) return false;
  return extractState.processResult.input_lines.some(line => line.status === 'Selected');
}
