// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(random)]

mod catapi;
mod constants;
mod message;
mod server;
mod ui;
mod utils;

use std::error::Error;
use utils::cron::schedule_at;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let window = AppWindow::new()?;
    let window_weak = window.as_weak();

    // NOTE: APIサーバーをバックグラウンドで起動
    let window_weak_server = window_weak.clone();
    tokio::spawn(async move {
        if let Err(e) = server::run_server(window_weak_server).await {
            eprintln!("API server error: {}", e);
        }
    });

    // NOTE: 毎日午前3時に期限切れ写真を削除し、メッセージをアーカイブする
    schedule_at(3, 0, 0, &window_weak, |window| {
        tokio::spawn(server::picture::cleanup_old_pictures());
        message::archive_messages(window.as_weak());
    });

    // NOTE: 起動時に既存のメッセージを読み込む
    let messages = message::load_messages();
    ui::renew_message(window_weak.clone(), messages);

    // NOTE: 1秒ごとに実行されるスレッド
    let window_weak_per_sec = window_weak.clone();
    std::thread::spawn(move || {
        loop {
            let delay_ms = 1000 - chrono::Local::now().timestamp_subsec_millis() + 5;
            std::thread::sleep(std::time::Duration::from_millis(delay_ms as u64));

            let window_weak_clone = window_weak_per_sec.clone();
            let _ = slint::invoke_from_event_loop(move || {
                // NOTE: 日時を更新
                ui::renew_datetime(window_weak_clone);
            });
        }
    });
    ui::renew_datetime(window.as_weak());

    // NOTE: 起動時に画像を読み込み、以降30分ごとに画像を入れ替える
    let window_weak_pic = window.as_weak();
    tokio::spawn(async move {
        let interval_mins = constants::PICTURE_SWITCH_INTERVAL as u64;
        let mut ticker = tokio::time::interval(std::time::Duration::from_mins(interval_mins));
        loop {
            ticker.tick().await;

            // NOTE: 画面をロード中にする
            let _ = slint::invoke_from_event_loop({
                let w = window_weak_pic.clone();
                move || {
                    if let Some(window) = w.upgrade() {
                        window.set_is_image_loading(true);
                        window.set_is_image_error(false);
                    }
                }
            });

            // NOTE: 50%/50% の確率でユーザ写真と Cat API の画像を表示
            let use_user_picture = std::random::random::<bool>(..);
            if !use_user_picture || !ui::renew_user_picture(window_weak_pic.clone()).await {
                ui::renew_cat_image(window_weak_pic.clone()).await;
            }
        }
    });

    // NOTE: 毎日午後7時にメッセージページに固定する
    schedule_at(19, 0, 0, &window_weak, |window| {
        window.set_enable_page_switching(false);
        window.set_current_page(0);
    });
    // NOTE: 毎日午前7時にページを可変にする
    schedule_at(7, 0, 0, &window_weak, |window| {
        window.set_enable_page_switching(true);
    });

    window.run()?;

    Ok(())
}
