use wasm_bindgen::prelude::*;
use std::io::Cursor;
use image::{codecs::gif::GifDecoder, AnimationDecoder};
use vtf::builder::VTFBuilder;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("GIF decoding error: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("VTF encoding error: {0}")]
    VtfError(#[from] vtf::Error),
}

pub fn gif_to_vtf(gif: &[u8]) -> Result<(Vec<u8>, f32), Error> {
    let image = GifDecoder::new(Cursor::new(gif))?;
    let frames = image.into_frames().collect_frames()?;

    let mut avg_delay = 0.0;
    let frames_count = frames.len();

    let mut vtf = VTFBuilder::new(vtf::ImageFormat::Dxt5);
    for frame in frames {
        let (numer, denom) = frame.delay().numer_denom_ms();
        let delay = (numer as f32 / denom as f32) / 1000.0;
        avg_delay += delay;

        let buffer = frame.into_buffer();
        let buffer = image::imageops::resize(
            &buffer,
            128,
            128,
            image::imageops::FilterType::Triangle,
        );

        vtf = vtf.add_frame(image::DynamicImage::ImageRgba8(buffer))?;
    }

    let avg_delay = avg_delay / frames_count.max(1) as f32;

    Ok((vtf.build()?, avg_delay))
}

#[wasm_bindgen]
pub fn wasm_gif_to_vtf(gif: &[u8]) -> Result<String, JsValue> {
    gif_to_vtf(gif)
        .map_err(|e| JsValue::from_str(&e.to_string()))
        .map(|(vtf_bytes, avg_delay)| {
            let mut string = String::with_capacity((vtf_bytes.len() * 4 / 3 + 4) + 32);
            string.push_str(avg_delay.to_string().as_str());
            string.push(':');
            string.push_str(&base64::encode(&vtf_bytes));
            string
        })
}