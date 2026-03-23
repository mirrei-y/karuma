import type { MessageData } from "../types";

function getHeaders(): HeadersInit {
    const passphrase = localStorage.getItem("karuma_passphrase") || "";
    // TextEncoderを使用してUTF-8エンコードし、Base64に変換 (unescapeの代替)
    const bytes = new TextEncoder().encode(passphrase);
    const binary = Array.from(bytes)
        .map((b) => String.fromCharCode(b))
        .join("");
    const base64Passphrase = btoa(binary);

    return {
        Authorization: `Bearer ${base64Passphrase}`,
        "Content-Type": "application/json",
    };
}

function handleResponseError(response: Response) {
    if (response.status === 401) {
        localStorage.removeItem("karuma_passphrase");
        window.location.href = "/login";
    }
}

/**
 * サーバーからメッセージリストを取得します。
 * @returns メッセージの配列
 */
export async function getMessages(): Promise<MessageData[]> {
    const response = await fetch("/api/messages", {
        headers: getHeaders(),
    });
    if (!response.ok) {
        handleResponseError(response);
        throw new Error("メッセージの取得に失敗しました。");
    }
    return response.json();
}

/**
 * サーバーのメッセージリストを更新します。
 * @param messages 更新するメッセージの配列
 */
export async function updateMessages(messages: MessageData[]): Promise<void> {
    const response = await fetch("/api/messages", {
        method: "PUT",
        headers: getHeaders(),
        body: JSON.stringify(messages),
    });
    if (!response.ok) {
        handleResponseError(response);
        throw new Error("メッセージの更新に失敗しました。");
    }
}
