import type { ViteDevServer } from "vite";
import type { AppConfigPayload } from "../src/types/config";

let mockConfig: AppConfigPayload = {
    example_setting: "mocked_value_from_vite",
};

export function setupConfigMock(server: ViteDevServer) {
    server.middlewares.use((req, res, next) => {
        if (req.url === "/api/config") {
            if (req.method === "GET") {
                res.setHeader("Content-Type", "application/json");
                res.end(JSON.stringify(mockConfig));
                return;
            }
            if (req.method === "POST") {
                let body = "";
                req.on("data", (chunk) => {
                    body += chunk.toString();
                });
                req.on("end", () => {
                    try {
                        const parsed = JSON.parse(body);
                        mockConfig = { ...mockConfig, ...parsed };
                        res.setHeader("Content-Type", "application/json");
                        res.end(JSON.stringify("Success"));
                    } catch (e) {
                        res.statusCode = 400;
                        res.end("Bad Request");
                    }
                });
                return;
            }
        }
        next();
    });
}
