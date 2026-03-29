use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use crate::constants::{DATA_DIR, ARCHIVES_DIR, MESSAGES_FILE, SAVE_DEBOUNCE_SECS};

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

    /// set_messages が呼ばれた UNIX タイムスタンプ（ナノ秒）。0 は未スケジュール。
    static ref PENDING_SAVE_AT: AtomicU64 = AtomicU64::new(0);
}

/// 現在メモリに保持されているメッセージリストを取得します。
pub fn get_messages() -> Vec<MessageData> {
    if let Ok(global) = GLOBAL_MESSAGES.lock() {
        global.clone()
    } else {
        Vec::new()
    }
}

/// 新しいメッセージリストをメモリに反映し、UI スレッドに即時更新を要求します。
/// ディスクへの書き込みは SAVE_DEBOUNCE_SECS 秒後に一度だけ行われます。
pub fn set_messages(
    window_weak: slint::Weak<crate::AppWindow>,
    messages: Vec<MessageData>,
) {
    if let Ok(mut global) = GLOBAL_MESSAGES.lock() {
        *global = messages.clone();
    }

    schedule_save();

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

/// アーカイブファイルの一覧（日付文字列）を返します。
///
/// 例: `["20250101", "20250102"]`（降順）
pub fn list_archives() -> Vec<String> {
    let dir = Path::new(ARCHIVES_DIR);
    if !dir.exists() {
        return Vec::new();
    }

    let mut dates: Vec<String> = match fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let name = entry.file_name().into_string().ok()?;
                // "messages-YYYYMMDD.toml" → "YYYYMMDD"
                let date = name
                    .strip_prefix("messages-")?
                    .strip_suffix(".toml")?
                    .to_string();
                Some(date)
            })
            .collect(),
        Err(e) => {
            eprintln!("Failed to read archives directory: {}", e);
            Vec::new()
        }
    };

    dates.sort_by(|a, b| b.cmp(a)); // 降順
    dates
}

/// 指定された日付のアーカイブを読み込みます。
///
/// `date` は `"YYYYMMDD"` 形式の文字列です。
/// 見つからない・パース失敗の場合は `None` を返します。
pub fn load_archive(date: &str) -> Option<Vec<MessageData>> {
    let path_str = format!("{}/messages-{}.toml", ARCHIVES_DIR, date);
    let path = Path::new(&path_str);

    if !path.exists() {
        return None;
    }

    match fs::read_to_string(path) {
        Ok(content) => match toml::from_str::<MessageFile>(&content) {
            Ok(parsed) => Some(parsed.messages),
            Err(e) => {
                eprintln!("Failed to parse archive {}: {}", date, e);
                None
            }
        },
        Err(e) => {
            eprintln!("Failed to read archive {}: {}", date, e);
            None
        }
    }
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

/// 保存をスケジュールします。
///
/// 既にスケジュール済みの場合はタイムスタンプを更新します（debounce）。
/// 初回スケジュール時のみ tokio タスクを起動します。
fn schedule_save() {
    /// 現在時刻を UNIX タイムスタンプ（秒）で返します。
    fn now_secs() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
    /// 指定されたメッセージリストをディスクに書き込みます。
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

    let deadline = now_secs() + SAVE_DEBOUNCE_SECS;
    let prev = PENDING_SAVE_AT.swap(deadline, Ordering::Relaxed);

    // prev == 0 のときのみタスクを起動（既に起動中なら swap で deadline を更新済み）
    if prev == 0 {
        tokio::spawn(async {
            loop {
                let deadline = PENDING_SAVE_AT.load(Ordering::Relaxed);
                let now = now_secs();

                if deadline == 0 {
                    // キャンセルされた（通常は発生しない）
                    break;
                }

                if now >= deadline {
                    // デッドライン到達：フラグをクリアしてから保存
                    PENDING_SAVE_AT.store(0, Ordering::Relaxed);
                    let messages = get_messages();
                    save_messages(&messages);
                    break;
                }

                // デッドラインまで待機（1秒刻みで再チェックし debounce に追従）
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        });
    }
}
