<template>
  <div class="app-container">
    <div class="main-layout">
      <!-- 左侧输入面板 -->
      <div class="input-panel">
        <div class="panel-header">
          <h3>输入文本</h3>
          <div class="status-info">
            <span v-if="extractState.processResult" class="line-count">
              共 {{ extractState.processResult.input_lines.length }} 行
            </span>
          </div>
        </div>
        <div class="text-area-container" ref="textAreaContainer">
          <div class="line-numbers" v-if="extractState.processResult || extractState.inputText" ref="lineNumbers">
            <div
              v-for="(_, index) in getDisplayLines()"
              :key="index"
              class="line-number"
            >
              {{ index + 1 }}
            </div>
          </div>
          <textarea
            v-model="extractState.inputText"
            class="input-textarea"
            :class="{ 'input-hidden': extractState.processResult }"
            placeholder="请在此输入要处理的文本，每行一条记录...

支持提取：
• 邮箱地址：example@domain.com
• 网址：http://example.com
• 手机号：13812345678
• 身份证号：123456789012345678
• 其他：至少6位数字字母组合"
            :readonly="extractState.isProcessing"
            @scroll="handleTextareaScroll"
            ref="textareaElement"
          ></textarea>
          <div class="text-overlay" v-if="extractState.processResult" @scroll="handleOverlayScroll" ref="textOverlay">
            <div
              v-for="line in extractState.processResult.input_lines"
              :key="line.line_number"
              :class="['text-line', getLineClass(line)]"
              @click="handleLineClick(line)"
              :title="getLineTooltip(line)"
            >
              {{ line.original_text }}
            </div>
          </div>
        </div>

        <!-- 状态说明 -->
        <div class="legend" v-if="extractState.processResult">
          <div class="legend-item">
            <span class="legend-color legend-normal"></span>
            <span>正常提取</span>
          </div>
          <div class="legend-item">
            <span class="legend-color legend-error"></span>
            <span>提取失败</span>
          </div>
          <div class="legend-item">
            <span class="legend-color legend-duplicate"></span>
            <span>存在重复</span>
          </div>
          <div class="legend-item">
            <span class="legend-color legend-selected"></span>
            <span>当前选中</span>
          </div>
        </div>
      </div>

      <!-- 中间控制面板 -->
      <div class="control-panel">
        <button
          @click="processText"
          :disabled="extractState.isProcessing || !extractState.inputText.trim()"
          class="control-btn convert-btn"
        >
          <span v-if="extractState.isProcessing">处理中...</span>
          <span v-else>转换</span>
        </button>

        <button
          @click="exportResult"
          :disabled="!extractState.filteredOutputLines.length"
          class="control-btn export-btn"
        >
          导出
        </button>

        <button
          @click="clearSelection"
          :disabled="!extractState.processResult || !hasSelectedLines()"
          class="control-btn clear-btn"
        >
          清除选中
        </button>

        <button
          @click="resetAll"
          class="control-btn reset-btn"
        >
          重置
        </button>

        <!-- 过滤已存在记录的勾选框 -->
        <div class="filter-section">
          <label class="filter-checkbox">
            <input
              type="checkbox"
              v-model="extractState.filterExistRecords"
              @change="onFilterChange"
            />
            <span class="checkmark"></span>
            <span class="filter-label">过滤已存在记录</span>
          </label>
          <div class="filter-hint">
            勾选后将过滤掉数据库中已存在的记录
          </div>
        </div>

        <!-- 跳转到任务页面按钮 -->
        <button
          @click="copyToTaskPage"
          :disabled="!extractState.filteredOutputLines.length"
          class="control-btn task-btn"
        >
          批量任务
        </button>

        <!-- 统计信息 -->
        <div class="stats" v-if="extractState.processResult">
          <div class="stat-item">
            <span class="stat-label">提取成功：</span>
            <span class="stat-value">{{ getSuccessCount() }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">提取失败：</span>
            <span class="stat-value">{{ getErrorCount() }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">重复项：</span>
            <span class="stat-value">{{ getDuplicateCount() }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">最终结果：</span>
            <span class="stat-value">{{ extractState.filteredOutputLines.length }}</span>
          </div>
        </div>
      </div>

      <!-- 右侧输出面板 -->
      <div class="output-panel">
        <div class="panel-header">
          <h3>处理结果</h3>
          <div class="result-info">
            <span v-if="extractState.filteredOutputLines.length" class="result-count">
              共 {{ extractState.filteredOutputLines.length }} 条记录
            </span>
          </div>
        </div>
        <div class="output-container">
          <div class="output-content">
            <div v-if="!extractState.processResult" class="placeholder">
              <div class="placeholder-icon">📝</div>
              <div class="placeholder-text">点击转换按钮查看处理结果</div>
            </div>
            <div v-else-if="!extractState.filteredOutputLines.length" class="placeholder">
              <div class="placeholder-icon">❌</div>
              <div class="placeholder-text">没有提取到有效内容</div>
            </div>
            <div v-else>
              <div
                v-for="(line, index) in extractState.filteredOutputLines"
                :key="index"
                class="output-line"
              >
                <span class="output-line-number">{{ index + 1 }}</span>
                <span class="output-text">{{ line }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { save } from "@tauri-apps/plugin-dialog";
import { writeTextFile } from "@tauri-apps/plugin-fs";
import {
  extractState,
  processText as processTextAction,
  toggleLineSelection as toggleLineSelectionAction,
  resetAll as resetAllAction,
  clearSelection as clearSelectionAction,
  fetchExistRecords,
  getCurrentOutputContent,
  getSuccessCount,
  getErrorCount,
  getDuplicateCount,
  hasSelectedLines
} from '@/store/extract';
import { navigateTo } from '@/store/app';
import { scrapTaskState } from '@/store/task';

// 类型定义
interface LineResult {
  line_number: number;
  original_text: string;
  extracted_content?: string;
  status: 'Normal' | 'Error' | 'Duplicate' | 'Selected';
}

// 模板引用
const textAreaContainer = ref<HTMLElement>();
const lineNumbers = ref<HTMLElement>();
const textareaElement = ref<HTMLTextAreaElement>();
const textOverlay = ref<HTMLElement>();

// 组件初始化
onMounted(async () => {
  // 初始化时获取已存在记录
  if (!extractState.existRecords.length) {
    await fetchExistRecords();
  }
});

// 处理textarea滚动，同步行号滚动
function handleTextareaScroll() {
  if (textareaElement.value && lineNumbers.value) {
    lineNumbers.value.scrollTop = textareaElement.value.scrollTop;
  }
}

// 处理overlay滚动，同步行号滚动
function handleOverlayScroll() {
  if (textOverlay.value && lineNumbers.value) {
    lineNumbers.value.scrollTop = textOverlay.value.scrollTop;
  }
}

// 获取显示行数（用于行号显示）
function getDisplayLines(): string[] {
  if (extractState.processResult) {
    return extractState.processResult.input_lines.map(line => line.original_text);
  } else if (extractState.inputText) {
    return extractState.inputText.split('\n');
  }
  return [];
}

// 处理文本的主函数
async function processText() {
  await processTextAction();
  // 处理完成后，总是获取最新的已存在记录来更新缓存
  await fetchExistRecords();
}

// 切换行选中状态
async function handleLineClick(line: LineResult) {
  if (line.status === 'Duplicate' || line.status === 'Selected') {
    if (line.extracted_content) {
      await toggleLineSelectionAction(line.extracted_content);
    }
  }
}

// 重置所有状态
function resetAll() {
  resetAllAction();
}

// 清除选中状态
function clearSelection() {
  clearSelectionAction();
}

// 导出结果
async function exportResult() {
  if (!extractState.filteredOutputLines.length) {
    return;
  }

  try {
    const content = extractState.filteredOutputLines.join('\n');

    // 使用 Tauri 的文件保存对话框
    const filePath = await save({
      filters: [{
        name: '文本文件',
        extensions: ['txt']
      }],
      defaultPath: 'processed_result.txt'
    });

    if (filePath) {
      // 写入文件
      await writeTextFile(filePath, content);

      // 可以添加成功提示
      console.log('文件导出成功:', filePath);
    }
  } catch (error) {
    console.error('导出文件时出错:', error);
  }
}

// 过滤状态变化处理
async function onFilterChange() {
  if (extractState.filterExistRecords) {
    // 勾选时，获取最新的已存在记录
    await fetchExistRecords();
  }
  // 状态会在 store 中自动更新过滤结果
}

// 复制到任务页面
function copyToTaskPage() {
  if (!extractState.filteredOutputLines.length) {
    return;
  }

  // 将当前输出内容复制到 batch 任务的输入框
  const content = getCurrentOutputContent();
  scrapTaskState.inputValue = content;

  // 跳转到任务页面并选中 batch 任务
  navigateTo('task');
}

// 获取行的CSS类
function getLineClass(line: LineResult): string {
  switch (line.status) {
    case 'Error': return 'line-error';
    case 'Duplicate': return 'line-duplicate';
    case 'Selected': return 'line-selected';
    default: return 'line-normal';
  }
}

// 获取行的提示信息
function getLineTooltip(line: LineResult): string {
  switch (line.status) {
    case 'Error':
      return '无法提取有效内容';
    case 'Duplicate':
      return `提取内容: ${line.extracted_content || '无'} (点击查看重复项)`;
    case 'Selected':
      return `提取内容: ${line.extracted_content || '无'} (点击取消选中)`;
    default:
      return `提取内容: ${line.extracted_content || '无'}`;
  }
}
</script>

<style scoped>
.app-container {
  height: 100vh;
  padding: 20px;
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
  display: flex;
  flex-direction: column;
}

.main-layout {
  display: flex;
  flex: 1;
  gap: 16px;
  min-height: 0;
}

/* 左侧输入面板 */
.input-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: white;
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.panel-header h3 {
  margin: 0;
  color: #333;
  font-size: 18px;
  font-weight: 600;
}

.status-info, .result-info {
  font-size: 12px;
  color: #666;
}

.line-count, .result-count {
  background: #f0f0f0;
  padding: 2px 8px;
  border-radius: 12px;
}

.text-area-container {
  position: relative;
  flex: 1;
  border: 2px solid #e1e5e9;
  border-radius: 8px;
  overflow: hidden;
  min-height: 300px;
}

.line-numbers {
  position: absolute;
  left: 0;
  top: 12px;
  width: 50px;
  height: calc(100% - 24px);
  background: #f8f9fa;
  border-right: 1px solid #e1e5e9;
  font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
  font-size: 12px;
  line-height: 1.5;
  z-index: 2;
  overflow: hidden;
}

.line-number {
  text-align: right;
  padding: 0 8px;
  color: #6c757d;
  height: 1.5em;
  font-weight: 500;
}

.input-textarea {
  width: 100%;
  height: 100%;
  border: none;
  outline: none;
  resize: none;
  padding: 12px;
  padding-left: 60px;
  font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
  font-size: 14px;
  line-height: 1.5;
  background: transparent;
  color: #495057;
}

.input-textarea.input-hidden {
  color: transparent;
}

.input-textarea::placeholder {
  color: #6c757d;
  opacity: 0.8;
}

.text-overlay {
  position: absolute;
  top: 12px;
  left: 60px;
  right: 12px;
  bottom: 12px;
  pointer-events: none;
  font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
  font-size: 14px;
  line-height: 1.5;
  padding: 0;
  overflow: auto; /* 允许滚动 */
  color: #495057;
}

.text-line {
  height: 1.5em;
  pointer-events: auto;
  cursor: pointer;
  margin-bottom: 0;
  border-radius: 3px;
  transition: all 0.2s ease;
}

.line-normal {
  background-color: transparent;
}

.line-error {
  background-color: rgba(220, 53, 69, 0.15);
  border-left: 3px solid #dc3545;
  padding-left: 5px;
}

.line-duplicate {
  background-color: rgba(0, 123, 255, 0.15);
  border-left: 3px solid #007bff;
  padding-left: 5px;
}

.line-selected {
  background-color: rgba(255, 193, 7, 0.25);
  border-left: 3px solid #ffc107;
  padding-left: 5px;
}

.line-duplicate:hover, .line-selected:hover {
  transform: translateX(2px);
}

/* 图例 */
.legend {
  display: flex;
  gap: 16px;
  margin-top: 12px;
  padding: 12px;
  background: #f8f9fa;
  border-radius: 6px;
  font-size: 12px;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 6px;
}

.legend-color {
  width: 12px;
  height: 12px;
  border-radius: 2px;
  border: 1px solid #ddd;
}

.legend-normal { background: white; }
.legend-error { background: rgba(220, 53, 69, 0.15); border-color: #dc3545; }
.legend-duplicate { background: rgba(0, 123, 255, 0.15); border-color: #007bff; }
.legend-selected { background: rgba(255, 193, 7, 0.25); border-color: #ffc107; }

/* 中间控制面板 */
.control-panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
  justify-content: flex-start;
  min-width: 140px;
  padding-top: 20px;
}

.control-btn {
  padding: 14px 20px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 600;
  transition: all 0.3s ease;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.convert-btn {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
}

.convert-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(102, 126, 234, 0.6);
}

.export-btn {
  background: linear-gradient(135deg, #56ab2f 0%, #a8e6cf 100%);
  color: white;
  box-shadow: 0 4px 15px rgba(86, 171, 47, 0.4);
}

.export-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(86, 171, 47, 0.6);
}

.clear-btn {
  background: linear-gradient(135deg, #fd7e14 0%, #ffb347 100%);
  color: white;
  box-shadow: 0 4px 15px rgba(253, 126, 20, 0.4);
}

.clear-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(253, 126, 20, 0.6);
}

.reset-btn {
  background: linear-gradient(135deg, #6c757d 0%, #adb5bd 100%);
  color: white;
  box-shadow: 0 4px 15px rgba(108, 117, 125, 0.4);
}

.reset-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(108, 117, 125, 0.6);
}

.task-btn {
  background: linear-gradient(135deg, #e91e63 0%, #f06292 100%);
  color: white;
  box-shadow: 0 4px 15px rgba(233, 30, 99, 0.4);
}

.task-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(233, 30, 99, 0.6);
}

.control-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none !important;
  box-shadow: none !important;
}

/* 过滤器部分 */
.filter-section {
  background: white;
  padding: 16px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  margin: 8px 0;
}

.filter-checkbox {
  display: flex;
  align-items: center;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
}

.filter-checkbox input[type="checkbox"] {
  margin-right: 8px;
  transform: scale(1.2);
}

.filter-label {
  color: #333;
}

.filter-hint {
  font-size: 12px;
  color: #666;
  margin-top: 8px;
  line-height: 1.4;
}

.stats {
  background: white;
  padding: 16px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  margin-top: 20px;
}

.stat-item {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
  font-size: 12px;
}

.stat-label {
  color: #6c757d;
}

.stat-value {
  font-weight: 600;
  color: #495057;
}

/* 右侧输出面板 */
.output-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: white;
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
}

.output-container {
  flex: 1;
  border: 2px solid #e1e5e9;
  border-radius: 8px;
  overflow: auto;
  min-height: 300px;
}

.output-content {
  padding: 16px;
  font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
  font-size: 14px;
  line-height: 1.5;
  overflow-x: auto;
  white-space: nowrap;
}

.placeholder {
  color: #6c757d;
  text-align: center;
  padding: 60px 16px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.placeholder-icon {
  font-size: 48px;
  opacity: 0.5;
}

.placeholder-text {
  font-size: 16px;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
}

.output-line {
  display: flex;
  margin-bottom: 0;
  padding: 0;
  border-radius: 4px;
  transition: background-color 0.2s ease;
  line-height: 1.5;
  height: 1.5em;
}

.output-line:hover {
  background-color: #f8f9fa;
}

.output-line-number {
  color: #6c757d;
  width: 50px;
  text-align: right;
  margin-right: 16px;
  flex-shrink: 0;
  font-weight: 500;
}

.output-text {
  flex: 1;
  color: #495057;
  white-space: nowrap;
  overflow: hidden;
}
</style>
