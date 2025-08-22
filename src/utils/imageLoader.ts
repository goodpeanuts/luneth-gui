import { invoke } from '@tauri-apps/api/core';
import { convertFileSrc } from '@tauri-apps/api/core';

export interface ImageLoadResult {
  src: string;
  isLocal: boolean;
}

/**
 * 优先从本地加载图片，如果本地不存在则使用远程 URL
 * @param recordId 记录 ID
 * @param imageIndex 图片索引 (-1: 显示图片/缩略图, 0: 封面, 1+: 样例图片)
 * @param remoteSrc 远程图片 URL
 * @returns Promise<ImageLoadResult>
 */
export async function loadImage(
  recordId: string, 
  imageIndex: number, 
  remoteSrc: string
): Promise<ImageLoadResult> {
  try {
    // 尝试从本地加载
    const localPath = await invoke<string | null>('check_local_image_exists', {
      record_id: recordId,
      image_index: imageIndex
    });

    if (localPath) {
      // 转换本地路径为 Tauri 可访问的 URL
      const localSrc = convertFileSrc(localPath);
      return {
        src: localSrc,
        isLocal: true
      };
    }
  } catch (error) {
    console.warn(`Failed to check local image for ${recordId}_${imageIndex}:`, error);
  }

  // 如果本地不存在，使用远程 URL
  return {
    src: remoteSrc,
    isLocal: false
  };
}

/**
 * 加载封面图片（优先本地）
 * @param recordId 记录 ID
 * @param remoteCoverSrc 远程封面 URL
 * @returns Promise<ImageLoadResult>
 */
export async function loadCoverImage(recordId: string, remoteCoverSrc: string): Promise<ImageLoadResult> {
  return loadImage(recordId, 0, remoteCoverSrc);
}

/**
 * 加载显示图片/缩略图（优先本地）
 * @param recordId 记录 ID
 * @param remoteCoverSrc 远程封面 URL（作为后备）
 * @returns Promise<ImageLoadResult>
 */
export async function loadDisplayImage(recordId: string, remoteCoverSrc: string): Promise<ImageLoadResult> {
  return loadImage(recordId, -1, remoteCoverSrc);
}

/**
 * 加载样例图片（优先本地）
 * @param recordId 记录 ID
 * @param sampleIndex 样例图片索引（从 0 开始）
 * @param remoteSampleSrc 远程样例图片 URL
 * @returns Promise<ImageLoadResult>
 */
export async function loadSampleImage(
  recordId: string, 
  sampleIndex: number, 
  remoteSampleSrc: string
): Promise<ImageLoadResult> {
  // 样例图片在本地是从 1 开始编号的（0 是封面）
  return loadImage(recordId, sampleIndex + 1, remoteSampleSrc);
}

/**
 * 批量加载样例图片
 * @param recordId 记录 ID
 * @param remoteSampleSrcs 远程样例图片 URL 数组
 * @returns Promise<ImageLoadResult[]>
 */
export async function loadSampleImages(
  recordId: string, 
  remoteSampleSrcs: string[]
): Promise<ImageLoadResult[]> {
  const results = await Promise.all(
    remoteSampleSrcs.map((src, index) => 
      loadSampleImage(recordId, index, src)
    )
  );
  return results;
}
