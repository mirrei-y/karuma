import type { MessageData, PictureEntry } from "../types";

class HttpResponseError extends Error {
    public response: Response;

    public constructor(response: Response) {
        super(`HTTP Error ${response.status}`);
        this.response = response;
    }
}

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
        throw new HttpResponseError(response);
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
        throw new HttpResponseError(response);
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
        throw new HttpResponseError(response);
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
        throw new HttpResponseError(response);
    }
    return response.json();
}

/**
 * 投稿された写真の一覧を取得します。
 * @returns 写真エントリの配列（最新順）
 */
export async function getPictures(): Promise<PictureEntry[]> {
    const response = await fetch("/api/pictures", {
        headers: getHeaders(),
    });
    if (!response.ok) {
        handleResponseError(response);
        throw new HttpResponseError(response);
    }
    return response.json();
}

/**
 * 現在の画面スナップショットを Blob URL として取得します。
 * @returns PNG の object URL
 */
export async function getSnapshot(): Promise<string> {
    const headers = getHeaders() as Record<string, string>;
    const response = await fetch("/api/snapshot", {
        headers: { Authorization: headers["Authorization"] },
    });
    if (!response.ok) {
        handleResponseError(response);
        throw new HttpResponseError(response);
    }
    const blob = await response.blob();
    return URL.createObjectURL(blob);
}

/**
 * 写真と説明文を投稿します。
 * @param image 画像ファイルまたは Blob（Canvas で圧縮済みのものを渡す）
 * @param description 説明文
 */
export async function uploadPicture(image: Blob, description: string): Promise<void> {
    // Authorization ヘッダーを含めるために getHeaders() からトークンを取り出す
    const headers = getHeaders() as Record<string, string>;
    const formData = new FormData();
    formData.append("image", image);
    formData.append("description", description);

    const response = await fetch("/api/pictures", {
        method: "POST",
        headers: { Authorization: headers["Authorization"] },
        body: formData,
    });
    if (!response.ok) {
        handleResponseError(response);
        throw new HttpResponseError(response);
    }
}
