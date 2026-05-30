use chrono::{Datelike, Local, Timelike};
use crate::catapi::fetch_cat_image;
use crate::constants::PICTURES_DIR;
use crate::{Date, Time, AppWindow};

fn convert_chrono_datetime(datetime: chrono::DateTime<chrono::Local>) -> (Date, Time) {
    let date = Date {
        month: datetime.month() as i32,
        day: datetime.day() as i32,
        dayOfWeek: datetime.weekday().num_days_from_monday() as i32,
    };
    let time = Time {
        hour: datetime.hour() as i32,
        minute: datetime.minute() as i32,
    };
    (date, time)
}

/// UI スレッドに現在の日時を更新するよう要求します。
pub fn renew_datetime(window_weak: slint::Weak<AppWindow>) {
    let window = window_weak.unwrap();
    let (date, time) = convert_chrono_datetime(Local::now());
    window.set_date(date);
    window.set_time(time);
}

/// The Cat API からランダムな猫の画像を取得し、UI スレッドに更新を要求します。
pub async fn renew_cat_image(window_weak: slint::Weak<AppWindow>) {
    match fetch_cat_image().await {
        Ok((width, height, raw_pixels)) => {
            let _ = slint::invoke_from_event_loop(move || {
                let window = window_weak.unwrap();
                let buffer = slint::SharedPixelBuffer::<slint::Rgb8Pixel>::clone_from_slice(
                    &raw_pixels,
                    width,
                    height,
                );
                let image = slint::Image::from_rgb8(buffer);
                window.set_image(image);
                window.set_is_image_loading(false);
                window.set_is_image_error(false);
            });
        }
        Err(e) => {
            eprintln!("猫画像の取得に失敗しました: {}", e);
            let _ = slint::invoke_from_event_loop(move || {
                let window = window_weak.unwrap();
                window.set_is_image_loading(false);
                window.set_is_image_error(true);
            });
        }
    }
}

/// 投稿写真をランダムに1枚選んで UI スレッドに更新を要求します。
/// 投稿写真が1枚もない場合は false を返します。
pub async fn renew_user_picture(window_weak: slint::Weak<AppWindow>) -> bool {
    let dir = std::path::PathBuf::from(PICTURES_DIR);
    if !dir.exists() {
        return false;
    }

    let mut webp_files: Vec<std::path::PathBuf> = Vec::new();
    if let Ok(mut entries) = tokio::fs::read_dir(&dir).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("webp") {
                webp_files.push(path);
            }
        }
    }

    if webp_files.is_empty() {
        return false;
    }

    // ランダムに1枚選択
    let index = std::random::random::<usize>(..) % webp_files.len();
    let chosen = webp_files[index].clone();

    let image_bytes = match tokio::fs::read(&chosen).await {
        Ok(b) => b,
        Err(_) => return false,
    };

    let stem = chosen
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();
    let txt_path = dir.join(format!("{}.txt", stem));
    let description = tokio::fs::read_to_string(&txt_path)
        .await
        .unwrap_or_default();

    let dynamic_image = match image::load_from_memory(&image_bytes) {
        Ok(img) => img,
        Err(_) => return false,
    };
    let rgb_image = dynamic_image.into_rgb8();
    let (width, height) = rgb_image.dimensions();
    let raw_pixels = rgb_image.into_raw();

    let _ = slint::invoke_from_event_loop(move || {
        if let Some(window) = window_weak.upgrade() {
            let buffer = slint::SharedPixelBuffer::<slint::Rgb8Pixel>::clone_from_slice(
                &raw_pixels,
                width,
                height,
            );
            let image = slint::Image::from_rgb8(buffer);
            window.set_image(image);
            window.set_picture_mode(true);
            window.set_picture_description(slint::SharedString::from(description));
            window.set_is_image_loading(false);
            window.set_is_image_error(false);
        }
    });

    true
}

/// UI スレッドにメッセージリストを更新するよう要求します。
pub fn renew_message(window_weak: slint::Weak<AppWindow>, messages: Vec<crate::message::MessageData>) {
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(window) = window_weak.upgrade() {
            let slint_messages = messages
                .into_iter()
                .map(|m| crate::Message {
                    date: crate::Date {
                        month: m.month,
                        day: m.day,
                        dayOfWeek: m.day_of_week,
                    },
                    time: crate::Time {
                        hour: m.hour,
                        minute: m.minute,
                    },
                    author: slint::SharedString::from(m.author),
                    text: slint::SharedString::from(m.text),
                })
                .collect::<Vec<_>>();
            let model: std::rc::Rc<slint::VecModel<crate::Message>> =
                std::rc::Rc::new(slint::VecModel::from(slint_messages));
            let model_rc = slint::ModelRc::from(model);
            window.set_messages(model_rc);
        }
    });
}
