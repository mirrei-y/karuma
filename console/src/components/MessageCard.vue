<script setup lang="ts">
import { Trash2 } from "@lucide/vue";
import type { MessageData } from "../types";

const props = defineProps<{
    modelValue: MessageData;
}>();

const emit = defineEmits<{
    (e: "update:modelValue", value: MessageData): void;
    (e: "delete"): void;
}>();

/** メッセージテキストを更新します。 */
function updateText(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    emit("update:modelValue", { ...props.modelValue, text: target.value });
}
</script>

<template>
    <section :class="$style.card">
        <header :class="$style.cardHeader">
            <span :class="$style.date"
                >{{ modelValue.month }}月{{ modelValue.day }}日 {{ modelValue.hour }}:{{
                    modelValue.minute.toString().padStart(2, "0")
                }}</span
            >
            <div :class="$style.headerActionsRight">
                <span :class="$style.authorText">{{ modelValue.author }}</span>
                <button :class="$style.deleteBtn" @click="$emit('delete')" type="button" title="削除">
                    <Trash2 :size="18" />
                </button>
            </div>
        </header>

        <div :class="$style.textareaRow">
            <!-- eslint-disable-next-line vuejs-accessibility/form-control-has-label -->
            <textarea
                :value="modelValue.text"
                @input="updateText"
                :class="$style.textarea"
                placeholder="メッセージを入力"
                maxlength="32"
            ></textarea>
            <div :class="$style.charCount">
                {{ modelValue.text.length }}/32
            </div>
        </div>
    </section>
</template>

<style module lang="scss">
.card {
    background-color: var(--card-bg-color);
    border-radius: 0.75rem;
    overflow: hidden;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.cardHeader {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    background-color: var(--card-bg-color);
    border-bottom: 1px solid var(--divider-color);
}

.date {
    font-size: 1rem;
    color: var(--text-muted-color);
    font-weight: 500;
}

.headerActionsRight {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.authorText {
    font-size: 0.875rem;
    color: var(--text-muted-color);
    margin-right: 0.5rem;
}

.deleteBtn {
    background: none;
    border: none;
    color: var(--danger-color);
    padding: 0.25rem;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    border-radius: 0.25rem;

    &:active {
        opacity: 0.7;
    }
}

.textareaRow {
    padding: 0.75rem 1rem;
    background-color: var(--card-bg-color);
}

.textarea {
    width: 100%;
    min-height: 5rem;
    font-size: 1rem;
    color: var(--text-color, inherit);
    background: transparent;
    border: none;
    outline: none;
    resize: none;

    &::placeholder {
        color: var(--text-muted-color);
    }
}

.charCount {
    text-align: right;
    font-size: 0.75rem;
    color: var(--text-muted-color);
    margin-top: 0.25rem;
}
</style>
