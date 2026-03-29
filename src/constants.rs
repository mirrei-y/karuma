pub const DATA_DIR: &str = "data";
pub const ARCHIVES_DIR: &str = "archives";
pub const MESSAGES_FILE: &str = "data/messages.toml";

// NOTE: thecatapi.com の使用により、10分以上開けないとレートリミットに引っかかる
pub const CAT_FETCH_INTERVAL: usize = 30;

/// メッセージのディスク書き込みを遅延させる秒数（debounce）
pub const SAVE_DEBOUNCE_SECS: u64 = 30;
