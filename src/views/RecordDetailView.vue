<template>
  <div class="record-detail-view">
    <div v-if="!record" class="no-record">
      <div class="no-record-icon">📝</div>
      <p>No record selected</p>
      <button class="back-btn" @click="navigateTo('record_list')">
        Back to Records
      </button>
    </div>

    <div v-else class="record-container">
      <!-- Header -->
      <div class="record-header">
        <button class="back-btn" @click="navigateTo('record_list')">
          ← Back to Records
        </button>
        <div class="record-actions">
          <button class="action-btn like" :class="{ active: record.is_liked }" @click="toggleLike">
            {{ record.is_liked ? '❤️ Liked' : '🤍 Like' }}
          </button>
          <button class="action-btn submit" :class="{ active: record.is_submitted }">
            {{ record.is_submitted ? '✅ Submitted' : '📤 Submit' }}
          </button>
        </div>
      </div>

      <div class="record-content">
        <!-- Left: Images -->
        <div class="record-images">
          <!-- Cover Image -->
          <div class="cover-section">
            <div class="cover-container">
              <img
                v-if="coverImageSrc"
                :src="coverImageSrc"
                :alt="record.title"
                class="cover-image"
                @error="handleImageError"
                @click="showImagePreview(coverImageSrc)"
              />
              <div v-else class="cover-placeholder">
                <span>🎬</span>
                <p>No cover image</p>
              </div>
            </div>
          </div>

          <!-- Sample Images -->
          <div v-if="sampleImageSrcs.length > 0" class="samples-section">
            <h3 class="samples-title">Sample Images</h3>
            <div class="samples-grid">
              <div v-for="(imageUrl, index) in sampleImageSrcs" :key="index" class="sample-item">
                <img
                  :src="imageUrl"
                  :alt="`Sample ${index + 1}`"
                  class="sample-image"
                  @error="handleImageError"
                  @click="showImagePreview(imageUrl)"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- Right: Information -->
        <div class="record-info">
          <!-- Tab Navigation -->
          <div class="tab-navigation">
            <button class="tab-btn" :class="{ active: activeTab === 'info' }" @click="activeTab = 'info'">
              Details
            </button>
            <button class="tab-btn" :class="{ active: activeTab === 'links' }" @click="activeTab = 'links'">
              Links
            </button>
          </div>

          <!-- Tab Content -->
          <div class="tab-content">
            <!-- Info Tab -->
            <div v-if="activeTab === 'info'" class="info-tab">
              <RecordInfo :record="record" class="record-info-content" />
            </div>

            <!-- Links Tab -->
            <div v-if="activeTab === 'links'" class="links-tab">
              <RecordLinks :record="record" />
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Image Preview Modal -->
    <div v-if="showPreview" class="image-preview-modal" @click="closeImagePreview">
      <div class="preview-container">
        <button class="preview-close" @click="closeImagePreview">&times;</button>
        <img :src="previewImageSrc" alt="Preview" class="preview-image" @click.stop />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { appState, navigateTo, markRecordViewed, markRecordLiked, markRecordUnliked } from '@/store';
import RecordInfo from '@/components/RecordInfo.vue';
import RecordLinks from '@/components/RecordLinks.vue';
import { loadCoverImage, loadSampleImages } from '@/utils/imageLoader';
import type { ImageLoadResult } from '@/utils/imageLoader';

const activeTab = ref<'info' | 'links'>('info');

const record = computed(() => appState.selectedRecord);

// 图片加载状态
const coverImageSrc = ref<string>('');
const sampleImageSrcs = ref<string[]>([]);
const imageLoading = ref(false);

// 图片预览状态
const showPreview = ref(false);
const previewImageSrc = ref<string>('');

// 监听记录变化，加载对应的图片并标记为已查看
watch(record, async (newRecord) => {
  if (!newRecord) {
    coverImageSrc.value = '';
    sampleImageSrcs.value = [];
    return;
  }

  // 标记为已查看（如果还未查看）
  if (!newRecord.viewed) {
    try {
      await markRecordViewed(newRecord.id);
    } catch (error) {
      console.warn('Failed to mark record as viewed:', error);
    }
  }

  imageLoading.value = true;

  try {
    const coverResult = await loadCoverImage(newRecord.id, newRecord.cover);
    coverImageSrc.value = coverResult.src || '';

    let allSampleResults: ImageLoadResult[] = [];

    if (newRecord.sample_image_links.length > 0) {
      // 直接使用 sample_image_links 数组
      allSampleResults = await loadSampleImages(newRecord.id, newRecord.sample_image_links);
    }

    // 设置样例图片
    sampleImageSrcs.value = allSampleResults
      .filter((result: ImageLoadResult) => result.src) // 只保留成功加载的图片
      .map((result: ImageLoadResult) => result.src);

  } catch (error) {
    console.warn('Failed to load images:', error);
    // 如果加载失败，尝试使用原始封面URL，但如果为空则保持空字符串
    coverImageSrc.value = newRecord.cover || '';
    sampleImageSrcs.value = newRecord.sample_image_links || [];
  } finally {
    imageLoading.value = false;
  }
}, { immediate: true });

