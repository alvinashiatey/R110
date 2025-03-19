use super::effects::get_effect;
use super::filters::get_filter;
use super::treatment::ImageTreatment;
use crate::errors::Error;
use crate::state::AppStateInner;
use image::{open, DynamicImage, ImageFormat};
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

    fn apply_filter(mut self, filter: Option<&crate::state::ImageFilter>) -> Self {
        if let Some(filter_type) = filter {
            let filter = get_filter(filter_type);
            self.image = filter.apply(&self.image);
        }
        self
    }

    fn apply_effect(mut self, effect: Option<&crate::state::ImageEffect>) -> Self {
        if let Some(effect_type) = effect {
            let effect = get_effect(effect_type);
            self.image = effect.apply(&self.image);
        }
        self
    }

    fn separate_channels(mut self) -> Result<Self, Error> {
        let channels = ImageTreatment::new(&self.image)?
            .process()?
            .into_iter()
            .map(|channel| DynamicImage::ImageRgba8(channel))
            .collect();

        self.processed_images = channels;
        Ok(self)
    }

    fn save(self, filename: &str) -> Result<Vec<ProcessResult>, Error> {
        let temp_dir = env::temp_dir();
        let output_path = temp_dir.join(filename);
        let mut results: Vec<ProcessResult> = vec![];

        for (i, img) in self.processed_images.iter().enumerate() {
            let channel = match i {
                0 => "cyan",
                1 => "magenta",
                2 => "yellow",
                3 => "black",
                _ => "unknown",
            };

            let channel_path = output_path.with_file_name(format!("{}_{}.png", channel, i));
            let file = File::create(&channel_path)
                .map_err(|e| Error::Processing(format!("Failed to create temp file: {}", e)))?;
            let mut writer = BufWriter::new(file);

            img.write_to(&mut writer, ImageFormat::Png)
                .map_err(|e| Error::Processing(e.to_string()))?;

            let result = ProcessResult {
                channel: channel.to_string(),
                image_path: channel_path.to_string_lossy().to_string(),
            };

            results.push(result);
        }

        Ok(results)
    }
}

pub fn process_image(file_path: &str, state: &AppStateInner) -> Result<Vec<ProcessResult>, Error> {
    let img = open(file_path).map_err(|e| Error::Processing(e.to_string()))?;
    let filter = state
        .process_settings
        .as_ref()
        .and_then(|s| s.filter.as_ref());
    let effect = state
        .process_settings
        .as_ref()
        .and_then(|s| s.effect.as_ref());

    let timestamp = chrono::Local::now().timestamp();
    let filename = format!(
        "processed_{}_{}.png",
        timestamp,
        state.image_name.clone().unwrap_or_default()
    );

    ImageProcessor::new(img)
        .separate_channels()?
        .apply_filter(filter)
        .apply_effect(effect)
        .save(&filename)
}
