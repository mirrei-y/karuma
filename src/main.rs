// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(random)]

mod catapi;
mod constants;
mod message;
mod server;
mod ui;

use std::error::Error;

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

    // NOTE: 毎日午前3時に期限切れ写真を削除する
    tokio::spawn(async move {
        loop {
            let now = chrono::Local::now();
            let mut next_3am = now.date_naive().and_hms_opt(3, 0, 0).unwrap();
            if now.time() >= chrono::NaiveTime::from_hms_opt(3, 0, 0).unwrap() {
                next_3am += chrono::Duration::days(1);
            }
            let next_3am_tz = next_3am.and_local_timezone(chrono::Local).unwrap();
            let delay = (next_3am_tz - now)
                .to_std()
                .unwrap_or(std::time::Duration::from_secs(0));
            tokio::time::sleep(delay).await;
            server::picture::cleanup_old_pictures().await;
        }
    });

    // NOTE: 起動時に既存のメッセージを読み込む
    let messages = message::load_messages();
    ui::renew_message(window_weak.clone(), messages);

    // NOTE: 毎日午前3時に実行されるスレッド
    let window_weak_cleanup = window_weak.clone();
    std::thread::spawn(move || {
        loop {
            let now = chrono::Local::now();
            let mut next_3am = now.date_naive().and_hms_opt(3, 0, 0).unwrap();

            // NOTE: すでに今日の午前3時を過ぎている場合は明日の午前3時をターゲットとする
            if now.time() >= chrono::NaiveTime::from_hms_opt(3, 0, 0).unwrap() {
                next_3am += chrono::Duration::days(1);
            }
            let next_3am_tz = next_3am.and_local_timezone(chrono::Local).unwrap();

            // NOTE: 待機時間を計算しスリープ
            let delay = (next_3am_tz - now).to_std().unwrap_or(std::time::Duration::from_secs(0));
            std::thread::sleep(delay);

            // NOTE: メッセージのアーカイブ
            message::archive_messages(window_weak_cleanup.clone());
        }
    });

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
    tokio::spawn({
        let window_weak = window_weak.clone();
        async move {
            loop {
                let now = chrono::Local::now();
                let mut next_7pm = now.date_naive().and_hms_opt(19, 0, 0).unwrap();
                if now.time() >= chrono::NaiveTime::from_hms_opt(19, 0, 0).unwrap() {
                    next_7pm += chrono::Duration::days(1);
                }
                let next_7pm_tz = next_7pm.and_local_timezone(chrono::Local).unwrap();
                let delay = (next_7pm_tz - now)
                    .to_std()
                    .unwrap_or(std::time::Duration::from_secs(0));
                tokio::time::sleep(delay).await;

                let window_weak = window_weak.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    window_weak.upgrade().map(|w| {
                        w.set_enable_page_switching(false);
                        w.set_current_page(0);
                    });
                });
            }
        }
    });
    // NOTE: 毎日午前7時にページを可変にする
    tokio::spawn({
        let window_weak = window_weak.clone();
        async move {
            loop {
                let now = chrono::Local::now();
                let mut next_7am = now.date_naive().and_hms_opt(7, 0, 0).unwrap();
                if now.time() >= chrono::NaiveTime::from_hms_opt(7, 0, 0).unwrap() {
                    next_7am += chrono::Duration::days(1);
                }
                let next_7am_tz = next_7am.and_local_timezone(chrono::Local).unwrap();
                let delay = (next_7am_tz - now)
                    .to_std()
                    .unwrap_or(std::time::Duration::from_secs(0));
                tokio::time::sleep(delay).await;

                let window_weak = window_weak.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    window_weak.upgrade().map(|w| {
                        w.set_enable_page_switching(true);
                    });
                });
            }
        }
    });

    // let timer = Timer::default();
    // timer.start(TimerMode::Repeated, std::time::Duration::from_millis(200), {
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let the_model: Rc<VecModel<Message>> = Rc::new(VecModel::from(vec![
    //             Message {
    //                 date: Date {
    //                     month: 1,
    //                     day: 1,
    //                     dayOfWeek: 0,
    //                 },
    //                 time: Time { hour: 12, minute: 0 },
    //                 author: SharedString::from(""),
    //                 text: SharedString::from("Happy New Year!"),
    //             }
    //         ]));
    //         let the_model_rc = ModelRc::from(the_model.clone());
    //         ui.set_messages(the_model_rc);
    //     }
    // });

    window.run()?;

    Ok(())
}
