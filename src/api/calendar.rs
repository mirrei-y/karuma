use crate::constants::CALENDAR_CACHE_DURATION_SECS;
use crate::constants::CALENDAR_ICS_URL;
use crate::Schedule;
use chrono::{DateTime, Duration, Local, TimeZone};
use tokio::sync::Mutex;

struct CachedCalendar {
    events: Vec<CalendarEvent>,
    fetched_at: DateTime<Local>,
}

#[derive(Clone)]
pub(crate) struct CalendarEvent {
    start: DateTime<Local>,
    end: DateTime<Local>,
    summary: String,
}

lazy_static::lazy_static! {
    static ref CALENDAR_CACHE: Mutex<Option<CachedCalendar>> = Mutex::new(None);
}

/// ICS データをパースして予定のリストを返します。
fn parse_ics(ics_data: &str) -> Vec<CalendarEvent> {
    let mut events = Vec::new();
    let mut current_start: Option<DateTime<Local>> = None;
    let mut current_end: Option<DateTime<Local>> = None;
    let mut current_summary: Option<String> = None;

    for line in ics_data.lines() {
        if line.starts_with("BEGIN:VEVENT") {
            current_start = None;
            current_end = None;
            current_summary = None;
        } else if line.starts_with("DTSTART") {
            if let Some(value) = line.split(':').nth(1) {
                current_start = parse_ical_datetime(value);
            }
        } else if line.starts_with("DTEND") {
            if let Some(value) = line.split(':').nth(1) {
                current_end = parse_ical_datetime(value);
            }
        } else if line.starts_with("SUMMARY:") {
            let summary = line.trim_start_matches("SUMMARY:").to_string();
            current_summary = Some(summary);
        } else if line.starts_with("END:VEVENT") {
            if let (Some(start), Some(end), Some(summary)) =
                (current_start, current_end, current_summary.take())
            {
                events.push(CalendarEvent {
                    start,
                    end,
                    summary,
                });
            }
        }
    }
    events
}

/// ICS 形式の日時文字列を chrono の DateTime に変換します。失敗した場合は None を返します。
fn parse_ical_datetime(s: &str) -> Option<DateTime<Local>> {
    let s = s.trim();
    if s.ends_with('Z') {
        // UTC 形式
        let utc_str = s.trim_end_matches('Z');
        let naive_utc = chrono::NaiveDateTime::parse_from_str(utc_str, "%Y%m%dT%H%M%S").ok()?;
        Some(
            DateTime::<chrono::Utc>::from_naive_utc_and_offset(naive_utc, chrono::Utc)
                .with_timezone(&Local),
        )
    } else {
        // ローカル時刻として扱う
        let naive = chrono::NaiveDateTime::parse_from_str(s, "%Y%m%dT%H%M%S").ok()?;
        match Local.from_local_datetime(&naive) {
            chrono::LocalResult::Single(dt) => Some(dt),
            _ => None,
        }
    }
}

/// キャッシュを無視して ICS を再取得します。
pub async fn fetch_calendar() -> Result<Vec<CalendarEvent>, reqwest::Error> {
    let response = reqwest::get(CALENDAR_ICS_URL).await?;
    let ics_data = response.text().await?;
    let events = parse_ics(&ics_data);

    // キャッシュを更新
    let mut cache = CALENDAR_CACHE.try_lock().unwrap();
    *cache = Some(CachedCalendar {
        events: events.clone(),
        fetched_at: Local::now(),
    });

    Ok(events)
}

/// キャッシュを考慮してカレンダー予定を取得します。
pub async fn get_calendar() -> Result<Vec<CalendarEvent>, reqwest::Error> {
    let should_fetch = {
        let cache = CALENDAR_CACHE.try_lock().unwrap();
        match *cache {
            Some(ref cached) => {
                let elapsed = Local::now()
                    .signed_duration_since(cached.fetched_at);
                elapsed > Duration::seconds(CALENDAR_CACHE_DURATION_SECS as i64)
            }
            None => true,
        }
    };

    if should_fetch {
        fetch_calendar().await
    } else {
        let events = {
            let cache = CALENDAR_CACHE.try_lock().unwrap();
            cache.as_ref().map(|c| c.events.clone())
        };
        match events {
            Some(events) => Ok(events),
            None => fetch_calendar().await,
        }
    }
}

/// 今日の予定を Slint 用の Schedule 構造体のリストに変換します。
pub fn convert_to_schedules(events: Vec<CalendarEvent>) -> Vec<Schedule> {
    let today = Local::now().date_naive();
    let now = Local::now();

    let mut schedules: Vec<Schedule> = events
        .into_iter()
        .filter(|e| e.start.date_naive() == today)
        .map(|e| {
            let time_range = format!(
                "{}～{}",
                e.start.format("%H:%M"),
                e.end.format("%H:%M")
            );
            let is_future = e.start > now;
            Schedule {
                time_range: slint::SharedString::from(time_range),
                title: slint::SharedString::from(e.summary),
                is_future,
            }
        })
        .collect();

    // 開始時刻順にソート
    schedules.sort_by(|a, b| {
        let a_time: String = a.time_range.as_str().into();
        let b_time: String = b.time_range.as_str().into();
        a_time.cmp(&b_time)
    });

    schedules
}
