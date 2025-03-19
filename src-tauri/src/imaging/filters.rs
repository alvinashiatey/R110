use image::{imageops, DynamicImage, GenericImageView};

pub trait ImageFilter {
    fn apply(&self, image: &DynamicImage) -> DynamicImage;
}

pub struct Grayscale;
pub struct Sepia;
pub struct Invert;
pub struct Pixelate;
pub struct Brighten;
pub struct Darken;
pub struct Contrast;
pub struct Blur;
pub struct Sharpen;

impl ImageFilter for Grayscale {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        image.grayscale()
    }
}

impl ImageFilter for Sepia {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        let mut img = image.to_rgba8();

        for pixel in img.pixels_mut() {
            let red = pixel[0] as f32;
            let green = pixel[1] as f32;
            let blue = pixel[2] as f32;

            let new_red = (0.393 * red + 0.769 * green + 0.189 * blue).min(255.0);
            let new_green = (0.349 * red + 0.686 * green + 0.168 * blue).min(255.0);
            let new_blue = (0.272 * red + 0.534 * green + 0.131 * blue).min(255.0);

            pixel[0] = new_red as u8;
            pixel[1] = new_green as u8;
            pixel[2] = new_blue as u8;
        }

        DynamicImage::ImageRgba8(img)
    }
}

impl ImageFilter for Invert {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        let mut img = image.to_rgba8();

        for pixel in img.pixels_mut() {
            pixel[0] = 255 - pixel[0];
            pixel[1] = 255 - pixel[1];
            pixel[2] = 255 - pixel[2];
        }

        DynamicImage::ImageRgba8(img)
    }
}

impl ImageFilter for Pixelate {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        let (width, height) = image.dimensions();
        let block_size = 10; // Adjustable pixelation factor

        let scaled_down = image.resize(
            width / block_size,
            height / block_size,
            imageops::FilterType::Nearest,
        );

        scaled_down.resize(width, height, imageops::FilterType::Nearest)
    }
}

impl ImageFilter for Brighten {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        image.brighten(30) // Positive value to brighten
    }
}

impl ImageFilter for Darken {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        image.brighten(-30) // Negative value to darken
    }
}

impl ImageFilter for Contrast {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        image.adjust_contrast(25.0) // Positive value increases contrast
    }
}

impl ImageFilter for Blur {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        image.blur(3.0) // Gaussian blur with sigma = 3.0
    }
}

impl ImageFilter for Sharpen {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        let img = image.clone();
        imageops::filter3x3(
            &image.to_rgb8(),
            &[-1.0, -1.0, -1.0, -1.0, 9.0, -1.0, -1.0, -1.0, -1.0],
        );
        img
    }
}

pub fn get_filter(filter_type: &crate::state::ImageFilter) -> Box<dyn ImageFilter> {
    match filter_type {
        crate::state::ImageFilter::Grayscale => Box::new(Grayscale),
        crate::state::ImageFilter::Sepia => Box::new(Sepia),
        crate::state::ImageFilter::Invert => Box::new(Invert),
        crate::state::ImageFilter::Pixelate => Box::new(Pixelate),
        crate::state::ImageFilter::Brighten => Box::new(Brighten),
        crate::state::ImageFilter::Darken => Box::new(Darken),
        crate::state::ImageFilter::Contrast => Box::new(Contrast),
        crate::state::ImageFilter::Blur => Box::new(Blur),
        crate::state::ImageFilter::Sharpen => Box::new(Sharpen),
    }
}
