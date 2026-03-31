/** メッセージデータ */
export interface MessageData {
    month: number;
    day: number;
    day_of_week: number;
    hour: number;
    minute: number;
    author: string;
    text: string;
}

/** 写真エントリ */
export interface PictureEntry {
    id: string;
    description: string;
}
