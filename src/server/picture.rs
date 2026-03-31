use axum::{
    body::Body,
    extract::multipart::Multipart,
    http::{header, StatusCode},
    response::Response,
    Json,
};
use chrono::Local;
use serde::Serialize;
use std::path::PathBuf;
use tokio::fs;

use crate::constants::{PICTURE_RETENTION_DAYS, PICTURES_DIR};

/// WebP のマジックバイトを確認します。
fn is_webp(bytes: &[u8]) -> bool {
    bytes.len() >= 12
        && &bytes[0..4] == b"RIFF"
        && &bytes[8..12] == b"WEBP"
}

#[derive(Serialize)]
pub struct PictureEntry {
    pub id: String,
    pub description: String,
}

/// 写真一覧を返します（最新順）。
pub async fn list_pictures() -> Result<Json<Vec<PictureEntry>>, StatusCode> {
    let dir = PathBuf::from(PICTURES_DIR);
    if !dir.exists() {
        return Ok(Json(vec![]));
    }

    let mut entries = fs::read_dir(&dir)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut pictures: Vec<PictureEntry> = Vec::new();

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // .webp ファイルのみ対象とし、対応する .txt を読む
        if !name_str.ends_with(".webp") {
            continue;
        }

        let id = name_str.trim_end_matches(".webp").to_string();
        let txt_path = dir.join(format!("{}.txt", id));
        let description = fs::read_to_string(&txt_path).await.unwrap_or_default();

        pictures.push(PictureEntry { id, description });
    }

    // ファイル名降順（= 投稿日時の新しい順）
    pictures.sort_by(|a, b| b.id.cmp(&a.id));

    Ok(Json(pictures))
}

/// 写真ファイルを返します。
pub async fn get_picture_file(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Response, StatusCode> {
    // パストラバーサル対策: id にスラッシュやドットドットを含めない
    if id.contains('/') || id.contains("..") {
        return Err(StatusCode::BAD_REQUEST);
    }

    let path = PathBuf::from(PICTURES_DIR).join(format!("{}.webp", id));
    let bytes = fs::read(&path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "image/webp")
        .body(Body::from(bytes))
        .unwrap())
}

/// 写真と説明文を受け取り、`data/pictures/YYYYMMDD-XXXX.{webp,txt}` に保存します。
pub async fn upload_picture(mut multipart: Multipart) -> Result<StatusCode, StatusCode> {
    let dir = PathBuf::from(PICTURES_DIR);
    fs::create_dir_all(&dir)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut image_bytes: Option<Vec<u8>> = None;
    let mut description = String::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        match field.name() {
            Some("image") => {
                let data = field
                    .bytes()
                    .await
                    .map_err(|_| StatusCode::BAD_REQUEST)?;
                image_bytes = Some(data.to_vec());
            }
            Some("description") => {
                description = field
                    .text()
                    .await
                    .map_err(|_| StatusCode::BAD_REQUEST)?;
            }
            _ => {}
        }
    }

    let webp_bytes = image_bytes.ok_or(StatusCode::BAD_REQUEST)?;

    // クライアント側で WebP に変換済みであることを確認
    if !is_webp(&webp_bytes) {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    let date_str = Local::now().format("%Y%m%d").to_string();

    // YYYYMMDD- で始まる既存ファイル数をカウントして連番を決定
    let prefix = format!("{}-", date_str);
    let mut count = 0u32;
    if let Ok(mut entries) = fs::read_dir(&dir).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with(&prefix) && name_str.ends_with(".webp") {
                count += 1;
            }
        }
    }
    let id = format!("{}{:04}", prefix, count);

    fs::write(dir.join(format!("{}.webp", id)), &webp_bytes)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    fs::write(dir.join(format!("{}.txt", id)), description.trim())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

/// 期限切れ（PICTURE_RETENTION_DAYS 日以上前）の写真ファイルを削除します。
pub async fn cleanup_old_pictures() {
    let dir = PathBuf::from(PICTURES_DIR);
    if !dir.exists() {
        return;
    }

    let threshold = Local::now() - chrono::Duration::days(PICTURE_RETENTION_DAYS);
    let threshold_str = threshold.format("%Y%m%d").to_string();

    let mut entries = match fs::read_dir(&dir).await {
        Ok(e) => e,
        Err(_) => return,
    };

    while let Ok(Some(entry)) = entries.next_entry().await {
        let name = entry.file_name();
        let name_str = name.to_string_lossy().to_string();

        // "YYYYMMDD-" の日付部分を抽出して比較
        let date_part = &name_str[..8.min(name_str.len())];
        if date_part.len() == 8 && date_part < threshold_str.as_str() {
            if let Err(e) = fs::remove_file(entry.path()).await {
                eprintln!("古い写真ファイルの削除に失敗しました: {}", e);
            }
        }
    }
}
