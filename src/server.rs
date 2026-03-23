mod console;

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::{get, post, put},
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
    if let Some(auth_header) = req.headers().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(base64_passphrase) = auth_str.strip_prefix("Bearer ") {
                if let Ok(decoded_bytes) = general_purpose::STANDARD.decode(base64_passphrase) {
                    if let Ok(decoded_str) = String::from_utf8(decoded_bytes) {
                        if decoded_str == PASSPHRASE {
                            return Ok(next.run(req).await);
                        }
                    }
                }
            }
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}

/// API サーバーを起動します。
pub async fn run_server(window_weak: slint::Weak<crate::AppWindow>) -> std::io::Result<()> {
    let api_routes = Router::new()
        .route("/config", get(get_config).post(update_config))
        .route("/messages", get(get_messages).put(update_messages))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(window_weak.clone());

    let app = Router::new()
        .nest("/api", api_routes)
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
