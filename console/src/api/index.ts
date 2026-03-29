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

/**
 * アーカイブの日付一覧を取得します。
 * @returns "YYYYMMDD" 形式の日付文字列の配列（降順）
 */
export async function getArchiveList(): Promise<string[]> {
    const response = await fetch("/api/messages/archives", {
        headers: getHeaders(),
    });
    if (!response.ok) {
        handleResponseError(response);
        throw new Error("アーカイブ一覧の取得に失敗しました。");
    }
    return response.json();
}

/**
 * 指定された日付のアーカイブを取得します。
 * @param date "YYYYMMDD" 形式の日付文字列
 * @returns メッセージの配列
 */
export async function getArchive(date: string): Promise<MessageData[]> {
    const response = await fetch(`/api/messages/archives/${date}`, {
        headers: getHeaders(),
    });
    if (!response.ok) {
        handleResponseError(response);
        throw new Error("アーカイブの取得に失敗しました。");
    }
    return response.json();
}
