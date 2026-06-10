use std::sync::Arc;

use slint::Weak;

use crate::AppWindow;

/// 次に指定した時刻が来るまでの時間を計算して返します。
fn get_time_until(hour: u32, minute: u32, second: u32) -> std::time::Duration {
    let now = chrono::Local::now();
    let mut next_time = now.date_naive().and_hms_opt(hour, minute, second).unwrap();
    if now.time() >= chrono::NaiveTime::from_hms_opt(hour, minute, second).unwrap() {
        next_time += chrono::Duration::days(1);
    }
    let next_time_tz = next_time.and_local_timezone(chrono::Local).unwrap();
    (next_time_tz - now).to_std().unwrap_or(std::time::Duration::from_secs(0))
}

/// 指定した時刻にコールバックを実行します。
pub fn schedule_at(hour: u32, minute: u32, second: u32, window_weak: &Weak<AppWindow>, callback: impl Fn(AppWindow) + Copy + Send + Sync + 'static) {
    tokio::spawn({
        let callback = Arc::new(callback);
        let window_weak = window_weak.clone();

        async move {
            loop {
                tokio::time::sleep(get_time_until(hour, minute, second)).await;

                let window_weak = window_weak.clone();
                let callback = Arc::clone(&callback);
                let _ = slint::invoke_from_event_loop(move || {
                    window_weak.upgrade().map(|w| {
                        callback(w);
                    });
                });
            }
        }
    });
}
