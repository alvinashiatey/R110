use image::ImageError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Image(#[from] ImageError),

    #[error("No image selected")]
    NoImageSelected,

    #[error("Image processing error: {0}")]
    Processing(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
