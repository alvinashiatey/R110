use std::sync::Mutex;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ImageEffect {
    Original,
    Dither,
    HalfTone,
    Threshold,
    Posterize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ImageFilter {
    Grayscale,
    Sepia,
    Invert,
    Pixelate,
    Brighten,
    Darken,
    Contrast,
    Blur,
    Sharpen,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessSettings {
    pub colors: Option<Vec<String>>,
    pub effect: Option<ImageEffect>,
    pub filter: Option<ImageFilter>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ProcessingStatus {
    Idle,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Default, Clone)]
pub struct AppStateInner {
    pub image_path: Option<String>,
    pub image_type: Option<String>,
    pub image_name: Option<String>,
    pub current_image: Option<String>,
    pub process_settings: Option<ProcessSettings>,
    pub processed_images: Option<Vec<crate::imaging::processes::ProcessResult>>,
    pub processing_status: ProcessingStatus,
}

impl Default for ProcessingStatus {
    fn default() -> Self {
        ProcessingStatus::Idle
    }
}

pub type AppState = Mutex<AppStateInner>;

pub fn create_state() -> AppState {
    Mutex::new(AppStateInner::default())
}
