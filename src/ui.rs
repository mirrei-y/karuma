use chrono::{Datelike, Local, Timelike};
use crate::catapi::fetch_cat_image;
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
    if let Ok((width, height, raw_pixels)) = fetch_cat_image().await {
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
        });
    }
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
