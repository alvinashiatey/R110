use super::effects::get_effect;
use super::filters::get_filter;
use super::treatment::ImageTreatment;
use crate::errors::Error;
use crate::state::AppStateInner;
use image::{open, DynamicImage, ImageFormat};
use std::env;
use std::fs::File;
use std::io::BufWriter;

struct ImageProcessor {
    image: DynamicImage,
}

impl ImageProcessor {
    fn new(image: DynamicImage) -> Self {
        Self { image }
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

    fn apply_color_treatment(mut self, colors: Option<&Vec<String>>) -> Result<Self, Error> {
        if let Some(colors) = colors {
            if !colors.is_empty() {
                let treatment = ImageTreatment::new(self.image)?;
                self.image = DynamicImage::ImageRgba8(treatment.process()?);
            }
        }
        Ok(self)
    }

    fn save(self, filename: &str) -> Result<String, Error> {
        let temp_dir = env::temp_dir();
        let output_path = temp_dir.join(filename);

        let file = File::create(&output_path)
            .map_err(|e| Error::Processing(format!("Failed to create temp file: {}", e)))?;
        let mut writer = BufWriter::new(file);

        self.image
            .write_to(&mut writer, ImageFormat::Png)
            .map_err(|e| Error::Processing(e.to_string()))?;

        Ok(output_path.to_string_lossy().to_string())
    }
}

pub fn process_image(file_path: &str, state: &AppStateInner) -> Result<String, Error> {
    let img = open(file_path).map_err(|e| Error::Processing(e.to_string()))?;
    let filter = state
        .process_settings
        .as_ref()
        .and_then(|s| s.filter.as_ref());
    let effect = state
        .process_settings
        .as_ref()
        .and_then(|s| s.effect.as_ref());
    let colors = state
        .process_settings
        .as_ref()
        .and_then(|s| s.colors.as_ref());

    let timestamp = chrono::Local::now().timestamp();
    let filename = format!(
        "processed_{}_{}.png",
        timestamp,
        state.image_name.clone().unwrap_or_default()
    );

    ImageProcessor::new(img)
        .apply_color_treatment(colors)?
        .apply_filter(filter)
        .apply_effect(effect)
        .save(&filename)
}
