use crate::errors::Error;
use crate::imaging::cmyk::{split_channels_new, CmykChannel};
use image::{DynamicImage, RgbaImage};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct ImageTreatment<'a> {
    image: &'a DynamicImage,
}

impl<'a> ImageTreatment<'a> {
    pub fn new(image: &'a DynamicImage) -> Result<Self> {
        Ok(Self { image })
    }

    fn one_color(&self) -> RgbaImage {
        let channel = CmykChannel::Cyan;
        if let Some(channels) = split_channels_new(&self.image, channel) {
            if !channels.is_empty() {
                return channels[0].clone();
            }
        }
        RgbaImage::new(self.image.width(), self.image.height())
    }

    pub fn process_channel(&self) -> Option<Vec<RgbaImage>> {
        let channel =
            CmykChannel::Cyan | CmykChannel::Magenta | CmykChannel::Yellow | CmykChannel::Black;
        let channels = split_channels_new(&self.image, channel);
        if let Some(channels) = channels {
            return Some(channels);
        }
        None
    }

    pub fn process(&self) -> Result<Vec<RgbaImage>> {
        let processed_channels = self.process_channel();
        if let Some(channels) = processed_channels {
            return Ok(channels);
        }
        Err(Error::Processing("Failed to process image".to_string()))
    }
}
