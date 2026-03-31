<script setup lang="ts">
import { Archive, ChevronRight, ImageUp, Mail, UserRoundPen } from "@lucide/vue";
import { RouterLink } from "vue-router";
import NavBar from "../components/NavBar.vue";
</script>

<template>
    <div :class="$style.page">
        <NavBar title="設定" />
        <main :class="$style.content">
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
</style>
