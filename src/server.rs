mod console;
pub mod picture;
pub mod snapshot;

use axum::{
    extract::{Path, Request},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfigPayload {
    // 将来的な設定フィールドをここに追加
    pub example_setting: Option<String>,
}

const PASSPHRASE: &str = "PASSPHRASE_HERE";

async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let encoded_passphrase = req.headers().get("authorization").ok_or(StatusCode::UNAUTHORIZED)?
        .to_str().or(Err(StatusCode::BAD_REQUEST))?
        .strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;
    let decoded_passphrase_bytes = general_purpose::STANDARD.decode(encoded_passphrase).or(Err(StatusCode::UNAUTHORIZED))?;
    let decoded_passphrase = String::from_utf8(decoded_passphrase_bytes).or(Err(StatusCode::UNAUTHORIZED))?;

    if decoded_passphrase == PASSPHRASE {
        return Ok(next.run(req).await);
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }
}

/// API サーバーを起動します。
pub async fn run_server(window_weak: slint::Weak<crate::AppWindow>) -> std::io::Result<()> {
    let authed_routes = Router::new()
        .route("/config", get(get_config).post(update_config))
        .route("/messages", get(get_messages).put(update_messages))
        .route("/messages/archives", get(get_archive_list))
        .route("/messages/archives/{date}", get(get_archive))
        .route("/snapshot", get(snapshot::get_snapshot))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(window_weak.clone());

    let public_routes = Router::new()
        .route("/pictures", get(picture::list_pictures).post(picture::upload_picture))
        .route("/pictures/{id}", get(picture::get_picture_file));

    let app = Router::new()
        .nest("/api", authed_routes)
        .nest("/api", public_routes)
        .fallback(console::get_static_file)
        .with_state(window_weak);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:20400").await?;
    axum::serve(listener, app).await
}

async fn get_messages() -> Json<Vec<crate::message::MessageData>> {
    Json(crate::message::get_messages())
}

async fn update_messages(
    axum::extract::State(window_weak): axum::extract::State<slint::Weak<crate::AppWindow>>,
    Json(payload): Json<Vec<crate::message::MessageData>>,
) -> StatusCode {
    crate::message::set_messages(window_weak, payload);
    StatusCode::NO_CONTENT
}

async fn get_archive_list() -> Json<Vec<String>> {
    Json(crate::message::list_archives())
}

async fn get_archive(
    Path(date): Path<String>,
) -> Result<Json<Vec<crate::message::MessageData>>, StatusCode> {
    match crate::message::load_archive(&date) {
        Some(messages) => Ok(Json(messages)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_config() -> Json<AppConfigPayload> {
    // 将来的には現在の設定を返す
    Json(AppConfigPayload {
        example_setting: Some("current_value".to_string()),
    })
}

async fn update_config(Json(payload): Json<AppConfigPayload>) -> Json<&'static str> {
    // 将来的には設定の更新処理を行う
    println!("Received config update: {:?}", payload);
    Json("Success")
}
