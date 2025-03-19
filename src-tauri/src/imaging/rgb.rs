use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use rayon::prelude::*;

// Allow unused variants for future expansion
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RgbChannel {
    Red = 0b001,
    Green = 0b010,
    Blue = 0b100,
    All = 0b111,
}

impl RgbChannel {
    pub fn contains(self, channel: RgbChannel) -> bool {
        (self as u8) & (channel as u8) != 0
    }

    // Add a convenience method to get all channels
    pub fn all() -> Self {
        Self::All
    }
}

impl std::ops::BitOr for RgbChannel {
    type Output = RgbChannel;

    fn bitor(self, rhs: Self) -> Self::Output {
        let result = (self as u8) | (rhs as u8);
        // Safety: we know this is valid since we're just combining existing flags
        unsafe { std::mem::transmute(result) }
    }
}

pub fn split_channels(image: &DynamicImage, channels: RgbChannel) -> Vec<RgbaImage> {
    let (width, height) = image.dimensions();

    // Determine which channels to process
    let channel_flags = [
        (RgbChannel::Red, "Red"),
        (RgbChannel::Green, "Green"),
        (RgbChannel::Blue, "Blue"),
    ];

    // Only allocate images for requested channels
    let active_channels: Vec<_> = channel_flags
        .iter()
        .filter(|(flag, _)| channels.contains(*flag))
        .collect();

    // Create empty images only for requested channels
    let mut images = Vec::new();
    for _ in 0..active_channels.len() {
        images.push(RgbaImage::new(width, height));
    }

    // Early return if no channels requested
    if active_channels.is_empty() {
        return images;
    }

    // Process pixels in parallel and collect the results
    let pixels: Vec<_> = image
        .to_rgba8()
        .enumerate_pixels()
        .par_bridge()
        .map(|(x, y, pixel)| {
            let image::Rgba([r, g, b, _]) = *pixel;

            // Only extract values for requested channels
            let mut channel_values = Vec::new();

            for (flag, _) in &active_channels {
                let value = match flag {
                    &RgbChannel::Red => r,
                    &RgbChannel::Green => g,
                    &RgbChannel::Blue => b,
                    _ => unreachable!(),
                };

                channel_values.push(value);
            }

            ((x, y), channel_values)
        })
        .collect();

    // Apply results to the images sequentially
    for ((x, y), values) in pixels {
        for (idx, value) in values.iter().enumerate() {
            images[idx].put_pixel(x, y, Rgba([*value, *value, *value, 255]));
        }
    }

    images
}
