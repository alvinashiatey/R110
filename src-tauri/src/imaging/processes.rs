use super::colormap::ColorMap;
use super::effects::get_effect;
use super::filters::get_filter;
use super::treatment::ImageTreatment;
use crate::errors::Error;
use crate::state::AppStateInner;
use image::{open, DynamicImage, ImageBuffer, Rgb};
use std::env;
use std::fs::File;
use std::io::BufWriter;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProcessResult {
    pub channel: String,
    pub image_path: String,
}

struct ImageProcessor {
    image: DynamicImage,
    processed_images: Vec<DynamicImage>,
}

impl ImageProcessor {
    fn new(image: DynamicImage) -> Self {
        Self {
            image,
            processed_images: vec![],
        }
    }

    fn from_channels(channels: &[ProcessResult]) -> Result<Self, Error> {
        let mut images = vec![];
        for channel in channels {
            let img = open(&channel.image_path).map_err(|e| Error::Processing(e.to_string()))?;
            images.push(img);
        }

        // Use the first channel as the base image (it won't be used for filtering anyway)
        let base = images
            .first()
            .ok_or_else(|| Error::Processing("No channels found".to_string()))?
            .clone();

        Ok(Self {
            image: base,
            processed_images: images,
        })
    }

    fn apply_filter(mut self, filter: Option<&crate::state::ImageFilter>) -> Self {
        if let Some(filter_type) = filter {
            let filter = get_filter(filter_type);
            self.image = filter.apply(&self.image);
        }
        self
    }

    fn apply_effect_to_channels(mut self, effect: Option<&crate::state::ImageEffect>) -> Self {
        if let Some(effect_type) = effect {
            let effect = get_effect(effect_type);
            self.processed_images = self
                .processed_images
                .iter()
                .map(|img| effect.apply(img))
                .collect();
        }
        self
    }

    fn separate_channels(mut self) -> Result<Self, Error> {
        let channels = ImageTreatment::new(&self.image)?
            .process()?
            .into_iter()
            .map(|channel| DynamicImage::ImageRgb8(channel))
            .collect();

        self.processed_images = channels;
        Ok(self)
    }

    fn save_jpeg_with_quality(
        img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        filepath: &str,
        quality: u8,
    ) -> Result<(), Error> {
        let file = File::create(filepath)?;
        let mut writer = BufWriter::new(file);

        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut writer, quality);
        encoder.encode(
            img.as_raw(),
            img.width(),
            img.height(),
            image::ExtendedColorType::Rgb8,
        )?;

        Ok(())
    }

    fn save(self, filename: &str) -> Result<Vec<ProcessResult>, Error> {
        let temp_dir = env::temp_dir();
        // Use the filename as a prefix to ensure uniqueness
        let prefix = std::path::Path::new(filename)
            .file_stem()
            .map(|s| s.to_string_lossy())
            .unwrap_or_else(|| "processed".into());

        let mut results: Vec<ProcessResult> = vec![];

        for (i, img) in self.processed_images.iter().enumerate() {
            let channel = match i {
                0 => "cyan",
                1 => "magenta",
                2 => "yellow",
                3 => "black",
                _ => "unknown",
            };

            // Create a unique filename for each channel using the prefix
            let channel_filename = format!("{}_{}_{}.jpeg", prefix, channel, i);
            let channel_path = temp_dir.join(channel_filename);

            ImageProcessor::save_jpeg_with_quality(
                img.as_rgb8().unwrap(),
                channel_path.to_str().unwrap(),
                70,
            )?;

            let result = ProcessResult {
                channel: channel.to_string(),
                image_path: channel_path.to_string_lossy().to_string(),
            };

            results.push(result);
        }

        Ok(results)
    }
}

pub fn process_image(
    file_path: &str,
    state: &AppStateInner,
    cached_channels: Option<&Vec<ProcessResult>>,
) -> Result<Vec<ProcessResult>, Error> {
    let img = open(file_path).map_err(|e| Error::Processing(e.to_string()))?;
    let filter = state
        .process_settings
        .as_ref()
        .and_then(|s| s.filter.as_ref());
    let effect = state
        .process_settings
        .as_ref()
        .and_then(|s| s.effect.as_ref());

    let timestamp = chrono::Local::now().timestamp_millis();
    let filename = format!(
        "processed_{}_{}.png",
        timestamp,
        state.image_name.clone().unwrap_or_default()
    );

    // If no filter is applied and we have cached channels, use them to skip separation
    if filter.is_none() {
        if let Some(channels) = cached_channels {
            return ImageProcessor::from_channels(channels)?
                .apply_effect_to_channels(effect)
                .save(&filename);
        }
    }

    ImageProcessor::new(img)
        .apply_filter(filter)
        .separate_channels()?
        .apply_effect_to_channels(effect)
        .save(&filename)
}

// New function for background processing
pub fn process_image_background(file_path: &str) -> Result<Vec<ProcessResult>, Error> {
    let img = open(file_path).map_err(|e| Error::Processing(e.to_string()))?;
    let timestamp = chrono::Local::now().timestamp_millis();
    let filename = format!("processed_{}.png", timestamp);

    // Only separate channels without filters/effects for the initial loading
    ImageProcessor::new(img)
        .separate_channels()?
        .save(&filename)
}

pub fn apply_colormap(file_path: &str, hex: &str) -> Result<String, Error> {
    let colormap = ColorMap::new(file_path.to_string(), hex.to_string());
    let processed_image = colormap.apply()?;
    Ok(processed_image.base64)
}
