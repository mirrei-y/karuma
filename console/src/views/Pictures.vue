<script setup lang="ts">
import { ref, onMounted } from "vue";
import { ImagePlus } from "@lucide/vue";
import type { PictureEntry } from "../types";
import { getPictures, uploadPicture } from "../api";
import NavBar from "../components/NavBar.vue";

const MAX_SIZE = 1000;
const WEBP_QUALITY = 0.85;

const pictures = ref<PictureEntry[]>([]);
const isLoading = ref(true);
const isUploading = ref(false);

const selectedFile = ref<File | null>(null);
const previewUrl = ref<string | null>(null);
const description = ref("");
const errorMessage = ref("");

onMounted(async () => {
    try {
        pictures.value = await getPictures();
    } catch (e) {
        console.error(e);
    } finally {
        isLoading.value = false;
    }
});

function onFileChange(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0] ?? null;
    selectedFile.value = file;
    if (previewUrl.value) {
        URL.revokeObjectURL(previewUrl.value);
    }
    previewUrl.value = file ? URL.createObjectURL(file) : null;
    errorMessage.value = "";
}

/**
 * Canvas API を使い、画像を最大 MAX_SIZE x MAX_SIZE にリサイズして
 * WebP Blob に変換して返します。
 */
function compressImage(file: File): Promise<Blob> {
    return new Promise((resolve, reject) => {
        const img = new Image();
        const objectUrl = URL.createObjectURL(file);

        img.onload = () => {
            URL.revokeObjectURL(objectUrl);

            let { width, height } = img;
            if (width > MAX_SIZE || height > MAX_SIZE) {
                if (width >= height) {
                    height = Math.round((height / width) * MAX_SIZE);
                    width = MAX_SIZE;
                } else {
                    width = Math.round((width / height) * MAX_SIZE);
                    height = MAX_SIZE;
                }
            }

            const canvas = document.createElement("canvas");
            canvas.width = width;
            canvas.height = height;
            const ctx = canvas.getContext("2d")!;
            ctx.drawImage(img, 0, 0, width, height);

            canvas.toBlob(
                (blob) => {
                    if (blob) {
                        resolve(blob);
                    } else {
                        reject(new Error("WebP への変換に失敗しました"));
                    }
                },
                "image/webp",
                WEBP_QUALITY,
            );
        };

        img.onerror = () => {
            URL.revokeObjectURL(objectUrl);
            reject(new Error("画像の読み込みに失敗しました"));
        };

        img.src = objectUrl;
    });
}

async function submit() {
    if (!selectedFile.value) {
        errorMessage.value = "写真を選択してください。";
        return;
    }
    isUploading.value = true;
    errorMessage.value = "";
    try {
        const blob = await compressImage(selectedFile.value);
        await uploadPicture(blob, description.value);
        // フォームをリセット
        selectedFile.value = null;
        if (previewUrl.value) {
            URL.revokeObjectURL(previewUrl.value);
            previewUrl.value = null;
        }
        description.value = "";
        // 一覧を更新
        pictures.value = await getPictures();
    } catch (e: any) {
        console.error(e);

        // NOTE: e は基本 Error オブジェクトだが、もしそうでないときのために、最初に汎用メッセージを書き込んでおく
        errorMessage.value = "投稿に失敗しました (不明なエラー)";
        errorMessage.value = "投稿に失敗しました: " + e.toString();
    } finally {
        isUploading.value = false;
    }
}
</script>

