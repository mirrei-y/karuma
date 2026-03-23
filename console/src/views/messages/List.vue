<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { Plus } from "@lucide/vue";
import type { MessageData } from "../../types";
import { getMessages, updateMessages } from "../../api";
import NavBar from "../../components/NavBar.vue";
import MessageCard from "../../components/MessageCard.vue";

const messages = ref<MessageData[]>([]);
const isSaving = ref(false);
const isLoading = ref(true);
let saveTimeout: ReturnType<typeof setTimeout> | null = null;
let isFirstLoad = true;
let skipNextSave = false;

onMounted(async () => {
    try {
        messages.value = await getMessages();
    } catch (e) {
        console.error(e);
        alert("読み込みに失敗しました");
    } finally {
        isLoading.value = false;
        // 初回ロード完了フラグを落としてから watch に反応させるための微小な待機
        setTimeout(() => {
            isFirstLoad = false;
        }, 0);
    }
});

watch(
    messages,
    () => {
        if (isFirstLoad) return;
        if (skipNextSave) {
            skipNextSave = false;
            return;
        }
        if (saveTimeout) clearTimeout(saveTimeout);
        saveTimeout = setTimeout(() => {
            saveChanges();
        }, 1000); // 1秒間のデバウンスで自動保存
    },
    { deep: true },
);

function addMessage() {
    const now = new Date();
    const author = localStorage.getItem("karuma_username") || "新規";
    messages.value.push({
        month: now.getMonth() + 1,
        day: now.getDate(),
        day_of_week: now.getDay(),
        hour: now.getHours(),
        minute: now.getMinutes(),
        author: author,
        text: "",
    });
    // ページ最下部までスクロール
    setTimeout(() => {
        window.scrollTo({
            top: document.documentElement.scrollHeight,
            behavior: "smooth",
        });
    }, 100);
    skipNextSave = true;
}

function removeMessage(index: number) {
    if (confirm("このメッセージを削除してよろしいですか？")) {
        messages.value.splice(index, 1);
    }
}

async function saveChanges() {
    isSaving.value = true;
    try {
        await updateMessages(messages.value);
    } catch (e) {
        console.error(e);
    } finally {
        isSaving.value = false;
    }
}
</script>

<template>
    <div :class="$style.page">
        <NavBar title="メッセージ管理" show-back back-text="設定" back-to="/">
            <template #right>
                <div :class="$style.saveIndicator" v-if="isSaving">
                    <span>保存中...</span>
                </div>
            </template>
        </NavBar>

        <main :class="$style.content">
            <section v-if="isLoading" :class="$style.empty">読み込み中...</section>
            <section v-else-if="messages.length === 0" :class="$style.empty">メッセージがありません</section>

            <div v-else :class="$style.listContainer">
                <MessageCard
                    v-for="(_, index) in messages"
                    :key="index"
                    v-model="messages[index]"
                    @delete="removeMessage(index)"
                />
            </div>

            <button :class="$style.fab" @click="addMessage" type="button" title="追加">
                <Plus :size="24" />
            </button>
        </main>
    </div>
</template>

<style module lang="scss">
.page {
    width: 100%;
    min-height: 100vh;
}

.saveIndicator {
    font-size: 0.75rem;
    color: var(--text-muted-color);
    padding-right: 0.5rem;
}

.content {
    padding: 1rem;
    padding-bottom: 5rem; /* FABのスペース確保 */
}

.empty {
    text-align: center;
    color: var(--text-muted-color);
    font-size: 1rem;
    margin-top: 2rem;
}

.listContainer {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

.fab {
    position: fixed;
    bottom: 2rem;
    right: 2rem;
    width: 3.5rem;
    height: 3.5rem;
    border-radius: 50%;
    background-color: var(--primary-color);
    color: white;
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
    cursor: pointer;
    z-index: 100;
    transition:
        transform 0.15s,
        background-color 0.15s;

    &:hover {
        background-color: var(--primary-hover-color);
    }

    &:active {
        transform: scale(0.95);
    }
}
</style>
