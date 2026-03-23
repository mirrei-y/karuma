<script setup lang="ts">
import { ChevronLeft } from "@lucide/vue";
import { useRouter } from "vue-router";

const props = defineProps<{
    title: string;
    showBack?: boolean;
    backText?: string;
    backTo?: string;
}>();

const router = useRouter();

function goBack() {
    if (props.backTo) {
        router.push(props.backTo);
    } else {
        router.back();
    }
}
</script>

<template>
    <nav :class="$style.navBar">
        <div :class="$style.left">
            <button v-if="showBack" @click="goBack" :class="$style.backBtn" type="button">
                <ChevronLeft :class="$style.icon" />
                <span>{{ backText || "戻る" }}</span>
            </button>
        </div>
        <h1>{{ title }}</h1>
        <div :class="$style.right">
            <slot name="right"></slot>
        </div>
    </nav>
</template>

<style module lang="scss">
.navBar {
    height: 2.75rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    background-color: var(--background-color);
    border-bottom: 1px solid var(--border-color);
    position: sticky;
    top: 0;
    z-index: 10;
    padding: 0 0.5rem;
    line-height: 1;

    h1 {
        font-size: 1rem;
        font-weight: 600;
        margin: 0;
        position: absolute;
        left: 50%;
        transform: translateX(-50%);
        white-space: nowrap;
    }

    .left,
    .right {
        display: flex;
        align-items: center;
        z-index: 1;
        min-width: 4rem; /* タイトルを中央に保つため */
    }

    .right {
        justify-content: flex-end;
    }

    .backBtn {
        background: none;
        border: none;
        color: var(--primary-color);
        display: flex;
        align-items: center;
        gap: 0.25rem;
        padding: 0.25rem;
        font-size: 1rem;
        line-height: 1;
        cursor: pointer;

        span {
            margin-left: -2px;
        }
    }
}
</style>
