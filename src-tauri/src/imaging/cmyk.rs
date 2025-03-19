use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use rayon::prelude::*;

// Allow unused variants for future expansion
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CmykChannel {
    Cyan = 0b0001,
    Magenta = 0b0010,
    Yellow = 0b0100,
    Black = 0b1000,
}

impl CmykChannel {
    #[inline]
    pub fn contains(&self, channel: CmykChannel) -> bool {
        (*self as u8) & (channel as u8) != 0
    }
}

// You can also implement the bitwise OR operator for convenience
impl std::ops::BitOr for CmykChannel {
    type Output = CmykChannel;

    fn bitor(self, rhs: Self) -> Self::Output {
        // Convert the enum values to u8, perform bitwise OR, then convert back to enum
        let result = (self as u8) | (rhs as u8);
        // Safety: we know this is valid since we're just combining existing flags
        unsafe { std::mem::transmute(result) }
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
    let c = round_u8(1.0 - r as f32 / max as f32);
    let m = round_u8(1.0 - g as f32 / max as f32);
    let y = round_u8(1.0 - b as f32 / max as f32);
    let k = 255 - max;
    (c, m, y, k)
}

pub fn cmyk_to_rgb(c: f64, m: f64, y: f64, k: f64) -> (u8, u8, u8) {
    let r = (255.0 * (1.0 - c) * (1.0 - k)).round().clamp(0.0, 255.0) as u8;
    let g = (255.0 * (1.0 - m) * (1.0 - k)).round().clamp(0.0, 255.0) as u8;
    let b = (255.0 * (1.0 - y) * (1.0 - k)).round().clamp(0.0, 255.0) as u8;
    (r, g, b)
}

// Optimized version for channel splitting
pub fn split_channels(image: &DynamicImage, channels: CmykChannel) -> Vec<RgbaImage> {
    let (width, height) = image.dimensions();

    // Determine which channels to process
    let channel_flags = [
        (CmykChannel::Cyan, "Cyan"),
        (CmykChannel::Magenta, "Magenta"),
        (CmykChannel::Yellow, "Yellow"),
        (CmykChannel::Black, "Black"),
    ];

    // Only process requested channels
    let active_channels: Vec<_> = channel_flags
        .iter()
        .filter(|(flag, _)| channels.contains(*flag))
        .collect();

    // Create empty images only for requested channels
    let mut images = vec![RgbaImage::new(width, height); active_channels.len()];

    // Early return if no channels requested
    if active_channels.is_empty() {
        return images;
    }

    // Process rows in parallel
    let rows: Vec<_> = (0..height).collect();
    let results: Vec<_> = rows
        .par_iter()
        .map(|y| {
            let mut channel_rows =
                vec![Vec::with_capacity(width as usize * 4); active_channels.len()];

            for x in 0..width {
                let pixel = image.get_pixel(x, *y);
                let Rgba([r, g, b, a]) = pixel;

                // Skip fully transparent pixels
                if a == 0 {
                    for channel_row in &mut channel_rows {
                        channel_row.extend_from_slice(&[0, 0, 0, 0]);
                    }
                    continue;
                }

                // Convert to CMYK
                let max = r.max(g.max(b));

                // Handle black pixels specially
                if max == 0 {
                    for (idx, (flag, _)) in active_channels.iter().enumerate() {
                        match flag {
                            &CmykChannel::Black => {
                                channel_rows[idx].extend_from_slice(&[0, 0, 0, 255])
                            }
                            _ => channel_rows[idx].extend_from_slice(&[255, 255, 255, 255]),
                        }
                    }
                    continue;
                }

                let c = 1.0 - r as f32 / max as f32;
                let m = 1.0 - g as f32 / max as f32;
                let y = 1.0 - b as f32 / max as f32;
                let k = 1.0 - (max as f32 / 255.0);

                // Process each requested channel
                for (idx, (flag, _)) in active_channels.iter().enumerate() {
                    let value = match flag {
                        &CmykChannel::Cyan => {
                            let r = ((1.0 - c) * (1.0 - k) * 255.0).round().min(255.0) as u8;
                            let g = ((1.0 - 0.0) * (1.0 - k) * 255.0).round().min(255.0) as u8;
                            let b = ((1.0 - 0.0) * (1.0 - k) * 255.0).round().min(255.0) as u8;
                            [r, g, b, 255]
                        }
                        &CmykChannel::Magenta => {
                            let r = ((1.0 - 0.0) * (1.0 - k) * 255.0).round().min(255.0) as u8;
                            let g = ((1.0 - m) * (1.0 - k) * 255.0).round().min(255.0) as u8;
                            let b = ((1.0 - 0.0) * (1.0 - k) * 255.0).round().min(255.0) as u8;
                            [r, g, b, 255]
                        }
                        &CmykChannel::Yellow => {
                            let r = ((1.0 - 0.0) * (1.0 - k) * 255.0).round().min(255.0) as u8;
                            let g = ((1.0 - 0.0) * (1.0 - k) * 255.0).round().min(255.0) as u8;
                            let b = ((1.0 - y) * (1.0 - k) * 255.0).round().min(255.0) as u8;
                            [r, g, b, 255]
                        }
                        &CmykChannel::Black => {
                            let value = (255.0 - k * 255.0).round() as u8;
                            [value, value, value, 255]
                        }
                        _ => unreachable!(),
                    };
                    channel_rows[idx].extend_from_slice(&value);
                }
            }

            (*y, channel_rows)
        })
        .collect();

    // Merge results back into final images
    for (y, rows) in results {
        for (idx, row) in rows.into_iter().enumerate() {
            for (x, chunk) in row.chunks_exact(4).enumerate() {
                let pixel = Rgba([chunk[0], chunk[1], chunk[2], chunk[3]]);
                images[idx].put_pixel(x as u32, y, pixel);
            }
        }
    }

    images
}

fn split_rgb_to_cmyk_channels(img: &DynamicImage) -> Option<Vec<RgbaImage>> {
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    let num_pixels = (width * height) as usize;

    // Pre-allocate pixel data for each channel
    let mut c_data = vec![0u8; num_pixels * 4];
    let mut m_data = vec![0u8; num_pixels * 4];
    let mut y_data = vec![0u8; num_pixels * 4];
    let mut k_data = vec![0u8; num_pixels * 4];

    // Process pixels in parallel
    c_data
        .par_chunks_mut(4)
        .zip(m_data.par_chunks_mut(4))
        .zip(y_data.par_chunks_mut(4))
        .zip(k_data.par_chunks_mut(4))
        .enumerate()
        .for_each(|(i, (((c, m), y), k))| {
            let x = (i as u32) % width;
            let y_pos = (i as u32) / width;
            let pixel = rgb_img.get_pixel(x, y_pos);
            let (c_val, m_val, y_val, k_val) = rgb_to_cmyk(pixel[0], pixel[1], pixel[2]);

            c.copy_from_slice(&[0, c_val, c_val, 255]);
            m.copy_from_slice(&[m_val, 0, m_val, 255]);
            y.copy_from_slice(&[y_val, y_val, 0, 255]);
            k.copy_from_slice(&[k_val, k_val, k_val, 255]);
        });

    // Construct images from raw pixel data
    let c_img = RgbaImage::from_raw(width, height, c_data)?;
    let m_img = RgbaImage::from_raw(width, height, m_data)?;
    let y_img = RgbaImage::from_raw(width, height, y_data)?;
    let k_img = RgbaImage::from_raw(width, height, k_data)?;

    Some(vec![c_img, m_img, y_img, k_img])
}

pub fn split_channels_new(image: &DynamicImage, channels: CmykChannel) -> Option<Vec<RgbaImage>> {
    let channel_images = split_rgb_to_cmyk_channels(image)?;

    let channel_flags = [
        (CmykChannel::Cyan, 0),
        (CmykChannel::Magenta, 1),
        (CmykChannel::Yellow, 2),
        (CmykChannel::Black, 3),
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
