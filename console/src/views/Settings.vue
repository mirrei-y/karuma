<script setup lang="ts">
import { ChevronRight } from "@lucide/vue";
import { RouterLink } from "vue-router";
import NavBar from "../components/NavBar.vue";
</script>

<template>
    <div :class="$style.page">
        <NavBar title="設定" />
        <main :class="$style.content">
            <RouterLink to="/messages" :class="$style.button">
                <div>
                    <span :class="$style.label">メッセージを追加・編集する</span>
                </div>
                <ChevronRight :class="$style.chevron" :size="20" />
            </RouterLink>
            <RouterLink to="/login" :class="$style.button">
                <div>
                    <span :class="$style.label">名前を変える</span>
                </div>
                <ChevronRight :class="$style.chevron" :size="20" />
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
}

.button {
    border: none;
    background: none;

    padding: 0.75rem 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    cursor: pointer;
    transition: background-color 0.15s;

    background-color: var(--card-bg-color);
    color: currentColor;
    text-decoration: none;
    border-radius: 0.75rem;

    &:active {
        background-color: var(--divider-color);
    }
}

.label {
    font-size: 1rem;
}

.chevron {
    color: var(--text-muted-color);
}
</style>
