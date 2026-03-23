import type { ViteDevServer } from "vite";
import type { MessageData } from "../src/types";

let mockMessages: MessageData[] = [
    {
        month: 3,
        day: 21,
        day_of_week: 6,
        hour: 12,
        minute: 0,
        author: "システム",
        text: "Vite のモックサーバーからのメッセージです。",
    },
];

export function setupMessagesMock(server: ViteDevServer) {
    server.middlewares.use((req, res, next) => {
        if (req.url === "/api/messages") {
            if (req.method === "GET") {
                res.setHeader("Content-Type", "application/json");
                res.end(JSON.stringify(mockMessages));
                return;
            }
            if (req.method === "PUT") {
                let body = "";
                req.on("data", (chunk) => {
                    body += chunk.toString();
                });
                req.on("end", () => {
                    try {
                        const parsed = JSON.parse(body);
                        mockMessages = parsed;
                        res.statusCode = 204;
                        res.end();
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
