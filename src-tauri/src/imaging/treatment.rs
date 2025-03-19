use crate::errors::Error;
use crate::imaging::cmyk::{split_channels_new, CmykChannel};
use image::{DynamicImage, RgbaImage};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct ImageTreatment {
    image: DynamicImage,
}

impl ImageTreatment {
    pub fn new(image: DynamicImage) -> Result<Self> {
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

    pub fn process(&self) -> Result<RgbaImage> {
        Ok(self.one_color())
    }
}
