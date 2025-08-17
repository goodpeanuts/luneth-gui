<script setup lang="ts">
import { ref, computed, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import { writeTextFile } from "@tauri-apps/plugin-fs";

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

// 响应式状态
const inputText = ref("");
const processResult = ref<ProcessResult | null>(null);
const isProcessing = ref(false);

// DOM 引用
const textareaRef = ref<HTMLTextAreaElement>();
const lineNumbersRef = ref<HTMLDivElement>();
const highlightOverlayRef = ref<HTMLDivElement>();

// 计算属性：显示的行数
const displayLines = computed(() => {
  const lines = inputText.value.split('\n');
  return lines.length > 0 ? lines : [''];
});

// 滚动同步
function syncScroll() {
  if (textareaRef.value && lineNumbersRef.value) {
    lineNumbersRef.value.scrollTop = textareaRef.value.scrollTop;
  }
  if (textareaRef.value && highlightOverlayRef.value) {
    highlightOverlayRef.value.scrollTop = textareaRef.value.scrollTop;
  }
}

// 更新行号显示
function updateLineNumbers() {
  // 这个函数会触发 displayLines 的重新计算
  nextTick(() => {
    syncScroll();
  });
}

// 处理文本的主函数
async function processText() {
  if (!inputText.value.trim()) {
    return;
  }
  
  isProcessing.value = true;
  try {
    const result = await invoke<ProcessResult>("process_text", { 
      input: inputText.value 
    });
    processResult.value = result;
  } catch (error) {
    console.error("处理文本时出错:", error);
  } finally {
    isProcessing.value = false;
  }
}

// 切换行选中状态
async function toggleLineSelection(extractedContent: string) {
  if (!processResult.value) return;
  
  try {
    const result = await invoke<ProcessResult>("toggle_line_selection", {
      processResult: processResult.value,
      extractedContent
    });
    processResult.value = result;
  } catch (error) {
    console.error("切换行选中状态时出错:", error);
  }
}

// 重置所有状态
function resetAll() {
  inputText.value = "";
  processResult.value = null;
}

// 清除选中状态
function clearSelection() {
  if (!processResult.value) return;
  
  // 将所有 Selected 状态恢复为 Duplicate
  processResult.value.input_lines.forEach(line => {
    if (line.status === 'Selected') {
      line.status = 'Duplicate';
    }
  });
}

// 导出结果
async function exportResult() {
  if (!processResult.value?.output_lines.length) {
    alert('没有可导出的内容');
    return;
  }
  
  try {
    const content = processResult.value.output_lines.join('\n');
    
    console.log('准备保存内容:', content.substring(0, 100) + '...');
    
    // 使用 Tauri 的文件保存对话框
    const filePath = await save({
      filters: [{
        name: '文本文件',
        extensions: ['txt']
      }],
      defaultPath: 'processed_result.txt'
    });
    
    console.log('选择的文件路径:', filePath);
    
    if (filePath) {
      // 写入文件
      await writeTextFile(filePath, content);
      
      console.log('文件写入成功:', filePath);
      alert(`文件导出成功: ${filePath}`);
    } else {
      console.log('用户取消了保存操作');
    }
  } catch (error) {
    console.error('导出文件时出错:', error);
    alert(`导出文件失败: ${error}`);
  }
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

// 处理行点击事件
function handleLineClick(line: LineResult) {
  if (line.status === 'Duplicate' || line.status === 'Selected') {
    if (line.extracted_content) {
      toggleLineSelection(line.extracted_content);
    }
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

// 统计函数
function getSuccessCount(): number {
  if (!processResult.value) return 0;
  return processResult.value.input_lines.filter(line => 
    line.status !== 'Error'
  ).length;
}

function getErrorCount(): number {
  if (!processResult.value) return 0;
  return processResult.value.input_lines.filter(line => 
    line.status === 'Error'
  ).length;
}

function getDuplicateCount(): number {
  if (!processResult.value) return 0;
  return Object.keys(processResult.value.duplicate_groups).length;
}

function hasSelectedLines(): boolean {
  if (!processResult.value) return false;
  return processResult.value.input_lines.some(line => line.status === 'Selected');
}
</script>

<template>
  <div class="app-container">
    <div class="header">
      <h1>文本提取去重工具</h1>
      <p class="subtitle">支持提取邮箱、URL、手机号、身份证号等信息并自动去重</p>
    </div>
    
    <div class="main-layout">
      <!-- 左侧输入面板 -->
      <div class="input-panel">
        <div class="panel-header">
          <h3>输入文本</h3>
          <div class="status-info">
            <span v-if="processResult" class="line-count">
              共 {{ processResult.input_lines.length }} 行
            </span>
          </div>
        </div>
        <div class="text-area-container">
          <div class="line-numbers" ref="lineNumbersRef">
            <div 
              v-for="(_, index) in displayLines" 
              :key="index + 1"
              class="line-number"
            >
              {{ index + 1 }}
            </div>
          </div>
          <textarea
            v-model="inputText"
            ref="textareaRef"
            class="input-textarea"
            @scroll="syncScroll"
            @input="updateLineNumbers"
            placeholder="请在此输入要处理的文本，每行一条记录...

支持提取：
• 邮箱地址：example@domain.com
• 网址：http://example.com
• 手机号：13812345678
• 身份证号：123456789012345678
• 其他：至少6位数字字母组合"
            :readonly="isProcessing"
          ></textarea>
          <div class="highlight-overlay" v-if="processResult" ref="highlightOverlayRef">
            <div 
              v-for="line in processResult.input_lines" 
              :key="line.line_number"
              :class="['highlight-line', getLineClass(line)]"
              @click="handleLineClick(line)"
              :title="getLineTooltip(line)"
            >
            </div>
          </div>
        </div>
        
        <!-- 状态说明 -->
        <div class="legend" v-if="processResult">
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
          :disabled="isProcessing || !inputText.trim()"
          class="control-btn convert-btn"
        >
          <span v-if="isProcessing">处理中...</span>
          <span v-else>转换</span>
        </button>
        
        <button 
          @click="exportResult"
          :disabled="!processResult?.output_lines.length"
          class="control-btn export-btn"
        >
          导出
        </button>
        
        <button 
          @click="clearSelection"
          :disabled="!processResult || !hasSelectedLines()"
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
        
        <!-- 统计信息 -->
        <div class="stats" v-if="processResult">
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
            <span class="stat-value">{{ processResult.output_lines.length }}</span>
          </div>
        </div>
      </div>

      <!-- 右侧输出面板 -->
      <div class="output-panel">
        <div class="panel-header">
          <h3>处理结果</h3>
          <div class="result-info">
            <span v-if="processResult?.output_lines.length" class="result-count">
              共 {{ processResult.output_lines.length }} 条记录
            </span>
          </div>
        </div>
        <div class="output-container">
          <div class="output-content">
            <div v-if="!processResult" class="placeholder">
              <div class="placeholder-icon">📝</div>
              <div class="placeholder-text">点击转换按钮查看处理结果</div>
            </div>
            <div v-else-if="!processResult.output_lines.length" class="placeholder">
              <div class="placeholder-icon">❌</div>
              <div class="placeholder-text">没有提取到有效内容</div>
            </div>
            <div v-else>
              <div 
                v-for="(line, index) in processResult.output_lines" 
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

<style scoped>
.app-container {
  height: 100vh;
  padding: 16px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  flex-direction: column;
}

.header {
  text-align: center;
  color: white;
  margin-bottom: 20px;
}

.header h1 {
  margin: 0 0 8px 0;
  font-size: 28px;
  font-weight: 600;
}

.subtitle {
  margin: 0;
  opacity: 0.9;
  font-size: 14px;
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
  justify-content: between;
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
  top: 0;
  width: 50px;
  height: 100%;
  background: #f8f9fa;
  border-right: 1px solid #e1e5e9;
  font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
  font-size: 12px;
  line-height: 1.5;
  z-index: 2;
  overflow: hidden;
  padding: 12px 0;
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
  overflow: auto;
}

.input-textarea::placeholder {
  color: #6c757d;
  opacity: 0.8;
}

.highlight-overlay {
  position: absolute;
  top: 0;
  left: 60px;
  right: 0;
  height: 100%;
  pointer-events: none;
  font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
  font-size: 14px;
  line-height: 1.5;
  padding: 12px 12px 12px 0;
  overflow: hidden;
  z-index: 1;
}

.highlight-line {
  height: 1.5em;
  pointer-events: auto;
  cursor: pointer;
  margin-bottom: 0;
  border-radius: 3px;
  transition: all 0.2s ease;
  width: 100%;
}

.line-normal {
  background-color: transparent;
  pointer-events: none;
}

.line-error {
  background-color: rgba(220, 53, 69, 0.15);
  border-left: 3px solid #dc3545;
  margin-left: -3px;
}

.line-duplicate {
  background-color: rgba(0, 123, 255, 0.15);
  border-left: 3px solid #007bff;
  margin-left: -3px;
}

.line-selected {
  background-color: rgba(255, 193, 7, 0.25);
  border-left: 3px solid #ffc107;
  margin-left: -3px;
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
  padding-top: 60px;
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

.control-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none !important;
  box-shadow: none !important;
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
  padding: 12px;
  font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
  font-size: 14px;
  line-height: 1.5;
  position: relative;
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
  height: 1.5em;
  align-items: center;
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
  font-size: 12px;
  background: #f8f9fa;
  padding: 0 8px;
}

.output-text {
  flex: 1;
  color: #495057;
  word-break: break-all;
}
</style>

<style>
body {
  margin: 0;
  padding: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
}

#app {
  height: 100vh;
}
</style>