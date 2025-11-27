use image::{DynamicImage, Rgb, RgbImage};

pub trait ImageEffect {
    fn apply(&self, image: &DynamicImage) -> DynamicImage;
}

pub struct Dither;
pub struct HalfTone;
pub struct Threshold;
pub struct Posterize;
pub struct Original;

impl ImageEffect for Original {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        image.clone()
    }
}

impl ImageEffect for Dither {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        let gray = image.to_luma8();
        let width = gray.width();
        let height = gray.height();

        // Buffer for error diffusion (using f32 to handle error distribution)
        let mut buffer: Vec<f32> = gray.pixels().map(|p| p[0] as f32).collect();

        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize;
                let old_pixel = buffer[idx];
                let new_pixel = if old_pixel > 128.0 { 255.0 } else { 0.0 };
                buffer[idx] = new_pixel;

                let quant_error = old_pixel - new_pixel;

                // Floyd-Steinberg distribution
                // x+1
                if x + 1 < width {
                    buffer[idx + 1] += quant_error * 7.0 / 16.0;
                }
                // x-1, y+1
                if x > 0 && y + 1 < height {
                    buffer[idx + width as usize - 1] += quant_error * 3.0 / 16.0;
                }
                // y+1
                if y + 1 < height {
                    buffer[idx + width as usize] += quant_error * 5.0 / 16.0;
                }
                // x+1, y+1
                if x + 1 < width && y + 1 < height {
                    buffer[idx + width as usize + 1] += quant_error * 1.0 / 16.0;
                }
            }
        }

        let mut output = RgbImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let val = buffer[(y * width + x) as usize].clamp(0.0, 255.0) as u8;
                output.put_pixel(x, y, Rgb([val, val, val]));
            }
        }
        DynamicImage::ImageRgb8(output)
    }
}

impl ImageEffect for HalfTone {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        let gray = image.to_luma8();
        let width = gray.width();
        let height = gray.height();
        let mut output = RgbImage::new(width, height);

        // Initialize with white
        for p in output.pixels_mut() {
            *p = Rgb([255, 255, 255]);
        }

        let cell_size = 6;

        for y_cell in 0..(height / cell_size) {
            for x_cell in 0..(width / cell_size) {
                let base_x = x_cell * cell_size;
                let base_y = y_cell * cell_size;

                let mut sum = 0u32;
                let mut count = 0u32;

                // Calculate average brightness
                for y in 0..cell_size {
                    for x in 0..cell_size {
                        if base_x + x < width && base_y + y < height {
                            sum += gray.get_pixel(base_x + x, base_y + y)[0] as u32;
                            count += 1;
                        }
                    }
                }

                if count == 0 {
                    continue;
                }
                let avg = sum / count;

                // Calculate radius based on darkness (inverted brightness)
                // 0 (black) -> max radius, 255 (white) -> 0 radius
                let max_radius = (cell_size as f32) / 1.3;
                let radius = max_radius * (1.0 - (avg as f32 / 255.0));
                let radius_sq = radius * radius;

                let center_x = base_x as f32 + (cell_size as f32 / 2.0);
                let center_y = base_y as f32 + (cell_size as f32 / 2.0);

                // Draw dot
                for y in 0..cell_size {
                    for x in 0..cell_size {
                        let px = base_x + x;
                        let py = base_y + y;
                        if px < width && py < height {
                            let dx = px as f32 + 0.5 - center_x;
                            let dy = py as f32 + 0.5 - center_y;
                            if dx * dx + dy * dy <= radius_sq {
                                output.put_pixel(px, py, Rgb([0, 0, 0]));
                            }
                        }
                    }
                }
            }
        }
        DynamicImage::ImageRgb8(output)
    }
}

impl ImageEffect for Threshold {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        let gray = image.to_luma8();
        let mut output = RgbImage::new(gray.width(), gray.height());
        let threshold = 128;

        for (x, y, pixel) in gray.enumerate_pixels() {
            let val = if pixel[0] > threshold { 255 } else { 0 };
            output.put_pixel(x, y, Rgb([val, val, val]));
        }
        DynamicImage::ImageRgb8(output)
    }
}

impl ImageEffect for Posterize {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        let gray = image.to_luma8();
        let mut output = RgbImage::new(gray.width(), gray.height());
        let levels = 4;
        let step = 255 / (levels - 1);

        for (x, y, pixel) in gray.enumerate_pixels() {
            let val = pixel[0];
            // Integer division handles the quantization
            let new_val = (val / step) * step;
            output.put_pixel(x, y, Rgb([new_val, new_val, new_val]));
        }
        DynamicImage::ImageRgb8(output)
    }
}

pub fn get_effect(effect: &crate::state::ImageEffect) -> Box<dyn ImageEffect> {
    match effect {
        crate::state::ImageEffect::Dither => Box::new(Dither),
        crate::state::ImageEffect::HalfTone => Box::new(HalfTone),
        crate::state::ImageEffect::Threshold => Box::new(Threshold),
        crate::state::ImageEffect::Posterize => Box::new(Posterize),
        crate::state::ImageEffect::Original => Box::new(Original),
    }
}
