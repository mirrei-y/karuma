use std::fs;
use std::path::Path;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use crate::constants::{DATA_DIR, ARCHIVES_DIR, MESSAGES_FILE};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageData {
    pub month: i32,
    pub day: i32,
    pub day_of_week: i32,
    pub hour: i32,
    pub minute: i32,
    pub author: String,
    pub text: String,
}

/// TOMLパース・出力用の非公開ラッパー構造体
#[derive(Serialize, Deserialize)]
struct MessageFile {
    messages: Vec<MessageData>,
}

lazy_static! {
    static ref GLOBAL_MESSAGES: Mutex<Vec<MessageData>> = Mutex::new(Vec::new());
}

/// 現在メモリに保持されているメッセージリストを取得します。
pub fn get_messages() -> Vec<MessageData> {
    if let Ok(global) = GLOBAL_MESSAGES.lock() {
        global.clone()
    } else {
        Vec::new()
    }
}

/// 新しいメッセージリストを保存し、UI スレッドに更新を要求します。
pub fn set_messages(
    window_weak: slint::Weak<crate::AppWindow>,
    messages: Vec<MessageData>,
) {
    if let Ok(mut global) = GLOBAL_MESSAGES.lock() {
        *global = messages.clone();
    }

    save_messages(&messages);

    crate::ui::renew_message(window_weak, messages);
}

/// メッセージデータをアーカイブ化し、メモリをクリアします。
pub fn archive_messages(window_weak: slint::Weak<crate::AppWindow>) {
    if let Err(e) = fs::create_dir_all(ARCHIVES_DIR) {
        eprintln!("Failed to create archives directory: {}", e);
        return;
    }

    // 午前3時実行を想定し、数時間引いて前日の日付をアーカイブ名とする
    let archive_date = (chrono::Local::now() - chrono::Duration::hours(4)).date_naive();
    let archive_name = format!("{}/messages-{}.toml", ARCHIVES_DIR, archive_date.format("%Y%m%d"));

    if let Err(e) = fs::rename(MESSAGES_FILE, &archive_name) {
        eprintln!("Failed to archive messages to {}: {}", archive_name, e);
        return;
    }

    if let Ok(mut global) = GLOBAL_MESSAGES.lock() {
        global.clear();
    }
    crate::ui::renew_message(window_weak, vec![]);
}

/// 保存されたメッセージリストを読み込みます。
pub fn load_messages() -> Vec<MessageData> {
    if let Err(e) = fs::create_dir_all(DATA_DIR) {
        eprintln!("Failed to create data directory: {}", e);
        return Vec::new();
    }

    let path = Path::new(MESSAGES_FILE);
    if path.exists() {
        match fs::read_to_string(path) {
            Ok(content) => match toml::from_str::<MessageFile>(&content) {
                Ok(parsed) => {
                    if let Ok(mut global) = GLOBAL_MESSAGES.lock() {
                        *global = parsed.messages.clone();
                    }
                    return parsed.messages;
                }
                Err(e) => eprintln!("Failed to parse messages file: {}", e),
            },
            Err(e) => eprintln!("Failed to read messages file: {}", e),
        }
    }

    if let Ok(mut global) = GLOBAL_MESSAGES.lock() {
        *global = Vec::new();
    }
    Vec::new()
}

/// 指定されたメッセージリストを保存します。
fn save_messages(messages: &[MessageData]) {
    if let Err(e) = fs::create_dir_all(DATA_DIR) {
        eprintln!("Failed to create data directory: {}", e);
        return;
    }

    let file_data = MessageFile { messages: messages.to_vec() };
    match toml::to_string(&file_data) {
        Ok(content) => {
            if let Err(e) = fs::write(MESSAGES_FILE, content) {
                eprintln!("An error occurred while saving messages: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to serialize messages: {}", e),
    }
}
