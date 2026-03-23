use std::error::Error;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CatApiResponse {
    url: String,
}

/// The Cat API からランダムな猫の画像を取得し、幅、高さ、およびピクセルデータのタプルとして返します。
pub async fn fetch_cat_image() -> Result<(u32, u32, Vec<u8>), Box<dyn Error>> {
    let api_url = "https://api.thecatapi.com/v1/images/search";
    let response: Vec<CatApiResponse> = reqwest::get(api_url).await?.json().await?;
    let image_url = &response.first().ok_or("レスポンスに画像が含まれていません")?.url;
    let image_bytes = reqwest::get(image_url).await?.bytes().await?;
    let dynamic_image = image::load_from_memory(&image_bytes)?;
    let rgb_image = dynamic_image.into_rgb8();
    let (width, height) = rgb_image.dimensions();
    let raw_pixels: Vec<u8> = rgb_image.into_raw();

    Ok((width, height, raw_pixels))
}
