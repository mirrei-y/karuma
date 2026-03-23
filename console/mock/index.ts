import type { ViteDevServer } from "vite";
import { setupMessagesMock } from "./messages";
import { setupConfigMock } from "./config";

export function setupMockServer(server: ViteDevServer) {
    setupMessagesMock(server);
    setupConfigMock(server);
}
