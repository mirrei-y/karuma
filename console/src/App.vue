<script setup lang="ts">
import { RouterView } from "vue-router";
</script>

<template>
    <div :class="$style.appLayout">
        <RouterView />
    </div>
</template>

<style module lang="scss">
:global(:root) {
    --background-color: #f2f2f7; /* iOS standard Light */
    --foreground-color: #000;
    --card-bg-color: #fff;
    --border-color: #c6c6c8;
    --text-muted-color: #8e8e93;
    --input-bg-color: #fff;
    --primary-color: #ff9100;
    --primary-hover-color: #ffaa29;
    --success-color: #34c759;
    --danger-color: #ff3b30;
    --divider-color: #c6c6c8;
    @media (prefers-color-scheme: dark) {
        --background-color: #000; /* iOS standard Dark */
        --foreground-color: #fff;
        --card-bg-color: #1c1c1e;
        --border-color: #38383a;
        --text-muted-color: #8e8e93;
        --input-bg-color: #1c1c1e;
        --divider-color: #38383a;
    }
}

body {
    background-color: var(--background-color);
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    margin: 0;
    padding: 0;
    -webkit-font-smoothing: antialiased;
}
body,
input,
button,
textarea {
    color: var(--foreground-color);
}

.appLayout {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
}
</style>
