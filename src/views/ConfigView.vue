<template>
  <div class="config-view">
    <div class="config-container">
      <h2 class="config-title">Configuration</h2>

      <!-- Task Base URL Section -->
      <div class="config-section">
        <h3 class="section-title">Scraping Configuration</h3>
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
                'disabled': isUrlSet && !hasUrlChanged,
                'valid': isUrlSet && hasUrlChanged
              }"
              :placeholder="appState.taskBaseUrl || 'https://www.example.com'"
              @input="onUrlInput"
            />
            <div v-if="isUrlSet && !hasUrlChanged" class="current-url">
              Current URL: {{ appState.taskBaseUrl }}
            </div>
          </div>

          <div class="form-actions">
            <button
              class="submit-btn"
              :class="{
                'disabled': !canSubmitUrl,
                'loading': isUrlSubmitting
              }"
              :disabled="!canSubmitUrl || isUrlSubmitting"
              @click="handleUrlSubmit"
            >
              <span v-if="isUrlSubmitting">Setting...</span>
              <span v-else-if="isUrlSet && !hasUrlChanged">URL Already Set</span>
              <span v-else>Set Base URL</span>
            </button>

            <button
              v-if="isUrlSet"
              class="reset-btn"
              :disabled="isUrlSubmitting"
              @click="handleUrlReset"
            >
              Reset URL
            </button>
          </div>

          <div v-if="urlErrorMessage" class="error-message">
            {{ urlErrorMessage }}
          </div>

          <div v-if="urlSuccessMessage" class="success-message">
            {{ urlSuccessMessage }}
          </div>
        </div>
      </div>

      <!-- Client Authentication Section -->
      <div class="config-section">
        <h3 class="section-title">Client Authentication</h3>
        <p class="config-description">
          Configure the client authentication settings for pulling records from remote server.
        </p>

        <div class="config-form">
          <div class="form-group">
            <label for="clientUrl" class="form-label">Client Server URL</label>
            <input
              id="clientUrl"
              v-model="inputClientUrl"
              type="url"
              class="form-input"
              :class="{
                'disabled': isClientAuthSet && !hasClientAuthChanged,
                'valid': isClientAuthSet && hasClientAuthChanged
              }"
              :placeholder="appState.clientAuth.url || 'https://api.example.com'"
              @input="onClientAuthInput"
            />
            <div v-if="isClientAuthSet && !hasClientAuthChanged" class="current-url">
              Current URL: {{ appState.clientAuth.url }}
            </div>
          </div>

          <div class="form-group">
            <label for="clientId" class="form-label">Client ID</label>
            <input
              id="clientId"
              v-model="inputClientId"
              type="text"
              class="form-input"
              placeholder="Enter your client ID"
              @input="onClientAuthInput"
            />
          </div>

          <div class="form-group">
            <label for="clientSecret" class="form-label">Client Secret</label>
            <input
              id="clientSecret"
              v-model="inputClientSecret"
              type="password"
              class="form-input"
              placeholder="Enter your client secret"
              @input="onClientAuthInput"
            />
          </div>

          <div class="form-actions">
            <button
              class="submit-btn"
              :class="{
                'disabled': !canSubmitClientAuth,
                'loading': isClientAuthSubmitting
              }"
              :disabled="!canSubmitClientAuth || isClientAuthSubmitting"
              @click="handleClientAuthSubmit"
            >
              <span v-if="isClientAuthSubmitting">Setting...</span>
              <span v-else-if="isClientAuthSet && !hasClientAuthChanged">Auth Already Set</span>
              <span v-else>Set Client Auth</span>
            </button>

            <button
              v-if="isClientAuthSet"
              class="reset-btn"
              :disabled="isClientAuthSubmitting"
              @click="handleClientAuthReset"
            >
              Clear Auth
            </button>
          </div>

          <div v-if="clientAuthErrorMessage" class="error-message">
            {{ clientAuthErrorMessage }}
          </div>

          <div v-if="clientAuthSuccessMessage" class="success-message">
            {{ clientAuthSuccessMessage }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { appState, setTaskBaseUrl, setClientAuth, clearClientAuth } from '@/store';

// Task Base URL related refs
const inputUrl = ref('');
const isUrlSubmitting = ref(false);
const urlErrorMessage = ref('');
const urlSuccessMessage = ref('');

// Client Auth related refs
const inputClientUrl = ref('');
const inputClientId = ref('');
const inputClientSecret = ref('');
const isClientAuthSubmitting = ref(false);
const clientAuthErrorMessage = ref('');
const clientAuthSuccessMessage = ref('');

// Task Base URL computed properties
const isUrlSet = computed(() => appState.isTaskBaseUrlSet);
const hasUrlChanged = computed(() => inputUrl.value !== (appState.taskBaseUrl || ''));
const canSubmitUrl = computed(() => inputUrl.value.trim() !== '' && hasUrlChanged.value);

// Client Auth computed properties
const isClientAuthSet = computed(() => appState.clientAuth.isSet);
const hasClientAuthChanged = computed(() => {
  return inputClientUrl.value !== appState.clientAuth.url ||
         inputClientId.value !== '' ||
         inputClientSecret.value !== '';
});
const canSubmitClientAuth = computed(() => {
  return inputClientUrl.value.trim() !== '' &&
         inputClientId.value.trim() !== '' &&
         inputClientSecret.value.trim() !== '' &&
         hasClientAuthChanged.value;
});

// Component mounted handler
onMounted(() => {
  if (appState.taskBaseUrl) {
    inputUrl.value = appState.taskBaseUrl;
  }
  if (appState.clientAuth.url) {
    inputClientUrl.value = appState.clientAuth.url;
  }
});

// Task Base URL handlers
function onUrlInput() {
  urlErrorMessage.value = '';
  urlSuccessMessage.value = '';
}

async function handleUrlSubmit() {
  if (!canSubmitUrl.value || isUrlSubmitting.value) return;

  isUrlSubmitting.value = true;
  urlErrorMessage.value = '';
  urlSuccessMessage.value = '';

  try {
    await invoke('set_task_base_url', { url: inputUrl.value.trim() });
    setTaskBaseUrl(inputUrl.value.trim());
    urlSuccessMessage.value = 'Base URL set successfully!';
  } catch (error) {
    urlErrorMessage.value = `Failed to set base URL: ${error}`;
  } finally {
    isUrlSubmitting.value = false;
  }
}

async function handleUrlReset() {
  if (isUrlSubmitting.value) return;

  isUrlSubmitting.value = true;
  urlErrorMessage.value = '';
  urlSuccessMessage.value = '';

  try {
    await invoke('set_task_base_url', { url: '' });
    setTaskBaseUrl('');
    inputUrl.value = '';
    urlSuccessMessage.value = 'Base URL reset successfully!';
  } catch (error) {
    urlErrorMessage.value = `Failed to reset base URL: ${error}`;
  } finally {
    isUrlSubmitting.value = false;
  }
}

// Client Auth handlers
function onClientAuthInput() {
  clientAuthErrorMessage.value = '';
  clientAuthSuccessMessage.value = '';
}

async function handleClientAuthSubmit() {
  if (!canSubmitClientAuth.value || isClientAuthSubmitting.value) return;

  isClientAuthSubmitting.value = true;
  clientAuthErrorMessage.value = '';
  clientAuthSuccessMessage.value = '';

  try {
    await invoke('set_client_auth', {
      url: inputClientUrl.value.trim(),
      id: inputClientId.value.trim(),
      secret: inputClientSecret.value.trim()
    });
    setClientAuth(inputClientUrl.value.trim());
    inputClientId.value = '';
    inputClientSecret.value = '';
    clientAuthSuccessMessage.value = 'Client authentication set successfully!';
  } catch (error) {
    clientAuthErrorMessage.value = `Failed to set client auth: ${error}`;
  } finally {
    isClientAuthSubmitting.value = false;
  }
}

async function handleClientAuthReset() {
  if (isClientAuthSubmitting.value) return;

  isClientAuthSubmitting.value = true;
  clientAuthErrorMessage.value = '';
  clientAuthSuccessMessage.value = '';

  try {
    await invoke('clear_client_auth');
    clearClientAuth();
    inputClientUrl.value = '';
    inputClientId.value = '';
    inputClientSecret.value = '';
    clientAuthSuccessMessage.value = 'Client authentication cleared successfully!';
  } catch (error) {
    clientAuthErrorMessage.value = `Failed to clear client auth: ${error}`;
  } finally {
    isClientAuthSubmitting.value = false;
  }
}
</script>

<style scoped>
.config-view {
  padding: 40px;
  max-width: 800px;
  margin: 0 auto;
  min-height: calc(100vh - 80px); /* 确保内容足够高以触发滚动 */
}

.config-container {
  background: white;
  border-radius: 12px;
  padding: 40px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  margin-bottom: 40px; /* 增加底部边距确保滚动空间 */
}

.config-title {
  font-size: 2rem;
  font-weight: 700;
  color: #333;
  margin-bottom: 32px;
  text-align: center;
}

.config-section {
  margin-bottom: 48px;
}

.config-section:last-child {
  margin-bottom: 0;
}

.section-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: #333;
  margin-bottom: 16px;
  border-bottom: 2px solid #e1e5e9;
  padding-bottom: 8px;
}

.config-description {
  color: #666;
  font-size: 1rem;
  line-height: 1.6;
  margin-bottom: 24px;
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
  font-size: 1rem;
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

.form-actions {
  display: flex;
  gap: 16px;
  justify-content: center;
  flex-wrap: wrap;
}

.submit-btn, .reset-btn {
  padding: 14px 28px;
  border: none;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  min-width: 160px;
}

.submit-btn {
  background-color: #007bff;
  color: white;
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

.reset-btn {
  background-color: #dc3545;
  color: white;
}

.reset-btn:hover:not(:disabled) {
  background-color: #c82333;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(220, 53, 69, 0.3);
}

.reset-btn:disabled {
  background-color: #6c757d;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
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

/* 响应式设计 */
@media (max-width: 768px) {
  .config-view {
    padding: 20px;
  }

  .config-container {
    padding: 20px;
  }

  .form-actions {
    flex-direction: column;
    align-items: stretch;
  }

  .submit-btn, .reset-btn {
    min-width: auto;
    width: 100%;
  }
}

@media (max-height: 600px) {
  .config-view {
    padding: 20px;
  }

  .config-container {
    padding: 20px;
  }
}
</style>
