use axum::{body::Body, http::{header, StatusCode, Uri}, response::{IntoResponse, Response}};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "console/dist"]
struct Asset;

/// バイナリに同梱された静的ファイルを返します。
pub(super) async fn get_static_file(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    // NOTE: 空なら index.html を返す
    let mut file_path = if path.is_empty() {
        "index.html".to_string()
    } else {
        path.to_string()
    };

    // NOTE: ファイルが存在しない場合は SPA 用に index.html を返す
    if Asset::get(&file_path).is_none() {
        if Asset::get("index.html").is_some() {
            file_path = "index.html".to_string();
        }
    }

    match Asset::get(&file_path) {
        Some(content) => {
            let mime = mime_guess::from_path(&file_path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(Body::from(content.data))
                .unwrap_or_else(|_| {
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::empty())
                        .unwrap()
                })
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap(),
    }
}
