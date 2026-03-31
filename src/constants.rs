pub const DATA_DIR: &str = "data";
pub const ARCHIVES_DIR: &str = "archives";
pub const MESSAGES_FILE: &str = "data/messages.toml";

/// メッセージのディスク書き込みを遅延させる秒数（debounce）
pub const SAVE_DEBOUNCE_SECS: u64 = 30;

/// 写真の保存ディレクトリ
pub const PICTURES_DIR: &str = "data/pictures";

/// 写真の保存日数（7日後に削除）
pub const PICTURE_RETENTION_DAYS: i64 = 7;

/// 30分ごとに PictureView の表示内容を切り替える間隔（分）
pub const PICTURE_SWITCH_INTERVAL: usize = 30;
//
