use crate::errors::Error;
use base64::{engine::general_purpose::STANDARD as base64_engine, Engine};
use image::{ImageBuffer, ImageFormat, Rgba, RgbaImage};
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
            Ok(img) => img.to_luma8(),
            Err(e) => return Err(Error::Processing(e.to_string())),
        };

        let (width, height) = img.dimensions();
        let mut new_img: RgbaImage = ImageBuffer::new(width, height);
        let (r, g, b) = Self::hex_to_rgb(&self.hex);

        // Pre-calculate floating-point values for efficiency
        let r_f32 = r as f32;
        let g_f32 = g as f32;
        let b_f32 = b as f32;

        new_img
            .enumerate_pixels_mut()
            .par_bridge()
            .for_each(|(x, y, pixel)| {
                let grayscale = img.get_pixel(x, y)[0];
                let inverted = 255 - grayscale;

                let grayscale_f32 = grayscale as f32 / 255.0;
                let inverted_f32 = inverted as f32 / 255.0;

                let new_r = (inverted_f32 * r_f32 + grayscale_f32 * 255.0) as u8;
                let new_g = (inverted_f32 * g_f32 + grayscale_f32 * 255.0) as u8;
                let new_b = (inverted_f32 * b_f32 + grayscale_f32 * 255.0) as u8;

                *pixel = Rgba([new_r, new_g, new_b, 255]);
            });

        let mut buffer = Cursor::new(Vec::new());
        new_img
            .write_to(&mut buffer, ImageFormat::Png)
            .map_err(|e| Error::Processing(e.to_string()))?;
        let base64_string = base64_engine.encode(buffer.into_inner());

        Ok(ProcessedImage {
            base64: format!("data:image/png;base64,{}", base64_string),
        })
    }
}
