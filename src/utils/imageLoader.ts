import { invoke } from '@tauri-apps/api/core';
import { getCachedImageInfo, setCachedImage } from '@/store/cache';

export interface ImageLoadResult {
  src: string;
  isLocal: boolean;
}

/**
 * 优先从本地加载图片，如果本地不存在则使用远程 URL
 * @param recordId 记录 ID
 * @param imageIndex 图片索引 (null: 封面, 0: 详情大图, 1+: 样例图片)
 * @param remoteSrc 远程图片 URL
 * @returns Promise<ImageLoadResult>
 */
export async function loadImage(
  recordId: string,
  imageIndex: number | null,
  remoteSrc: string
): Promise<ImageLoadResult> {
  // 先检查缓存
  const cacheKey = `${recordId}_${imageIndex}`;
  const cachedInfo = getCachedImageInfo(cacheKey);
  if (cachedInfo) {
    return {
      src: cachedInfo.url,
      isLocal: cachedInfo.isLocal
    };
  }

  try {
    // 尝试从本地加载
    const localImageBytes = await invoke<number[] | null>('read_local_record_image', {
      record_id: recordId,
      image_index: imageIndex
    });

    if (localImageBytes && localImageBytes.length > 0) {
      // 将字节数组转换为 Uint8Array
      const uint8Array = new Uint8Array(localImageBytes);

      // 创建 Blob 并生成 URL
      const blob = new Blob([uint8Array]);
      const localSrc = URL.createObjectURL(blob);

      // 缓存结果
      setCachedImage(cacheKey, localSrc, true);

      return {
        src: localSrc,
        isLocal: true
      };
    }
  } catch (error) {
    console.warn(`Failed to load local image for ${recordId}_${imageIndex}:`, error);
  }

  // 如果本地不存在或加载失败，使用远程 URL
  if (remoteSrc) {
    // 缓存远程URL
    setCachedImage(cacheKey, remoteSrc, false);

    return {
      src: remoteSrc,
      isLocal: false
    };
  }

  // 如果没有远程URL，返回空结果
  return {
    src: '',
    isLocal: false
  };
}

/**
 * 加载展示图片（优先本地）
 * @param recordId 记录 ID
 * @param remoteCoverSrc 远程封面 URL
 * @returns Promise<ImageLoadResult>
 */
export async function loadDisplayImage(recordId: string, remoteCoverSrc: string): Promise<ImageLoadResult> {
  return loadImage(recordId, null, remoteCoverSrc);
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
