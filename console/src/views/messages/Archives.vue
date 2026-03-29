<script setup lang="ts">
import { ref, onMounted } from "vue";
import type { MessageData } from "../../types";
import { getArchiveList, getArchive } from "../../api";
import NavBar from "../../components/NavBar.vue";

const DAY_OF_WEEK_LABELS = ["日", "月", "火", "水", "木", "金", "土"];

/** アーカイブエントリの状態 */
interface ArchiveEntry {
    date: string;
    /** 表示用ラベル（例: "2025年01月01日"）*/
    label: string;
    messages: MessageData[] | null;
    isOpen: boolean;
    isLoading: boolean;
}

const archives = ref<ArchiveEntry[]>([]);
const isLoading = ref(true);

onMounted(async () => {
    try {
        const dates = await getArchiveList();
        archives.value = dates.map((date) => ({
            date,
            label: formatDateLabel(date),
            messages: null,
            isOpen: false,
            isLoading: false,
        }));
    } catch (e) {
        console.error(e);
        alert("アーカイブ一覧の読み込みに失敗しました");
    } finally {
        isLoading.value = false;
    }
});

/** "YYYYMMDD" → "YYYY年MM月DD日" */
function formatDateLabel(date: string): string {
    const year = date.slice(0, 4);
    const month = date.slice(4, 6);
    const day = date.slice(6, 8);
    return `${year}年${parseInt(month)}月${parseInt(day)}日`;
}

function formatTime(message: MessageData): string {
    return `${message.hour}:${message.minute.toString().padStart(2, "0")}`;
}

function formatDate(message: MessageData): string {
    const dow = DAY_OF_WEEK_LABELS[message.day_of_week] ?? "";
    return `${message.month}月${message.day}日(${dow})`;
}

async function toggleEntry(entry: ArchiveEntry) {
    if (entry.isOpen) {
        entry.isOpen = false;
        return;
    }

    entry.isOpen = true;

    if (entry.messages !== null) {
        return;
    }

    entry.isLoading = true;
    try {
        entry.messages = await getArchive(entry.date);
    } catch (e) {
        console.error(e);
        alert(`${entry.label} の読み込みに失敗しました`);
        entry.isOpen = false;
    } finally {
        entry.isLoading = false;
    }
}
</script>

<template>
    <div :class="$style.page">
        <NavBar title="過去のメッセージ" show-back back-text="設定" back-to="/" />

        <main :class="$style.content">
            <section v-if="isLoading" :class="$style.empty">読み込み中...</section>
            <section v-else-if="archives.length === 0" :class="$style.empty">
                過去のメッセージがありません
            </section>

            <ul v-else :class="$style.list">
                <li v-for="entry in archives" :key="entry.date" :class="$style.item">
                    <button
                        :class="[$style.dateHeader, entry.isOpen && $style.dateHeaderOpen]"
                        type="button"
                        @click="toggleEntry(entry)"
                    >
                        <span>{{ entry.label }}</span>
                        <span :class="$style.chevron">{{ entry.isOpen ? "▲" : "▼" }}</span>
                    </button>

                    <div v-if="entry.isOpen" :class="$style.messageList">
                        <section v-if="entry.isLoading" :class="$style.innerEmpty">
                            読み込み中...
                        </section>
                        <section
                            v-else-if="entry.messages && entry.messages.length === 0"
                            :class="$style.innerEmpty"
                        >
                            メッセージがありません
                        </section>
                        <div
                            v-else-if="entry.messages"
                            v-for="(message, index) in entry.messages"
                            :key="index"
                            :class="$style.messageCard"
                        >
                            <header :class="$style.cardHeader">
                                <span :class="$style.cardDate">
                                    {{ formatDate(message) }} {{ formatTime(message) }}
                                </span>
                                <span :class="$style.cardAuthor">{{ message.author }}</span>
                            </header>
                            <p :class="$style.cardText">{{ message.text }}</p>
                        </div>
                    </div>
                </li>
            </ul>
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
}

.empty {
    text-align: center;
    color: var(--text-muted-color);
    font-size: 1rem;
    margin-top: 2rem;
}

.list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.item {
    border-radius: 0.75rem;
    overflow: hidden;
    background-color: var(--card-bg-color);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.dateHeader {
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.875rem 1rem;
    background: none;
    border: none;
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-color);
    cursor: pointer;
    text-align: left;
    transition: background-color 0.15s;

    &:active {
        background-color: var(--divider-color);
    }
}

.dateHeaderOpen {
    border-bottom: 1px solid var(--divider-color);
}

.chevron {
    font-size: 0.75rem;
    color: var(--text-muted-color);
}

.messageList {
    display: flex;
    flex-direction: column;
    gap: 0;
}

.innerEmpty {
    text-align: center;
    color: var(--text-muted-color);
    font-size: 0.875rem;
    padding: 1rem;
}

.messageCard {
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--divider-color);

    &:last-child {
        border-bottom: none;
    }
}

.cardHeader {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.375rem;
}

.cardDate {
    font-size: 0.875rem;
    color: var(--text-muted-color);
}

.cardAuthor {
    font-size: 0.875rem;
    color: var(--text-muted-color);
}

.cardText {
    margin: 0;
    font-size: 1rem;
    color: var(--text-color);
    white-space: pre-wrap;
    word-break: break-all;
}
</style>
