use image::Rgba;

pub struct ColorProcessor<'a> {
    colors: &'a [Rgba<u8>],
}

impl<'a> ColorProcessor<'a> {
    pub fn new(colors: &'a [Rgba<u8>]) -> Self {
        Self { colors }
    }

    pub fn process_pixel(&self, pixel: &Rgba<u8>) -> Rgba<u8> {
        // Early return for fully transparent pixels
        if pixel[3] == 0 {
            return *pixel;
        }

        let gray = self.to_grayscale(pixel);

        // Uncomment this for multi-color processing or use the current approach
        match self.colors.len() {
            0 => *pixel, // Return original if no colors defined
            1 => self.monotone(gray),
            2 => self.duotone(gray),
            _ => self.multi_tone(gray),
        }
    }

    fn to_grayscale(&self, pixel: &Rgba<u8>) -> f32 {
        // Using proper luminance weights with gamma correction
        let r = (pixel[0] as f32 / 255.0).powf(2.2);
        let g = (pixel[1] as f32 / 255.0).powf(2.2);
        let b = (pixel[2] as f32 / 255.0).powf(2.2);

        // Calculate luminance
        let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;

        // Apply gamma correction
        luminance.powf(1.0 / 2.2)
    }

    fn monotone(&self, gray: f32) -> Rgba<u8> {
        let white = Rgba([255, 255, 255, 255]);
        self.blend_colors(&self.colors[0], &white, gray)
    }

    fn duotone(&self, gray: f32) -> Rgba<u8> {
        self.blend_colors(&self.colors[0], &self.colors[1], gray)
    }

    fn multi_tone(&self, gray: f32) -> Rgba<u8> {
        // For 3 or more colors, determine which segment the gray value falls into
        let segments = self.colors.len() - 1;
        let segment_size = 1.0 / segments as f32;

        let segment_index = (gray * segments as f32).floor() as usize;
        let segment_index = segment_index.min(segments - 1); // Ensure we don't exceed bounds

        let segment_start = segment_index as f32 * segment_size;
        let segment_ratio = (gray - segment_start) / segment_size;

        self.blend_colors(
            &self.colors[segment_index],
            &self.colors[segment_index + 1],
            segment_ratio,
        )
    }

    fn blend_colors(&self, color1: &Rgba<u8>, color2: &Rgba<u8>, ratio: f32) -> Rgba<u8> {
        // Ensure ratio is in the valid range
        let ratio = ratio.max(0.0).min(1.0);

        let blend = |a: u8, b: u8| {
            // Linear interpolation in gamma-corrected space for better perceptual results
            let a_linear = (a as f32 / 255.0).powf(2.2);
            let b_linear = (b as f32 / 255.0).powf(2.2);
            let mixed = (1.0 - ratio) * a_linear + ratio * b_linear;

            // Convert back to sRGB space
            (mixed.powf(1.0 / 2.2) * 255.0).round().min(255.0) as u8
        };

        // Blend the alpha channel as well for proper transparency handling
        let alpha = (1.0 - ratio) * (color1[3] as f32) + ratio * (color2[3] as f32);

        Rgba([
            blend(color1[0], color2[0]),
            blend(color1[1], color2[1]),
            blend(color1[2], color2[2]),
            alpha.round().min(255.0) as u8,
        ])
    }
}
