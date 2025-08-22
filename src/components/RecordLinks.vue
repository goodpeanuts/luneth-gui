<template>
  <div class="record-links">
    <div v-if="record.share_magnet_links.length === 0" class="no-links">
      <div class="no-links-icon">🔗</div>
      <p>No magnet links available</p>
    </div>

    <div v-else class="links-container">
      <div class="links-header">
        <h3 class="links-title">Magnet Links ({{ record.share_magnet_links.length }})</h3>
        <div class="links-actions">
          <button class="copy-all-btn" @click="copyAllLinks">
            📋 Copy All
          </button>
        </div>
      </div>

      <div class="links-list">
        <div
          v-for="(link, index) in record.share_magnet_links"
          :key="index"
          class="link-item"
        >
          <div class="link-header">
            <div class="link-info">
              <h4 class="link-name">{{ link.name || `Link ${index + 1}` }}</h4>
              <div class="link-meta">
                <span v-if="link.size" class="link-size">{{ link.size }}</span>
              </div>
            </div>
            <div class="link-actions">
              <button
                class="action-btn copy"
                @click="copyLink(link.link)"
                :title="'Copy magnet link'"
              >
                📋
              </button>
              <a
                :href="link.link"
                class="action-btn download"
                :title="'Open magnet link'"
              >
                🧲
              </a>
            </div>
          </div>

          <div class="link-url">
            <code class="magnet-link">{{ truncateLink(link.link) }}</code>
          </div>
        </div>
      </div>
    </div>

    <!-- Copy notification -->
    <div v-if="showCopyNotification" class="copy-notification">
      {{ copyMessage }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import type { RecordModel } from '@/types';

interface Props {
  record: RecordModel;
}

const props = defineProps<Props>();

const showCopyNotification = ref(false);
const copyMessage = ref('');

async function copyLink(link: string) {
  try {
    await navigator.clipboard.writeText(link);
    showCopyMessage('Magnet link copied to clipboard!');
  } catch (error) {
    console.error('Failed to copy link:', error);
    showCopyMessage('Failed to copy link');
  }
}

async function copyAllLinks() {
  try {
    const allLinks = props.record.share_magnet_links
      .map(link => link.link)
      .join('\n');

    await navigator.clipboard.writeText(allLinks);
    showCopyMessage(`${props.record.share_magnet_links.length} links copied to clipboard!`);
  } catch (error) {
    console.error('Failed to copy all links:', error);
    showCopyMessage('Failed to copy links');
  }
}

function showCopyMessage(message: string) {
  copyMessage.value = message;
  showCopyNotification.value = true;

  setTimeout(() => {
    showCopyNotification.value = false;
  }, 2000);
}

function truncateLink(link: string): string {
  if (link.length <= 80) return link;
  return link.substring(0, 77) + '...';
}
</script>

<style scoped>
.record-links {
  height: 100%;
  overflow-y: auto;
  position: relative;
}

.no-links {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 200px;
  gap: 16px;
  color: #6c757d;
}

.no-links-icon {
  font-size: 3rem;
  opacity: 0.6;
}

.links-container {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.links-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 16px;
  border-bottom: 2px solid #007bff;
}

.links-title {
  font-size: 1.2rem;
  font-weight: 600;
  color: #333;
  margin: 0;
}

.links-actions {
  display: flex;
  gap: 8px;
}

.copy-all-btn {
  padding: 6px 12px;
  background-color: #28a745;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: background-color 0.2s ease;
}

.copy-all-btn:hover {
  background-color: #218838;
}

.links-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.link-item {
  background: #f8f9fa;
  border: 1px solid #e9ecef;
  border-radius: 8px;
  padding: 16px;
  transition: all 0.2s ease;
}

.link-item:hover {
  border-color: #007bff;
  box-shadow: 0 2px 8px rgba(0, 123, 255, 0.1);
}

.link-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 12px;
}

.link-info {
  flex: 1;
  min-width: 0;
}

.link-name {
  font-size: 1rem;
  font-weight: 600;
  color: #333;
  margin: 0 0 8px 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.link-meta {
  display: flex;
  gap: 12px;
  align-items: center;
}

.link-size {
  background-color: #e9ecef;
  color: #495057;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.85rem;
  font-weight: 500;
}

.link-actions {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
}

.action-btn {
  width: 32px;
  height: 32px;
  border: 1px solid #e9ecef;
  background: white;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  text-decoration: none;
  color: #666;
  transition: all 0.2s ease;
  font-size: 1rem;
}

.action-btn:hover {
  border-color: #007bff;
  background-color: #f8f9ff;
  color: #007bff;
}

.action-btn.copy:hover {
  border-color: #28a745;
  background-color: #f8fff8;
  color: #28a745;
}

.link-url {
  background: white;
  border: 1px solid #e9ecef;
  border-radius: 4px;
  padding: 8px 12px;
}

.magnet-link {
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 0.85rem;
  color: #495057;
  background: transparent;
  word-break: break-all;
  white-space: pre-wrap;
}

.copy-notification {
  position: fixed;
  top: 20px;
  right: 20px;
  background-color: #28a745;
  color: white;
  padding: 12px 20px;
  border-radius: 6px;
  font-size: 0.9rem;
  font-weight: 500;
  z-index: 1000;
  animation: slideIn 0.3s ease-out;
}

@keyframes slideIn {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}
</style>
