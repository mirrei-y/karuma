<script setup lang="ts">
import { Archive, ChevronRight, ImageUp, Mail, RefreshCw, UserRoundPen } from "@lucide/vue";
import { onMounted, onUnmounted, ref } from "vue";
import { RouterLink } from "vue-router";
import { getSnapshot } from "../api";
import NavBar from "../components/NavBar.vue";

const snapshotUrl = ref<string | null>(null);
const isSnapshotLoading = ref(false);
const snapshotError = ref<string | null>(null);

async function refreshSnapshot() {
    isSnapshotLoading.value = true;
    snapshotError.value = null;

    // 古い Blob URL を解放してメモリリークを防ぐ
    if (snapshotUrl.value) {
        URL.revokeObjectURL(snapshotUrl.value);
        snapshotUrl.value = null;
    }

    try {
        snapshotUrl.value = await getSnapshot();
    } catch (e) {
        console.error(e);
        snapshotError.value = "スナップショットの取得に失敗しました。";
    } finally {
        isSnapshotLoading.value = false;
    }
}

onMounted(() => {
    refreshSnapshot();
});

onUnmounted(() => {
    if (snapshotUrl.value) {
        URL.revokeObjectURL(snapshotUrl.value);
    }
});
</script>

<template>
    <div :class="$style.page">
        <NavBar title="設定" />
        <main :class="$style.content">
            <h2>現在の画面</h2>
            <div :class="$style.snapshotCard">
                <div :class="$style.snapshotWrapper">
                    <img
                        v-if="snapshotUrl"
                        :src="snapshotUrl"
                        :class="$style.snapshot"
                        alt="現在の画面"
                    />
                    <div v-else-if="isSnapshotLoading" :class="$style.snapshotPlaceholder">
                        <RefreshCw :size="24" :class="$style.spin" />
                    </div>
                    <div v-else :class="$style.snapshotPlaceholder">
                        <span>{{ snapshotError ?? "画像がありません" }}</span>
                    </div>
                </div>
                <button
                    :class="$style.refreshButton"
                    :disabled="isSnapshotLoading"
                    @click="refreshSnapshot"
                >
                    <RefreshCw :size="16" :class="{ [$style.spin]: isSnapshotLoading }" />
                    <span>更新</span>
                </button>
            </div>
            <h2>掲示板の編集</h2>
            <RouterLink to="/messages" :class="$style.button">
                <Mail :class="$style.icon" :size="32" />
                <div>
                    <span :class="$style.title">メッセージを書き換える</span>
                    <span :class="$style.description">画面に表示されるメッセージを変更します</span>
                </div>
                <ChevronRight :size="20" />
            </RouterLink>
            <RouterLink to="/pictures" :class="$style.button">
                <ImageUp :class="$style.icon" :size="32" />
                <div>
                    <span :class="$style.title">写真を投稿する</span>
                    <span :class="$style.description">新しい写真を投稿できます</span>
                </div>
                <ChevronRight :size="20" />
            </RouterLink>
            <h2>データを見る</h2>
            <RouterLink to="/messages/archives" :class="$style.button">
                <Archive :class="$style.icon" :size="32" />
                <div>
                    <span :class="$style.title">過去のメッセージを見る</span>
                    <span :class="$style.description">1日以上経過したメッセージを見ることができます</span>
                </div>
                <ChevronRight :size="20" />
            </RouterLink>
            <h2>設定</h2>
            <RouterLink to="/login" :class="$style.button">
                <UserRoundPen :class="$style.icon" :size="32" />
                <div>
                    <span :class="$style.title">名前を変える</span>
                    <span :class="$style.description">メッセージの投稿者名やログイン情報を変更できます</span>
                </div>
                <ChevronRight :size="20" />
            </RouterLink>
        </main>
    </div>
</template>

<style module lang="scss">
.page {
    width: 100%;
}

.content {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    padding: 1rem;

    h2 {
        font-size: 1rem;
        // font-weight: normal;
        color: var(--text-muted-color);
        margin: 1rem 0.5rem 0rem 0.5rem;

        &:first-child {
            margin-top: 0px;
        }
    }
    a {
        border: none;
        background: none;

        padding: 0.75rem 1rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        cursor: pointer;
        transition: background-color 0.15s;

        background-color: var(--card-bg-color);
        color: currentColor;
        text-decoration: none;
        border-radius: 0.75rem;
        line-height: 1;

        border: 1px solid var(--border-color);

        &:active {
            background-color: var(--divider-color);
        }

        div {
            flex: 1;

            display: flex;
            flex-direction: column;
            gap: 0.5rem;
        }
        svg {
            color: var(--text-muted-color);
        }

        .icon {
            margin: 0.5em;
        }
        .title {
            font-size: 1rem;
            font-weight: 600;
        }
        .description {
            color: var(--text-muted-color);
            font-size: 0.875em;
        }
    }
}

.snapshotCard {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    background-color: var(--card-bg-color);
    border: 1px solid var(--border-color);
    border-radius: 0.75rem;
    overflow: hidden;
    padding: 0.75rem;
}

.snapshotWrapper {
    width: 100%;
    aspect-ratio: 16 / 9;
    background-color: var(--divider-color);
    border-radius: 0.5rem;
    overflow: hidden;
}

.snapshot {
    width: 100%;
    height: 100%;
    object-fit: contain;
    display: block;
}

.snapshotPlaceholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted-color);
    font-size: 0.875rem;
}

.refreshButton {
    align-self: flex-end;
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.375rem 0.75rem;
    border-radius: 0.5rem;
    border: 1px solid var(--border-color);
    background: none;
    color: currentColor;
    font-size: 0.875rem;
    cursor: pointer;
    transition: background-color 0.15s;

    &:active:not(:disabled) {
        background-color: var(--divider-color);
    }

    &:disabled {
        opacity: 0.5;
        cursor: default;
    }
}

@keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
}

.spin {
    animation: spin 1s linear infinite;
}
</style>
