<template>
  <div class="record-info">
    <div class="info-sections">
      <!-- Basic Information -->
      <div class="info-section">
        <h3 class="section-title">Basic Information</h3>
        <div class="info-grid">
          <div class="info-item">
            <span class="info-label">ID:</span>
            <span class="info-value">{{ record.id }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Title:</span>
            <span class="info-value">{{ record.title || 'N/A' }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Release Date:</span>
            <span class="info-value">{{ record.release_date || 'N/A' }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Length:</span>
            <span class="info-value">{{ record.length || 'N/A' }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Local Images:</span>
            <span class="info-value">{{ record.local_image_count }}</span>
          </div>
        </div>
      </div>

      <!-- Director -->
      <div v-if="hasDirectors" class="info-section">
        <h3 class="section-title">Director</h3>
        <div class="link-list">
          <a
            v-for="(link, name) in record.director"
            :key="name"
            :href="link"
            target="_blank"
            class="link-item"
          >
            {{ name }}
          </a>
        </div>
      </div>

      <!-- Studio -->
      <div v-if="hasStudios" class="info-section">
        <h3 class="section-title">Studio</h3>
        <div class="link-list">
          <a
            v-for="(link, name) in record.studio"
            :key="name"
            :href="link"
            target="_blank"
            class="link-item"
          >
            {{ name }}
          </a>
        </div>
      </div>

      <!-- Label -->
      <div v-if="hasLabels" class="info-section">
        <h3 class="section-title">Label</h3>
        <div class="link-list">
          <a
            v-for="(link, name) in record.label"
            :key="name"
            :href="link"
            target="_blank"
            class="link-item"
          >
            {{ name }}
          </a>
        </div>
      </div>

      <!-- Series -->
      <div v-if="hasSeries" class="info-section">
        <h3 class="section-title">Series</h3>
        <div class="link-list">
          <a
            v-for="(link, name) in record.series"
            :key="name"
            :href="link"
            target="_blank"
            class="link-item"
          >
            {{ name }}
          </a>
        </div>
      </div>

      <!-- Genre -->
      <div v-if="hasGenres" class="info-section">
        <h3 class="section-title">Genre</h3>
        <div class="tag-list">
          <a
            v-for="(link, name) in record.genre"
            :key="name"
            :href="link"
            target="_blank"
            class="tag-item"
          >
            {{ name }}
          </a>
        </div>
      </div>

      <!-- Idols -->
      <div v-if="hasIdols" class="info-section">
        <h3 class="section-title">Cast</h3>
        <div class="link-list">
          <a
            v-for="(link, name) in record.idols"
            :key="name"
            :href="link"
            target="_blank"
            class="link-item"
          >
            {{ name }}
          </a>
        </div>
      </div>

      <!-- Metadata -->
      <div class="info-section">
        <h3 class="section-title">Metadata</h3>
        <div class="info-grid">
          <div class="info-item">
            <span class="info-label">Source URL:</span>
            <a :href="record.url" target="_blank" class="info-link">
              {{ truncateUrl(record.url) }}
            </a>
          </div>
          <div class="info-item">
            <span class="info-label">Base URL:</span>
            <span class="info-value">{{ record.base_url }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Created:</span>
            <span class="info-value">{{ formatDate(record.created_at) }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Updated:</span>
            <span class="info-value">{{ formatDate(record.updated_at) }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Cached Locally:</span>
            <span class="info-value" :class="{ 'cached': record.is_cached_locally }">
              {{ record.is_cached_locally ? 'Yes' : 'No' }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { RecordModel } from '@/types';

interface Props {
  record: RecordModel;
}

const props = defineProps<Props>();

// 计算属性用于检查是否有相关数据
const hasDirectors = computed(() => Object.keys(props.record.director).length > 0);
const hasStudios = computed(() => Object.keys(props.record.studio).length > 0);
const hasLabels = computed(() => Object.keys(props.record.label).length > 0);
const hasSeries = computed(() => Object.keys(props.record.series).length > 0);
const hasGenres = computed(() => Object.keys(props.record.genre).length > 0);
const hasIdols = computed(() => Object.keys(props.record.idols).length > 0);

function formatDate(dateString: string): string {
  try {
    return new Date(dateString).toLocaleString();
  } catch {
    return 'Invalid date';
  }
}

function truncateUrl(url: string): string {
  if (url.length <= 50) return url;
  return url.substring(0, 47) + '...';
}
</script>

<style scoped>
.record-info {
  height: 100%;
  overflow-y: auto;
}

.info-sections {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.info-section {
  border-bottom: 1px solid #f0f0f0;
  padding-bottom: 20px;
}

.info-section:last-child {
  border-bottom: none;
  padding-bottom: 0;
}

.section-title {
  font-size: 1.2rem;
  font-weight: 600;
  color: #333;
  margin-bottom: 16px;
  padding-bottom: 8px;
  border-bottom: 2px solid #007bff;
}

.info-grid {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.info-item {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  min-height: 24px;
}

.info-label {
  font-weight: 600;
  color: #666;
  min-width: 100px;
  flex-shrink: 0;
}

.info-value {
  color: #333;
  word-break: break-word;
  flex: 1;
}

.info-value.cached {
  color: #28a745;
  font-weight: 600;
}

.info-link {
  color: #007bff;
  text-decoration: none;
  word-break: break-all;
}

.info-link:hover {
  text-decoration: underline;
}

.link-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.link-item {
  color: #007bff;
  text-decoration: none;
  padding: 8px 12px;
  border: 1px solid #e9ecef;
  border-radius: 6px;
  transition: all 0.2s ease;
  background-color: #f8f9fa;
}

.link-item:hover {
  background-color: #e9ecef;
  border-color: #007bff;
  text-decoration: none;
}

.tag-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.tag-item {
  color: #495057;
  text-decoration: none;
  padding: 6px 12px;
  background-color: #e9ecef;
  border-radius: 20px;
  font-size: 0.9rem;
  transition: all 0.2s ease;
}

.tag-item:hover {
  background-color: #007bff;
  color: white;
  text-decoration: none;
}
</style>
