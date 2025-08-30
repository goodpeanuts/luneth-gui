<template>
  <div class="pagination-container">
    <div class="pagination-info">
      <span v-if="totalCount > 0">
        Showing {{ startRecord }}-{{ endRecord }} of {{ totalCount }} records
      </span>
      <span v-else>No records found</span>
    </div>

    <div class="pagination-controls" v-if="totalPages > 1">
      <!-- Previous button -->
      <button
        class="pagination-btn"
        :disabled="currentPage <= 1"
        @click="$emit('prev-page')"
      >
        ‹
      </button>

      <!-- Page numbers -->
      <template v-for="page in pageRange" :key="page">
        <span v-if="page === -1" class="pagination-ellipsis">...</span>
        <button
          v-else
          class="pagination-btn"
          :class="{ active: page === currentPage }"
          @click="$emit('go-to-page', page)"
        >
          {{ page }}
        </button>
      </template>

      <!-- Next button -->
      <button
        class="pagination-btn"
        :disabled="currentPage >= totalPages"
        @click="$emit('next-page')"
      >
        ›
      </button>

      <!-- Page input -->
      <div class="page-input-container">
        <span>Go to:</span>
        <input
          v-model="pageInput"
          type="number"
          min="1"
          :max="totalPages"
          class="page-input"
          @keyup.enter="goToInputPage"
          @blur="goToInputPage"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';

interface Props {
  currentPage: number;
  totalPages: number;
  totalCount: number;
  pageSize: number;
  pageRange: number[];
}

interface Emits {
  (e: 'prev-page'): void;
  (e: 'next-page'): void;
  (e: 'go-to-page', page: number): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const pageInput = ref<string>('');

// 计算显示范围
const startRecord = computed(() => {
  if (props.totalCount === 0) return 0;
  return (props.currentPage - 1) * props.pageSize + 1;
});

const endRecord = computed(() => {
  const end = props.currentPage * props.pageSize;
  return Math.min(end, props.totalCount);
});

// 监听当前页变化，更新输入框
watch(() => props.currentPage, (newPage) => {
  pageInput.value = newPage.toString();
}, { immediate: true });

function goToInputPage() {
  const page = parseInt(pageInput.value);
  if (!isNaN(page) && page >= 1 && page <= props.totalPages && page !== props.currentPage) {
    emit('go-to-page', page);
  } else {
    // 重置为当前页
    pageInput.value = props.currentPage.toString();
  }
}
</script>

<style scoped>
.pagination-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  background: white;
  border-top: 1px solid #e9ecef;
  border-radius: 0 0 8px 8px;
  position: sticky;
  bottom: 0;
  z-index: 10;
  box-shadow: 0 -2px 8px rgba(0, 0, 0, 0.1);
}

.pagination-info {
  font-size: 0.9rem;
  color: #6c757d;
  font-weight: 500;
}

.pagination-controls {
  display: flex;
  align-items: center;
  gap: 8px;
}

.pagination-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 32px;
  height: 32px;
  padding: 0 8px;
  border: 1px solid #dee2e6;
  background: white;
  color: #495057;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9rem;
  font-weight: 500;
  transition: all 0.2s ease;
}

.pagination-btn:hover:not(:disabled) {
  background: #e9ecef;
  border-color: #adb5bd;
}

.pagination-btn:disabled {
  background: #f8f9fa;
  color: #6c757d;
  cursor: not-allowed;
  opacity: 0.6;
}

.pagination-btn.active {
  background: #007bff;
  border-color: #007bff;
  color: white;
}

.pagination-ellipsis {
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 32px;
  height: 32px;
  color: #6c757d;
  font-weight: bold;
}

.page-input-container {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-left: 16px;
  padding-left: 16px;
  border-left: 1px solid #dee2e6;
  font-size: 0.9rem;
  color: #495057;
}

.page-input {
  width: 60px;
  height: 32px;
  padding: 0 8px;
  border: 1px solid #ced4da;
  border-radius: 4px;
  text-align: center;
  font-size: 0.9rem;
  transition: border-color 0.2s ease;
}

.page-input:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.25);
}

/* 移动端响应式 */
@media (max-width: 768px) {
  .pagination-container {
    flex-direction: column;
    gap: 12px;
    padding: 12px;
  }

  .pagination-controls {
    gap: 4px;
  }

  .pagination-btn {
    min-width: 28px;
    height: 28px;
    font-size: 0.8rem;
  }

  .page-input-container {
    margin-left: 0;
    padding-left: 0;
    border-left: none;
    border-top: 1px solid #dee2e6;
    padding-top: 8px;
  }

  .page-input {
    width: 50px;
    height: 28px;
  }
}
</style>
