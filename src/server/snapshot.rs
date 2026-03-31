use axum::{
    body::Body,
    http::{header, StatusCode},
    response::Response,
};
use image::{ImageEncoder, codecs::png::PngEncoder};
use slint::ComponentHandle;
use tokio::sync::oneshot;

/// 現在の UI をキャプチャし、PNG 画像として返します。
pub async fn get_snapshot(
    axum::extract::State(window_weak): axum::extract::State<slint::Weak<crate::AppWindow>>,
) -> Result<Response, StatusCode> {
    let (tx, rx) = oneshot::channel::<Result<Vec<u8>, String>>();

    // take_snapshot() は Slint のイベントループ上でしか呼べないため
    // invoke_from_event_loop で実行する
    slint::invoke_from_event_loop(move || {
        let result = (|| -> Result<Vec<u8>, String> {
            let window = window_weak
                .upgrade()
                .ok_or_else(|| "window upgrade failed".to_string())?;

            let pixel_buffer = window
                .window()
                .take_snapshot()
                .map_err(|e| format!("take_snapshot failed: {e}"))?;

            let width = pixel_buffer.width();
            let height = pixel_buffer.height();
            // SharedPixelBuffer<Rgba<u8>> の生バイト列を取得
            let raw: &[u8] = pixel_buffer.as_bytes();

            let mut png_bytes: Vec<u8> = Vec::new();
            PngEncoder::new(&mut png_bytes)
                .write_image(raw, width, height, image::ExtendedColorType::Rgba8)
                .map_err(|e| format!("PNG encode failed: {e}"))?;

            Ok(png_bytes)
        })();

        // 受信側が既にタイムアウト・ドロップしていても無視
        let _ = tx.send(result);
    })
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let png_bytes = rx
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "image/png")
        .header(header::CACHE_CONTROL, "no-store")
        .body(Body::from(png_bytes))
        .unwrap())
}
