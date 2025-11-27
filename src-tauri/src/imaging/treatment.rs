use crate::errors::Error;
use crate::imaging::cmyk::{split_channels, CmykChannels};
use image::{DynamicImage, RgbImage};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct ImageTreatment<'a> {
    image: &'a DynamicImage,
}

impl<'a> ImageTreatment<'a> {
    pub fn new(image: &'a DynamicImage) -> Result<Self> {
        Ok(Self { image })
    }

    pub fn process_channel(&self) -> Option<Vec<RgbImage>> {
        let channels = split_channels(&self.image, CmykChannels::ALL);
        if let Some(channels) = channels {
            return Some(channels);
        }
        None
    }

    pub fn process(&self) -> Result<Vec<RgbImage>> {
        let processed_channels = self.process_channel();
        if let Some(channels) = processed_channels {
            return Ok(channels);
        }
        Err(Error::Processing("Failed to process image".to_string()))
    }
}
