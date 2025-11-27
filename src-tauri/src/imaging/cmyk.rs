use image::{DynamicImage, RgbImage};
use rayon::prelude::*;

/// Represents a set of CMYK channels using bitflags
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CmykChannels(u8);

#[allow(dead_code)]
impl CmykChannels {
    pub const CYAN: CmykChannels = CmykChannels(0b0001);
    pub const MAGENTA: CmykChannels = CmykChannels(0b0010);
    pub const YELLOW: CmykChannels = CmykChannels(0b0100);
    pub const BLACK: CmykChannels = CmykChannels(0b1000);
    pub const ALL: CmykChannels = CmykChannels(0b1111);

    #[inline]
    pub fn contains(self, channel: CmykChannels) -> bool {
        (self.0 & channel.0) != 0
    }
}

impl std::ops::BitOr for CmykChannels {
    type Output = CmykChannels;

    fn bitor(self, rhs: Self) -> Self::Output {
        CmykChannels(self.0 | rhs.0)
    }
}

#[inline]
fn mul_add(multiplier: f32, multiplicand: f32, addend: f32) -> f32 {
    if cfg!(target_feature = "fma") {
        multiplier.mul_add(multiplicand, addend)
    } else {
        multiplier * multiplicand + addend
    }
}

#[inline]
fn round_u8(value: f32) -> u8 {
    mul_add(value, 255.0, 0.5) as u8
}

pub fn rgb_to_cmyk(r: u8, g: u8, b: u8) -> (u8, u8, u8, u8) {
    let max = std::cmp::max(std::cmp::max(r, g), b);

    // Handle pure black (r=g=b=0) case to avoid division by zero
    if max == 0 {
        return (0, 0, 0, 255); // Pure black: no CMY, full K
    }

    let c = round_u8(1.0 - r as f32 / max as f32);
    let m = round_u8(1.0 - g as f32 / max as f32);
    let y = round_u8(1.0 - b as f32 / max as f32);
    let k = 255 - max;
    (c, m, y, k)
}

fn split_rgb_to_cmyk_channels(img: &DynamicImage) -> Option<Vec<RgbImage>> {
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    let num_pixels = (width * height) as usize;

    // Pre-allocate pixel data for each channel
    let mut c_data = vec![0u8; num_pixels * 3];
    let mut m_data = vec![0u8; num_pixels * 3];
    let mut y_data = vec![0u8; num_pixels * 3];
    let mut k_data = vec![0u8; num_pixels * 3];

    // Process pixels in parallel
    c_data
        .par_chunks_mut(3)
        .zip(m_data.par_chunks_mut(3))
        .zip(y_data.par_chunks_mut(3))
        .zip(k_data.par_chunks_mut(3))
        .enumerate()
        .for_each(|(i, (((c, m), y), k))| {
            let x = (i as u32) % width;
            let y_pos = (i as u32) / width;
            let pixel = rgb_img.get_pixel(x, y_pos);
            let (c_val, m_val, y_val, k_val) = rgb_to_cmyk(pixel[0], pixel[1], pixel[2]);

            // Use grayscale (black and white) for all channels
            // Inverting values since in CMYK, 0 means no ink (white) and 255 means full ink (black)
            let c_gray = 255 - c_val;
            let m_gray = 255 - m_val;
            let y_gray = 255 - y_val;
            let k_gray = 255 - k_val;
            // K channel is already correct (255 = black, 0 = white)

            // Set all RGB channels to the same value to create grayscale
            c.copy_from_slice(&[c_gray, c_gray, c_gray]);
            m.copy_from_slice(&[m_gray, m_gray, m_gray]);
            y.copy_from_slice(&[y_gray, y_gray, y_gray]);
            k.copy_from_slice(&[k_gray, k_gray, k_gray]);
        });

    // Construct images from raw pixel data
    let c_img = RgbImage::from_raw(width, height, c_data)?;
    let m_img = RgbImage::from_raw(width, height, m_data)?;
    let y_img = RgbImage::from_raw(width, height, y_data)?;
    let k_img = RgbImage::from_raw(width, height, k_data)?;

    Some(vec![c_img, m_img, y_img, k_img])
}

pub fn split_channels(image: &DynamicImage, channels: CmykChannels) -> Option<Vec<RgbImage>> {
    let channel_images = split_rgb_to_cmyk_channels(image)?;

    let channel_flags = [
        (CmykChannels::CYAN, 0),
        (CmykChannels::MAGENTA, 1),
        (CmykChannels::YELLOW, 2),
        (CmykChannels::BLACK, 3),
    ];

    // Filter to get only the requested channels
    let active_channels: Vec<_> = channel_flags
        .iter()
        .filter(|(flag, _)| channels.contains(*flag))
        .collect();

    // Early return if no channels requested
    if active_channels.is_empty() {
        return Some(Vec::new());
    }

    let images: Vec<_> = active_channels
        .iter()
        .map(|(_, idx)| channel_images[*idx].clone())
        .collect();

    // print length of images
    Some(images)
}
