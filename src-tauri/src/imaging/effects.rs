use image::DynamicImage;

pub trait ImageEffect {
    fn apply(&self, image: &DynamicImage) -> DynamicImage;
}

pub struct Dither;
pub struct HalfTone;
pub struct Threshold;
pub struct Posterize;

impl ImageEffect for Dither {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        // TODO: Implement Dither effect
        image.clone()
    }
}

impl ImageEffect for HalfTone {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        // TODO: Implement HalfTone effect
        image.clone()
    }
}

impl ImageEffect for Threshold {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        // TODO: Implement Threshold effect
        image.clone()
    }
}

impl ImageEffect for Posterize {
    fn apply(&self, image: &DynamicImage) -> DynamicImage {
        // TODO: Implement Posterize effect
        image.clone()
    }
}

pub fn get_effect(effect: &crate::state::ImageEffect) -> Box<dyn ImageEffect> {
    match effect {
        crate::state::ImageEffect::Dither => Box::new(Dither),
        crate::state::ImageEffect::HalfTone => Box::new(HalfTone),
        crate::state::ImageEffect::Threshold => Box::new(Threshold),
        crate::state::ImageEffect::Posterize => Box::new(Posterize),
        crate::state::ImageEffect::Original => Box::new(Dither),
    }
}
