use axum::{body::Body, http::{header, StatusCode, Uri}, response::{IntoResponse, Response}};
use rust_embed::RustEmbed;
use std::path::PathBuf;

use crate::constants::WEB_DIR;

#[derive(RustEmbed)]
#[folder = "console/dist"]
struct Asset;

/// 静的ファイルを返します。
///
/// `web/` ディレクトリが存在する場合はファイルシステムから直接提供します。
/// 存在しない場合はバイナリに埋め込まれたファイルにフォールバックします。
/// どちらの場合も、ファイルが見つからないときは SPA 用に `index.html` を返します。
pub(super) async fn get_static_file(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let file_path = if path.is_empty() { "index.html" } else { path };

    if PathBuf::from(WEB_DIR).is_dir() {
        serve_from_filesystem(file_path).await
    } else {
        serve_from_embed(file_path)
    }
}

async fn serve_from_filesystem(file_path: &str) -> Response {
    let web_dir = PathBuf::from(WEB_DIR);
    let target = web_dir.join(file_path);

    // パストラバーサル対策: web/ の外に出ないことを確認
    let canonical_web = match web_dir.canonicalize() {
        Ok(p) => p,
        Err(_) => return internal_server_error(),
    };
    let canonical_target = match target.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            // ファイルが存在しない場合は SPA 用に index.html を返す
            return serve_filesystem_index(&canonical_web).await;
        }
    };
    if !canonical_target.starts_with(&canonical_web) {
        return bad_request();
    }

    if canonical_target.is_file() {
        match tokio::fs::read(&canonical_target).await {
            Ok(bytes) => {
                let mime = mime_guess::from_path(file_path).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(Body::from(bytes))
                    .unwrap_or_else(|_| internal_server_error())
            }
            Err(_) => internal_server_error(),
        }
    } else {
        // ディレクトリや存在しないパスは SPA 用に index.html を返す
        serve_filesystem_index(&canonical_web).await
    }
}

async fn serve_filesystem_index(web_dir: &PathBuf) -> Response {
    let index_path = web_dir.join("index.html");
    match tokio::fs::read(&index_path).await {
        Ok(bytes) => Response::builder()
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(Body::from(bytes))
            .unwrap_or_else(|_| internal_server_error()),
        Err(_) => not_found(),
    }
}

fn serve_from_embed(file_path: &str) -> Response {
    let mut resolved = file_path.to_string();

    if Asset::get(&resolved).is_none() {
        resolved = "index.html".to_string();
    }

    match Asset::get(&resolved) {
        Some(content) => {
            let mime = mime_guess::from_path(&resolved).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(Body::from(content.data))
                .unwrap_or_else(|_| internal_server_error())
        }
        None => not_found(),
    }
}

fn not_found() -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}

fn bad_request() -> Response {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::empty())
        .unwrap()
}

fn internal_server_error() -> Response {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())
        .unwrap()
}
