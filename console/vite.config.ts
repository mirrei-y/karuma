import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { setupMockServer } from "./mock/index";

function mockPlugin() {
    return {
        name: "mock-plugin",
        configureServer(server: any) {
            setupMockServer(server);
        },
    };
}

// https://vite.dev/config/
export default defineConfig({
    plugins: [vue(), mockPlugin()],
});
