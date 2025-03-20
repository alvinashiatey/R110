use crate::errors::Error;
use base64::{engine::general_purpose::STANDARD as base64_engine, Engine};
use image::ImageFormat;
use rayon::prelude::*;
use std::io::Cursor;

#[derive(serde::Serialize)]
pub struct ProcessedImage {
    pub base64: String,
}

pub struct ColorMap {
    image_path: String,
    hex: String,
}

impl ColorMap {
    fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
        (r, g, b)
    }

    pub fn new(image_path: String, hex: String) -> Self {
        Self { image_path, hex }
    }

    pub fn apply(&self) -> Result<ProcessedImage, Error> {
        let img = match image::open(&self.image_path) {
            Ok(img) => img,
            Err(e) => return Err(Error::Processing(format!("Failed to open image: {}", e))),
        };

        let mut rgb_img = img.to_rgb8();
        let (r, g, b) = Self::hex_to_rgb(&self.hex);

        rgb_img.par_chunks_mut(3).for_each(|pixel| {
            let grayscale =
                (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;
            let inverted = 255 - grayscale;

            let grayscale_f32 = grayscale as f32 / 255.0;
            let inverted_f32 = inverted as f32 / 255.0;

            pixel[0] = (inverted_f32 * r as f32 + grayscale_f32 * 255.0) as u8;
            pixel[1] = (inverted_f32 * g as f32 + grayscale_f32 * 255.0) as u8;
            pixel[2] = (inverted_f32 * b as f32 + grayscale_f32 * 255.0) as u8;
        });

        let mut buffer = Cursor::new(Vec::new());
        rgb_img
            .write_to(&mut buffer, ImageFormat::Png)
            .map_err(|e| Error::Processing(format!("Failed to encode image: {}", e)))?;

        let base64_string = base64_engine.encode(buffer.into_inner());

        Ok(ProcessedImage {
            base64: format!("data:image/png;base64,{}", base64_string),
        })
    }
}
