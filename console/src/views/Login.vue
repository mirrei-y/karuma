<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import NavBar from "../components/NavBar.vue";

const router = useRouter();
const currentPassphrase = localStorage.getItem("karuma_passphrase") || "";
const passphrase = ref(currentPassphrase);
const username = ref(localStorage.getItem("karuma_username") || "");
const isLoggedIn = !!currentPassphrase;

function login() {
    if (!passphrase.value || !username.value) {
        alert("名前とあいことばを入力してください");
        return;
    }
    localStorage.setItem("karuma_passphrase", passphrase.value);
    localStorage.setItem("karuma_username", username.value);
    router.push("/");
}
</script>

<template>
    <div :class="$style.page">
        <NavBar
            :title="isLoggedIn ? '名前を変える' : 'ログイン'"
            :show-back="isLoggedIn"
            back-text="戻る"
            back-to="/"
        />
        <main :class="$style.content">
            <section :class="$style.section">
                <div :class="$style.inputGroup">
                    <label :class="$style.label" for="username">名前</label>
                    <input
                        id="username"
                        type="text"
                        v-model="username"
                        :class="$style.input"
                        placeholder="例: 管理者"
                    />
                </div>
                <div :class="$style.inputGroup">
                    <label :class="$style.label" for="passphrase">あいことば</label>
                    <input
                        id="passphrase"
                        type="text"
                        v-model="passphrase"
                        :class="$style.input"
                        placeholder="あいことば"
                    />
                </div>
            </section>
            <button :class="$style.loginButton" @click="login">
                {{ isLoggedIn ? "保存" : "ログイン" }}
            </button>
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
    gap: 1.5rem;
}

.section {
    background-color: var(--card-bg-color);
    border-radius: 0.75rem;
    overflow: hidden;
}

.inputGroup {
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--divider-color);
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    &:last-child {
        border-bottom: none;
    }
}

.label {
    font-size: 0.875rem;
    color: var(--text-muted-color);
}

.input {
    font-size: 1rem;
    color: var(--text-color, inherit);
    background: transparent;
    border: none;
    outline: none;
    width: 100%;

    &::placeholder {
        color: var(--text-muted-color);
    }
}

.loginButton {
    background-color: var(--primary-color);
    color: white;
    border: none;
    border-radius: 0.5rem;
    padding: 1rem;
    font-size: 1rem;
    font-weight: bold;
    cursor: pointer;
    text-align: center;

    &:hover {
        background-color: var(--primary-hover-color);
    }
    &:active {
        opacity: 0.8;
    }
}
</style>
