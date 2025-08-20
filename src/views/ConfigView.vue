<template>
  <div class="config-view">
    <div class="config-container">
      <h2 class="config-title">Configuration</h2>
      <p class="config-description">
        Set the base URL for scraping tasks. This URL will be used as the foundation for all crawling operations.
      </p>

      <div class="config-form">
        <div class="form-group">
          <label for="baseUrl" class="form-label">Task Base URL</label>
          <input
            id="baseUrl"
            v-model="inputUrl"
            type="url"
            class="form-input"
            :class="{
              'disabled': isUrlSet && !hasChanged,
              'valid': isUrlSet && hasChanged
            }"
            placeholder="https://example.com"
            @input="onUrlInput"
          />
          <div v-if="isUrlSet && !hasChanged" class="current-url">
            Current URL: {{ appState.taskBaseUrl }}
          </div>
        </div>

        <button
          class="submit-btn"
          :class="{
            'disabled': !canSubmit,
            'loading': isSubmitting
          }"
          :disabled="!canSubmit || isSubmitting"
          @click="handleSubmit"
        >
          <span v-if="isSubmitting">Setting...</span>
          <span v-else-if="isUrlSet && !hasChanged">URL Already Set</span>
          <span v-else>Set Base URL</span>
        </button>

        <div v-if="errorMessage" class="error-message">
          {{ errorMessage }}
        </div>

        <div v-if="successMessage" class="success-message">
          {{ successMessage }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { appState, setTaskBaseUrl } from '@/store';

const inputUrl = ref('');
const isSubmitting = ref(false);
const errorMessage = ref('');
const successMessage = ref('');

// 计算属性
const isUrlSet = computed(() => appState.isTaskBaseUrlSet);
const hasChanged = computed(() => inputUrl.value !== (appState.taskBaseUrl || ''));
const canSubmit = computed(() => inputUrl.value.trim() !== '' && hasChanged.value);

// 组件挂载时的处理
onMounted(() => {
  if (appState.taskBaseUrl) {
    inputUrl.value = appState.taskBaseUrl;
  }
});

// 输入处理
function onUrlInput() {
  errorMessage.value = '';
  successMessage.value = '';
}

// 提交处理
async function handleSubmit() {
  if (!canSubmit.value || isSubmitting.value) return;

  isSubmitting.value = true;
  errorMessage.value = '';
  successMessage.value = '';

  try {
    await invoke('set_task_base_url', { url: inputUrl.value.trim() });
    setTaskBaseUrl(inputUrl.value.trim());
    successMessage.value = 'Base URL set successfully!';
  } catch (error) {
    errorMessage.value = `Failed to set base URL: ${error}`;
  } finally {
    isSubmitting.value = false;
  }
}
</script>

<style scoped>
.config-view {
  padding: 40px;
  max-width: 600px;
  margin: 0 auto;
}

.config-container {
  background: white;
  border-radius: 12px;
  padding: 40px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
}

.config-title {
  font-size: 2rem;
  font-weight: 700;
  color: #333;
  margin-bottom: 16px;
  text-align: center;
}

.config-description {
  color: #666;
  font-size: 1.1rem;
  line-height: 1.6;
  margin-bottom: 32px;
  text-align: center;
}

.config-form {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-label {
  font-weight: 600;
  color: #333;
  font-size: 1.1rem;
}

.form-input {
  padding: 12px 16px;
  border: 2px solid #e1e5e9;
  border-radius: 8px;
  font-size: 1rem;
  transition: all 0.3s ease;
}

.form-input:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
}

.form-input.disabled {
  background-color: #f8f9fa;
  color: #6c757d;
}

.form-input.valid {
  border-color: #28a745;
}

.current-url {
  font-size: 0.9rem;
  color: #6c757d;
  font-style: italic;
  margin-top: 4px;
}

.submit-btn {
  padding: 14px 28px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 1.1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  align-self: center;
  min-width: 180px;
}

.submit-btn:hover:not(.disabled) {
  background-color: #0056b3;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 123, 255, 0.3);
}

.submit-btn.disabled {
  background-color: #6c757d;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.submit-btn.loading {
  background-color: #007bff;
  cursor: wait;
}

.error-message {
  color: #dc3545;
  background-color: #f8d7da;
  border: 1px solid #f5c6cb;
  border-radius: 6px;
  padding: 12px 16px;
  font-size: 0.95rem;
  text-align: center;
}

.success-message {
  color: #155724;
  background-color: #d4edda;
  border: 1px solid #c3e6cb;
  border-radius: 6px;
  padding: 12px 16px;
  font-size: 0.95rem;
  text-align: center;
}
</style>