<template>
    <div :class="$style.page">
        <NavBar title="写真を投稿する" show-back back-text="戻る" back-to="/" />

        <main :class="$style.content">
            <!-- 投稿フォーム -->
            <section :class="$style.card">
                <h2 :class="$style.sectionTitle">新しい写真を投稿</h2>

                <label :class="$style.fileLabel">
                    <input
                        type="file"
                        accept="image/*"
                        :class="$style.fileInput"
                        @change="onFileChange"
                    />
                    <div :class="[$style.filePlaceholder, previewUrl && $style.hasPreview]">
                        <img
                            v-if="previewUrl"
                            :src="previewUrl"
                            :class="$style.previewImage"
                            alt="プレビュー"
                        />
                        <template v-else>
                            <ImagePlus :size="32" :class="$style.uploadIcon" />
                            <span>タップして写真を選択</span>
                        </template>
                    </div>
                </label>

                <textarea
                    v-model="description"
                    :class="$style.textarea"
                    placeholder="説明（任意）"
                    rows="3"
                />

                <p v-if="errorMessage" :class="$style.error">{{ errorMessage }}</p>

                <button
                    :class="$style.submitBtn"
                    type="button"
                    :disabled="isUploading"
                    @click="submit"
                >
                    {{ isUploading ? "投稿中..." : "投稿する" }}
                </button>
            </section>

            <!-- 投稿済み一覧 -->
            <section :class="$style.card">
                <h2 :class="$style.sectionTitle">投稿済みの写真</h2>
                <p :class="$style.hint">7日後に自動削除されます</p>

                <div v-if="isLoading" :class="$style.empty">読み込み中...</div>
                <div v-else-if="pictures.length === 0" :class="$style.empty">
                    まだ写真がありません
                </div>
                <div v-else :class="$style.gallery">
                    <div
                        v-for="pic in pictures"
                        :key="pic.id"
                        :class="$style.galleryItem"
                    >
                        <img
                            :src="`/api/pictures/${pic.id}`"
                            :class="$style.galleryImage"
                            :alt="pic.description || pic.id"
                        />
                        <p v-if="pic.description" :class="$style.galleryDesc">
                            {{ pic.description }}
                        </p>
                    </div>
                </div>
            </section>
        </main>
    </div>
</template>

<style module lang="scss">
.page {
    width: 100%;
    min-height: 100vh;
}

.content {
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.card {
    background-color: var(--card-bg-color);
    border-radius: 0.75rem;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
}

.sectionTitle {
    font-size: 1rem;
    font-weight: 600;
    margin: 0;
}

.hint {
    font-size: 0.75rem;
    color: var(--text-muted-color);
    margin: 0;
}

.fileInput {
    display: none;
}

.filePlaceholder {
    border: 2px dashed var(--border-color);
    border-radius: 0.5rem;
    min-height: 10rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    cursor: pointer;
    color: var(--text-muted-color);
    font-size: 0.875rem;
    overflow: hidden;
    transition: border-color 0.15s;

    &:active {
        border-color: var(--primary-color);
    }

    &.hasPreview {
        border-style: solid;
        border-color: var(--border-color);
        min-height: unset;
    }
}

.uploadIcon {
    color: var(--text-muted-color);
}

.previewImage {
    width: 100%;
    max-height: 20rem;
    object-fit: contain;
    display: block;
}

.textarea {
    width: 100%;
    box-sizing: border-box;
    padding: 0.625rem 0.75rem;
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    background-color: var(--input-bg-color);
    font-size: 1rem;
    font-family: inherit;
    resize: vertical;
    line-height: 1.5;

    &:focus {
        outline: none;
        border-color: var(--primary-color);
    }
}

.error {
    color: var(--danger-color);
    font-size: 0.875rem;
    margin: 0;
}

.submitBtn {
    padding: 0.75rem;
    border: none;
    border-radius: 0.625rem;
    background-color: var(--primary-color);
    color: #fff;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition:
        background-color 0.15s,
        opacity 0.15s;

    &:hover {
        background-color: var(--primary-hover-color);
    }

    &:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }
}

.empty {
    text-align: center;
    color: var(--text-muted-color);
    font-size: 0.875rem;
    padding: 1rem 0;
}

.gallery {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(8rem, 1fr));
    gap: 0.75rem;
}

.galleryItem {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.galleryImage {
    width: 100%;
    aspect-ratio: 1;
    object-fit: cover;
    border-radius: 0.5rem;
    background-color: var(--border-color);
}

.galleryDesc {
    font-size: 0.75rem;
    color: var(--text-muted-color);
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}
</style>