// 切换喜欢状态
async function toggleLike() {
  if (!record.value) return;

  try {
    if (record.value.is_liked) {
      await markRecordUnliked(record.value.id);
    } else {
      await markRecordLiked(record.value.id);
    }
  } catch (error) {
    console.error('Failed to toggle like status:', error);
    // 可以在这里添加错误提示
  }
}

function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement;
  img.style.display = 'none';
}

// 图片预览功能
function showImagePreview(imageSrc: string) {
  previewImageSrc.value = imageSrc;
  showPreview.value = true;
  // 阻止页面滚动
  document.body.style.overflow = 'hidden';
}

function closeImagePreview() {
  showPreview.value = false;
  previewImageSrc.value = '';
  // 恢复页面滚动
  document.body.style.overflow = '';
}
</script>

<style scoped>
.record-detail-view {
  height: 100vh;
  overflow: hidden;
}

.no-record {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 16px;
}

.no-record-icon {
  font-size: 4rem;
  opacity: 0.6;
}

.record-container {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.record-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid #e9ecef;
  background: white;
}

.back-btn {
  padding: 8px 16px;
  background-color: #6c757d;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: background-color 0.2s ease;
}

.back-btn:hover {
  background-color: #545b62;
}

.record-actions {
  display: flex;
  gap: 12px;
}

.action-btn {
  padding: 8px 16px;
  border: 2px solid #e9ecef;
  border-radius: 6px;
  background: white;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.2s ease;
}

.action-btn.like.active {
  border-color: #dc3545;
  background-color: #f8d7da;
  color: #721c24;
}

.action-btn.submit.active {
  border-color: #28a745;
  background-color: #d4edda;
  color: #155724;
}

.action-btn:hover:not(.active) {
  border-color: #007bff;
  background-color: #f8f9ff;
}

.record-content {
  flex: 1;
  display: flex;
  overflow: hidden;
  min-height: 0; /* 添加最小高度约束 */
}

.record-images {
  width: 45%;
  padding: 24px;
  overflow-y: auto;
  background-color: #f8f9fa;
  flex-shrink: 0;
}

.cover-section {
  margin-bottom: 32px;
}

.cover-container {
  width: 100%;
  max-width: 400px;
  margin: 0 auto;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  cursor: pointer;
}

.cover-image {
  width: 100%;
  height: auto;
  display: block;
  transition: transform 0.2s ease;
}

.cover-image:hover {
  transform: scale(1.02);
}

.cover-placeholder {
  aspect-ratio: 3/4;
  background-color: #e9ecef;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #6c757d;
}

.cover-placeholder span {
  font-size: 4rem;
  margin-bottom: 16px;
}

.samples-section {
  margin-top: 32px;
}

.samples-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: #333;
  margin-bottom: 20px;
  text-align: center;
}

.samples-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 16px;
}

.sample-item {
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  transition: transform 0.2s ease;
  cursor: pointer;
}

.sample-item:hover {
  transform: scale(1.05);
}

.sample-image {
  width: 100%;
  height: 120px;
  object-fit: cover;
  display: block;
  transition: transform 0.2s ease;
}

.record-info {
  width: 55%;
  display: flex;
  flex-direction: column;
  background: white;
  min-width: 0;
  flex-shrink: 0;
}

.tab-navigation {
  display: flex;
  border-bottom: 1px solid #e9ecef;
}

.tab-btn {
  flex: 1;
  padding: 16px 24px;
  border: none;
  background: none;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 500;
  color: #6c757d;
  transition: all 0.2s ease;
  border-bottom: 3px solid transparent;
}

.tab-btn:hover {
  background-color: #f8f9fa;
  color: #333;
}

.tab-btn.active {
  color: #007bff;
  border-bottom-color: #007bff;
  background-color: #f8f9ff;
}

.tab-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
  min-height: 0; /* 重要：允许flex子项缩小 */
  width: 100%;
}

.info-tab {
  height: 100%; /* 确保info tab占据全部高度 */
  width: 100%; /* 确保info tab占据全部宽度 */
}

.links-tab {
  height: 100%; /* 确保links tab占据全部高度 */
  width: 100%; /* 确保links tab占据全部宽度 */
}

/* Image Preview Modal */
.image-preview-modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.9);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  cursor: pointer;
}

.preview-container {
  position: relative;
  max-width: 90vw;
  max-height: 90vh;
  display: flex;
  align-items: center;
  justify-content: center;
}

.preview-close {
  position: absolute;
  top: -40px;
  right: 0;
  background: none;
  border: none;
  color: white;
  font-size: 2rem;
  cursor: pointer;
  z-index: 10000;
  padding: 8px;
  border-radius: 50%;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s ease;
}

.preview-close:hover {
  background-color: rgba(255, 255, 255, 0.2);
}

.preview-image {
  max-width: 90vw;
  max-height: 90vh;
  object-fit: contain;
  border-radius: 8px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
}
</style>
